use super::*;
use serde_json::Value;
fn place(s: &mut Session, programme: u16, start_tick: u32, advertiser: Option<u16>) -> u32 {
    let previous = s
        .report()
        .schedule
        .iter()
        .map(|a| a.airing_id)
        .collect::<Vec<_>>();
    s.execute(Command::Place {
        programme,
        start_tick,
        advertiser,
    })
    .unwrap()
    .schedule
    .iter()
    .find(|a| !previous.contains(&a.airing_id))
    .unwrap()
    .airing_id
}
fn quote(s: &mut Session, id: u32) -> String {
    s.execute(Command::RequestOffer { airing_id: id })
        .unwrap()
        .offer
        .unwrap()
        .offer_id
}
fn reject(s: &mut Session, command: Command, code: &str) {
    let state = format!("{:?}", s.planner);
    let before = s.report();
    let counter = s.next_offer;
    assert_eq!(s.execute(command).unwrap_err().code, code);
    assert_eq!(before, s.report());
    assert_eq!(state, format!("{:?}", s.planner));
    assert_eq!(counter, s.next_offer);
}

#[test]
fn place_move_remove_ids_and_overlap_are_atomic() {
    let mut s = Session::new().unwrap();
    let a = place(&mut s, 1, 72, None);
    let b = place(&mut s, 2, 74, None);
    assert_eq!(
        s.report()
            .schedule
            .iter()
            .find(|a| a.airing_id == b)
            .unwrap()
            .end_tick,
        77
    );
    reject(
        &mut s,
        Command::Move {
            airing_id: b,
            start_tick: 73,
        },
        "overlap",
    );
    reject(
        &mut s,
        Command::Place {
            programme: 1,
            start_tick: 71,
            advertiser: None,
        },
        "overlap",
    );
    reject(
        &mut s,
        Command::Place {
            programme: 999,
            start_tick: 80,
            advertiser: None,
        },
        "unknown_programme",
    );
    reject(
        &mut s,
        Command::Place {
            programme: 1,
            start_tick: 80,
            advertiser: Some(999),
        },
        "unknown_advertiser",
    );
    reject(
        &mut s,
        Command::Place {
            programme: 5,
            start_tick: 670,
            advertiser: None,
        },
        "invalid_time",
    );
    let rival = s
        .report()
        .schedule
        .iter()
        .find(|a| a.station != PLAYER.0)
        .unwrap()
        .airing_id;
    reject(
        &mut s,
        Command::Remove { airing_id: rival },
        "competitor_read_only",
    );
    s.execute(Command::Move {
        airing_id: b,
        start_tick: 80,
    })
    .unwrap();
    s.execute(Command::Remove { airing_id: a }).unwrap();
    let c = place(&mut s, 1, 72, None);
    assert_ne!(a, c);
    assert_eq!(s.report().next_airing, Some(c));
    assert!(s
        .report()
        .gaps
        .iter()
        .any(|g| g.start_tick == 74 && g.end_tick == 80));
}

