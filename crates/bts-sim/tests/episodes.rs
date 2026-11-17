use bts_sim::{
    economy::Money,
    episodes_demo::{self, ARC, FIRST, MOVIE, SECOND, THIRD},
    *,
};
fn view(run: &demo::ExperimentRun, index: usize) -> &Viewing {
    &run.events[index].allocations[0].viewing[0]
}
fn action(world: &World, id: ProgrammeId, at: u32) -> Slot {
    Slot {
        at: SimHour(at),
        broadcasts: vec![Broadcast {
            station: world.stations[0].id,
            programme: id,
            advertisement: None,
        }],
    }
}
#[test]
fn invalid_series_links_sequences_and_continuity_are_rejected() {
    let base = episodes_demo::run().unwrap().remove(1).world;
    for kind in [
        ProgrammeKind::Episode {
            series: SeriesId(999),
            sequence: 1,
        },
        ProgrammeKind::Episode {
            series: ARC,
            sequence: 0,
        },
        ProgrammeKind::Episode {
            series: ARC,
            sequence: 2,
        },
    ] {
        let mut w = base.clone();
        w.programmes
            .iter_mut()
            .find(|p| p.id == FIRST)
            .unwrap()
            .kind = kind;
        assert!(Simulation::new(w, Parameters::default()).is_err());
    }
    for value in [f64::NAN, f64::INFINITY, -0.1, 1.1] {
        let mut w = base.clone();
        w.series[0].continuity_strength = value;
        assert!(Simulation::new(w, Parameters::default()).is_err());
    }
    let mut w = base.clone();
    w.series.push(w.series[0].clone());
    assert!(Simulation::new(w, Parameters::default()).is_err());
    let mut gap = base;
    gap.programmes.retain(|p| p.id == THIRD);
    let mut s = Simulation::new(gap.clone(), Parameters::default()).unwrap();
    let e = s.step(&action(&gap, THIRD, 18)).unwrap();
    assert_eq!(e.allocations[0].viewing[0].history.prior_familiarity, 0.0);
}
#[test]
fn standalone_has_history_and_familiarity_without_series_attachment() {
    let run = episodes_demo::run().unwrap().remove(0);
    let a = run.world.audiences[0].id;
    assert!(run.world.series.is_empty());
    assert_eq!(run.final_state.loyalty(a, ARC), None);
    assert_eq!(view(&run, 0).series, None);
    assert_eq!(view(&run, 0).loyalty_after, None);
    assert!(run.final_state.familiarity(a, MOVIE).unwrap() > 0.0);
    assert_eq!(run.final_state.broadcast_history().len(), 1);
    assert_eq!(run.final_state.broadcast_history()[0].programme, MOVIE);
    assert_eq!(run.final_state.series_progress(a, ARC), None);
}
#[test]
fn reception_changes_subsequent_episode_interest_without_changing_base_attributes() {
    let runs = episodes_demo::run().unwrap();
    let good = &runs[1];
    let bad = &runs[2];
    assert_eq!(view(good, 0).viewers, view(bad, 0).viewers);
    assert_eq!(
        view(good, 0).familiarity_after,
        view(bad, 0).familiarity_after
    );
    assert!(view(good, 0).satisfaction.trace.output > view(bad, 0).satisfaction.trace.output);
    assert_eq!(view(good, 1).attraction, view(bad, 1).attraction);
    assert_eq!(
        view(good, 1).history.prior_familiarity,
        view(bad, 1).history.prior_familiarity
    );
    assert!(view(good, 1).history.reception_factor > view(bad, 1).history.reception_factor);
    assert!(view(good, 1).loyalty_before > view(bad, 1).loyalty_before);
    assert!(view(good, 1).viewers > view(bad, 1).viewers + 500.0);
    assert!(view(bad, 2).viewers > view(bad, 1).viewers); // Good subsequent episodes can recover interest.
}
#[test]
fn unviewed_opening_does_not_leak_its_execution_into_later_interest() {
    let runs = episodes_demo::run().unwrap();
    let mut later = Vec::new();
    for run in [&runs[1], &runs[2]] {
        let mut w = run.world.clone();
        w.audiences[0].availability[18] = 0.0;
        w.audiences[0].availability[19] = 0.8;
        let mut s = Simulation::new(w.clone(), run.parameters).unwrap();
        s.step(&action(&w, FIRST, 18)).unwrap();
        assert_eq!(s.state().reception(w.audiences[0].id, FIRST), None);
        assert_eq!(s.state().series_progress(w.audiences[0].id, ARC), Some(0));
        let e = s.step(&action(&w, SECOND, 19)).unwrap();
        later.push(e.allocations[0].viewing[0].clone());
    }
    assert_eq!(later[0].utility, later[1].utility);
    assert_eq!(later[0].viewers, later[1].viewers);
    assert_eq!(later[0].history.predecessor_reception, None);
}
#[test]
fn repeated_earlier_episode_is_not_new_progress_and_has_fewer_expected_fresh_viewers() {
    let runs = episodes_demo::run().unwrap();
    let repeat = &runs[3];
    let fresh = &runs[4];
    let r = view(repeat, 2);
    let f = view(fresh, 2);
    assert_eq!(r.programme, FIRST);
    assert_eq!(r.history.frontier_before, Some(2));
    assert_eq!(r.history.frontier_after, Some(2));
    assert_eq!(view(repeat, 3).history.frontier_after, Some(3));
    assert_eq!(r.attraction, f.attraction);
    assert_eq!(r.history.attachment_factor, f.history.attachment_factor);
    assert_eq!(r.history.continuity_factor, f.history.continuity_factor);
    assert_eq!(r.history.reception_factor, f.history.reception_factor);
    assert!(r.history.repeat_factor < f.history.repeat_factor);
    assert!(r.viewers < f.viewers);
    assert!(r.fresh_viewers < r.viewers * 0.5);
    assert_eq!(f.fresh_viewers, f.viewers);
    assert!((r.fresh_viewers - r.viewers * (1.0 - r.history.familiarity)).abs() < 1e-9);
    assert_eq!(
        repeat
            .final_state
            .broadcast_history()
            .iter()
            .filter(|b| b.programme == FIRST)
            .count(),
        2
    );
    assert!(r.familiarity_after > r.history.familiarity);
}
#[test]
fn missing_predecessors_matter_more_for_high_continuity_but_new_viewers_can_join() {
    let runs = episodes_demo::run().unwrap();
    let low = view(&runs[5], 0);
    let high = view(&runs[6], 0);
    assert_eq!(low.attraction, high.attraction);
    assert_eq!(low.history.prior_familiarity, 0.0);
    assert_eq!(high.history.prior_familiarity, 0.0);
    assert_eq!(low.history.continuity_factor, 1.0);
    assert_eq!(high.history.continuity_factor, 0.4);
    assert!((high.utility / low.utility - 0.4).abs() < 1e-12);
    assert!(high.viewers > 0.0 && high.viewers < low.viewers);
    assert_eq!(high.fresh_viewers, high.viewers);
    assert_eq!(high.history.frontier_after, Some(3));
    assert!(view(&runs[6], 2).history.prior_familiarity > 0.0);
    assert!(view(&runs[6], 2).history.continuity_factor > 0.4);
}
#[test]
fn simultaneous_work_broadcasts_share_one_familiarity_snapshot() {
    let mut w = episodes_demo::run().unwrap().remove(1).world;
    let mut second = w.stations[0].clone();
    second.id = StationId(99);
    w.stations.push(second);
    let mut plan = action(&w, FIRST, 18);
    plan.broadcasts.push(Broadcast {
        station: StationId(99),
        programme: FIRST,
        advertisement: None,
    });
    let mut s = Simulation::new(w.clone(), Parameters::default()).unwrap();
    let e = s.step(&plan).unwrap();
    let a = &e.allocations[0];
    let total = a.viewing.iter().map(|v| v.viewers).sum::<f64>();
    assert!(
        (s.state().familiarity(a.audience, FIRST).unwrap() - total / f64::from(a.population)).abs()
            < 1e-12
    );
    assert!(a
        .viewing
        .iter()
        .all(|v| v.history.familiarity == 0.0 && v.fresh_viewers == v.viewers));
    let mut other = Simulation::new(w, Parameters::default()).unwrap();
    plan.broadcasts.reverse();
    other.step(&plan).unwrap();
    assert_eq!(s.state(), other.state());
}
#[test]
fn off_air_has_no_history_familiarity_reception_or_series_progress_effects() {
    let mut w = episodes_demo::run().unwrap().remove(1).world;
    w.stations[0].initial_cash = Money(0);
    let a = w.audiences[0].id;
    let mut s = Simulation::new(w.clone(), Parameters::default()).unwrap();
    let e = s.step(&action(&w, FIRST, 18)).unwrap();
    assert_eq!(e.off_air.len(), 1);
    assert!(s.state().broadcast_history().is_empty());
    assert_eq!(s.state().familiarity(a, FIRST), Some(0.0));
    assert_eq!(s.state().reception(a, FIRST), None);
    assert_eq!(s.state().series_progress(a, ARC), Some(0));
    assert_eq!(s.state().loyalty(a, ARC), Some(0.5));
}
#[test]
fn episode_runs_reproduce_and_numerical_extremes_remain_bounded() {
    let runs = episodes_demo::run().unwrap();
    assert_eq!(runs, episodes_demo::run().unwrap());
    for run in &runs {
        let before = run.clone();
        let _ = developer::report(run);
        assert_eq!(run, &before);
        for e in &run.events {
            for a in &e.allocations {
                let sum = a.viewing.iter().map(|v| v.viewers).sum::<f64>();
                assert!((sum + a.not_watching - f64::from(a.population)).abs() < 1e-8);
                for v in &a.viewing {
                    assert!((0.0..=1.0).contains(&v.familiarity_after));
                    assert!(v.fresh_viewers <= v.viewers);
                }
            }
        }
    }
    let mut w = runs[1].world.clone();
    w.audiences[0].availability = [1.0; 24];
    for p in &mut w.programmes {
        p.awareness = 1.0;
        p.cast_appeal = 1.0;
        p.execution = 1.0;
    }
    w.series[0].continuity_strength = 0.0;
    let mut s = Simulation::new(
        w.clone(),
        Parameters {
            outside_utility: f64::MIN_POSITIVE,
            loyalty_learning: 1.0,
            ..Parameters::default()
        },
    )
    .unwrap();
    s.step(&action(&w, FIRST, 18)).unwrap();
    let e = s.step(&action(&w, SECOND, 42)).unwrap();
    assert!(e.allocations[0].viewing[0].utility <= 2.34 + 1e-12);
    for day in 2..100 {
        let e = s.step(&action(&w, FIRST, day * 24 + 18)).unwrap();
        let v = &e.allocations[0].viewing[0];
        assert!(v.utility.is_finite());
        assert!((0.0..=1.0).contains(&v.familiarity_after));
        assert!((0.0..=1.0).contains(&v.loyalty_after.unwrap()));
    }
}

