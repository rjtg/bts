use bts_sim::{
    advertiser_value,
    demo::{self, Case, EQUIPMENT, MAIN, RIVAL, SNACKS},
    economy::*,
    *,
};
fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "{a} != {b}");
}
fn action(at: u32) -> Slot {
    let mut action = demo::schedule(Case::Broad).remove(0);
    action.at = SimHour(at);
    action.broadcasts[0].advertisement = Some(EQUIPMENT);
    action.broadcasts[1].advertisement = Some(SNACKS);
    action
}
fn controlled_world() -> World {
    let mut world = demo::world();
    world.audiences.truncate(2);
    world.stations.truncate(1);
    world.programmes.truncate(1);
    world.brands.truncate(1);
    let template = world.audiences[0].clone();
    for (i, audience) in world.audiences.iter_mut().enumerate() {
        *audience = template.clone();
        audience.id = AudienceId(i as u16 + 1);
        audience.population = 10000;
        audience.genre_tastes = [1.0; 3];
        audience.preferred_tone = 0.8;
        audience.purchasing_power = 1.0;
        audience.product_interests = [1.0; 2];
        audience.availability = [0.0; 24];
        audience.availability[18 + i] = 1.0;
        audience.availability[20] = 1.0;
    }
    let programme = &mut world.programmes[0];
    programme.tone = 0.8;
    programme.awareness = 1.0;
    programme.cast_appeal = 1.0;
    programme.execution = 1.0;
    programme.broadcast_cost = Money(10);
    world.stations[0].initial_cash = Money(100000);
    let brand = &mut world.brands[0];
    brand.creative_quality = 0.8;
    brand.marketing_budget = Money(100000);
    brand.maximum_offer = Money(100000);
    brand.rate_per_thousand = Money(1000);
    brand.targets = vec![
        Target {
            audience: AudienceId(1),
            priority: 1.0,
            desired_hype: 5.0,
        },
        Target {
            audience: AudienceId(2),
            priority: 1.0,
            desired_hype: 85.0,
        },
    ];
    world
}
fn controlled_action(at: u32) -> Slot {
    Slot {
        at: SimHour(at),
        broadcasts: vec![Broadcast {
            station: MAIN,
            programme: ProgrammeId(1),
            advertisement: Some(SNACKS),
        }],
    }
}
#[test]
fn audience_value_favours_unmet_demand_without_cross_subsidizing_satisfied_delivery() {
    let simulation = Simulation::new(controlled_world(), Parameters::default()).unwrap();
    let before = simulation.state().clone();
    let a = controlled_action(18);
    let b = controlled_action(19);
    let both = controlled_action(20);
    let satisfied = simulation.quote(&a, MAIN, SNACKS, a.at).unwrap();
    let deficient = simulation.quote(&b, MAIN, SNACKS, b.at).unwrap();
    let combined = simulation.quote(&both, MAIN, SNACKS, both.at).unwrap();
    assert_eq!(simulation.state(), &before);
    close(
        satisfied.forecast.relevant_delivery,
        deficient.forecast.relevant_delivery,
    );
    close(
        satisfied.forecast.audiences[0].viewers,
        deficient.forecast.audiences[1].viewers,
    );
    assert_eq!(
        satisfied.forecast.audiences[0].effectiveness,
        deficient.forecast.audiences[1].effectiveness
    );
    assert_eq!(satisfied.demand.pressure, deficient.demand.pressure);
    assert!(satisfied.demand.pressure > 0.0);
    assert!(satisfied.promised_delivery > 0);
    assert_eq!(satisfied.payment, Money(0));
    assert_eq!(satisfied.value_weighted_delivery, 0.0);
    assert!(deficient.payment.0 > 0);
    assert_eq!(satisfied.valuation[0].uncapped_value, 0.0);
    close(
        deficient.value_weighted_delivery,
        deficient.forecast.relevant_delivery * 0.8,
    );
    close(
        combined.forecast.relevant_delivery,
        2.0 * deficient.forecast.relevant_delivery,
    );
    assert_eq!(combined.payment, deficient.payment);
    assert_eq!(combined.uncapped_price, deficient.uncapped_price);
}
#[test]
fn no_reached_deficits_means_no_payable_offer_even_with_positive_delivery() {
    let mut world = controlled_world();
    for target in &mut world.brands[0].targets {
        target.desired_hype = 5.0;
    }
    let mut simulation = Simulation::new(world, Parameters::default()).unwrap();
    let action = controlled_action(20);
    let offer = simulation.quote(&action, MAIN, SNACKS, action.at).unwrap();
    assert!(offer.forecast.relevant_delivery > 1000.0);
    assert!(offer.promised_delivery > 0);
    assert!(offer
        .valuation
        .iter()
        .all(|v| v.deficit == 0.0 && v.uncapped_value == 0.0));
    assert_eq!(offer.payment, Money(0));
    let state = simulation.state().clone();
    assert!(simulation.accept_offer(&offer).is_err());
    assert_eq!(simulation.state(), &state);
}
#[test]
fn settled_delivery_does_not_reprice_the_accepted_value_after_hype_changes() {
    let mut world = controlled_world();
    world.audiences[0].availability[18] = 0.0;
    world.audiences[1].availability[18] = 1.0;
    world.brands[0].targets[0].desired_hype = 0.0;
    world.brands[0].targets[1].desired_hype = 10.0;
    let mut promo = world.programmes[0].clone();
    promo.id = ProgrammeId(50);
    promo.kind = ProgrammeKind::Standalone;
    world.programmes.push(promo);
    let mut simulation = Simulation::new(
        world,
        Parameters {
            loyalty_learning: 0.0,
            ..Parameters::default()
        },
    )
    .unwrap();
    let booked = controlled_action(19);
    let offer = simulation.quote(&booked, MAIN, SNACKS, booked.at).unwrap();
    assert!(offer.payment.0 > 0);
    simulation.accept_offer(&offer).unwrap();
    let mut promotion = controlled_action(18);
    promotion.broadcasts[0].programme = ProgrammeId(50);
    simulation.step(&promotion).unwrap();
    assert_eq!(simulation.demand(SNACKS).unwrap().pressure, 0.0);
    let event = simulation.step(&booked).unwrap();
    let settlement = &event.financial.settlements[0];
    assert_eq!(settlement.promised_delivery, offer.promised_delivery);
    assert_eq!(settlement.credited_delivery, offer.promised_delivery);
    assert_eq!(settlement.payment, offer.payment);
    assert_eq!(settlement.refund, Money(0));
}
#[test]
fn either_insolvent_station_goes_off_air_and_other_viewers_redistribute() {
    let plan = action(18);
    let baseline = Simulation::new(demo::world(), Parameters::default())
        .unwrap()
        .step(&plan)
        .unwrap();
    for failed in [MAIN, RIVAL] {
        let mut world = demo::world();
        world
            .stations
            .iter_mut()
            .find(|s| s.id == failed)
            .unwrap()
            .initial_cash = Money(0);
        let mut simulation = Simulation::new(world.clone(), Parameters::default()).unwrap();
        let initial = simulation.state().clone();
        let total = initial.economy().total_money();
        let event = simulation.step(&plan).unwrap();
        assert_eq!(event.action, plan);
        assert_eq!(event.off_air.len(), 1);
        assert_eq!(event.off_air[0].planned.station, failed);
        assert_eq!(event.off_air[0].available_cash, Money(0));
        assert_eq!(event.aired.broadcasts.len(), 1);
        assert_ne!(event.aired.broadcasts[0].station, failed);
        let failed_programme = world
            .programmes
            .iter()
            .find(|p| p.id == event.off_air[0].planned.programme)
            .unwrap();
        for allocation in &event.allocations {
            assert_eq!(allocation.viewing.len(), 1);
            assert_ne!(allocation.viewing[0].station, failed);
            let baseline_view = baseline
                .allocations
                .iter()
                .find(|a| a.audience == allocation.audience)
                .unwrap()
                .viewing
                .iter()
                .find(|v| v.station != failed)
                .unwrap();
            assert!(allocation.viewing[0].viewers > baseline_view.viewers);
            close(
                allocation.viewing[0].viewers + allocation.not_watching,
                f64::from(allocation.population),
            );
            assert_eq!(
                simulation
                    .state()
                    .loyalty(allocation.audience, failed_programme.series().unwrap()),
                initial.loyalty(allocation.audience, failed_programme.series().unwrap())
            );
        }
        let failed_brand = event.off_air[0].planned.advertisement.unwrap();
        for h in event.hype.iter().filter(|h| h.brand == failed_brand) {
            assert_eq!(h.exposures, 0.0);
            assert_eq!(h.campaign_generated, 0.0);
            assert_eq!(h.after, h.decayed);
            assert!(h.advertisements.is_empty());
        }
        assert!(event
            .financial
            .ledger
            .iter()
            .all(|e| e.from != Account::Station(failed)));
        assert_eq!(simulation.state().economy().total_money(), total);
        // Resolving a cancelled plan has exactly the effects of omitting its broadcast.
        let mut reduced = plan.clone();
        reduced.broadcasts.retain(|b| b.station != failed);
        let mut reference = Simulation::new(world, Parameters::default()).unwrap();
        let expected = reference.step(&reduced).unwrap();
        assert_eq!(event.allocations, expected.allocations);
        assert_eq!(event.hype, expected.hype);
        assert_eq!(simulation.state(), reference.state());
    }
}
#[test]
fn all_off_air_advances_decay_and_time_without_show_effects_and_reproduces() {
    let mut world = demo::world();
    for station in &mut world.stations {
        station.initial_cash = Money(0);
    }
    let mut a = Simulation::new(world.clone(), Parameters::default()).unwrap();
    let mut b = a.clone();
    let initial = a.state().clone();
    let mut reversed = action(18);
    reversed.broadcasts.reverse();
    let one = a.step(&action(18)).unwrap();
    let two = b.step(&reversed).unwrap();
    assert_eq!(one.aired, two.aired);
    assert_eq!(one.off_air, two.off_air);
    assert_eq!(a.state(), b.state());
    assert!(one.aired.broadcasts.is_empty());
    assert_eq!(one.off_air.len(), 2);
    assert!(one.financial.ledger.is_empty());
    for allocation in &one.allocations {
        assert!(allocation.viewing.is_empty());
        assert_eq!(allocation.not_watching, f64::from(allocation.population));
        assert_eq!(allocation.available_not_watching, allocation.available);
    }
    for h in &one.hype {
        assert_eq!(h.exposures, 0.0);
        assert_eq!(h.campaign_generated, 0.0);
        assert!(h.after < h.before);
    }
    for audience in &world.audiences {
        for programme in &world.programmes {
            assert_eq!(
                a.state().loyalty(audience.id, programme.series().unwrap()),
                initial.loyalty(audience.id, programme.series().unwrap())
            );
        }
    }
    assert_eq!(a.state().now(), SimHour(18));
    assert_eq!(
        a.state().economy().total_money(),
        initial.economy().total_money()
    );
    assert_eq!(a.step(&action(21)).unwrap(), b.step(&action(21)).unwrap());
    assert_eq!(a.state(), b.state());
}
#[test]
fn cash_at_cost_is_enough_and_free_programmes_can_air_at_zero_cash() {
    let mut world = demo::world();
    world.stations[0].initial_cash = world.programmes[0].broadcast_cost;
    world.stations[1].initial_cash = world.programmes[3].broadcast_cost;
    let mut simulation = Simulation::new(world.clone(), Parameters::default()).unwrap();
    assert!(simulation.step(&action(18)).unwrap().off_air.is_empty());
    assert_eq!(simulation.state().economy().cash(MAIN), Some(Money(0)));
    assert_eq!(simulation.state().economy().cash(RIVAL), Some(Money(0)));
    assert_eq!(simulation.step(&action(21)).unwrap().off_air.len(), 2);
    world.programmes[0].broadcast_cost = Money(0);
    world.stations[0].initial_cash = Money(0);
    let event = Simulation::new(world, Parameters::default())
        .unwrap()
        .step(&action(18))
        .unwrap();
    assert!(event.aired.broadcasts.iter().any(|b| b.station == MAIN));
}
#[test]
fn insolvency_after_booking_refunds_once_without_blocking_player_or_rival() {
    for failed in [MAIN, RIVAL] {
        let mut world = demo::world();
        let programme = if failed == MAIN {
            &world.programmes[0]
        } else {
            &world.programmes[3]
        };
        world
            .stations
            .iter_mut()
            .find(|s| s.id == failed)
            .unwrap()
            .initial_cash = programme.broadcast_cost;
        let mut simulation = Simulation::new(world, Parameters::default()).unwrap();
        let total = simulation.state().economy().total_money();
        let mut booking = action(21);
        booking
            .broadcasts
            .iter_mut()
            .find(|b| b.station == failed)
            .unwrap()
            .advertisement = Some(SNACKS);
        let offer = simulation
            .quote(&booking, failed, SNACKS, booking.at)
            .unwrap();
        assert!(offer.payment.0 > 0);
        let id = simulation.accept_offer(&offer).unwrap();
        let mut prior = action(18);
        for b in &mut prior.broadcasts {
            b.advertisement = None;
        }
        simulation.step(&prior).unwrap();
        let mut bad = booking.clone();
        bad.broadcasts
            .iter_mut()
            .find(|b| b.station == failed)
            .unwrap()
            .advertisement = Some(EQUIPMENT);
        let before = simulation.state().clone();
        assert!(simulation.step(&bad).is_err());
        assert_eq!(simulation.state(), &before);
        let event = simulation.step(&booking).unwrap();
        assert_eq!(event.off_air.len(), 1);
        assert_eq!(event.off_air[0].planned.station, failed);
        let settlement = &event.financial.settlements[0];
        assert_eq!(settlement.contract, id);
        assert_eq!(settlement.payment, Money(0));
        assert_eq!(settlement.refund, offer.payment);
        assert_eq!(settlement.credited_delivery, 0);
        assert_eq!(
            simulation
                .state()
                .economy()
                .contract(id)
                .unwrap()
                .actual
                .relevant_delivery,
            0.0
        );
        assert_eq!(
            simulation.state().economy().budget(SNACKS),
            Some(Money(20000))
        );
        assert_eq!(simulation.state().economy().escrow(), 0);
        assert_eq!(simulation.state().economy().total_money(), total);
        assert_eq!(event.aired.broadcasts.len(), 1);
        let entries = simulation.state().economy().ledger().to_vec();
        let idle = simulation
            .step(&Slot {
                at: SimHour(22),
                broadcasts: vec![],
            })
            .unwrap();
        assert!(idle.financial.settlements.is_empty());
        assert_eq!(simulation.state().economy().ledger(), entries);
        let mut future = booking;
        future.at = SimHour(42);
        let quote = simulation
            .quote(&future, failed, SNACKS, future.at)
            .unwrap();
        assert_eq!(quote.forecast.relevant_delivery, 0.0);
        assert_eq!(quote.payment, Money(0));
        assert!(quote
            .forecast
            .audiences
            .iter()
            .all(|a| a.effectiveness.is_none()));
        assert_eq!(quote.forecast.off_air.len(), 1);
        assert!(simulation.accept_offer(&quote).is_err());
    }
}
#[test]
fn malformed_unaffordable_plans_remain_atomic_errors() {
    let mut world = demo::world();
    for s in &mut world.stations {
        s.initial_cash = Money(0);
    }
    let mut simulation = Simulation::new(world, Parameters::default()).unwrap();
    let mut invalid = Vec::new();
    let mut bad = action(18);
    bad.broadcasts[0].programme = ProgrammeId(999);
    invalid.push(bad);
    let mut bad = action(18);
    bad.broadcasts[0].advertisement = Some(BrandId(999));
    invalid.push(bad);
    let mut bad = action(18);
    bad.broadcasts.push(bad.broadcasts[0].clone());
    invalid.push(bad);
    let mut bad = action(18);
    bad.broadcasts[0].station = StationId(999);
    invalid.push(bad);
    for plan in invalid {
        let before = simulation.state().clone();
        assert!(simulation.step(&plan).is_err());
        assert_eq!(simulation.state(), &before);
    }
    assert_eq!(simulation.step(&action(18)).unwrap().off_air.len(), 2);
}
#[test]
fn controlled_saturation_and_repetition_expose_remaining_pricing_incentives() {
    let cases = advertiser_value::run().unwrap();
    assert_eq!(cases, advertiser_value::run().unwrap());
    let fresh = &cases[0];
    let saturated = &cases[1];
    let repeated = &cases[2];
    let weak = &cases[4];
    assert_eq!(fresh.offer.forecast, repeated.offer.forecast);
    assert_eq!(fresh.offer.payment, repeated.offer.payment);
    assert_eq!(fresh.offer.valuation, repeated.offer.valuation);
    assert_eq!(fresh.offer.forecast, saturated.offer.forecast);
    assert!(saturated.offer.payment < fresh.offer.payment);
    let gain = |case: &advertiser_value::CaseResult| case.event.hype[0].campaign_generated;
    assert!(gain(fresh) > gain(repeated) * 3.0);
    assert!(gain(fresh) > gain(saturated));
    assert!(weak.offer.forecast.relevant_delivery < fresh.offer.forecast.relevant_delivery);
    assert!(weak.offer.payment < fresh.offer.payment);
    assert!(weak.post_campaign_demand.pressure > fresh.post_campaign_demand.pressure);
    let ratio = |case: &advertiser_value::CaseResult| case.offer.payment.0 as f64 / gain(case);
    assert!(ratio(repeated) > ratio(fresh) * 3.0);
    for case in &cases {
        assert_eq!(case.final_state.economy().total_money(), 200000);
        assert_eq!(
            case.event.financial.settlements[0].payment,
            case.offer.payment
        );
        let _report = developer::advertiser_value_report(case);
    }
}

#[test]
fn receipts_due_this_slot_can_fund_later_slots_but_do_not_retroactively_air_a_show() {
    let mut world = demo::world();
    world.stations[0].initial_cash = world.programmes[0].broadcast_cost;
    let mut simulation = Simulation::new(world, Parameters::default()).unwrap();
    let mut first = action(18);
    first.broadcasts[0].advertisement = Some(SNACKS);
    let offer = simulation.quote(&first, MAIN, SNACKS, SimHour(21)).unwrap();
    assert!(offer.payment.0 > 150);
    simulation.accept_offer(&offer).unwrap();
    simulation.step(&first).unwrap();
    assert_eq!(simulation.state().economy().cash(MAIN), Some(Money(0)));
    let event = simulation.step(&action(21)).unwrap();
    assert_eq!(event.off_air[0].planned.station, MAIN);
    assert_eq!(event.off_air[0].available_cash, Money(0));
    assert_eq!(event.financial.settlements[0].payment, offer.payment);
    assert_eq!(simulation.state().economy().cash(MAIN), Some(offer.payment));
    let next = simulation.step(&action(22)).unwrap();
    assert!(next.aired.broadcasts.iter().any(|b| b.station == MAIN));
}
