use bts_sim::economy::{Account, ContractStatus, LedgerReason, Money};
use bts_sim::planning::{AiringId, CompetitorAiring, Planner};
use bts_sim::{demo::*, *};

fn world() -> World {
    let mut w = bts_sim::demo::world();
    for a in &mut w.audiences {
        a.availability = [0.8; 24];
    }
    w
}
fn planner() -> Planner {
    Planner::new(world(), Parameters::default(), MAIN, SimTick(672), &[]).unwrap()
}
fn unchanged(
    p: &mut Planner,
    f: impl FnOnce(&mut Planner) -> Result<(), bts_sim::planning::PlanError>,
    code: &str,
) {
    let before = format!("{p:?}");
    assert_eq!(f(p).unwrap_err().code, code);
    assert_eq!(format!("{p:?}"), before);
}

#[test]
fn real_durations_adjacency_overlaps_moves_removal_and_boundaries() {
    let mut p = planner();
    let first = p.place(ProgrammeId(1), SimTick(72), None).unwrap();
    assert_eq!(p.airing(first).unwrap().end, SimTick(76));
    unchanged(
        &mut p,
        |p| p.place(ProgrammeId(2), SimTick(75), None).map(|_| ()),
        "overlap",
    );
    let second = p.place(ProgrammeId(2), SimTick(76), None).unwrap();
    unchanged(&mut p, |p| p.move_airing(second, SimTick(73)), "overlap");
    p.move_airing(second, SimTick(80)).unwrap();
    assert_eq!(p.airing(second).unwrap().id, second);
    p.remove(first).unwrap();
    let replacement = p.place(ProgrammeId(1), SimTick(72), None).unwrap();
    assert_ne!(first, replacement);
    let midnight = p.place(ProgrammeId(4), SimTick(94), None).unwrap();
    assert_eq!(p.airing(midnight).unwrap().end, SimTick(98));
    unchanged(
        &mut p,
        |p| p.place(ProgrammeId(1), SimTick(670), None).map(|_| ()),
        "invalid_time",
    );
    unchanged(
        &mut p,
        |p| p.place(ProgrammeId(65535), SimTick(60), None).map(|_| ()),
        "unknown_programme",
    );
    unchanged(&mut p, |p| p.remove(AiringId(u32::MAX)), "unknown_airing");
}

#[test]
fn positive_tick_aligned_programme_durations_are_validated() {
    for bad in [0, 1, 14, 16, 59] {
        let mut w = world();
        w.programmes[0].duration_minutes = bad;
        assert!(Simulation::new(w.clone(), Parameters::default()).is_err());
        assert!(Planner::new(w, Parameters::default(), MAIN, SimTick(96), &[]).is_err());
    }
    for duration in [15, 30, 45, 90, 180] {
        let mut w = world();
        w.programmes[0].duration_minutes = duration;
        assert!(Simulation::new(w, Parameters::default()).is_ok());
    }
}

#[test]
fn a_long_airing_has_one_cost_one_exposure_one_completion_and_one_settlement() {
    let mut p = planner();
    let initial = p.state().economy().total_money();
    let id = p.place(ProgrammeId(1), SimTick(72), Some(SNACKS)).unwrap();
    let quote = p.quote(id).unwrap();
    let promised = quote.offer.payment;
    let contract = p.accept(&quote).unwrap();
    unchanged(&mut p, |p| p.move_airing(id, SimTick(80)), "booking_locked");
    unchanged(&mut p, |p| p.remove(id), "booking_locked");
    unchanged(
        &mut p,
        |p| p.set_advertiser(id, Some(EQUIPMENT)),
        "booking_locked",
    );
    let first = p.advance_to(SimTick(73)).unwrap();
    assert!(p.airing(id).unwrap().started());
    assert!(p.airing(id).unwrap().result.is_none());
    assert_eq!(p.state().familiarity(YOUNG, ProgrammeId(1)), Some(0.0));
    assert_eq!(p.state().broadcast_history().len(), 0);
    assert_eq!(p.state().economy().cash(MAIN), Some(Money(19850)));
    assert!(matches!(
        p.state().economy().contract(contract).unwrap().status,
        ContractStatus::Reserved
    ));
    let actual = p
        .state()
        .economy()
        .contract(contract)
        .unwrap()
        .actual
        .clone();
    let rest = p.advance_to(SimTick(76)).unwrap();
    assert!(rest.iter().all(|e| e
        .hype
        .iter()
        .all(|h| h.exposures == 0.0 && h.campaign_generated == 0.0)));
    assert_eq!(
        p.state().economy().contract(contract).unwrap().actual,
        actual
    );
    assert!(p.state().familiarity(YOUNG, ProgrammeId(1)).unwrap() > 0.0);
    assert_eq!(p.state().broadcast_history().len(), 1);
    let ContractStatus::Settled(settlement) =
        &p.state().economy().contract(contract).unwrap().status
    else {
        panic!("deadline not settled");
    };
    assert_eq!(settlement.payment, promised);
    assert_eq!(settlement.refund, Money(0));
    assert_eq!(
        p.state()
            .economy()
            .ledger()
            .iter()
            .filter(|l| l.reason == LedgerReason::BroadcastCost)
            .count(),
        1
    );
    assert!(first.last().unwrap().hype.iter().any(|h| h.exposures > 0.0));
    let cash = p.state().economy().cash(MAIN);
    p.advance_to(SimTick(96)).unwrap();
    assert_eq!(p.state().economy().cash(MAIN), cash);
    assert_eq!(p.state().economy().total_money(), initial);
}

