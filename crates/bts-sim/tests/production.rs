use bts_sim::economy::{Account, ContractStatus, LedgerReason, Money};
use bts_sim::planning::production::*;
use bts_sim::planning::{PlanError, Planner};
use bts_sim::{demo::*, *};
fn planner() -> Planner {
    let mut p = Planner::new(
        bts_sim::demo::world(),
        Parameters::default(),
        MAIN,
        SimTick(672),
        &[],
    )
    .unwrap();
    let (roster, request) = demo_roster();
    p.configure_production(roster, request).unwrap();
    p.configure_cooperation(demo_cooperation()).unwrap();
    p
}
fn concept() -> Concept {
    Concept {
        title: "Workshop Conversations".into(),
        host: PersonId(1),
        guests: [PersonId(3), PersonId(4)],
        subject: Subject::Science,
        tone: Tone::Calm,
        budget: Money(1000),
        record_at: SimTick(40),
    }
}
fn unchanged(p: &mut Planner, f: impl FnOnce(&mut Planner) -> Result<(), PlanError>, code: &str) {
    let before = format!("{p:?}");
    assert_eq!(f(p).unwrap_err().code, code);
    assert_eq!(before, format!("{p:?}"));
}

#[test]
fn complete_pipeline_reserves_spends_once_releases_work_then_broadcasts() {
    let mut p = planner();
    let total = p.state().economy().total_money();
    let id = p.create_production(concept()).unwrap();
    assert_eq!(p.state().economy().cash(MAIN), Some(Money(20000)));
    assert!(p.state().economy().ledger().is_empty());
    p.book_production(id, None, 0).unwrap();
    let work = p.production().project(id).unwrap().programme.unwrap();
    assert_eq!(p.state().economy().cash(MAIN), Some(Money(19000)));
    assert_eq!(p.state().economy().production_escrow(), 1000);
    assert_eq!(p.state().economy().production_costs(), Money(0));
    unchanged(
        &mut p,
        |p| p.place(work, SimTick(72), None).map(|_| ()),
        "production_not_ready",
    );
    p.advance_to(SimTick(41)).unwrap();
    assert_eq!(p.production().project(id).unwrap().phase, Phase::Recording);
    assert_eq!(p.state().economy().production_escrow(), 0);
    assert_eq!(p.state().economy().production_costs(), Money(1000));
    p.advance_to(SimTick(44)).unwrap();
    assert_eq!(
        p.production().project(id).unwrap().phase,
        Phase::PostProduction
    );
    assert!(!p.world().programmes.iter().any(|p| p.id == work));
    p.advance_to(SimTick(52)).unwrap();
    assert_eq!(p.production().project(id).unwrap().phase, Phase::Completed);
    assert_eq!(
        p.world().programmes.iter().filter(|p| p.id == work).count(),
        1
    );
    assert_eq!(
        p.world()
            .programmes
            .iter()
            .find(|p| p.id == work)
            .unwrap()
            .kind,
        ProgrammeKind::Standalone
    );
    assert_eq!(p.state().familiarity(YOUNG, work), Some(0.0));
    let airing = p.place(work, SimTick(72), Some(EQUIPMENT)).unwrap();
    let q = p.quote(airing).unwrap();
    let contract = p.accept(&q).unwrap();
    p.advance_to(SimTick(75)).unwrap();
    assert!(p.airing(airing).unwrap().result.is_some());
    assert!(matches!(
        p.state().economy().contract(contract).unwrap().status,
        ContractStatus::Settled(_)
    ));
    let repeat = p.place(work, SimTick(80), None).unwrap();
    p.advance_to(SimTick(96)).unwrap();
    assert!(p.airing(repeat).unwrap().result.is_some());
    assert_eq!(p.state().economy().production_costs(), Money(1000));
    assert_eq!(
        p.world().programmes.iter().filter(|p| p.id == work).count(),
        1
    );
    assert_eq!(
        p.state()
            .economy()
            .ledger()
            .iter()
            .filter(|l| l.reason == LedgerReason::ProductionReserve)
            .count(),
        1
    );
    assert_eq!(
        p.state()
            .economy()
            .ledger()
            .iter()
            .filter(|l| l.reason == LedgerReason::ProductionSpend)
            .count(),
        1
    );
    assert_eq!(p.state().economy().total_money(), total);
}

