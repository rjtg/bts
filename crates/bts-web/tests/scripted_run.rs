use bts_sim::planning::film::FilmBooking;
use bts_sim::planning::production::{PersonId, Subject};
use bts_sim::{ProgrammeKind, SimTick};

#[test]
fn identical_finale_is_harder_after_large_shifts_and_credits_stay_in_their_roles() {
    let mut base = bts_web::scenario::planner().unwrap();
    let mut abrupt = base.clone();
    let brief = [0.35, 0.38, 0.45, 0.68, 0.72, 0.28, 0.55];
    let series = base
        .pitch_scripted_run(
            "Harbour Lights".into(),
            Subject::EverydayLife,
            brief,
            [0.45, 0.60, 0.75],
        )
        .unwrap();
    abrupt
        .pitch_scripted_run(
            "Harbour Lights".into(),
            Subject::EverydayLife,
            brief,
            [0.10, 0.90, 0.75],
        )
        .unwrap();
    let ids = base.scripted_runs().next().unwrap().episodes;
    assert_eq!(
        base.pitch_sequel(series, "Too soon".into())
            .unwrap_err()
            .code,
        "series_unfinished"
    );
    for candidate in [&mut base, &mut abrupt] {
        for id in ids {
            candidate.write_film(id, PersonId(101), None).unwrap();
            candidate
                .advance_to(candidate.film(id).unwrap().due.unwrap())
                .unwrap();
            candidate.accept_script(id).unwrap();
        }
    }
    let profile = |planner: &bts_sim::planning::Planner| {
        planner
            .film(ids[2])
            .unwrap()
            .script
            .as_ref()
            .unwrap()
            .profile
            .values[2]
            .unwrap()
    };
    let gradual_tension = profile(&base);
    let abrupt_tension = profile(&abrupt);
    assert!(gradual_tension > abrupt_tension);
    println!("Final script tension after gradual change: {gradual_tension:.3}; after abrupt changes: {abrupt_tension:.3}");
    let mut booking = FilmBooking {
        cast: [PersonId(105), PersonId(106)],
        director: PersonId(103),
        studio: 1,
        start: SimTick(base.now().0 + 4),
    };
    assert_eq!(
        base.book_film(ids[1], booking).unwrap_err().code,
        "continuity_cast"
    );
    base.book_film(ids[0], booking).unwrap();
    base.advance_to(base.film(ids[0]).unwrap().due.unwrap())
        .unwrap();
    booking.start = SimTick(base.now().0 + 4);
    booking.cast.swap(0, 1);
    assert_eq!(
        base.book_film(ids[1], booking).unwrap_err().code,
        "continuity_cast"
    );
    booking.cast.swap(0, 1);
    base.book_film(ids[1], booking).unwrap();
    base.advance_to(base.film(ids[1]).unwrap().due.unwrap())
        .unwrap();
    booking.start = SimTick(base.now().0 + 4);
    base.book_film(ids[2], booking).unwrap();
    base.advance_to(base.film(ids[2]).unwrap().due.unwrap())
        .unwrap();
    for (index, id) in ids.into_iter().enumerate() {
        let programme = base.film(id).unwrap().programme.unwrap();
        let work = base
            .world()
            .programmes
            .iter()
            .find(|work| work.id == programme)
            .unwrap();
        assert_eq!(
            work.kind,
            ProgrammeKind::Episode {
                series,
                sequence: index as u32 + 1
            }
        );
        assert_eq!(work.duration_minutes, 45);
    }
    let sequel = base.pitch_sequel(series, "The Next Tide".into()).unwrap();
    let next_run = base
        .scripted_runs()
        .find(|run| run.series == sequel)
        .unwrap();
    assert_eq!(next_run.sequel_of, Some(series));
    assert_eq!(next_run.cast, Some(booking.cast));
    assert!(next_run.episodes.iter().all(|id| !ids.contains(id)));
    assert_eq!(
        base.pitch_sequel(series, "Another sequel".into())
            .unwrap_err()
            .code,
        "series_sequel_exists"
    );
}

#[test]
fn later_episode_tension_is_editable_without_changing_the_committed_script() {
    let mut planner = bts_web::scenario::planner().unwrap();
    let series = planner
        .pitch_scripted_run(
            "The Crossing".into(),
            Subject::EverydayLife,
            [0.5; 7],
            [0.5; 3],
        )
        .unwrap();
    let [first, second, third] = planner
        .scripted_runs()
        .find(|run| run.series == series)
        .unwrap()
        .episodes;
    planner.write_film(first, PersonId(101), None).unwrap();
    planner
        .advance_to(planner.film(first).unwrap().due.unwrap())
        .unwrap();
    let script = planner.film(first).unwrap().clone();
    assert_eq!(
        planner.amend_episode_tension(first, 0.9).unwrap_err().code,
        "brief_locked"
    );
    assert!(planner.amend_episode_tension(second, f64::NAN).is_err());
    assert_eq!(planner.film(second).unwrap().ambition.values[2], Some(0.5));
    planner.amend_episode_tension(second, 0.95).unwrap();
    assert_eq!(planner.film(first).unwrap(), &script);
    assert_eq!(planner.film(second).unwrap().ambition.values[2], Some(0.95));
    assert!(planner.film(third).unwrap().episode.unwrap().strain > 0.0);
}
