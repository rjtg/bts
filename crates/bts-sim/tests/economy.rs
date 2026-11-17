use bts_sim::{
    demo::{self, Case, EQUIPMENT, MAIN, RIVAL, SNACKS},
    economy::*,
    economy_demo,
    models::Models,
    *,
};

fn sim() -> Simulation {
    Simulation::new(demo::world(), Parameters::default()).unwrap()
}
fn slot(case: Case, brand: BrandId, at: u32) -> Slot {
    let mut action = demo::schedule(case).remove(0);
    action.at = SimHour(at);
    action
        .broadcasts
        .iter_mut()
        .find(|b| b.station == MAIN)
        .unwrap()
        .advertisement = Some(brand);
    action
}
fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "{a} != {b}");
}
fn arbitrary_world(audiences: usize, stations: usize, brands: usize, programmes: usize) -> World {
    let fixture = demo::world();
    World {
        audiences: (0..audiences)
            .map(|i| {
                let mut a = fixture.audiences[i % 3].clone();
                a.id = AudienceId(i as u16);
                a.availability = [0.5; 24];
                a
            })
            .collect(),
        stations: (0..stations)
            .map(|i| {
                let mut s = fixture.stations[0].clone();
                s.id = StationId(i as u16);
                s
            })
            .collect(),
        programmes: (0..programmes)
            .map(|i| {
                let mut p = fixture.programmes[i % 4].clone();
                p.id = ProgrammeId(i as u16);
                p.series = SeriesId(i as u16);
                p
            })
            .collect(),
        brands: (0..brands)
            .map(|i| {
                let mut b = fixture.brands[i % 2].clone();
                b.id = BrandId(i as u16);
                b.targets = (0..audiences)
                    .map(|a| Target {
                        audience: AudienceId(a as u16),
                        priority: 1.0,
                        desired_hype: 50.0,
                    })
                    .collect();
                b
            })
            .collect(),
    }
}
#[test]
fn varied_sizes_all_empty_collections_and_arbitrary_hours_are_valid() {
    for (a, s, b, p) in [
        (0, 0, 0, 0),
        (1, 1, 1, 1),
        (5, 4, 3, 6),
        (2, 0, 2, 0),
        (1, 2, 0, 0),
        (0, 2, 1, 1),
        (3, 2, 0, 2),
    ] {
        let world = arbitrary_world(a, s, b, p);
        let mut simulation = Simulation::new(world, Parameters::default()).unwrap();
        let broadcasts = if p > 0 {
            (0..s)
                .map(|i| Broadcast {
                    station: StationId(i as u16),
                    programme: ProgrammeId(0),
                    advertisement: None,
                })
                .collect()
        } else {
            vec![]
        };
        let event = simulation
            .step(&Slot {
                at: SimHour(3),
                broadcasts,
            })
            .unwrap();
        assert_eq!(event.allocations.len(), a);
        assert_eq!(event.hype.len(), a * b);
        for allocation in event.allocations {
            close(
                allocation.viewing.iter().map(|v| v.viewers).sum::<f64>() + allocation.not_watching,
                f64::from(allocation.population),
            );
            close(allocation.available, f64::from(allocation.population) * 0.5);
        }
    }
    let mut world = arbitrary_world(1, 1, 0, 0);
    let mut simulation = Simulation::new(world.clone(), Parameters::default()).unwrap();
    assert!(simulation
        .step(&Slot {
            at: SimHour(1),
            broadcasts: vec![Broadcast {
                station: StationId(0),
                programme: ProgrammeId(0),
                advertisement: None
            }]
        })
        .is_err());
    world.brands = arbitrary_world(0, 0, 1, 0).brands;
    let simulation = Simulation::new(world, Parameters::default()).unwrap();
    assert_eq!(simulation.demand(BrandId(0)).unwrap().pressure, 0.0);
    let mut empty_audience =
        Simulation::new(arbitrary_world(0, 1, 1, 1), Parameters::default()).unwrap();
    let action = Slot {
        at: SimHour(1),
        broadcasts: vec![Broadcast {
            station: StationId(0),
            programme: ProgrammeId(0),
            advertisement: Some(BrandId(0)),
        }],
    };
    let offer = empty_audience
        .quote(&action, StationId(0), BrandId(0), action.at)
        .unwrap();
    assert_eq!(offer.payment, Money(0));
    assert_eq!(offer.promised_delivery, 0);
    assert!(empty_audience.accept_offer(&offer).is_err());
}
#[test]
fn creative_quality_is_required_and_every_ad_factor_has_an_independent_effect() {
    let models = Models::default();
    let effect = |i, a, c, s| models.advertising(i, a, c, s).unwrap().trace.output;
    assert_eq!(effect(1.0, 1.0, 0.0, 1.0), 0.0);
    close(effect(1.0, 1.0, 1.0, 0.0), 0.75);
    assert!(effect(1.0, 1.0, 0.8, 0.5) > effect(1.0, 1.0, 0.2, 0.5));
    assert!(effect(1.0, 1.0, 0.8, 1.0) > effect(1.0, 1.0, 0.8, 0.0));
    assert!(effect(0.8, 1.0, 1.0, 1.0) > effect(0.2, 1.0, 1.0, 1.0));
    assert!(effect(1.0, 0.8, 1.0, 1.0) > effect(1.0, 0.2, 1.0, 1.0));
    let mut world = demo::world();
    world.brands[0].creative_quality = 0.0;
    let mut simulation = Simulation::new(world, Parameters::default()).unwrap();
    let action = slot(Case::Broad, SNACKS, 18);
    let offer = simulation.quote(&action, MAIN, SNACKS, action.at).unwrap();
    assert_eq!(offer.forecast.relevant_delivery, 0.0);
    assert_eq!(offer.payment, Money(0));
    let event = simulation.step(&action).unwrap();
    for hype in event.hype.iter().filter(|h| h.brand == SNACKS) {
        assert!(hype.exposures > 0.0);
        assert_eq!(hype.campaign_generated, 0.0);
    }
    for creative in [0.0, 0.25, 0.5, 1.0] {
        for satisfaction in [0.0, 0.5, 1.0] {
            assert!(effect(1.0, 1.0, creative, satisfaction) <= creative);
        }
    }
}
#[test]
fn justified_interactions_distinguish_inputs_with_equal_old_linear_scores() {
    let models = Models::default();
    // .35*1+.25*0 == .35*.5+.25*.7, but only the second combines fit and cast.
    let one_sided = models.attraction(1.0, 0.0, 0.0, 0.0).unwrap();
    let joint = models.attraction(0.5, 0.0, 0.0, 0.7).unwrap();
    close(0.35, 0.35 * 0.5 + 0.25 * 0.7);
    assert!(joint.trace.output > one_sided.trace.output + 0.09);
    // Both old satisfaction scores are .5; matched craft adds an interaction.
    let matched = models.satisfaction(1.0, 0.0, 0.5).unwrap();
    let mismatched = models.satisfaction(0.0, 0.0, 5.0 / 7.0).unwrap();
    close(0.15 + 0.7 * 0.5, 0.7 * (5.0 / 7.0));
    assert!(matched.trace.output > mismatched.trace.output + 0.1);
    assert!(matched
        .trace
        .rules
        .iter()
        .any(|r| r.name.contains("execution with genre fit")));
}
#[test]
fn financial_conservation_and_exact_ledger_balances_at_every_action() {
    use Account::*;
    let world = demo::world();
    let mut simulation = sim();
    let initial = simulation.state().economy().total_money();
    let mut balances = std::collections::BTreeMap::<Account, i128>::new();
    for b in &world.brands {
        balances.insert(Brand(b.id), i128::from(b.marketing_budget.0));
    }
    for s in &world.stations {
        balances.insert(Station(s.id), i128::from(s.initial_cash.0));
    }
    for day in 0..10 {
        let action = slot(Case::Broad, SNACKS, day * 24 + 18);
        let offer = simulation.quote(&action, MAIN, SNACKS, action.at).unwrap();
        if offer.payment.0 > 0 {
            simulation.accept_offer(&offer).unwrap();
        }
        assert_eq!(simulation.state().economy().total_money(), initial);
        simulation.step(&action).unwrap();
        assert_eq!(simulation.state().economy().total_money(), initial);
    }
    let book = simulation.state().economy();
    for entry in book.ledger() {
        assert_ne!(entry.from, entry.to);
        assert!(entry.amount.0 > 0);
        *balances.entry(entry.from).or_default() -= i128::from(entry.amount.0);
        *balances.entry(entry.to).or_default() += i128::from(entry.amount.0);
        assert!(balances.values().all(|v| *v >= 0));
    }
    for b in &world.brands {
        assert_eq!(
            balances[&Brand(b.id)],
            i128::from(book.budget(b.id).unwrap().0)
        );
    }
    for s in &world.stations {
        assert_eq!(
            balances[&Station(s.id)],
            i128::from(book.cash(s.id).unwrap().0)
        );
    }
    assert_eq!(balances[&BroadcastCosts], i128::from(book.cost_sink().0));
    assert_eq!(book.escrow(), 0);
    for c in book.contracts() {
        assert_eq!(balances[&Escrow(c.id)], 0);
    }
}
#[test]
fn budgets_are_reserved_offers_are_capped_and_inventory_cannot_be_sold_twice() {
    let mut world = demo::world();
    world.brands[0].marketing_budget = Money(10);
    let mut simulation = Simulation::new(world, Parameters::default()).unwrap();
    let action = slot(Case::Broad, SNACKS, 18);
    let offer = simulation
        .quote(&action, MAIN, SNACKS, SimHour(21))
        .unwrap();
    assert_eq!(offer.payment, Money(10));
    let mut other = action.clone();
    other.broadcasts[0].advertisement = Some(EQUIPMENT);
    let competing = simulation
        .quote(&other, MAIN, EQUIPMENT, SimHour(21))
        .unwrap();
    let id = simulation.accept_offer(&offer).unwrap();
    assert_eq!(simulation.state().economy().budget(SNACKS), Some(Money(0)));
    assert_eq!(simulation.state().economy().escrow(), 10);
    let before = simulation.state().clone();
    assert!(simulation.accept_offer(&offer).is_err());
    assert!(simulation.accept_offer(&competing).is_err());
    assert_eq!(simulation.state(), &before);
    let future = slot(Case::Broad, SNACKS, 42);
    let exhausted = simulation.quote(&future, MAIN, SNACKS, future.at).unwrap();
    assert!(exhausted.demand.pressure > 0.0);
    assert_eq!(exhausted.payment, Money(0));
    assert!(simulation.accept_offer(&exhausted).is_err());
    assert!(matches!(
        simulation.state().economy().contract(id).unwrap().status,
        ContractStatus::Reserved
    ));
    let mut world = demo::world();
    world.brands[0].maximum_offer = Money(7);
    let capped = Simulation::new(world, Parameters::default())
        .unwrap()
        .quote(&action, MAIN, SNACKS, action.at)
        .unwrap();
    assert_eq!(capped.payment, Money(7));
}
#[test]
fn forecast_is_read_only_actual_delivery_is_independent_and_deadline_settles_once() {
    let mut simulation = sim();
    let proposed = slot(Case::Broad, SNACKS, 18);
    let before = simulation.state().clone();
    let offer = simulation
        .quote(&proposed, MAIN, SNACKS, SimHour(21))
        .unwrap();
    assert_eq!(simulation.state(), &before);
    let id = simulation.accept_offer(&offer).unwrap();
    let actual = slot(Case::Targeted, SNACKS, 18);
    let event = simulation.step(&actual).unwrap();
    assert!(event.financial.settlements.is_empty());
    let c = simulation.state().economy().contract(id).unwrap();
    assert!(c.actual.credited_units < c.offer.promised_delivery);
    assert!(c.actual.relevant_delivery < c.offer.forecast.relevant_delivery);
    assert_eq!(
        simulation.state().economy().escrow(),
        u128::from(offer.payment.0)
    );
    let before_deadline = simulation
        .step(&Slot {
            at: SimHour(20),
            broadcasts: vec![],
        })
        .unwrap();
    assert!(before_deadline.financial.settlements.is_empty());
    let event = simulation
        .step(&Slot {
            at: SimHour(21),
            broadcasts: vec![],
        })
        .unwrap();
    assert_eq!(event.financial.settlements.len(), 1);
    let settlement = &event.financial.settlements[0];
    assert!(settlement.payment.0 > 0);
    assert!(settlement.payment < offer.payment);
    assert!(settlement.refund.0 > 0);
    assert_eq!(settlement.payment.0 + settlement.refund.0, offer.payment.0);
    assert_eq!(
        u128::from(settlement.payment.0),
        u128::from(offer.payment.0) * u128::from(settlement.credited_delivery)
            / u128::from(offer.promised_delivery)
    );
    let balances = simulation.state().economy().clone();
    let event = simulation
        .step(&Slot {
            at: SimHour(22),
            broadcasts: vec![],
        })
        .unwrap();
    assert!(event.financial.settlements.is_empty());
    assert_eq!(simulation.state().economy().ledger(), balances.ledger());
    assert_eq!(simulation.state().economy().escrow(), 0);
}
#[test]
fn skipped_or_omitted_advertisements_refund_and_overdelivery_earns_no_bonus() {
    for skip in [false, true] {
        let mut simulation = sim();
        let action = slot(Case::Broad, SNACKS, 18);
        let offer = simulation
            .quote(&action, MAIN, SNACKS, SimHour(21))
            .unwrap();
        simulation.accept_offer(&offer).unwrap();
        let event = if skip {
            simulation
                .step(&Slot {
                    at: SimHour(25),
                    broadcasts: vec![],
                })
                .unwrap()
        } else {
            let mut actual = action;
            actual.broadcasts[0].advertisement = None;
            simulation.step(&actual).unwrap();
            simulation
                .step(&Slot {
                    at: SimHour(21),
                    broadcasts: vec![],
                })
                .unwrap()
        };
        assert_eq!(event.financial.settlements[0].payment, Money(0));
        assert_eq!(event.financial.settlements[0].refund, offer.payment);
        assert_eq!(
            simulation.state().economy().budget(SNACKS),
            Some(Money(20000))
        );
    }
    let mut simulation = sim();
    let proposal = slot(Case::Targeted, SNACKS, 18);
    let offer = simulation
        .quote(&proposal, MAIN, SNACKS, proposal.at)
        .unwrap();
    simulation.accept_offer(&offer).unwrap();
    let event = simulation.step(&slot(Case::Broad, SNACKS, 18)).unwrap();
    let settlement = &event.financial.settlements[0];
    assert!(settlement.credited_delivery > settlement.promised_delivery);
    assert_eq!(settlement.payment, offer.payment);
    assert_eq!(settlement.refund, Money(0));
}
#[test]
fn stale_modified_conflicting_and_unfunded_actions_are_atomic() {
    let mut simulation = sim();
    let action = slot(Case::Broad, SNACKS, 18);
    assert!(simulation
        .quote(&action, MAIN, SNACKS, SimHour(17))
        .is_err());
    let offer = simulation.quote(&action, MAIN, SNACKS, action.at).unwrap();
    let mut modified = offer.clone();
    modified.payment = Money(1);
    let old = simulation.state().clone();
    assert!(simulation.accept_offer(&modified).is_err());
    assert_eq!(simulation.state(), &old);
    simulation
        .step(&Slot {
            at: SimHour(1),
            broadcasts: vec![],
        })
        .unwrap();
    let old = simulation.state().clone();
    assert!(simulation.accept_offer(&offer).is_err());
    assert_eq!(simulation.state(), &old);
    let offer = simulation.quote(&action, MAIN, SNACKS, action.at).unwrap();
    simulation.accept_offer(&offer).unwrap();
    let mut conflicting = action.clone();
    conflicting.broadcasts[0].advertisement = Some(EQUIPMENT);
    let old = simulation.state().clone();
    assert!(simulation.step(&conflicting).is_err());
    assert_eq!(simulation.state(), &old);
    let mut world = demo::world();
    world.stations[1].initial_cash = Money(0);
    let mut simulation = Simulation::new(world, Parameters::default()).unwrap();
    let offer = simulation.quote(&action, MAIN, SNACKS, action.at).unwrap();
    simulation.accept_offer(&offer).unwrap();
    let old = simulation.state().clone();
    let event = simulation.step(&action).unwrap();
    assert_eq!(event.off_air.len(), 1);
    assert_eq!(event.off_air[0].planned.station, RIVAL);
    assert_eq!(event.aired.broadcasts.len(), 1);
    assert_eq!(simulation.state().now(), action.at);
    assert_eq!(
        simulation.state().economy().total_money(),
        old.economy().total_money()
    );
}
#[test]
fn targeting_priorities_and_deficits_are_inspectable_and_bounded() {
    let action = slot(Case::Broad, SNACKS, 18);
    let simulation = sim();
    let offer = simulation.quote(&action, MAIN, SNACKS, action.at).unwrap();
    close(
        offer
            .demand
            .targets
            .iter()
            .map(|t| t.weighted_deficit)
            .sum::<f64>()
            / offer.demand.total_priority,
        offer.demand.pressure,
    );
    for t in &offer.demand.targets {
        close(
            t.deficit,
            (t.desired_hype - t.current_hype).max(0.0) / 100.0,
        );
    }
    let mut world = demo::world();
    world.brands[0]
        .targets
        .iter_mut()
        .for_each(|t| t.priority = 0.0);
    let quote = Simulation::new(world, Parameters::default())
        .unwrap()
        .quote(&action, MAIN, SNACKS, action.at)
        .unwrap();
    assert_eq!(quote.payment, Money(0));
    let mut world = demo::world();
    world.brands[0]
        .targets
        .iter_mut()
        .for_each(|t| t.desired_hype = 0.0);
    let quote = Simulation::new(world, Parameters::default())
        .unwrap()
        .quote(&action, MAIN, SNACKS, action.at)
        .unwrap();
    assert_eq!(quote.demand.pressure, 0.0);
    assert_eq!(quote.payment, Money(0));
    let mut world = demo::world();
    world.brands[0].targets[0].priority = 0.0;
    let quote = Simulation::new(world, Parameters::default())
        .unwrap()
        .quote(&action, MAIN, SNACKS, action.at)
        .unwrap();
    assert!(quote.forecast.relevant_delivery < offer.forecast.relevant_delivery);
    for bad in [f64::NAN, -1.0, 101.0] {
        let mut world = demo::world();
        world.brands[0].targets[0].desired_hype = bad;
        assert!(Simulation::new(world, Parameters::default()).is_err());
    }
    let mut world = demo::world();
    world.brands[0].targets[0].audience = AudienceId(999);
    assert!(Simulation::new(world, Parameters::default()).is_err());
    let mut world = demo::world();
    world.brands[0].marketing_budget = Money(Money::MAX_CONFIGURED + 1);
    assert!(Simulation::new(world, Parameters::default()).is_err());
}
#[test]
fn campaigns_reduce_demand_decay_restores_it_without_replenishing_budget_and_runs_reproduce() {
    for case in economy_demo::Case::ALL {
        let a = economy_demo::run(case).unwrap();
        let b = economy_demo::run(case).unwrap();
        assert_eq!(a, b);
        let first = &a.rounds[0].offer;
        let last = &a.rounds.last().unwrap().offer;
        assert!(last.demand.pressure < first.demand.pressure);
        assert!(a.demand_after_campaigns.pressure < first.demand.pressure);
        assert!(a.recovery.demand.pressure > a.demand_after_campaigns.pressure);
        assert!(a.recovery.payment.0 > 0);
        assert_eq!(a.before_cooling_quote.forecast, a.recovery.forecast);
        assert_eq!(
            a.before_cooling_quote.demand.available_budget,
            a.recovery.demand.available_budget
        );
        assert!(a.recovery.payment > a.before_cooling_quote.payment);
        assert!(a.recovery.payment <= a.recovery.demand.available_budget);
        let before = a.rounds.last().unwrap().event.action.at;
        assert_eq!(a.cooldown.action.at.0 - before.0, 48);
        for h in &a.cooldown.hype {
            assert_eq!(h.campaign_generated, 0.0);
            close(h.after, h.before * 0.5);
        }
        let book = a.experiment.final_state.economy();
        assert_eq!(book.total_money(), 80000);
        assert!(book.budget(case.brand()).unwrap().0 < 20000);
        let snapshot = a.clone();
        let _report = developer::economy_report(&a);
        assert_eq!(a, snapshot);
    }
    // At fixed programme inputs, priority and creative data the only difference
    // after the cooldown is elapsed time and hype/repetition; no money is added.
    let mut simulation = sim();
    let action = slot(Case::Broad, SNACKS, 18);
    let offer = simulation.quote(&action, MAIN, SNACKS, action.at).unwrap();
    simulation.accept_offer(&offer).unwrap();
    simulation.step(&action).unwrap();
    let budget = simulation.state().economy().budget(SNACKS);
    let demand = simulation.demand(SNACKS).unwrap().pressure;
    simulation
        .step(&Slot {
            at: SimHour(66),
            broadcasts: vec![],
        })
        .unwrap();
    assert!(simulation.demand(SNACKS).unwrap().pressure > demand);
    assert_eq!(simulation.state().economy().budget(SNACKS), budget);
}
#[test]
fn controlled_mass_and_specialist_comparisons_expose_reach_profit_tradeoffs() {
    let broad =
        developer::economy_report(&economy_demo::run(economy_demo::Case::SpecialistBroad).unwrap());
    let targeted = developer::economy_report(
        &economy_demo::run(economy_demo::Case::SpecialistTargeted).unwrap(),
    );
    assert_eq!(
        broad.broadcasts.initial_world,
        targeted.broadcasts.initial_world
    );
    assert!(
        broad.broadcasts.summary.stations[0].viewer_slot_impressions
            > targeted.broadcasts.summary.stations[0].viewer_slot_impressions
    );
    assert!(
        targeted.offers[0].forecast_relevant_delivery > broad.offers[0].forecast_relevant_delivery
    );
    assert!(targeted.finances.stations[0].profit > broad.finances.stations[0].profit);
    assert!(targeted.finances.stations[0].profit > 0);
    let mass =
        developer::economy_report(&economy_demo::run(economy_demo::Case::MassBroad).unwrap());
    assert!(mass.finances.stations[0].revenue > 0);
    assert!(mass.finances.stations[0].profit > 0);
    assert_eq!(mass.finances.stations[1].station, RIVAL);
}

