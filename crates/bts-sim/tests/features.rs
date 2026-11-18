use bts_sim::{demo::*, *};

#[test]
fn absence_is_excluded_and_low_is_scored() {
    let mut profile = FeatureProfile::nonfiction([None; 10]);
    profile.values[0] = Some(1.0);
    assert_eq!(profile.fit(&[1.0; 10]), 1.0);
    profile.values[EDITORIAL_RIGOUR] = Some(0.0);
    assert_eq!(profile.fit(&[1.0; 10]), 0.5);
    let mut world = demo::world();
    world.programmes[0].profile.values[EDITORIAL_RIGOUR] = Some(0.0);
    assert!(Simulation::new(world, Parameters::default()).is_err());
}

#[test]
fn hidden_rigour_changes_experience_without_changing_initial_viewing() {
    let mut world = demo::world();
    let actions = schedule(Case::Targeted);
    let run = |world| {
        let mut sim = Simulation::new(world, Parameters::default()).unwrap();
        sim.step(&actions[0]).unwrap()
    };
    let before = run(world.clone());
    world.programmes[1].profile.values[EDITORIAL_RIGOUR] = Some(0.0);
    let after = run(world);
    let get = |event: &SlotEvent| {
        let viewing = event
            .allocations
            .iter()
            .find(|a| a.audience == SPECIALISTS)
            .unwrap()
            .viewing
            .iter()
            .find(|v| v.station == MAIN)
            .unwrap();
        (viewing.viewers, viewing.satisfaction.trace.output)
    };
    assert_eq!(get(&before).0, get(&after).0);
    assert!(get(&before).1 > get(&after).1);
}

#[test]
fn genre_is_derived_without_a_fit_discontinuity() {
    let a = FeatureProfile::fiction([0.6499, 0.5, 0.2, 0.5, 0.5, 0.5, 0.5]);
    let mut b = a;
    b.values[0] = Some(0.6501);
    assert_ne!(a.genre(), b.genre());
    assert!((a.fit(&[0.5; 10]) - b.fit(&[0.5; 10])).abs() < 0.0001);
}
