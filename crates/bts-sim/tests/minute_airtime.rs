use bts_sim::demo::{self, MAIN};
use bts_sim::economy::{LedgerReason, Money};
use bts_sim::planning::Planner;
use bts_sim::{AirMinute, Parameters, ProgrammeId, SimTick};

#[test]
fn exact_adjacent_airings_split_one_quarter_without_double_delivery() {
    let mut world = demo::world();
    world.programmes[0].duration_minutes = 45;
    let mut planner = Planner::new(world, Parameters::default(), MAIN, SimTick(96), &[]).unwrap();
    let before = planner.state().economy().total_money();
    let first = planner
        .place_exact(ProgrammeId(1), AirMinute(1080), 9)
        .unwrap();
    assert_eq!(planner.airing(first).unwrap().end, AirMinute(1134));
    assert_eq!(
        planner
            .place_exact(ProgrammeId(2), AirMinute(1133), 0)
            .unwrap_err()
            .code,
        "overlap"
    );
    let second = planner
        .place_exact(ProgrammeId(2), AirMinute(1134), 0)
        .unwrap();
    let offer = planner.minute_offers(first).unwrap().remove(0);
    planner.accept_minutes(&offer).unwrap();
    let mut replay = planner.clone();
    let events = planner.advance_to(SimTick(76)).unwrap();
    let quarter = events
        .iter()
        .filter(|event| event.action.at.0 > 1125)
        .collect::<Vec<_>>();
    assert_eq!(quarter.last().unwrap().action.at, AirMinute(1140));
    assert!(quarter
        .iter()
        .any(|event| event.action.at == AirMinute(1134)));
    for event in quarter {
        for allocation in &event.allocations {
            assert!(
                allocation
                    .viewing
                    .iter()
                    .map(|group| group.viewers)
                    .sum::<f64>()
                    <= f64::from(allocation.population)
            );
        }
    }
    assert!(planner.airing(first).unwrap().result.is_some());
    assert!(planner.airing(second).unwrap().started());
    let sale = planner.state().economy().minute_sales().next().unwrap();
    assert_eq!(
        sale.settlement.as_ref().unwrap().aired_minutes,
        offer.minutes
    );
    assert_eq!(sale.settlement.as_ref().unwrap().at, AirMinute(1134));
    let spot_end = 1125 + u32::from(offer.minutes);
    let spot = events
        .iter()
        .find(|event| event.action.at == AirMinute(spot_end))
        .unwrap();
    let viewers: f64 = spot
        .allocations
        .iter()
        .flat_map(|group| &group.viewing)
        .filter(|viewing| viewing.station == MAIN)
        .map(|viewing| viewing.viewers)
        .sum();
    assert!(viewers > 0.0);
    assert_eq!(
        sale.settlement.as_ref().unwrap().viewer_minutes,
        viewers * f64::from(offer.minutes)
    );
    assert_eq!(
        planner
            .state()
            .economy()
            .ledger()
            .iter()
            .filter(|entry| entry.reason == LedgerReason::BroadcastCost)
            .count(),
        2
    );
    assert_eq!(planner.state().economy().total_money(), before);
    for tick in 1..=76 {
        replay.advance_to(SimTick(tick)).unwrap();
    }
    assert_eq!(planner.state(), replay.state());
}

#[test]
fn one_minute_teaser_falls_back_to_ident_if_the_target_moves() {
    let mut world = demo::world();
    world.programmes[0].duration_minutes = 1;
    world.programmes[0].broadcast_cost = Money(0);
    let mut planner = Planner::new(world, Parameters::default(), MAIN, SimTick(96), &[]).unwrap();
    planner.configure_filler(ProgrammeId(1)).unwrap();
    let show = planner
        .place_exact(ProgrammeId(2), AirMinute(1080), 0)
        .unwrap();
    let teaser = planner.place_filler(AirMinute(1079), Some(show)).unwrap();
    assert!(planner.teaser_is_current(planner.airing(teaser).unwrap()));
    assert_eq!(
        planner.resize_ad_block(teaser, 1).unwrap_err().code,
        "filler_inventory"
    );
    planner.move_exact(show, AirMinute(1081)).unwrap();
    assert!(!planner.teaser_is_current(planner.airing(teaser).unwrap()));
    planner.advance_exact(AirMinute(1080)).unwrap();
    let aired = planner.airing(teaser).unwrap();
    assert_eq!(aired.end, AirMinute(1080));
    assert!(!aired.result.as_ref().unwrap().off_air);
    assert_eq!(aired.teaser_played, Some(false));
    assert!(planner.state().broadcast_history().is_empty());
}