#[test]
fn offers_and_commitments_bind_to_selected_identity_not_next_airing() {
    let mut s = Session::new().unwrap();
    let first = place(&mut s, 1, 72, None);
    let selected = place(&mut s, 2, 80, Some(2));
    let token = quote(&mut s, selected);
    let report = s.report();
    assert_eq!(report.next_airing, Some(first));
    assert_eq!(report.offer.unwrap().airing_id, selected);
    reject(
        &mut s,
        Command::AcceptOffer {
            airing_id: first,
            offer_id: token.clone(),
        },
        "stale_offer",
    );
    s.execute(Command::AcceptOffer {
        airing_id: selected,
        offer_id: token.clone(),
    })
    .unwrap();
    assert_eq!(s.report().contracts[0].airing_id, selected);
    reject(
        &mut s,
        Command::AcceptOffer {
            airing_id: selected,
            offer_id: token,
        },
        "stale_offer",
    );
    reject(
        &mut s,
        Command::Move {
            airing_id: selected,
            start_tick: 84,
        },
        "booking_locked",
    );
    reject(
        &mut s,
        Command::Remove {
            airing_id: selected,
        },
        "booking_locked",
    );
    reject(
        &mut s,
        Command::SetAdvertiser {
            airing_id: selected,
            advertiser: None,
        },
        "booking_locked",
    );
    s.execute(Command::Advance { to_tick: 74 }).unwrap();
    assert!(s.report().contracts[0].settlement.is_none());
    let report = s.execute(Command::Advance { to_tick: 83 }).unwrap();
    assert_eq!(
        report
            .results
            .iter()
            .map(|r| r.airing_id)
            .collect::<Vec<_>>(),
        vec![first, selected]
    );
    assert!(report.contracts[0].settlement.is_some());
    assert_eq!(report.active_airing, None);
    assert_eq!(report.finances.costs_paid, "200");
    let settlement = report.contracts[0].settlement.as_ref().unwrap();
    assert_eq!(
        settlement.payment.parse::<u64>().unwrap() + settlement.refund.parse::<u64>().unwrap(),
        report.contracts[0].maximum_payment.parse::<u64>().unwrap()
    );
}

#[test]
fn stale_offers_edits_reset_and_session_isolation() {
    let mut a = Session::new().unwrap();
    let mut b = Session::new().unwrap();
    let id = place(&mut a, 1, 72, Some(1));
    let other = place(&mut b, 1, 72, Some(1));
    let old = quote(&mut a, id);
    let own = quote(&mut b, other);
    reject(
        &mut b,
        Command::AcceptOffer {
            airing_id: other,
            offer_id: old.clone(),
        },
        "stale_offer",
    );
    a.execute(Command::Move {
        airing_id: id,
        start_tick: 76,
    })
    .unwrap();
    reject(
        &mut a,
        Command::AcceptOffer {
            airing_id: id,
            offer_id: old,
        },
        "stale_offer",
    );
    let fresh = quote(&mut a, id);
    a.execute(Command::Reset {}).unwrap();
    let newid = place(&mut a, 1, 72, Some(1));
    assert_ne!(id, newid);
    reject(&mut a, Command::Remove { airing_id: id }, "unknown_airing");
    quote(&mut a, newid);
    reject(
        &mut a,
        Command::AcceptOffer {
            airing_id: newid,
            offer_id: fresh,
        },
        "stale_offer",
    );
    b.execute(Command::AcceptOffer {
        airing_id: other,
        offer_id: own,
    })
    .unwrap();
    b.execute(Command::Advance { to_tick: 74 }).unwrap();
    assert_eq!(a.report().now_tick, 0);
    assert!(a.report().results.is_empty());
    assert_eq!(a.report().cash, "20000");
}

#[test]
fn native_planner_parity_and_tick_vs_batch_reproducibility() {
    let mut a = Session::new().unwrap();
    let mut native = scenario::planner().unwrap();
    let id = place(&mut a, 1, 72, Some(1));
    let nativeid = native
        .place(ProgrammeId(1), SimTick(72), Some(BrandId(1)))
        .unwrap();
    assert_eq!(id, nativeid.0);
    let token = quote(&mut a, id);
    let nq = native.quote(nativeid).unwrap();
    native.accept(&nq).unwrap();
    a.execute(Command::AcceptOffer {
        airing_id: id,
        offer_id: token,
    })
    .unwrap();
    let mut individual = a.clone();
    a.execute(Command::Advance { to_tick: 96 }).unwrap();
    native.advance_to(SimTick(96)).unwrap();
    for tick in 1..=96 {
        individual
            .execute(Command::Advance { to_tick: tick })
            .unwrap();
    }
    assert_eq!(a.planner.state(), native.state());
    assert_eq!(a.report(), individual.report());
    for result in &a.report().results {
        for audience in &result.audiences {
            assert!(
                (audience.player_viewers + audience.rival_viewers + audience.not_watching
                    - f64::from(audience.population))
                .abs()
                    < 1e-8
            );
        }
    }
    assert_eq!(a.planner.state().economy().total_money(), 80000);
}