#[test]
fn spent_budget_is_not_replenished_by_decay() {
    let mut world = demo::world();
    world.brands[0].marketing_budget = Money(10);
    let mut simulation = Simulation::new(world, Parameters::default()).unwrap();
    let action = slot(Case::Broad, SNACKS, 18);
    let offer = simulation.quote(&action, MAIN, SNACKS, action.at).unwrap();
    assert_eq!(offer.payment, Money(10));
    simulation.accept_offer(&offer).unwrap();
    simulation.step(&action).unwrap();
    assert_eq!(simulation.state().economy().budget(SNACKS), Some(Money(0)));
    assert_eq!(simulation.state().economy().escrow(), 0);
    let demand = simulation.demand(SNACKS).unwrap().pressure;
    simulation
        .step(&Slot {
            at: SimHour(66),
            broadcasts: vec![],
        })
        .unwrap();
    assert!(simulation.demand(SNACKS).unwrap().pressure > demand);
    let future = slot(Case::Broad, SNACKS, 90);
    let offer = simulation.quote(&future, MAIN, SNACKS, future.at).unwrap();
    assert!(offer.demand.pressure > 0.0);
    assert_eq!(offer.payment, Money(0));
    assert!(simulation.accept_offer(&offer).is_err());
}

#[test]
fn changing_only_costs_can_reverse_profit_ranking_without_affecting_delivery() {
    let reference = economy_demo::run(economy_demo::Case::MassBroad).unwrap();
    let targeted =
        developer::economy_report(&economy_demo::run(economy_demo::Case::MassTargeted).unwrap());
    let normal = developer::economy_report(&reference);
    assert!(normal.finances.stations[0].profit < targeted.finances.stations[0].profit);
    let mut world = reference.experiment.world.clone();
    world.programmes[0].broadcast_cost = Money(0);
    let mut simulation = Simulation::new(world, Parameters::default()).unwrap();
    for round in &reference.rounds {
        let proposal = round.offer.proposal();
        let offer = simulation
            .quote(proposal, MAIN, SNACKS, proposal.at)
            .unwrap();
        if offer.payment.0 > 0 {
            simulation.accept_offer(&offer).unwrap();
        }
        let event = simulation.step(&round.event.action).unwrap();
        assert_eq!(event.allocations, round.event.allocations);
        assert_eq!(event.hype, round.event.hype);
    }
    let profit = i128::from(simulation.state().economy().cash(MAIN).unwrap().0) - 20000;
    assert_eq!(profit, normal.finances.stations[0].revenue as i128);
    assert!(profit > targeted.finances.stations[0].profit);
}

