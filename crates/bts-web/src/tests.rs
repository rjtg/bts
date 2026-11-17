use super::*;
use serde_json::{json, Value};

fn plan(session: &mut Session, programme: u16, advertiser: Option<u16>) -> PlayerReport {
    session
        .execute(Command::Plan {
            slot: session.cursor as u32,
            programme,
            advertiser,
        })
        .unwrap()
}
fn quote(session: &mut Session) -> String {
    session
        .execute(Command::RequestOffer {})
        .unwrap()
        .offer
        .unwrap()
        .offer_id
}
fn reject(session: &mut Session, command: Command, code: &str) {
    let before = format!("{:?}", session.simulation.state());
    let observation = session.report();
    let token_counter = session.next_offer;
    assert_eq!(session.execute(command).unwrap_err().code, code);
    assert_eq!(session.report(), observation);
    assert_eq!(format!("{:?}", session.simulation.state()), before);
    assert_eq!(session.next_offer, token_counter);
}

#[test]
fn native_sequence_matches_direct_simulation_and_settlement() {
    let mut session = Session::new().unwrap();
    plan(&mut session, 1, Some(1));
    let mut native = Simulation::new(scenario::world(), Parameters::default()).unwrap();
    let slot = session.slot(true).unwrap();
    let offer = native.quote(&slot, PLAYER, BrandId(1), slot.at).unwrap();
    let contract = native.accept_offer(&offer).unwrap();
    let token = quote(&mut session);
    session
        .execute(Command::AcceptOffer { offer_id: token })
        .unwrap();
    let report = session.execute(Command::Advance {}).unwrap();
    let event = native.step(&slot).unwrap();
    assert_eq!(native.state(), session.simulation.state());
    assert_eq!(
        report.cash,
        native.state().economy().cash(PLAYER).unwrap().0.to_string()
    );
    assert_eq!(
        report.contracts[0],
        session.booking(native.state().economy().contract(contract).unwrap())
    );
    for (observed, internal) in report.results[0].audiences.iter().zip(event.allocations) {
        assert!(
            (observed.player_viewers + observed.rival_viewers + observed.not_watching
                - internal.population as f64)
                .abs()
                < 1e-8
        );
    }
    assert!(
        report.contracts[0]
            .settlement
            .as_ref()
            .unwrap()
            .payment
            .parse::<u64>()
            .unwrap()
            > 0
    );
}