#[test]
fn conflicting_talent_bookings_and_immutable_booked_projects_are_atomic() {
    let mut p = planner();
    let a = p.create_production(concept()).unwrap();
    let b = p
        .create_production(Concept {
            record_at: SimTick(42),
            ..concept()
        })
        .unwrap();
    p.book_production(a, None, 0).unwrap();
    unchanged(&mut p, |p| p.book_production(b, None, 0), "talent_conflict");
    unchanged(
        &mut p,
        |p| p.revise_production(a, concept()),
        "production_locked",
    );
    unchanged(&mut p, |p| p.discard_production(a), "production_locked");
    unchanged(
        &mut p,
        |p| p.book_production(a, None, 0),
        "production_locked",
    );
    p.revise_production(
        b,
        Concept {
            record_at: SimTick(44),
            ..concept()
        },
    )
    .unwrap();
    p.book_production(b, None, 0).unwrap();
    assert_eq!(p.state().economy().cash(MAIN), Some(Money(18000)));
}

#[test]
fn budgets_roles_dates_and_missing_people_are_validated_without_mutation() {
    let mut p = planner();
    for (bad, code) in [
        (
            Concept {
                guests: [PersonId(3), PersonId(3)],
                ..concept()
            },
            "duplicate_participant",
        ),
        (
            Concept {
                host: PersonId(3),
                guests: [PersonId(4), PersonId(5)],
                ..concept()
            },
            "wrong_role",
        ),
        (
            Concept {
                host: PersonId(999),
                ..concept()
            },
            "unknown_person",
        ),
        (
            Concept {
                budget: Money(1),
                ..concept()
            },
            "invalid_budget",
        ),
        (
            Concept {
                budget: Money(u64::MAX),
                ..concept()
            },
            "invalid_budget",
        ),
        (
            Concept {
                record_at: SimTick(668),
                ..concept()
            },
            "invalid_recording_time",
        ),
        (
            Concept {
                title: "\n".into(),
                ..concept()
            },
            "invalid_title",
        ),
    ] {
        unchanged(&mut p, |p| p.create_production(bad).map(|_| ()), code);
    }
    let id = p
        .create_production(Concept {
            budget: Money(21000),
            ..concept()
        })
        .unwrap();
    unchanged(
        &mut p,
        |p| p.book_production(id, None, 0),
        "insufficient_production_funds",
    );
    p.advance_to(SimTick(41)).unwrap();
    unchanged(
        &mut p,
        |p| p.book_production(id, None, 0),
        "invalid_recording_time",
    );
}

