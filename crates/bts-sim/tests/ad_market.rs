use bts_sim::demo::*;
use bts_sim::economy::{LedgerReason, Money};
use bts_sim::planning::{AiringId, Planner};
use bts_sim::*;

fn planner(mut world: World) -> Planner {
    for audience in &mut world.audiences {
        audience.availability = [0.8; 24];
    }
    Planner::new(world, Parameters::default(), MAIN, SimTick(672), &[]).unwrap()
}

fn list(p: &mut Planner, start: u32, minutes: u8) -> AiringId {
    let id = p.place(ProgrammeId(1), SimTick(start), None).unwrap();
    p.resize_ad_block(id, minutes).unwrap();
    id
}

#[test]
fn partial_sales_revalidate_inventory_and_payment_and_settle_once() {
    let mut p = planner(demo::world());
    let total = p.state().economy().total_money();
    let id = list(&mut p, 72, 16);
    assert_eq!(p.airing(id).unwrap().end, AirMinute(1156));
    let offers = p.minute_offers(id).unwrap();
    assert_eq!(offers.len(), 2);
    let first = &offers[0];
    let old_budget = p.state().economy().budget(first.brand).unwrap().0;
    p.accept_minutes(first).unwrap();
    assert_eq!(
        p.state().economy().budget(first.brand).unwrap().0,
        old_budget - first.payment.0
    );
    assert_eq!(p.state().economy().escrow(), u128::from(first.payment.0));
    let before = format!("{p:?}");
    assert_eq!(
        p.accept_minutes(&offers[1]).unwrap_err().code,
        "stale_minute_offer"
    );
    assert_eq!(p.resize_ad_block(id, 0).unwrap_err().code, "ad_inventory");
    assert_eq!(p.remove(id).unwrap_err().code, "booking_locked");
    assert_eq!(
        p.move_airing(id, SimTick(80)).unwrap_err().code,
        "booking_locked"
    );
    assert_eq!(format!("{p:?}"), before);
    let second = p.minute_offers(id).unwrap().remove(0);
    assert_ne!(first.brand, second.brand);
    p.accept_minutes(&second).unwrap();
    p.resize_ad_block(id, first.minutes + second.minutes)
        .unwrap();
    assert_eq!(
        p.airing(id).unwrap().end,
        AirMinute(1140 + u32::from(first.minutes + second.minutes))
    );
    let mut replay = p.clone();
    let events = p.advance_to(SimTick(78)).unwrap();
    for tick in 1..=78 {
        replay.advance_to(SimTick(tick)).unwrap();
    }
    assert_eq!(p.state().economy(), replay.state().economy());
    assert_eq!(p.state().economy().total_money(), total);
    assert_eq!(p.state().economy().escrow(), 0);
    let settled: Vec<_> = events
        .iter()
        .flat_map(|e| &e.financial.minute_sales)
        .collect();
    assert_eq!(settled.len(), 2);
    for sale in settled {
        let outcome = sale.settlement.as_ref().unwrap();
        assert_eq!(outcome.payment, sale.offer.payment);
        assert_eq!(outcome.aired_minutes, sale.offer.minutes);
        assert!(outcome.viewer_minutes > 0.0);
    }
    for brand in [SNACKS, EQUIPMENT] {
        assert!(events
            .iter()
            .flat_map(|e| &e.hype)
            .any(|h| h.brand == brand && h.campaign_generated > 0.0));
    }
    p.advance_to(SimTick(85)).unwrap();
    assert_eq!(
        p.state()
            .economy()
            .ledger()
            .iter()
            .filter(|l| l.reason == LedgerReason::MinutePayment)
            .count(),
        2
    );
}

#[test]
fn oversubscribed_offers_and_overlapping_extensions_are_atomic() {
    let mut p = planner(demo::world());
    let id = list(&mut p, 72, 1);
    p.place(ProgrammeId(2), SimTick(77), None).unwrap();
    let start = format!("{p:?}");
    assert_eq!(p.resize_ad_block(id, 16).unwrap_err().code, "overlap");
    assert_eq!(format!("{p:?}"), start);
    let offers = p.minute_offers(id).unwrap();
    assert_eq!(offers.len(), 2);
    assert!(offers.iter().all(|o| o.minutes == 1));
    p.accept_minutes(&offers[0]).unwrap();
    assert!(p.minute_offers(id).unwrap().is_empty());
    assert!(p.accept_minutes(&offers[1]).is_err());
    let end = p.airing(id).unwrap().end;
    p.advance_exact(end).unwrap();
    assert_eq!(p.sold_minutes(id), 1);
}

#[test]
fn finite_budget_and_unfunded_airing_release_reserved_minutes() {
    let mut world = demo::world();
    world.stations[0].initial_cash = Money(150);
    world.brands[0].marketing_budget = Money(7);
    let mut p = planner(world);
    let total = p.state().economy().total_money();
    let early = list(&mut p, 60, 0);
    let late = list(&mut p, 72, 12);
    let offer = p
        .minute_offers(late)
        .unwrap()
        .into_iter()
        .find(|o| o.brand == SNACKS)
        .unwrap();
    assert_eq!(offer.payment, Money(7));
    p.accept_minutes(&offer).unwrap();
    assert_eq!(p.state().economy().budget(SNACKS), Some(Money(0)));
    let end = p.airing(late).unwrap().end;
    let events = p.advance_exact(end).unwrap();
    assert!(!p.airing(early).unwrap().cancelled());
    assert!(p.airing(late).unwrap().cancelled());
    let outcome = p
        .state()
        .economy()
        .minute_sales()
        .next()
        .unwrap()
        .settlement
        .as_ref()
        .unwrap();
    assert_eq!(outcome.aired_minutes, 0);
    assert_eq!(outcome.viewer_minutes, 0.0);
    assert_eq!(outcome.payment, Money(0));
    assert_eq!(outcome.refund, offer.payment);
    assert!(events
        .iter()
        .flat_map(|e| &e.hype)
        .all(|h| h.campaign_generated == 0.0));
    assert_eq!(p.state().economy().budget(SNACKS), Some(Money(7)));
    assert_eq!(p.state().economy().total_money(), total);
}

#[test]
fn unfilled_inventory_reserves_time_without_creating_extra_exposure() {
    let mut p = planner(demo::world());
    let id = list(&mut p, 72, 16);
    let offer = p.minute_offers(id).unwrap().remove(0);
    p.accept_minutes(&offer).unwrap();
    let mut trimmed = p.clone();
    trimmed.resize_ad_block(id, offer.minutes).unwrap();
    let original = p.advance_to(SimTick(78)).unwrap();
    let shorter = trimmed.advance_to(SimTick(78)).unwrap();
    assert_eq!(p.airing(id).unwrap().end, AirMinute(1156));
    assert_eq!(
        trimmed.airing(id).unwrap().end,
        AirMinute(1140 + u32::from(offer.minutes))
    );
    assert_eq!(original[72].hype, shorter[72].hype);
    assert!(original[72]
        .hype
        .iter()
        .filter(|h| h.brand != offer.brand)
        .all(|h| h.exposures == 0.0));
    let untrimmed_sale = p
        .state()
        .economy()
        .minute_sales()
        .next()
        .unwrap()
        .settlement
        .as_ref()
        .unwrap();
    let trimmed_sale = trimmed
        .state()
        .economy()
        .minute_sales()
        .next()
        .unwrap()
        .settlement
        .as_ref()
        .unwrap();
    assert_eq!(untrimmed_sale.viewer_minutes, trimmed_sale.viewer_minutes);
    assert_eq!(untrimmed_sale.payment, trimmed_sale.payment);
}
