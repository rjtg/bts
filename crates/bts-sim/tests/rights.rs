use bts_sim::demo::*;
use bts_sim::economy::{LedgerReason, Money};
use bts_sim::planning::rights::{TitleSeed, TitleStatus, ASSESSMENT_TICKS};
use bts_sim::planning::Planner;
use bts_sim::*;

const TITLE: ProgrammeId = ProgrammeId(60000);

fn planner(player: u64, rival: u64) -> Planner {
    let mut world = demo::world();
    world.stations[0].initial_cash = Money(player);
    world.stations[1].initial_cash = Money(rival);
    let mut work = world.programmes[0].clone();
    work.id = TITLE;
    work.kind = ProgrammeKind::Standalone;
    work.execution = 0.25;
    let ages = world.audiences.iter().map(|group| (group.id, 40)).collect();
    let mut planner = Planner::new(world, Parameters::default(), MAIN, SimTick(672), &[]).unwrap();
    planner.configure_research(719, 12, ages).unwrap();
    planner
        .configure_rights(
            RIVAL,
            vec![TitleSeed {
                programme: work.clone(),
                public_profile: work.profile,
                public_execution: 0.90,
                release: SimTick(0),
                deadline: SimTick(144),
                base_price: Money(300),
                evidence: "Old reviews.",
                art: "corner",
            }],
        )
        .unwrap();
    planner
}

#[test]
fn acquisition_and_resale_are_exclusive_and_protect_scheduled_sales() {
    let mut p = planner(20000, 20000);
    let total = p.state().economy().total_money();
    let offer = p.titles().next().unwrap().acquisition.clone().unwrap();
    assert!(!p.owns_programme(MAIN, TITLE));
    assert!(p.place(TITLE, SimTick(72), None).is_err());
    p.buy_rights(TITLE, offer.token).unwrap();
    assert!(p.owns_programme(MAIN, TITLE));
    assert!(!p.owns_programme(RIVAL, TITLE));
    assert_eq!(
        p.state().economy().cash(MAIN).unwrap().0,
        20000 - offer.price.0
    );
    assert!(p.buy_rights(TITLE, offer.token).is_err());
    let airing = p.place(TITLE, SimTick(72), None).unwrap();
    p.resize_ad_block(airing, 5).unwrap();
    let ad = p.minute_offers(airing).unwrap().remove(0);
    p.accept_minutes(&ad).unwrap();
    p.list_for_resale(TITLE).unwrap();
    assert_eq!(p.resale_blocker(TITLE), Some(airing));
    let bid = p.titles().next().unwrap().resale.clone().unwrap();
    let before = format!("{p:?}");
    assert_eq!(
        p.accept_resale(TITLE, bid.token).unwrap_err().code,
        "rights_airing_conflict"
    );
    assert_eq!(format!("{p:?}"), before);
    p.advance_exact(p.airing(airing).unwrap().end).unwrap();
    let history = p.airing(airing).unwrap().result.clone();
    p.accept_resale(TITLE, bid.token).unwrap();
    assert_eq!(p.airing(airing).unwrap().result, history);
    assert!(!p.owns_programme(MAIN, TITLE));
    assert!(p.owns_programme(RIVAL, TITLE));
    assert!(p
        .airings()
        .any(|a| a.programme == TITLE && a.station == RIVAL));
    assert!(p.place(TITLE, SimTick(168), None).is_err());
    assert!(p.accept_resale(TITLE, bid.token).is_err());
    assert_eq!(p.state().economy().total_money(), total);
    assert_eq!(
        p.state()
            .economy()
            .ledger()
            .iter()
            .filter(|line| line.rights_title == Some(TITLE))
            .map(|line| line.reason)
            .collect::<Vec<_>>(),
        [LedgerReason::RightsPurchase, LedgerReason::RightsResale]
    );
}

#[test]
fn only_a_solvent_rival_bids_and_acquires_at_expiry() {
    let mut solvent = planner(20000, 20000);
    let mut broke = planner(1, 50);
    let price = |p: &Planner| {
        p.titles()
            .next()
            .unwrap()
            .acquisition
            .as_ref()
            .unwrap()
            .price
            .0
    };
    assert_eq!(price(&solvent), price(&broke) + price(&broke) / 5);
    let offer = broke
        .titles()
        .next()
        .unwrap()
        .acquisition
        .as_ref()
        .unwrap()
        .token;
    let before = format!("{broke:?}");
    assert_eq!(
        broke.buy_rights(TITLE, offer).unwrap_err().code,
        "insufficient_rights_funds"
    );
    assert_eq!(format!("{broke:?}"), before);
    let mut step = solvent.clone();
    solvent.advance_to(SimTick(145)).unwrap();
    for tick in 1..=145 {
        step.advance_to(SimTick(tick)).unwrap();
    }
    assert_eq!(solvent.state(), step.state());
    assert_eq!(solvent.rights_news(), step.rights_news());
    assert!(solvent.owns_programme(RIVAL, TITLE));
    assert!(solvent.buy_rights(TITLE, offer).is_err());
    broke.advance_to(SimTick(145)).unwrap();
    assert_eq!(broke.titles().next().unwrap().status, TitleStatus::Expired);
    assert!(!broke.owns_programme(RIVAL, TITLE));
}

#[test]
fn delayed_assessment_can_lower_an_estimate_at_diminishing_returns() {
    let mut p = planner(20000, 20000);
    let f = |p: &Planner| p.title_forecast(TITLE).unwrap();
    let baseline = f(&p);
    let price = p.titles().next().unwrap().acquisition.clone();
    let before = format!("{p:?}");
    assert!(p.commission_title_assessment(TITLE, 13).is_err());
    assert_eq!(format!("{p:?}"), before);
    p.commission_title_assessment(TITLE, 3).unwrap();
    assert_eq!(p.research_view().unwrap().insight, 9);
    assert_eq!(f(&p), baseline);
    p.advance_to(SimTick(ASSESSMENT_TICKS)).unwrap();
    let first = f(&p);
    assert!(first.expected_viewers < baseline.expected_viewers);
    p.commission_title_assessment(TITLE, 3).unwrap();
    p.advance_to(SimTick(2 * ASSESSMENT_TICKS)).unwrap();
    let second = f(&p);
    assert!(second.expected_viewers < first.expected_viewers);
    assert!(
        first.expected_viewers - second.expected_viewers
            < baseline.expected_viewers - first.expected_viewers
    );
    assert!(second.relative_uncertainty >= 0.12);
    assert_eq!(p.titles().next().unwrap().acquisition, price);
}