fn earn(p: &mut Planner) {
    p.accept_request(RequestId(1)).unwrap();
    let id = p.create_production(concept()).unwrap();
    p.book_production(id, None, 0).unwrap();
    p.advance_to(SimTick(52)).unwrap();
    assert_eq!(p.production().sway(), 4);
}
fn later() -> Concept {
    Concept {
        host: PersonId(2),
        guests: [PersonId(5), PersonId(6)],
        record_at: SimTick(232),
        subject: Subject::Entertainment,
        tone: Tone::Calm,
        ..concept()
    }
}
#[test]
fn recording_quest_earns_once_and_pool_secures_a_different_person_with_cooperation() {
    let mut p = planner();
    let total = p.state().economy().total_money();
    p.accept_request(RequestId(1)).unwrap();
    let first = p.create_production(concept()).unwrap();
    p.book_production(first, None, 0).unwrap();
    assert_eq!(p.production().sway(), 0);
    p.advance_to(SimTick(43)).unwrap();
    assert_eq!(p.production().sway(), 0);
    p.advance_to(SimTick(44)).unwrap();
    assert_eq!(p.production().sway(), 4);
    assert_eq!(
        p.production()
            .requests()
            .next()
            .unwrap()
            .acceptance
            .as_ref()
            .unwrap()
            .fulfilled_at,
        Some(SimTick(44))
    );
    p.advance_to(SimTick(52)).unwrap();
    assert_eq!(p.production().sway_history().len(), 1);
    let second = p.create_production(later()).unwrap();
    unchanged(
        &mut p,
        |p| p.book_production(second, None, 0),
        "exceptional_participation_required",
    );
    p.book_production(second, Some(PersonId(5)), 2).unwrap();
    assert_eq!(p.production().sway(), 0);
    assert_eq!(p.production().sway_history().len(), 3);
    assert_eq!(p.state().economy().cash(MAIN), Some(Money(18000)));
    assert!(matches!(
        p.production().sway_history()[1].reason,
        SwayReason::ExceptionalParticipation(PersonId(5))
    ));
    unchanged(
        &mut p,
        |p| p.book_production(second, Some(PersonId(5)), 2),
        "production_locked",
    );
    p.advance_to(SimTick(244)).unwrap();
    assert_eq!(p.production().sway(), 0);
    assert_eq!(p.state().economy().total_money(), total);
    let mut balance = 0_i64;
    for entry in p.production().sway_history() {
        balance += i64::from(entry.change);
        assert_eq!(balance, i64::from(entry.balance));
        assert!(balance >= 0);
    }
}
#[test]
fn second_quest_requires_matching_completed_work_and_accepted_requests_are_finite() {
    let mut p = planner();
    p.accept_request(RequestId(2)).unwrap();
    unchanged(
        &mut p,
        |p| p.accept_request(RequestId(2)),
        "quest_already_accepted",
    );
    let wrong = p
        .create_production(Concept {
            guests: [PersonId(4), PersonId(6)],
            ..concept()
        })
        .unwrap();
    p.book_production(wrong, None, 0).unwrap();
    p.advance_to(SimTick(52)).unwrap();
    assert_eq!(p.production().sway(), 0);
    let id = p
        .create_production(Concept {
            guests: [PersonId(4), PersonId(6)],
            subject: Subject::EverydayLife,
            record_at: SimTick(60),
            ..concept()
        })
        .unwrap();
    p.book_production(id, None, 0).unwrap();
    p.advance_to(SimTick(64)).unwrap();
    assert_eq!(p.production().sway(), 0);
    p.advance_to(SimTick(72)).unwrap();
    assert_eq!(p.production().sway(), 3);
    let repeat = p
        .create_production(Concept {
            guests: [PersonId(4), PersonId(6)],
            subject: Subject::EverydayLife,
            record_at: SimTick(80),
            ..concept()
        })
        .unwrap();
    p.book_production(repeat, None, 0).unwrap();
    p.advance_to(SimTick(96)).unwrap();
    assert_eq!(p.production().sway(), 3);
    assert_eq!(p.production().sway_history().len(), 1);
}
#[test]
fn unaccepted_work_and_work_started_before_acceptance_cannot_reward_retroactively() {
    let mut p = planner();
    let id = p.create_production(concept()).unwrap();
    p.book_production(id, None, 0).unwrap();
    p.advance_to(SimTick(41)).unwrap();
    p.accept_request(RequestId(1)).unwrap();
    p.advance_to(SimTick(52)).unwrap();
    assert_eq!(p.production().sway(), 0);
    let id = p
        .create_production(Concept {
            record_at: SimTick(60),
            ..concept()
        })
        .unwrap();
    p.book_production(id, None, 0).unwrap();
    p.advance_to(SimTick(72)).unwrap();
    assert_eq!(p.production().sway(), 4);
    let mut ordinary = planner();
    let id = ordinary.create_production(concept()).unwrap();
    ordinary.book_production(id, None, 0).unwrap();
    ordinary.advance_to(SimTick(52)).unwrap();
    assert_eq!(ordinary.production().sway(), 0);
    ordinary.accept_request(RequestId(1)).unwrap();
    ordinary.advance_to(SimTick(53)).unwrap();
    assert_eq!(ordinary.production().sway(), 0);
}
#[test]
fn quest_deadline_includes_achievement_boundary_but_late_work_earns_nothing() {
    for (start, reward) in [(180, 3), (181, 0)] {
        let mut p = planner();
        p.accept_request(RequestId(2)).unwrap();
        let acceptance = p
            .production()
            .requests()
            .nth(1)
            .unwrap()
            .acceptance
            .as_ref()
            .unwrap();
        assert_eq!(acceptance.deadline, SimTick(192));
        let id = p
            .create_production(Concept {
                guests: [PersonId(4), PersonId(6)],
                subject: Subject::EverydayLife,
                record_at: SimTick(start),
                ..concept()
            })
            .unwrap();
        p.book_production(id, None, 0).unwrap();
        p.advance_to(SimTick(200)).unwrap();
        assert_eq!(p.production().sway(), reward);
        unchanged(
            &mut p,
            |p| p.accept_request(RequestId(2)),
            "quest_already_accepted",
        );
    }
}

