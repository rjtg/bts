use super::*;
use crate::demo::{HOUSEHOLDS, MAIN, SNACKS, YOUNG};

#[derive(Clone, Copy, Debug)]
enum Campaign {
    Normal,
    WeakCreative,
    WeakTurnout,
    Unsold,
}

#[test]
fn three_reviews_separate_hype_deficit_from_budget_and_bid_posture() {
    let mut world = crate::demo::world();
    for group in &mut world.audiences {
        group.availability.fill(0.9);
    }
    let programme = world.programmes[0].id;
    let schedule = Planner::new(world, Parameters::default(), MAIN, SimTick(2800), &[]).unwrap();
    let quote = |planner: &Planner, id| {
        planner
            .minute_offers(id)
            .unwrap()
            .into_iter()
            .find(|offer| offer.brand == SNACKS)
            .unwrap()
    };
    let run = |campaign| {
        let mut planner = schedule.clone();
        let conserved = planner.state().economy().total_money();
        for week in 0..3 {
            let start = week * WEEK_MINUTES;
            for time in [start + 5 * 1440 + 1080, start + 6 * 1440 + 1080] {
                let airing = planner.place_exact(programme, AirMinute(time), 10).unwrap();
                if !matches!(campaign, Campaign::Unsold) {
                    let offer = quote(&planner, airing);
                    planner.accept_minutes(&offer).unwrap();
                }
                // Only the actual campaign sees these changed conditions. Quote terms are signed.
                match campaign {
                    Campaign::WeakCreative => planner.engine.world.brands[0].creative_quality = 0.0,
                    Campaign::WeakTurnout => {
                        for group in &mut planner.engine.world.audiences {
                            group.availability.fill(0.01);
                        }
                    }
                    _ => {}
                }
                planner.advance_exact(AirMinute(time + 120)).unwrap();
                planner.engine.world = schedule.engine.world.clone();
            }
            planner
                .advance_exact(AirMinute(start + WEEK_MINUTES))
                .unwrap();
            assert_eq!(planner.state().economy().total_money(), conserved);
            let settled = planner
                .state()
                .economy()
                .minute_sales()
                .map(|sale| {
                    let result = sale.settlement.as_ref().unwrap();
                    assert_eq!(result.payment, sale.offer.payment);
                    result.payment.0
                })
                .sum::<u64>();
            let state = planner.state();
            let hype = (state.hype(SNACKS, YOUNG).unwrap()
                + state.hype(SNACKS, HOUSEHOLDS).unwrap())
                / 2.0;
            println!(
                "{campaign:?} week {}: hype {hype:.2}, bid {}%, receipts {settled}, budget {}.",
                week + 1,
                state.bid_index(SNACKS),
                state.economy().budget(SNACKS).unwrap().0
            );
        }
        let next = planner
            .place_exact(programme, AirMinute(3 * WEEK_MINUTES + 1080), 10)
            .unwrap();
        let offer = quote(&planner, next);
        println!(
            "{campaign:?} same next slot: {} Funds for {} minutes.",
            offer.payment.0, offer.minutes
        );
        (planner, offer)
    };
    let (normal, normal_quote) = run(Campaign::Normal);
    let (creative, creative_quote) = run(Campaign::WeakCreative);
    let (turnout, turnout_quote) = run(Campaign::WeakTurnout);
    let (unsold, unsold_quote) = run(Campaign::Unsold);
    assert_eq!(
        normal.state().bid_index(SNACKS),
        creative.state().bid_index(SNACKS)
    );
    assert!(normal.state().bid_index(SNACKS) > turnout.state().bid_index(SNACKS));
    assert_eq!(unsold.state().bid_index(SNACKS), 100);
    assert!(creative.state().hype(SNACKS, YOUNG) < normal.state().hype(SNACKS, YOUNG));
    assert!(creative_quote.demand_pressure > normal_quote.demand_pressure);
    assert!(unsold_quote.demand_pressure > normal_quote.demand_pressure);
    assert!(turnout_quote.demand_pressure > normal_quote.demand_pressure);
    let budget = |planner: &Planner| planner.state().economy().budget(SNACKS).unwrap();
    assert!(budget(&creative) < budget(&normal));
    assert!(budget(&turnout) < budget(&normal));
    assert!(budget(&unsold) > budget(&normal));
    assert!(unsold_quote.payment > normal_quote.payment);
}
