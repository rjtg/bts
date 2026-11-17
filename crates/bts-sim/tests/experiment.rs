use bts_fuzzy::{Condition, Degree, Membership, Rule, RuleSet};
use bts_sim::{
    demo::{self, Case, EQUIPMENT, MAIN, RIVAL, SPECIALISTS, YOUNG},
    models::Models,
    *,
};

fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-8, "{a} != {b}");
}
fn first(case: Case) -> Slot {
    demo::schedule(case).remove(0)
}
fn view(event: &SlotEvent, audience: AudienceId, station: StationId) -> &Viewing {
    event
        .allocations
        .iter()
        .find(|a| a.audience == audience)
        .unwrap()
        .viewing
        .iter()
        .find(|v| v.station == station)
        .unwrap()
}
fn simulation() -> Simulation {
    Simulation::new(demo::world(), Parameters::default()).unwrap()
}

#[test]
fn population_is_partitioned_between_stations_and_not_watching() {
    for case in Case::ALL {
        for event in demo::run(case).unwrap().events {
            for allocation in &event.allocations {
                let watched = allocation.viewing.iter().map(|v| v.viewers).sum::<f64>();
                close(
                    watched + allocation.not_watching,
                    f64::from(allocation.population),
                );
                close(
                    watched + allocation.available_not_watching,
                    allocation.available,
                );
                assert!(watched <= allocation.available);
                assert!(allocation.not_watching >= 0.0);
                let stations: std::collections::BTreeSet<_> =
                    allocation.viewing.iter().map(|v| v.station).collect();
                assert_eq!(stations.len(), allocation.viewing.len());
            }
            for hype in &event.hype {
                let exposure = hype.advertisements.iter().map(|a| a.viewers).sum::<f64>();
                close(hype.exposures, exposure);
                for ad in &hype.advertisements {
                    close(ad.viewers, view(&event, hype.audience, ad.station).viewers);
                }
            }
        }
    }
}

#[test]
fn audiences_disagree_and_niche_schedules_trade_reach_for_relevance() {
    let broad = demo::run(Case::Broad).unwrap();
    let target = demo::run(Case::Targeted).unwrap();
    let young = view(&target.events[0], YOUNG, MAIN);
    let specialist = view(&target.events[0], SPECIALISTS, MAIN);
    assert!(specialist.attraction.trace.output > young.attraction.trace.output + 0.2);
    assert!(specialist.satisfaction.trace.output > young.satisfaction.trace.output);
    let broad_report = developer::report(&broad);
    let target_report = developer::report(&target);
    assert!(
        broad_report.summary.stations[0].viewer_slot_impressions
            > target_report.summary.stations[0].viewer_slot_impressions
    );
    let specialist_hype = |report: &developer::DeveloperReport| {
        report
            .summary
            .brands
            .iter()
            .find(|b| b.brand == EQUIPMENT)
            .unwrap()
            .specialist_hype_after_broadcasts
    };
    assert!(specialist_hype(&target_report) > specialist_hype(&broad_report));
    // Every case has exactly the same config and competitor/ad/time actions.
    assert_eq!(broad.world, target.world);
    for (a, b) in broad.events.iter().zip(&target.events) {
        assert_eq!(a.action.at, b.action.at);
        assert_eq!(
            a.action.broadcasts.iter().find(|b| b.station == RIVAL),
            b.action.broadcasts.iter().find(|b| b.station == RIVAL)
        );
        assert_eq!(
            a.action
                .broadcasts
                .iter()
                .find(|b| b.station == MAIN)
                .map(|b| b.advertisement),
            b.action
                .broadcasts
                .iter()
                .find(|b| b.station == MAIN)
                .map(|b| b.advertisement)
        );
    }
}

#[test]
fn promotion_attracts_without_satisfaction_and_return_willingness_declines() {
    let run = demo::run(Case::Disappointing).unwrap();
    let initial = view(&run.events[0], YOUNG, MAIN);
    assert!(initial.attraction.trace.output > 0.9);
    assert!(initial.satisfaction.trace.output < 0.35);
    assert!(initial.loyalty_after < initial.loyalty_before);
    let final_early = view(&run.events[10], YOUNG, MAIN);
    assert!(final_early.viewers < initial.viewers * 0.85);
    assert!(final_early.loyalty_after < initial.loyalty_after);
    assert_eq!(initial.attraction, final_early.attraction); // Return changes independently.
    let models = Models::default();
    let low = models.satisfaction(1.0, 0.9, 0.05).unwrap();
    let high = models.satisfaction(1.0, 0.9, 0.9).unwrap();
    assert!(high.trace.output > low.trace.output + 0.5);
}