#[test]
fn simultaneous_distinct_ad_slots_credit_only_their_own_allocated_viewers() {
    let mut simulation = sim();
    let mut action = slot(Case::Broad, SNACKS, 18);
    action.broadcasts[1].advertisement = Some(SNACKS);
    let a = simulation.quote(&action, MAIN, SNACKS, action.at).unwrap();
    let first = simulation.accept_offer(&a).unwrap();
    let b = simulation.quote(&action, RIVAL, SNACKS, action.at).unwrap();
    let second = simulation.accept_offer(&b).unwrap();
    let event = simulation.step(&action).unwrap();
    assert_eq!(event.financial.settlements.len(), 2);
    for (id, station) in [(first, MAIN), (second, RIVAL)] {
        let contract = simulation.state().economy().contract(id).unwrap();
        for delivered in &contract.actual.audiences {
            let allocation = event
                .allocations
                .iter()
                .find(|a| a.audience == delivered.audience)
                .unwrap();
            let viewing = allocation
                .viewing
                .iter()
                .find(|v| v.station == station)
                .unwrap();
            assert_eq!(delivered.viewers, viewing.viewers);
        }
    }
    assert_eq!(simulation.state().economy().total_money(), 80000);
    for h in event.hype.iter().filter(|h| h.brand == SNACKS) {
        let allocation = event
            .allocations
            .iter()
            .find(|a| a.audience == h.audience)
            .unwrap();
        close(
            h.exposures,
            allocation.viewing.iter().map(|v| v.viewers).sum(),
        );
        assert!(h.exposures <= allocation.available);
    }
}