#[test]
fn invalid_commands_are_atomic_and_keep_a_valid_offer_usable() {
    let mut session = Session::new().unwrap();
    reject(&mut session, Command::Advance {}, "plan_required");
    plan(&mut session, 1, Some(1));
    let token = quote(&mut session);
    reject(
        &mut session,
        Command::Plan {
            slot: 0,
            programme: 65535,
            advertiser: None,
        },
        "unknown_programme",
    );
    reject(
        &mut session,
        Command::Plan {
            slot: 14,
            programme: 1,
            advertiser: None,
        },
        "invalid_slot",
    );
    reject(
        &mut session,
        Command::Plan {
            slot: 0,
            programme: 1,
            advertiser: Some(99),
        },
        "unknown_advertiser",
    );
    for invalid in [
        "{}",
        "not json",
        r#"{"api_version":1,"command":{"type":"advance","extra":true}}"#,
        r#"{"api_version":1,"command":{"type":"plan","slot":0,"programme":-1}}"#,
        r#"{"api_version":1,"command":{"type":"plan","slot":0,"programme":1.5}}"#,
        r#"{"api_version":1,"command":{"type":"accept_offer","offer_id":9007199254740993}}"#,
        r#"{"api_version":1,"command":{"type":"plan","slot":4294967296,"programme":1}}"#,
    ] {
        let before = session.report();
        let response: Value = serde_json::from_str(&session.dispatch_json(invalid)).unwrap();
        assert_eq!(response["error"]["code"], "malformed_command");
        assert_eq!(session.report(), before);
    }
    let before = session.report();
    assert!(session
        .dispatch_json(r#"{"api_version":2,"command":{"type":"inspect"}}"#)
        .contains("unsupported_version"));
    assert_eq!(before, session.report());
    session
        .execute(Command::AcceptOffer { offer_id: token })
        .unwrap();
    reject(
        &mut session,
        Command::Plan {
            slot: 0,
            programme: 2,
            advertiser: Some(2),
        },
        "booking_locked",
    );
    reject(&mut session, Command::RequestOffer {}, "already_booked");
    session.execute(Command::Advance {}).unwrap();
    assert_eq!(session.report().contracts.len(), 1);
    reject(
        &mut session,
        Command::Plan {
            slot: 0,
            programme: 1,
            advertiser: None,
        },
        "invalid_slot",
    );
}

#[test]
fn stale_offers_reset_and_isolation() {
    let mut a = Session::new().unwrap();
    let mut b = Session::new().unwrap();
    plan(&mut a, 1, Some(1));
    plan(&mut b, 1, Some(1));
    let old = quote(&mut a);
    let b_token = quote(&mut b);
    reject(
        &mut b,
        Command::AcceptOffer {
            offer_id: old.clone(),
        },
        "stale_offer",
    );
    plan(&mut a, 2, Some(2));
    reject(
        &mut a,
        Command::AcceptOffer {
            offer_id: old.clone(),
        },
        "stale_offer",
    );
    let fresh = quote(&mut a);
    a.execute(Command::Reset {}).unwrap();
    assert_eq!(a.report(), Session::new().unwrap().report());
    plan(&mut a, 1, Some(1));
    let newest = quote(&mut a);
    assert_ne!(old, newest);
    assert_ne!(fresh, newest);
    reject(
        &mut a,
        Command::AcceptOffer { offer_id: fresh },
        "stale_offer",
    );
    b.execute(Command::AcceptOffer { offer_id: b_token })
        .unwrap();
    b.execute(Command::Advance {}).unwrap();
    assert_eq!(a.cursor, 0);
    assert_eq!(a.report().cash, "20000");
    assert_eq!(a.simulation.state().economy().contracts().count(), 0);
}

#[test]
fn seven_days_unbooked_ads_completion_and_reproducibility() {
    let mut a = Session::new().unwrap();
    let mut b = Session::new().unwrap();
    for index in 0..SLOT_COUNT {
        for s in [&mut a, &mut b] {
            plan(s, 1 + (index % 7) as u16, Some(1));
            s.execute(Command::Advance {}).unwrap();
        }
        assert_eq!(a.report(), b.report());
        assert_eq!(a.simulation.state(), b.simulation.state());
        assert!(a
            .report()
            .results
            .last()
            .unwrap()
            .aired_advertiser
            .is_none());
    }
    assert_eq!(a.report().next_slot, None);
    reject(&mut a, Command::Advance {}, "scenario_complete");
    reject(&mut a, Command::RequestOffer {}, "scenario_complete");
}

fn audit(value: &Value) {
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                assert!(
                    ![
                        "execution",
                        "genre_tastes",
                        "preferred_tone",
                        "availability",
                        "purchasing_power",
                        "product_interests",
                        "hype",
                        "current_hype",
                        "desired_hype",
                        "deficit",
                        "repetition",
                        "rules",
                        "trace",
                        "activation",
                        "effective_weight",
                        "utility",
                        "satisfaction",
                        "loyalty",
                        "familiarity",
                        "creative_quality",
                        "marketing_budget",
                        "rate_per_thousand",
                        "demand"
                    ]
                    .contains(&key.as_str()),
                    "leaked {key}"
                );
                if [
                    "cash",
                    "cash_after",
                    "broadcast_cost",
                    "maximum_payment",
                    "allocated_payment",
                    "payment",
                    "refund",
                    "contract_id",
                    "promised_units",
                    "credited_units",
                ]
                .contains(&key.as_str())
                {
                    let text = value.as_str().expect("lossless decimal string");
                    assert!(!text.is_empty() && text.bytes().all(|b| b.is_ascii_digit()));
                    text.parse::<u64>().unwrap();
                }
                audit(value);
            }
        }
        Value::Array(values) => {
            for v in values {
                audit(v);
            }
        }
        _ => (),
    }
}
#[test]
fn all_serialized_player_phases_are_whitelisted_and_finances_lossless() {
    let mut s = Session::new().unwrap();
    for command in [
        json!({"type":"inspect"}),
        json!({"type":"plan","slot":0,"programme":1,"advertiser":1}),
        json!({"type":"request_offer"}),
    ] {
        audit(
            &serde_json::from_str::<Value>(
                &s.dispatch_json(&json!({"api_version":1,"command":command}).to_string()),
            )
            .unwrap(),
        );
    }
    let token = s.report().offer.unwrap().offer_id;
    s.execute(Command::AcceptOffer { offer_id: token }).unwrap();
    audit(&serde_json::to_value(s.report()).unwrap());
    s.execute(Command::Advance {}).unwrap();
    audit(&serde_json::to_value(s.report()).unwrap());
    // Full u64 range remains textual, even beyond JavaScript's safe integer range.
    let mut report = s.report();
    report.cash = u64::MAX.to_string();
    let serialized = serde_json::to_value(report).unwrap();
    assert_eq!(serialized["cash"], "18446744073709551615");
    audit(&serialized);
    // Changing hidden content does not change a pre-broadcast catalogue observation.
    let mut hidden = Session::new().unwrap();
    let mut world = scenario::world();
    for p in &mut world.programmes {
        p.execution = 0.123456789;
    }
    hidden.simulation = Simulation::new(world, Parameters::default()).unwrap();
    assert_eq!(hidden.report(), Session::new().unwrap().report());
}

#[test]
fn no_payable_offer_and_missing_advertiser_do_not_change_session() {
    let mut s = Session::new().unwrap();
    plan(&mut s, 1, None);
    reject(&mut s, Command::RequestOffer {}, "no_advertiser");
    let mut world = scenario::world();
    world.brands[0].marketing_budget = bts_sim::economy::Money(0);
    s.simulation = Simulation::new(world, Parameters::default()).unwrap();
    plan(&mut s, 1, Some(1));
    reject(&mut s, Command::RequestOffer {}, "no_payable_offer");
}

#[test]
fn unaffordable_plan_reports_off_air_without_blocking_rival() {
    let mut s = Session::new().unwrap();
    // A low-cash fixture isolates the engine's funding path through the adapter.
    let mut world = scenario::world();
    world
        .stations
        .iter_mut()
        .find(|station| station.id == PLAYER)
        .unwrap()
        .initial_cash = bts_sim::economy::Money(0);
    s.simulation = Simulation::new(world, Parameters::default()).unwrap();
    let initial_money = s.simulation.state().economy().total_money();
    plan(&mut s, 1, Some(1));
    reject(&mut s, Command::RequestOffer {}, "no_payable_offer");
    let report = s.execute(Command::Advance {}).unwrap();
    let r = &report.results[0];
    assert!(r.off_air);
    assert!(r.aired_programme.is_none() && r.aired_advertiser.is_none());
    assert_eq!(r.broadcast_cost, "0");
    assert_eq!(r.cash_after, "0");
    assert!(r
        .audiences
        .iter()
        .all(|a| a.player_viewers == 0.0 && a.rival_viewers > 0.0));
    assert_eq!(s.simulation.state().economy().total_money(), initial_money);
    audit(&serde_json::to_value(report).unwrap());
}