#[test]
fn malformed_commands_preserve_the_pending_quote_and_useful_errors() {
    let mut s = Session::new().unwrap();
    let id = place(&mut s, 1, 72, Some(1));
    let token = quote(&mut s, id);
    for invalid in [
        "{}",
        "not json",
        r#"{"api_version":4,"command":{"type":"advance","to_tick":80,"extra":true}}"#,
        r#"{"api_version":4,"command":{"type":"place","start_tick":-1,"programme":1}}"#,
        r#"{"api_version":4,"command":{"type":"place","start_tick":1.5,"programme":1}}"#,
        r#"{"api_version":4,"command":{"type":"remove","airing_id":4294967296}}"#,
    ] {
        let before = s.report();
        let response: Value = serde_json::from_str(&s.dispatch_json(invalid)).unwrap();
        assert_eq!(response["error"]["code"], "malformed_command");
        assert_eq!(s.report(), before);
    }
    let response: Value =
        serde_json::from_str(&s.dispatch_json(r#"{"api_version":1,"command":{"type":"inspect"}}"#))
            .unwrap();
    assert_eq!(response["error"]["code"], "unsupported_version");
    reject(&mut s, Command::Advance { to_tick: 673 }, "invalid_advance");
    s.execute(Command::AcceptOffer {
        airing_id: id,
        offer_id: token,
    })
    .unwrap();
}

fn audit(value: &Value) {
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                assert!(
                    ![
                        "execution",
                        "craft",
                        "subject_fit",
                        "preferred_tone",
                        "recognition",
                        "output",
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
                    "opening_cash",
                    "budget",
                    "fees",
                    "fee",
                    "production_committed",
                    "production_in_escrow",
                    "production_spent",
                    "costs_paid",
                    "revenue",
                    "amount",
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
                    text.parse::<u128>().unwrap();
                }
                audit(value);
            }
        }
        Value::Array(values) => {
            for value in values {
                audit(value);
            }
        }
        _ => (),
    }
}
#[test]
fn serialized_player_phases_hide_internal_state_and_preserve_money_strings() {
    let mut s = Session::new().unwrap();
    audit(&serde_json::to_value(s.report()).unwrap());
    let id = place(&mut s, 1, 72, Some(1));
    let token = quote(&mut s, id);
    audit(&serde_json::to_value(s.report()).unwrap());
    s.execute(Command::AcceptOffer {
        airing_id: id,
        offer_id: token,
    })
    .unwrap();
    audit(&serde_json::to_value(s.report()).unwrap());
    s.execute(Command::Advance { to_tick: 96 }).unwrap();
    audit(&serde_json::to_value(s.report()).unwrap());
    let mut report = s.report();
    report.cash = u64::MAX.to_string();
    let encoded = serde_json::to_value(report).unwrap();
    assert_eq!(encoded["cash"], "18446744073709551615");
    audit(&encoded);
    let mut hidden = Session::new().unwrap();
    let mut w = scenario::world();
    for p in &mut w.programmes {
        p.execution = 0.123456789;
    }
    hidden.planner =
        Planner::new(w, bts_sim::Parameters::default(), PLAYER, SimTick(672), &[]).unwrap();
    // Catalogue observation is independent of hidden execution.
    assert_eq!(
        hidden.report().catalogue,
        Session::new().unwrap().report().catalogue
    );
}

