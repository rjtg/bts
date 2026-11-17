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
    p.configure_production(roster, Some(request)).unwrap();
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
    p.book_production(id, None, None).unwrap();
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
    p.book_production(a, None, None).unwrap();
    unchanged(
        &mut p,
        |p| p.book_production(b, None, None),
        "talent_conflict",
    );
    unchanged(
        &mut p,
        |p| p.revise_production(a, concept()),
        "production_locked",
    );
    unchanged(&mut p, |p| p.discard_production(a), "production_locked");
    unchanged(
        &mut p,
        |p| p.book_production(a, None, None),
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
    p.book_production(b, None, None).unwrap();
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
        |p| p.book_production(id, None, None),
        "insufficient_production_funds",
    );
    p.advance_to(SimTick(41)).unwrap();
    unchanged(
        &mut p,
        |p| p.book_production(id, None, None),
        "invalid_recording_time",
    );
}

#[test]
fn a_recorded_promise_earns_one_personal_favour_and_call_secures_later_participation() {
    let mut p = planner();
    let first = p.create_production(concept()).unwrap();
    p.book_production(first, Some(RequestId(1)), None).unwrap();
    let promise = p.production().request().unwrap().promise.as_ref().unwrap();
    assert_eq!(promise.production, first);
    assert_eq!(promise.promised_at, SimTick(0));
    assert!(promise.fulfilled_at.is_none());
    assert_eq!(p.production().favours().count(), 0);
    p.advance_to(SimTick(43)).unwrap();
    assert_eq!(p.production().favours().count(), 0);
    p.advance_to(SimTick(44)).unwrap();
    assert_eq!(p.production().favours().count(), 1);
    let favour = p.production().favours().next().unwrap().clone();
    assert_eq!(favour.owed_by, PersonId(3));
    assert_eq!(favour.earned_at, SimTick(44));
    p.advance_to(SimTick(52)).unwrap();
    assert_eq!(p.production().favours().count(), 1);
    let later = p
        .create_production(Concept {
            title: "A later conversation".into(),
            record_at: SimTick(232),
            guests: [PersonId(3), PersonId(6)],
            ..concept()
        })
        .unwrap();
    unchanged(
        &mut p,
        |p| p.book_production(later, None, None),
        "favour_required",
    );
    p.book_production(later, None, Some(favour.id)).unwrap();
    let used = p.production().favours().next().unwrap();
    assert_eq!(used.called_for, Some(later));
    assert_eq!(used.called_at, Some(SimTick(52)));
    let third = p
        .create_production(Concept {
            record_at: SimTick(240),
            ..concept()
        })
        .unwrap();
    unchanged(
        &mut p,
        |p| p.book_production(third, None, Some(favour.id)),
        "favour_already_called",
    );
    p.advance_to(SimTick(244)).unwrap();
    assert_eq!(
        p.production().project(later).unwrap().phase,
        Phase::Completed
    );
    assert_eq!(p.production().favours().count(), 1);
    assert_eq!(
        p.production()
            .request()
            .unwrap()
            .promise
            .as_ref()
            .unwrap()
            .earned_favour,
        Some(favour.id)
    );
}

#[test]
fn ordinary_work_wrong_pair_and_repeat_promises_cannot_farm_or_transfer_rewards() {
    let mut p = planner();
    let wrong = p
        .create_production(Concept {
            guests: [PersonId(3), PersonId(5)],
            ..concept()
        })
        .unwrap();
    unchanged(
        &mut p,
        |p| p.book_production(wrong, Some(RequestId(1)), None),
        "promise_mismatch",
    );
    p.discard_production(wrong).unwrap();
    let ordinary = p.create_production(concept()).unwrap();
    p.book_production(ordinary, None, None).unwrap();
    p.advance_to(SimTick(52)).unwrap();
    assert_eq!(p.production().favours().count(), 0);
    assert!(p.production().request().unwrap().promise.is_none());
    let promised = p
        .create_production(Concept {
            record_at: SimTick(56),
            ..concept()
        })
        .unwrap();
    p.book_production(promised, Some(RequestId(1)), None)
        .unwrap();
    p.advance_to(SimTick(68)).unwrap();
    let favour = p.production().favours().next().unwrap().id;
    let repeat = p
        .create_production(Concept {
            record_at: SimTick(80),
            ..concept()
        })
        .unwrap();
    unchanged(
        &mut p,
        |p| p.book_production(repeat, Some(RequestId(1)), None),
        "request_already_promised",
    );
    p.book_production(repeat, None, None).unwrap();
    p.advance_to(SimTick(96)).unwrap();
    assert_eq!(p.production().favours().count(), 1);
    let another = p
        .create_production(Concept {
            guests: [PersonId(5), PersonId(6)],
            record_at: SimTick(240),
            ..concept()
        })
        .unwrap();
    unchanged(
        &mut p,
        |p| p.book_production(another, None, Some(favour)),
        "favour_wrong_person",
    );
}

#[test]
fn ticking_in_batches_or_individually_reproduces_production_money_and_favours() {
    let mut a = planner();
    let id = a.create_production(concept()).unwrap();
    a.book_production(id, Some(RequestId(1)), None).unwrap();
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
}

#[test]
fn production_reserves_cash_before_broadcast_funding_and_ad_refunds_still_conserve() {
    let mut w = bts_sim::demo::world();
    w.stations[0].initial_cash = Money(1000);
    let mut p = Planner::new(w, Parameters::default(), MAIN, SimTick(672), &[]).unwrap();
    let (r, q) = demo_roster();
    p.configure_production(r, Some(q)).unwrap();
    let total = p.state().economy().total_money();
    let airing = p.place(ProgrammeId(1), SimTick(72), Some(SNACKS)).unwrap();
    let quote = p.quote(airing).unwrap();
    let payment = quote.offer.payment;
    let contract = p.accept(&quote).unwrap();
    let project = p.create_production(concept()).unwrap();
    p.book_production(project, None, None).unwrap();
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
    p.book_production(id, None, None).unwrap();
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
    mismatch
        .configure_production(roster, Some(request))
        .unwrap();
    let run = |p: &mut Planner| {
        let id = p.create_production(concept()).unwrap();
        p.book_production(id, None, None).unwrap();
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