#[test]
fn sway_cannot_bypass_fees_conflicts_or_nonnegotiable_dates_and_failures_are_atomic() {
    let mut p = planner();
    let id = p.create_production(later()).unwrap();
    unchanged(
        &mut p,
        |p| p.book_production(id, Some(PersonId(5)), 2),
        "insufficient_sway",
    );
    p.discard_production(id).unwrap();
    earn(&mut p);
    let id = p.create_production(later()).unwrap();
    unchanged(
        &mut p,
        |p| p.book_production(id, Some(PersonId(3)), 0),
        "participation_mismatch",
    );
    p.book_production(id, Some(PersonId(5)), 0).unwrap();
    let conflict = p.create_production(later()).unwrap();
    unchanged(
        &mut p,
        |p| p.book_production(conflict, Some(PersonId(5)), 2),
        "talent_conflict",
    );
    let expensive = p
        .create_production(Concept {
            budget: Money(20000),
            record_at: SimTick(240),
            ..later()
        })
        .unwrap();
    unchanged(
        &mut p,
        |p| p.book_production(expensive, Some(PersonId(5)), 0),
        "insufficient_production_funds",
    );
    let next = p
        .create_production(Concept {
            record_at: SimTick(244),
            ..later()
        })
        .unwrap();
    unchanged(
        &mut p,
        |p| p.book_production(next, Some(PersonId(5)), 2),
        "insufficient_sway",
    );
    // A listed person can still have a hard, non-negotiable unavailable window.
    let mut hard = planner();
    let (mut roster, requests) = demo_roster();
    roster
        .iter_mut()
        .find(|p| p.id == PersonId(5))
        .unwrap()
        .negotiable_from = None;
    hard.configure_production(roster, requests).unwrap();
    earn(&mut hard);
    let id = hard.create_production(later()).unwrap();
    unchanged(
        &mut hard,
        |p| p.book_production(id, Some(PersonId(5)), 0),
        "participation_not_negotiable",
    );
    let ordinary = hard
        .create_production(Concept {
            record_at: SimTick(60),
            ..concept()
        })
        .unwrap();
    unchanged(
        &mut hard,
        |p| p.book_production(ordinary, Some(PersonId(3)), 0),
        "participation_not_needed",
    );
}
#[test]
fn cooperation_has_a_bounded_concrete_effect_without_changing_fees_or_guaranteeing_quality() {
    let mut a = planner();
    earn(&mut a);
    let mut b = a.clone();
    let c = Concept {
        host: PersonId(2),
        guests: [PersonId(5), PersonId(6)],
        tone: Tone::Calm,
        record_at: SimTick(60),
        ..concept()
    };
    let x = a.create_production(c.clone()).unwrap();
    let y = b.create_production(c).unwrap();
    a.book_production(x, None, 0).unwrap();
    b.book_production(y, None, 2).unwrap();
    a.advance_to(SimTick(72)).unwrap();
    b.advance_to(SimTick(72)).unwrap();
    let work = |p: &Planner| p.world().programmes.last().unwrap().clone();
    let plain = work(&a);
    let smooth = work(&b);
    assert!(smooth.execution > plain.execution);
    assert!(smooth.execution - plain.execution <= 0.12);
    assert!(smooth.execution < 0.85); // weak topic fit is not rescued by cooperation
    assert_eq!(plain.cast_appeal, smooth.cast_appeal);
    assert_eq!(a.state().economy(), b.state().economy());
    assert_eq!(a.production().sway(), 4);
    assert_eq!(b.production().sway(), 2);
}

#[test]
fn ticking_in_batches_or_individually_reproduces_production_money_and_sway() {
    let mut a = planner();
    let id = a.create_production(concept()).unwrap();
    a.accept_request(RequestId(1)).unwrap();
    a.book_production(id, None, 0).unwrap();
    let total = a.state().economy().total_money();
    let mut b = a.clone();
    a.advance_to(SimTick(96)).unwrap();
    for tick in 1..=96 {
        b.advance_to(SimTick(tick)).unwrap();
        assert_eq!(b.state().economy().total_money(), total);
    }
    assert_eq!(a.state(), b.state());
    assert_eq!(a.production(), b.production());
    assert_eq!(a.world(), b.world());
    let x = a.create_production(later()).unwrap();
    let y = b.create_production(later()).unwrap();
    a.book_production(x, Some(PersonId(5)), 2).unwrap();
    b.book_production(y, Some(PersonId(5)), 2).unwrap();
    a.advance_to(SimTick(244)).unwrap();
    for tick in 97..=244 {
        b.advance_to(SimTick(tick)).unwrap();
    }
    assert_eq!(a.state(), b.state());
    assert_eq!(a.production(), b.production());
    assert_eq!(a.world(), b.world());
}