#[test]
fn targeting_changes_hype_at_controlled_reach() {
    let mut interested = demo::world();
    interested.audiences[0].product_interests[1] = 0.9;
    interested.audiences[0].purchasing_power = 0.95;
    let mut uninterested = interested.clone();
    uninterested.audiences[0].product_interests[1] = 0.05;
    let slot = first(Case::Broad);
    let high = Simulation::new(interested, Parameters::default())
        .unwrap()
        .step(&slot)
        .unwrap();
    let low = Simulation::new(uninterested, Parameters::default())
        .unwrap()
        .step(&slot)
        .unwrap();
    assert_eq!(high.allocations, low.allocations);
    let change = |event: &SlotEvent| {
        event
            .hype
            .iter()
            .find(|h| h.audience == YOUNG && h.brand == EQUIPMENT)
            .unwrap()
            .clone()
    };
    let high = change(&high);
    let low = change(&low);
    assert_eq!(high.exposures, low.exposures);
    assert!(high.campaign_generated > low.campaign_generated * 5.0);
    let models = Models::default();
    assert!(
        models.advertising(0.9, 0.9, 0.8, 0.7).unwrap().trace.output
            > models.advertising(0.9, 0.1, 0.8, 0.7).unwrap().trace.output
    );
}

#[test]
fn no_viewers_or_no_ads_means_zero_exposure_driven_hype() {
    let mut world = demo::world();
    for audience in &mut world.audiences {
        audience.availability = [0.0; 24];
    }
    let mut sim = Simulation::new(world, Parameters::default()).unwrap();
    let event = sim.step(&first(Case::Broad)).unwrap();
    assert!(event
        .hype
        .iter()
        .flat_map(|h| &h.advertisements)
        .any(|ad| ad.effectiveness.trace.output > 0.0));
    for h in event.hype {
        assert_eq!(h.exposures, 0.0);
        assert_eq!(h.campaign_generated, 0.0);
        assert_eq!(h.after, h.decayed);
    }
    let mut slot = first(Case::Broad);
    for broadcast in &mut slot.broadcasts {
        broadcast.advertisement = None;
    }
    for h in simulation().step(&slot).unwrap().hype {
        assert_eq!(h.exposures, 0.0);
        assert_eq!(h.campaign_generated, 0.0);
    }
    assert_eq!(
        Models::default()
            .advertising(0.0, 1.0, 1.0, 1.0)
            .unwrap()
            .trace
            .output,
        0.0
    );
}

#[test]
fn idle_hype_decays_by_elapsed_time_and_saturation_and_repetition_are_bounded() {
    let mut sim = simulation();
    let a = sim
        .step(&Slot {
            at: SimHour(18),
            broadcasts: vec![],
        })
        .unwrap();
    let b = sim
        .step(&Slot {
            at: SimHour(66),
            broadcasts: vec![],
        })
        .unwrap();
    for (a, b) in a.hype.iter().zip(&b.hype) {
        close(b.after, a.after * 0.5);
        assert_eq!(b.campaign_generated, 0.0);
    }
    let mut world = demo::world();
    for a in &mut world.audiences {
        a.availability = [1.0; 24];
    }
    let parameters = Parameters {
        loyalty_learning: 0.0,
        ..Parameters::default()
    };
    let mut sim = Simulation::new(world, parameters).unwrap();
    let mut previous_response = None;
    for day in 0..200 {
        let mut slot = first(Case::Targeted);
        slot.at = SimHour(day * 24 + 18);
        let event = sim.step(&slot).unwrap();
        for h in &event.hype {
            assert!((0.0..=100.0).contains(&h.after));
            assert!(h.repetition_after.is_finite());
        }
        let h = event
            .hype
            .iter()
            .find(|h| h.brand == EQUIPMENT && h.audience == SPECIALISTS)
            .unwrap();
        let response = h.campaign_generated / (100.0 - h.decayed);
        if day == 1 {
            assert!(response < previous_response.unwrap());
        }
        previous_response = Some(response);
    }
}

#[test]
fn complementary_memberships_produce_gradual_responses_not_activation_scaled_outputs() {
    let high = Condition::is((), "high", Membership::rising(0.0, 1.0).unwrap());
    let lone = RuleSet::new(
        vec![Rule::new("lone", high.clone(), Degree::ONE, 1.0).unwrap()],
        0.0,
    )
    .unwrap();
    let paired = RuleSet::new(
        vec![
            Rule::new("low", high.clone().negate(), Degree::ONE, 0.0).unwrap(),
            Rule::new("high", high, Degree::ONE, 1.0).unwrap(),
        ],
        0.0,
    )
    .unwrap();
    assert_eq!(lone.evaluate(&[((), 0.25)]).unwrap().output, 1.0);
    assert_eq!(paired.evaluate(&[((), 0.25)]).unwrap().output, 0.25);
    let models = Models::default();
    for i in 0..=100 {
        let x = f64::from(i) / 100.0;
        close(models.attraction(x, x, x, x).unwrap().trace.output, x);
        close(models.satisfaction(x, x, x).unwrap().trace.output, x);
        close(
            models.advertising(x, 1.0, 1.0, 1.0).unwrap().trace.output,
            x,
        );
    }
}