#[test]
fn simultaneous_distinct_episodes_do_not_see_each_others_new_reception() {
    let mut w = episodes_demo::run().unwrap().remove(1).world;
    w.series[0].continuity_strength = 1.0;
    let mut second = w.stations[0].clone();
    second.id = StationId(99);
    w.stations.push(second);
    let mut plan = action(&w, FIRST, 18);
    plan.broadcasts.push(Broadcast {
        station: StationId(99),
        programme: SECOND,
        advertisement: None,
    });
    let mut sim = Simulation::new(w.clone(), Parameters::default()).unwrap();
    let event = sim.step(&plan).unwrap();
    let second = event.allocations[0]
        .viewing
        .iter()
        .find(|v| v.programme == SECOND)
        .unwrap();
    assert_eq!(second.history.predecessor_reception, None);
    assert_eq!(second.history.prior_familiarity, 0.0);
    assert_eq!(second.history.continuity_factor, 0.4);
    assert_eq!(second.history.frontier_after, Some(2));
    assert!(sim.state().reception(w.audiences[0].id, FIRST).is_some());
    plan.broadcasts.reverse();
    let mut reversed = Simulation::new(w, Parameters::default()).unwrap();
    reversed.step(&plan).unwrap();
    assert_eq!(sim.state(), reversed.state());
}