#[test]
fn production_reserves_cash_before_broadcast_funding_and_ad_refunds_still_conserve() {
    let mut w = bts_sim::demo::world();
    w.stations[0].initial_cash = Money(1000);
    let mut p = Planner::new(w, Parameters::default(), MAIN, SimTick(672), &[]).unwrap();
    let (r, q) = demo_roster();
    p.configure_production(r, q).unwrap();
    let total = p.state().economy().total_money();
    let airing = p.place(ProgrammeId(1), SimTick(72), Some(SNACKS)).unwrap();
    let quote = p.quote(airing).unwrap();
    let payment = quote.offer.payment;
    let contract = p.accept(&quote).unwrap();
    let project = p.create_production(concept()).unwrap();
    p.book_production(project, None, 0).unwrap();
    assert_eq!(p.state().economy().cash(MAIN), Some(Money(0)));
    p.advance_to(SimTick(96)).unwrap();
    assert!(p.airing(airing).unwrap().result.as_ref().unwrap().off_air);
    let ContractStatus::Settled(s) = &p.state().economy().contract(contract).unwrap().status else {
        panic!()
    };
    assert_eq!(s.payment, Money(0));
    assert_eq!(s.refund, payment);
    assert_eq!(p.state().economy().total_money(), total);
    let mut balances = std::collections::BTreeMap::<Account, i128>::new();
    balances.insert(Account::Station(MAIN), 1000);
    balances.insert(Account::Station(RIVAL), 20000);
    balances.insert(Account::Brand(SNACKS), 20000);
    balances.insert(Account::Brand(EQUIPMENT), 20000);
    for l in p.state().economy().ledger() {
        *balances.entry(l.from).or_default() -= i128::from(l.amount.0);
        *balances.entry(l.to).or_default() += i128::from(l.amount.0);
        assert!(balances.values().all(|v| *v >= 0));
    }
    assert_eq!(balances[&Account::ProductionCosts], 1000);
    assert_eq!(balances[&Account::ProductionEscrow(project)], 0);
}

fn produce_and_air(c: Concept) -> (Planner, ProductionId) {
    let mut p = planner();
    let id = p.create_production(c).unwrap();
    p.book_production(id, None, 0).unwrap();
    p.advance_to(SimTick(52)).unwrap();
    let work = p.production().project(id).unwrap().programme.unwrap();
    p.place(work, SimTick(72), None).unwrap();
    p.advance_to(SimTick(75)).unwrap();
    (p, id)
}
#[test]
fn creative_and_talent_choices_change_audiences_costs_and_experienced_satisfaction() {
    let (science, _) = produce_and_air(concept());
    let entertainment = Concept {
        host: PersonId(2),
        guests: [PersonId(5), PersonId(6)],
        subject: Subject::Entertainment,
        tone: Tone::Lively,
        budget: Money(600),
        ..concept()
    };
    let (popular, _) = produce_and_air(entertainment);
    let viewers = |p: &Planner, audience| {
        p.airings()
            .find_map(|a| a.result.as_ref())
            .unwrap()
            .allocations
            .iter()
            .find(|a| a.audience == audience)
            .unwrap()
            .viewing[0]
            .viewers
    };
    assert!(viewers(&science, SPECIALISTS) > viewers(&popular, SPECIALISTS));
    assert!(viewers(&popular, YOUNG) > viewers(&science, YOUNG));
    assert_eq!(science.state().economy().production_costs(), Money(1000));
    assert_eq!(popular.state().economy().production_costs(), Money(600));
    let mut expert = planner();
    let mut mismatch = planner();
    let (mut roster, request) = demo_roster();
    roster[0].subject_fit[0] = 0.05;
    mismatch.configure_production(roster, request).unwrap();
    let run = |p: &mut Planner| {
        let id = p.create_production(concept()).unwrap();
        p.book_production(id, None, 0).unwrap();
        p.advance_to(SimTick(52)).unwrap();
        let work = p.production().project(id).unwrap().programme.unwrap();
        p.place(work, SimTick(72), None).unwrap();
        p.advance_to(SimTick(75)).unwrap();
        p.airings()
            .find_map(|a| a.result.as_ref())
            .unwrap()
            .allocations
            .iter()
            .find(|a| a.audience == SPECIALISTS)
            .unwrap()
            .viewing[0]
            .satisfaction
            .trace
            .output
    };
    assert!(run(&mut expert) > run(&mut mismatch) + 0.04);
}