#[test]
fn reproducible_runs_and_report_construction_is_read_only() {
    for case in Case::ALL {
        let a = demo::run(case).unwrap();
        let b = demo::run(case).unwrap();
        assert_eq!(a, b);
        let before = a.clone();
        let _report = developer::report(&a);
        assert_eq!(a, before);
    }
    let mut a = simulation();
    let mut b = simulation();
    let slot = first(Case::Broad);
    let mut reversed = slot.clone();
    reversed.broadcasts.reverse();
    let ea = a.step(&slot).unwrap();
    let eb = b.step(&reversed).unwrap();
    assert_eq!(ea.allocations, eb.allocations);
    assert_eq!(ea.hype, eb.hype);
    assert_eq!(a.state(), b.state());
}

#[test]
fn simultaneous_same_series_uses_one_snapshot_and_aggregate_loyalty_update() {
    let mut sim = simulation();
    let mut slot = first(Case::Broad);
    slot.broadcasts[1].programme = slot.broadcasts[0].programme;
    slot.broadcasts[1].advertisement = slot.broadcasts[0].advertisement;
    let event = sim.step(&slot).unwrap();
    for a in &event.allocations {
        assert_eq!(a.viewing[0].loyalty_before, 0.5);
        assert_eq!(a.viewing[1].loyalty_before, 0.5);
        close(a.viewing[0].viewers, a.viewing[1].viewers);
        close(a.viewing[0].loyalty_after, a.viewing[1].loyalty_after);
        let expected = 0.5
            + 0.9
                * a.viewing
                    .iter()
                    .map(|v| {
                        v.viewers / f64::from(a.population) * (v.satisfaction.trace.output - 0.5)
                    })
                    .sum::<f64>();
        close(a.viewing[0].loyalty_after, expected);
    }
    for h in event.hype.iter().filter(|h| h.brand == EQUIPMENT) {
        let a = event
            .allocations
            .iter()
            .find(|a| a.audience == h.audience)
            .unwrap();
        close(h.exposures, a.viewing.iter().map(|v| v.viewers).sum());
        assert!(h.exposures <= a.available);
    }
}

#[test]
fn invalid_actions_are_atomic_and_configuration_is_validated() {
    let mut sim = simulation();
    let good = first(Case::Broad);
    let mut invalid = vec![
        Slot {
            at: SimHour(0),
            broadcasts: vec![],
        },
        Slot {
            at: SimHour(0),
            broadcasts: vec![],
        },
    ];
    let mut duplicate = good.clone();
    duplicate.broadcasts.push(duplicate.broadcasts[0].clone());
    invalid.push(duplicate);
    let mut bad = good.clone();
    bad.broadcasts[0].programme = ProgrammeId(999);
    invalid.push(bad);
    let mut bad = good.clone();
    bad.broadcasts[0].station = StationId(999);
    invalid.push(bad);
    let mut bad = good.clone();
    bad.broadcasts[0].advertisement = Some(BrandId(999));
    invalid.push(bad);
    for action in invalid {
        let old = sim.state().clone();
        assert!(sim.step(&action).is_err());
        assert_eq!(sim.state(), &old);
    }
    sim.step(&good).unwrap();
    let state = sim.state().clone();
    assert!(sim.step(&good).is_err());
    assert_eq!(sim.state(), &state);
    for bad in [f64::NAN, f64::INFINITY, -0.1, 1.1] {
        let mut world = demo::world();
        world.audiences[0].availability[18] = bad;
        assert!(Simulation::new(world, Parameters::default()).is_err());
        assert!(Models::default().attraction(bad, 0.5, 0.5, 0.5).is_err());
    }
    let mut world = demo::world();
    world.programmes[0].execution = f64::NAN;
    assert!(Simulation::new(world, Parameters::default()).is_err());
    let mut world = demo::world();
    world.brands[0].required_purchasing_power = 0.0;
    assert!(Simulation::new(world, Parameters::default()).is_err());
    let mut world = demo::world();
    world.audiences[1].id = world.audiences[0].id;
    assert!(Simulation::new(world, Parameters::default()).is_err());
    for half_life in [0.0, f64::NAN, f64::INFINITY] {
        assert!(Simulation::new(
            demo::world(),
            Parameters {
                hype_half_life_hours: half_life,
                ..Parameters::default()
            }
        )
        .is_err());
    }
}