#[test]
fn varied_evening_results_unaccepted_ads_gaps_and_week_completion() {
    let mut s = Session::new().unwrap();
    for (programme, tick) in [(1, 72), (2, 74), (5, 80)] {
        place(&mut s, programme, tick, Some(1));
    }
    s.execute(Command::Advance { to_tick: 96 }).unwrap();
    let report = s.report();
    assert_eq!(report.results.len(), 3);
    assert!(report.results.iter().all(|r| r.aired_advertiser.is_none()));
    assert!(report.contracts.is_empty());
    assert_eq!(report.finances.costs_paid, "300");
    let immutable = report.results;
    s.execute(Command::Advance { to_tick: 672 }).unwrap();
    assert_eq!(s.report().results, immutable);
    assert_eq!(s.report().now_tick, s.report().end_tick);
    reject(&mut s, Command::Advance { to_tick: 673 }, "invalid_advance");
    s.execute(Command::Reset {}).unwrap();
    assert_eq!(s.report().now_tick, 0);
    assert_eq!(s.report().cash, "20000");
    assert!(s.report().results.is_empty());
}

#[test]
fn an_unpayable_quote_does_not_replace_a_valid_pending_offer() {
    let mut s = Session::new().unwrap();
    let evening = place(&mut s, 1, 72, Some(1));
    let overnight = place(&mut s, 1, 0, Some(1));
    let no_ad = place(&mut s, 1, 80, None);
    let token = quote(&mut s, evening);
    reject(
        &mut s,
        Command::RequestOffer {
            airing_id: overnight,
        },
        "no_payable_offer",
    );
    reject(
        &mut s,
        Command::RequestOffer { airing_id: no_ad },
        "no_advertiser",
    );
    s.execute(Command::AcceptOffer {
        airing_id: evening,
        offer_id: token,
    })
    .unwrap();
}