#[test]
fn invalid_request_configuration_cannot_create_infinite_or_duplicate_rewards() {
    let mut p = planner();
    let (roster, requests) = demo_roster();
    let mut duplicate = requests.clone();
    duplicate.push(requests[0].clone());
    unchanged(
        &mut p,
        |p| p.configure_production(roster.clone(), duplicate),
        "invalid_request",
    );
    let mut zero = requests.clone();
    zero[0].reward = 0;
    unchanged(
        &mut p,
        |p| p.configure_production(roster.clone(), zero),
        "invalid_request",
    );
    let mut bad = requests.clone();
    bad[0].condition = RequestCondition::CoGuests {
        first: PersonId(1),
        second: PersonId(4),
    };
    unchanged(
        &mut p,
        |p| p.configure_production(roster.clone(), bad),
        "invalid_request",
    );
    let mut huge = requests.clone();
    huge[0].reward = u32::MAX;
    unchanged(
        &mut p,
        |p| p.configure_production(roster, huge),
        "invalid_request",
    );
}

#[test]
fn support_recovers_only_cooperation_losses_with_diminishing_returns_for_every_integer() {
    let mut seed = planner();
    earn(&mut seed);
    let mut outcomes = Vec::new();
    let mut financial = None;
    for support in 0..=4 {
        let mut p = seed.clone();
        let id = p
            .create_production(Concept {
                record_at: SimTick(60),
                ..later()
            })
            .unwrap();
        p.book_production(id, None, support).unwrap();
        let o = p.production().project(id).unwrap().outcome.clone().unwrap();
        assert!((o.cooperation_penalty - 0.12).abs() < 1e-12);
        assert!((o.remaining_penalty - 0.24 / (2.0 + f64::from(support))).abs() < 1e-12);
        assert!(o.execution <= o.base_execution && o.execution >= 0.0);
        assert!(o.recovered_penalty >= 0.0 && o.recovered_penalty <= o.cooperation_penalty);
        assert_eq!(p.production().sway(), 4 - support);
        if let Some(ref economy) = financial {
            assert_eq!(p.state().economy(), economy);
        } else {
            financial = Some(p.state().economy().clone());
        }
        outcomes.push(o);
    }
    for o in &outcomes {
        assert_eq!(o.craft, outcomes[0].craft);
        assert_eq!(o.subject_fit, outcomes[0].subject_fit);
        assert_eq!(o.capability, outcomes[0].capability);
        assert_eq!(o.base_execution, outcomes[0].base_execution);
    }
    let gains = outcomes
        .windows(2)
        .map(|p| p[1].execution - p[0].execution)
        .collect::<Vec<_>>();
    assert!(gains.iter().all(|g| *g > 0.0));
    assert!(gains.windows(2).all(|p| p[1] < p[0]));
    let mut p = seed;
    let id = p
        .create_production(Concept {
            record_at: SimTick(60),
            ..concept()
        })
        .unwrap();
    unchanged(
        &mut p,
        |p| p.book_production(id, None, u32::MAX),
        "insufficient_sway",
    );
    unchanged(
        &mut p,
        |p| p.book_production(id, None, 5),
        "insufficient_sway",
    );
}
#[test]
fn no_deficiency_means_no_bonus_and_support_cannot_replace_ability() {
    let mut seed = planner();
    earn(&mut seed);
    let run = |mut p: Planner, support, c: Concept| {
        let id = p.create_production(c).unwrap();
        p.book_production(id, None, support).unwrap();
        p.production().project(id).unwrap().outcome.clone().unwrap()
    };
    let c = Concept {
        record_at: SimTick(60),
        ..concept()
    };
    let plain = run(seed.clone(), 0, c.clone());
    let supported = run(seed, 4, c);
    assert_eq!(plain, supported);
    assert_eq!(supported.cooperation_penalty, 0.0);
    let mut weak = planner();
    let (mut roster, requests) = demo_roster();
    for t in &mut roster {
        t.craft = 0.05;
        t.subject_fit = [0.05; 3];
    }
    weak.configure_production(roster, requests).unwrap();
    weak.configure_cooperation(demo_cooperation()).unwrap();
    earn(&mut weak);
    let c = Concept {
        record_at: SimTick(60),
        ..later()
    };
    let unsupported = run(weak.clone(), 0, c.clone());
    let supported = run(weak, 4, c);
    assert!(supported.execution > unsupported.execution);
    assert!(supported.execution < 0.35);
    assert_eq!(supported.craft, unsupported.craft);
    assert_eq!(supported.subject_fit, unsupported.subject_fit);
}
#[test]
fn cooperation_configuration_rejects_invalid_parameters_without_mutating_state() {
    let mut p = planner();
    for penalty in [f64::NAN, f64::INFINITY, -0.1, 0.0, 0.26] {
        unchanged(
            &mut p,
            |p| {
                p.configure_cooperation(vec![CooperationIssue {
                    people: [PersonId(2), PersonId(5)],
                    penalty,
                    known_note: None,
                }])
            },
            "invalid_cooperation",
        );
    }
    let mut duplicate = demo_cooperation();
    duplicate.push(CooperationIssue {
        people: [PersonId(5), PersonId(2)],
        penalty: 0.1,
        known_note: None,
    });
    unchanged(
        &mut p,
        |p| p.configure_cooperation(duplicate),
        "invalid_cooperation",
    );
    unchanged(
        &mut p,
        |p| {
            p.configure_cooperation(vec![CooperationIssue {
                people: [PersonId(2), PersonId(999)],
                penalty: 0.1,
                known_note: None,
            }])
        },
        "invalid_cooperation",
    );
}