#[test]
fn splitting_duration_into_ticks_does_not_multiply_delivery_or_cost() {
    let run = |duration| {
        let mut w = world();
        w.programmes[0].duration_minutes = duration;
        let mut p = Planner::new(w, Parameters::default(), MAIN, SimTick(96), &[]).unwrap();
        let id = p.place(ProgrammeId(1), SimTick(72), Some(SNACKS)).unwrap();
        let quote = p.quote(id).unwrap();
        let contract = p.accept(&quote).unwrap();
        let events = p.advance_to(SimTick(96)).unwrap();
        let exposures = events
            .iter()
            .flat_map(|e| &e.hype)
            .map(|h| h.exposures)
            .sum::<f64>();
        (
            p.state()
                .economy()
                .contract(contract)
                .unwrap()
                .actual
                .clone(),
            p.state().economy().cash(MAIN),
            p.state().familiarity(YOUNG, ProgrammeId(1)),
            exposures,
        )
    };
    assert_eq!(run(15), run(90));
}

#[test]
fn batch_advance_processes_every_tick_and_matches_incremental_advancement() {
    let mut batch = planner();
    for (id, time) in [(1, 72), (2, 78), (3, 84)] {
        batch.place(ProgrammeId(id), SimTick(time), None).unwrap();
    }
    let mut individual = batch.clone();
    let events = batch.advance_to(SimTick(96)).unwrap();
    for tick in 1..=96 {
        individual.advance_to(SimTick(tick)).unwrap();
    }
    assert_eq!(events.len(), 96);
    assert_eq!(batch.state(), individual.state());
    assert_eq!(
        batch.airings().collect::<Vec<_>>(),
        individual.airings().collect::<Vec<_>>()
    );
    for event in events {
        for a in event.allocations {
            assert!(
                (a.viewing.iter().map(|v| v.viewers).sum::<f64>() + a.not_watching
                    - f64::from(a.population))
                .abs()
                    < 1e-8
            );
        }
    }
}

#[test]
fn a_future_booking_can_go_off_air_after_earlier_spending_and_refunds() {
    let mut w = world();
    w.stations[0].initial_cash = Money(150);
    let rivals = [CompetitorAiring {
        station: RIVAL,
        programme: ProgrammeId(4),
        start: SimTick(80),
    }];
    let mut p = Planner::new(w, Parameters::default(), MAIN, SimTick(96), &rivals).unwrap();
    let first = p.place(ProgrammeId(1), SimTick(72), None).unwrap();
    let booked = p.place(ProgrammeId(1), SimTick(80), Some(SNACKS)).unwrap();
    let quote = p.quote(booked).unwrap();
    let payment = quote.offer.payment;
    let contract = p.accept(&quote).unwrap();
    let total = p.state().economy().total_money();
    let events = p.advance_to(SimTick(84)).unwrap();
    assert!(!p.airing(first).unwrap().result.as_ref().unwrap().off_air);
    let failed = p.airing(booked).unwrap().result.as_ref().unwrap();
    assert!(failed.off_air);
    assert_eq!(failed.cost, Money(0));
    let actual = p.state().economy().contract(contract).unwrap();
    assert_eq!(actual.actual.relevant_delivery, 0.0);
    let ContractStatus::Settled(s) = &actual.status else {
        panic!()
    };
    assert_eq!(s.payment, Money(0));
    assert_eq!(s.refund, payment);
    assert!(events[80..84].iter().all(|e| e
        .allocations
        .iter()
        .all(|a| a.viewing.iter().all(|v| v.station == RIVAL))));
    assert!(events
        .iter()
        .all(|e| e.hype.iter().all(|h| h.campaign_generated == 0.0)));
    assert_eq!(p.state().economy().total_money(), total);
    assert_eq!(
        p.state()
            .economy()
            .ledger()
            .iter()
            .filter(|l| l.from == Account::Station(MAIN))
            .count(),
        1
    );
}

