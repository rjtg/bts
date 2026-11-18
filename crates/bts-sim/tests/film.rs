use bts_sim::planning::film::*;
use bts_sim::planning::production::{PersonId, Subject};
use bts_sim::planning::Planner;
use bts_sim::{demo, Parameters, SimTick};

const THRILLER: [f64; 7] = [0.2, 0.2, 0.93, 0.32, 0.48, 0.95, 0.86];

#[test]
fn staged_delivery_and_conflicts_preserve_exact_cash_and_determinism() {
    let mut p = Planner::new(
        demo::world(),
        Parameters::default(),
        demo::MAIN,
        SimTick(672),
        &[],
    )
    .unwrap();
    let total = p.state().economy().total_money();
    let first = p
        .pitch_film("First".into(), Subject::EverydayLife, THRILLER)
        .unwrap();
    let second = p
        .pitch_film("Second".into(), Subject::EverydayLife, THRILLER)
        .unwrap();
    let writing = |p: &mut Planner| {
        p.write_film(first, PersonId(101), None).unwrap();
        let state = format!("{p:?}");
        assert!(p.write_film(second, PersonId(101), None).is_err());
        assert_eq!(state, format!("{p:?}"));
        p.write_film(second, PersonId(102), None).unwrap();
        p.advance_to(SimTick(48)).unwrap();
    };
    writing(&mut p);
    let initial = p.film(first).unwrap().script.as_ref().unwrap().profile;
    assert!(
        p.film(second)
            .unwrap()
            .script
            .as_ref()
            .unwrap()
            .profile
            .values[5]
            > initial.values[5]
    );
    p.write_film(first, PersonId(101), Some(5)).unwrap();
    p.advance_to(SimTick(80)).unwrap();
    assert!(
        p.film(first)
            .unwrap()
            .script
            .as_ref()
            .unwrap()
            .profile
            .values[5]
            > initial.values[5]
    );
    assert!(p.write_film(first, PersonId(101), Some(5)).is_err());
    p.accept_script(first).unwrap();
    p.accept_script(second).unwrap();
    let booking = FilmBooking {
        cast: [PersonId(105), PersonId(106)],
        director: PersonId(104),
        studio: 1,
        start: SimTick(88),
    };
    assert!(p.review_film(first, booking).unwrap().studio_shortfall);
    p.book_film(first, booking).unwrap();
    let state = format!("{p:?}");
    assert!(p.book_film(second, booking).is_err());
    assert_eq!(state, format!("{p:?}"));
    assert!(p.film(first).unwrap().programme.is_none());
    assert_eq!(p.state().economy().production_escrow(), 1730);
    let mut rerun = p.clone();
    p.advance_to(SimTick(112)).unwrap();
    for tick in 81..=112 {
        rerun.advance_to(SimTick(tick)).unwrap();
    }
    assert_eq!(p.film(first).unwrap(), rerun.film(first).unwrap());
    assert_eq!(p.state().economy(), rerun.state().economy());
    assert_eq!(p.state().economy().total_money(), total);
    assert_eq!(p.state().economy().production_costs().0, 2950);
    assert!(p
        .place(
            p.film(first).unwrap().programme.unwrap(),
            SimTick(168),
            None
        )
        .is_ok());
}