#[test]
fn stacked_deficiencies_cannot_exceed_skill_baseline_or_create_threshold_returns() {
    let mut seed = planner();
    let (mut roster, requests) = demo_roster();
    for t in &mut roster {
        t.craft = 0.0;
        t.subject_fit = [0.0; 3];
    }
    seed.configure_production(roster, requests).unwrap();
    seed.configure_cooperation(vec![
        CooperationIssue {
            people: [PersonId(2), PersonId(5)],
            penalty: 0.25,
            known_note: None,
        },
        CooperationIssue {
            people: [PersonId(5), PersonId(6)],
            penalty: 0.25,
            known_note: None,
        },
    ])
    .unwrap();
    earn(&mut seed);
    let mut last_gain = f64::INFINITY;
    let mut last = 0.0;
    for support in 0..=4 {
        let mut p = seed.clone();
        let id = p
            .create_production(Concept {
                record_at: SimTick(60),
                ..later()
            })
            .unwrap();
        p.book_production(id, None, support).unwrap();
        let o = p
            .production()
            .project(id)
            .unwrap()
            .outcome
            .as_ref()
            .unwrap();
        assert!(o.cooperation_penalty <= o.base_execution);
        assert!(o.execution <= o.base_execution);
        if support > 0 {
            let gain = o.execution - last;
            assert!(gain > 0.0 && gain < last_gain);
            last_gain = gain;
        }
        last = o.execution;
    }
}

#[test]
fn accepting_a_quest_locks_configuration_and_deadlines_are_clipped_to_scenario_end() {
    let mut p = planner();
    p.accept_request(RequestId(1)).unwrap();
    let (roster, requests) = demo_roster();
    unchanged(
        &mut p,
        |p| p.configure_production(roster, requests),
        "production_config_locked",
    );
    unchanged(
        &mut p,
        |p| p.configure_cooperation(vec![]),
        "production_config_locked",
    );
    p.advance_to(SimTick(650)).unwrap();
    p.accept_request(RequestId(2)).unwrap();
    assert_eq!(
        p.production()
            .requests()
            .nth(1)
            .unwrap()
            .acceptance
            .as_ref()
            .unwrap()
            .deadline,
        SimTick(672)
    );
    let mut ended = planner();
    ended.advance_to(SimTick(672)).unwrap();
    unchanged(
        &mut ended,
        |p| p.accept_request(RequestId(2)),
        "scenario_ended",
    );
}