#[test]
fn continuing_funded_airings_do_not_go_off_air_after_paying_and_rival_failure_redistributes() {
    let run = |rival_cash| {
        let mut w = world();
        w.stations[0].initial_cash = Money(150);
        w.stations[1].initial_cash = Money(rival_cash);
        let rivals = [CompetitorAiring {
            station: RIVAL,
            programme: ProgrammeId(4),
            start: SimTick(72),
        }];
        let mut p = Planner::new(w, Parameters::default(), MAIN, SimTick(96), &rivals).unwrap();
        p.place(ProgrammeId(1), SimTick(72), None).unwrap();
        let events = p.advance_to(SimTick(76)).unwrap();
        assert!(events[72..76].iter().all(|e| e
            .aired
            .broadcasts
            .iter()
            .any(|b| b.station == MAIN)));
        events[72]
            .allocations
            .iter()
            .flat_map(|a| &a.viewing)
            .filter(|v| v.station == MAIN)
            .map(|v| v.viewers)
            .sum::<f64>()
    };
    assert!(run(0) > run(20000));
}

#[test]
fn explicit_airing_identity_stale_quotes_and_fixed_competitors() {
    let mut p = planner();
    let a = p.place(ProgrammeId(1), SimTick(72), Some(SNACKS)).unwrap();
    let b = p.place(ProgrammeId(1), SimTick(80), Some(SNACKS)).unwrap();
    assert_ne!(a, b);
    let quoted = p.quote(b).unwrap();
    assert_eq!(quoted.airing_id, b);
    assert_eq!(quoted.offer.deadline, SimTick(84));
    p.move_airing(a, SimTick(64)).unwrap();
    unchanged(&mut p, |p| p.accept(&quoted).map(|_| ()), "stale_offer");
    p.advance_to(SimTick(65)).unwrap();
    unchanged(&mut p, |p| p.move_airing(a, SimTick(66)), "already_started");
    unchanged(
        &mut p,
        |p| p.advance_to(SimTick(65)).map(|_| ()),
        "invalid_advance",
    );
}

#[test]
fn gaps_and_the_full_seven_day_horizon_are_processed_without_costs_or_exposure() {
    let mut p = planner();
    let total = p.state().economy().total_money();
    let events = p.advance_to(SimTick(672)).unwrap();
    assert_eq!(events.len(), 672);
    assert_eq!(p.state().economy().total_money(), total);
    assert!(p.state().economy().ledger().is_empty());
    assert!(events
        .iter()
        .all(|e| e.aired.broadcasts.is_empty() && e.hype.iter().all(|h| h.exposures == 0.0)));
    let expected = 5.0 * 2.0_f64.powf(-168.0 / 48.0);
    assert!((p.state().hype(SNACKS, YOUNG).unwrap() - expected).abs() < 1e-10);
    unchanged(
        &mut p,
        |p| p.place(ProgrammeId(1), SimTick(670), None).map(|_| ()),
        "invalid_time",
    );
    unchanged(
        &mut p,
        |p| p.advance_to(SimTick(673)).map(|_| ()),
        "invalid_advance",
    );
}

#[test]
fn staggered_completion_samples_keep_learning_bounded() {
    let mut w = world();
    for a in &mut w.audiences {
        a.availability = [1.0; 24];
        a.feature_preferences = [0.8; 10];
    }
    w.programmes[1].duration_minutes = 45;
    w.programmes[1].kind = ProgrammeKind::Episode {
        series: SeriesId(1),
        sequence: 2,
    };
    let rivals = [CompetitorAiring {
        station: RIVAL,
        programme: ProgrammeId(2),
        start: SimTick(73),
    }];
    let mut p = Planner::new(
        w,
        Parameters {
            loyalty_learning: 1.0,
            ..Parameters::default()
        },
        MAIN,
        SimTick(96),
        &rivals,
    )
    .unwrap();
    p.place(ProgrammeId(1), SimTick(72), None).unwrap();
    p.advance_to(SimTick(76)).unwrap();
    for a in &p.world().audiences {
        assert!((0.0..=1.0).contains(&p.state().loyalty(a.id, SeriesId(1)).unwrap()));
    }
}