fn production_concept() -> ProductionConceptInput {
    ProductionConceptInput {
        title: "Workshop Conversations".into(),
        host: 1,
        guests: [3, 4],
        subject: bts_sim::planning::production::Subject::Science,
        tone: bts_sim::planning::production::Tone::Calm,
        budget: "1000".into(),
        record_at: 40,
    }
}
#[test]
fn production_reports_remain_whitelisted_and_money_is_not_charged_twice() {
    let mut s = Session::new().unwrap();
    let report = s
        .execute(Command::CreateProduction {
            concept: production_concept(),
        })
        .unwrap();
    let id = report.production.projects[0].id;
    assert_eq!(report.cash, "20000");
    let booked = s
        .execute(Command::BookProduction {
            production_id: id,
            promise_request: Some(1),
            exceptional_person: None,
            smooth_cooperation: false,
        })
        .unwrap();
    let work = booked.production.projects[0].programme.unwrap();
    assert_eq!(booked.cash, "19000");
    assert_eq!(booked.finances.production_in_escrow, "1000");
    audit(&serde_json::to_value(booked).unwrap());
    reject(
        &mut s,
        Command::Place {
            programme: work,
            start_tick: 72,
            advertiser: None,
        },
        "production_not_ready",
    );
    let recorded = s.execute(Command::Advance { to_tick: 44 }).unwrap();
    assert_eq!(recorded.cash, "19000");
    assert_eq!(recorded.finances.production_spent, "1000");
    assert_eq!(recorded.finances.production_in_escrow, "0");
    assert_eq!(recorded.production.sway_balance, 4);
    audit(&serde_json::to_value(recorded).unwrap());
    let ready = s.execute(Command::Advance { to_tick: 52 }).unwrap();
    assert!(ready
        .catalogue
        .iter()
        .any(|p| p.id == work && p.production_id == Some(id)));
    audit(&serde_json::to_value(ready).unwrap());
    let airing = place(&mut s, work, 72, Some(2));
    let token = quote(&mut s, airing);
    s.execute(Command::AcceptOffer {
        airing_id: airing,
        offer_id: token,
    })
    .unwrap();
    let report = s.execute(Command::Advance { to_tick: 75 }).unwrap();
    audit(&serde_json::to_value(&report).unwrap());
    assert_eq!(report.finances.costs_paid, "50");
    assert_eq!(report.finances.production_committed, "1000");
    assert_eq!(report.finances.production_spent, "1000");
    assert_eq!(
        report.cash.parse::<u64>().unwrap(),
        20000 + report.finances.revenue.parse::<u64>().unwrap() - 1050
    );
    assert!(report
        .finances
        .ledger
        .iter()
        .any(|l| l.production_id == Some(id) && l.kind == "production_spend"));
}
#[test]
fn production_invalid_commands_reset_ids_and_pooled_sway_isolation() {
    let mut a = Session::new().unwrap();
    let mut b = Session::new().unwrap();
    let id = a
        .execute(Command::CreateProduction {
            concept: production_concept(),
        })
        .unwrap()
        .production
        .projects[0]
        .id;
    a.execute(Command::BookProduction {
        production_id: id,
        promise_request: Some(1),
        exceptional_person: None,
        smooth_cooperation: false,
    })
    .unwrap();
    a.execute(Command::Advance { to_tick: 52 }).unwrap();
    assert!(b.report().production.projects.is_empty());
    assert!(b.report().production.sway_balance == 0);
    reject(
        &mut b,
        Command::BookProduction {
            production_id: id,
            promise_request: None,
            exceptional_person: Some(5),
            smooth_cooperation: false,
        },
        "unknown_production",
    );
    a.execute(Command::Reset {}).unwrap();
    assert!(a.report().production.projects.is_empty());
    assert!(a.report().production.sway_balance == 0);
    assert_eq!(a.report().cash, "20000");
    let next = a
        .execute(Command::CreateProduction {
            concept: production_concept(),
        })
        .unwrap()
        .production
        .projects[0]
        .id;
    assert_ne!(id, next);
    reject(
        &mut a,
        Command::BookProduction {
            production_id: id,
            promise_request: Some(1),
            exceptional_person: None,
            smooth_cooperation: false,
        },
        "unknown_production",
    );
    for json in [
        r#"{"api_version":4,"command":{"type":"create_production","concept":{"title":"Test","host":1,"guests":[3,4],"subject":"science","tone":"calm","budget":9007199254740993,"record_at":40}}}"#,
        r#"{"api_version":4,"command":{"type":"create_production","concept":{"title":"Test","host":1,"guests":[3,4],"subject":"unknown","tone":"calm","budget":"1000","record_at":40}}}"#,
    ] {
        let before = a.report();
        let response: Value = serde_json::from_str(&a.dispatch_json(json)).unwrap();
        assert_eq!(response["error"]["code"], "malformed_command");
        assert_eq!(before, a.report());
    }
    let mut bad = production_concept();
    bad.budget = "18446744073709551616".into();
    reject(
        &mut a,
        Command::CreateProduction { concept: bad },
        "invalid_budget",
    );
}

#[test]
fn old_redemption_fields_are_rejected_and_failed_sway_spending_preserves_quotes() {
    let mut s = Session::new().unwrap();
    let project = s
        .execute(Command::CreateProduction {
            concept: production_concept(),
        })
        .unwrap()
        .production
        .projects[0]
        .id;
    let airing = place(&mut s, 1, 72, Some(1));
    let token = quote(&mut s, airing);
    reject(
        &mut s,
        Command::BookProduction {
            production_id: project,
            promise_request: Some(1),
            exceptional_person: None,
            smooth_cooperation: true,
        },
        "insufficient_sway",
    );
    let before = s.report();
    let result: Value = serde_json::from_str(&s.dispatch_json(r#"{"api_version":4,"command":{"type":"book_production","production_id":1,"promise_request":1,"call_favour":1,"smooth_cooperation":false}}"#)).unwrap();
    assert_eq!(result["error"]["code"], "malformed_command");
    assert_eq!(before, s.report());
    let result: Value =
        serde_json::from_str(&s.dispatch_json(r#"{"api_version":3,"command":{"type":"inspect"}}"#))
            .unwrap();
    assert_eq!(result["error"]["code"], "unsupported_version");
    s.execute(Command::AcceptOffer {
        airing_id: airing,
        offer_id: token,
    })
    .unwrap();
}