fn station_resources(studios: u16) -> Vec<bts_sim::planning::StationResources> {
    use bts_sim::planning::StationResources;
    vec![
        StationResources {
            station: MAIN,
            programmes: [ProgrammeId(1), ProgrammeId(2)].into_iter().collect(),
            staff: [PersonId(1), PersonId(2)].into_iter().collect(),
            studios,
        },
        StationResources {
            station: RIVAL,
            programmes: [ProgrammeId(3), ProgrammeId(4)].into_iter().collect(),
            staff: Default::default(),
            studios: 1,
        },
    ]
}
#[test]
fn studios_and_people_are_independent_half_open_reservations() {
    for capacity in [1, 2] {
        let mut p = planner();
        p.configure_station_resources(station_resources(capacity))
            .unwrap();
        let first = p.create_production(concept()).unwrap();
        p.book_production(first, None, 0).unwrap();
        let second = p
            .create_production(Concept {
                record_at: SimTick(40),
                ..later()
            })
            .unwrap();
        if capacity == 1 {
            unchanged(
                &mut p,
                |p| p.book_production(second, None, 0),
                "studio_unavailable",
            );
            p.revise_production(
                second,
                Concept {
                    record_at: SimTick(44),
                    ..later()
                },
            )
            .unwrap();
        }
        p.book_production(second, None, 0).unwrap();
        assert_eq!(
            p.production().project(first).unwrap().studio,
            Some(StudioId(1))
        );
        assert_eq!(
            p.production().project(second).unwrap().studio,
            Some(StudioId(if capacity == 1 { 1 } else { 2 }))
        );
        let conflict = p
            .create_production(Concept {
                record_at: SimTick(42),
                ..concept()
            })
            .unwrap();
        unchanged(
            &mut p,
            |p| p.book_production(conflict, None, 0),
            "talent_conflict",
        );
        assert_eq!(p.state().economy().cash(MAIN), Some(Money(18000)));
    }
    let mut p = planner();
    p.configure_station_resources(station_resources(0)).unwrap();
    let id = p.create_production(concept()).unwrap();
    unchanged(
        &mut p,
        |p| p.book_production(id, None, 0),
        "studio_unavailable",
    );
}
#[test]
fn station_rights_staff_and_completed_originals_do_not_cross_ownership() {
    let mut p = planner();
    let mut resources = station_resources(1);
    resources[0].staff.remove(&PersonId(2));
    resources[1].staff.insert(PersonId(2));
    p.configure_station_resources(resources).unwrap();
    unchanged(
        &mut p,
        |p| p.place(ProgrammeId(3), SimTick(72), None).map(|_| ()),
        "programme_not_owned",
    );
    unchanged(
        &mut p,
        |p| {
            p.create_production(Concept {
                host: PersonId(2),
                ..concept()
            })
            .map(|_| ())
        },
        "foreign_staff",
    );
    let id = p.create_production(concept()).unwrap();
    p.book_production(id, None, 0).unwrap();
    p.advance_to(SimTick(52)).unwrap();
    let work = p.production().project(id).unwrap().programme.unwrap();
    assert!(p.owns_programme(MAIN, work));
    assert!(!p.owns_programme(RIVAL, work));
    p.place(work, SimTick(72), None).unwrap();
}
#[test]
fn station_resource_configuration_is_validated_and_cannot_override_live_commitments() {
    let mut p = planner();
    let mut bad = station_resources(1);
    bad[1].staff.insert(PersonId(1));
    unchanged(
        &mut p,
        |p| p.configure_station_resources(bad),
        "invalid_station_resources",
    );
    let mut bad = station_resources(1);
    bad[0].programmes.insert(ProgrammeId(999));
    unchanged(
        &mut p,
        |p| p.configure_station_resources(bad),
        "invalid_station_resources",
    );
    let mut empty = station_resources(0);
    for r in &mut empty {
        r.programmes.clear();
        r.staff.clear();
    }
    p.configure_station_resources(empty).unwrap();
    unchanged(
        &mut p,
        |p| p.place(ProgrammeId(1), SimTick(72), None).map(|_| ()),
        "programme_not_owned",
    );
    p = planner();
    p.accept_request(RequestId(1)).unwrap();
    unchanged(
        &mut p,
        |p| p.configure_station_resources(station_resources(1)),
        "station_resources_locked",
    );
}

#[test]
fn host_capability_changes_achievement_with_the_same_ambition_and_craft() {
    let run = |skill| {
        let mut p = planner();
        let (mut roster, requests) = demo_roster();
        roster[0].capabilities.0[7] = Some(skill);
        p.configure_production(roster, requests).unwrap();
        let id = p.create_production(concept()).unwrap();
        p.book_production(id, None, 0).unwrap();
        p.advance_to(SimTick(52)).unwrap();
        p.production().project(id).unwrap().outcome.clone().unwrap()
    };
    let weak = run(0.1);
    let strong = run(0.9);
    assert_eq!(strong, run(0.9));
    assert_eq!(weak.ambition, strong.ambition);
    assert_eq!(weak.craft, strong.craft);
    assert_eq!(weak.subject_fit, strong.subject_fit);
    assert_eq!(weak.achieved.values[0], strong.achieved.values[0]);
    assert!(weak.achieved.values[7] < strong.achieved.values[7]);
    assert!(strong.achieved.values[7] < strong.ambition.values[7]);
}
