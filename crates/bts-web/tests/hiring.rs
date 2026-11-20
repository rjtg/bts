use bts_sim::economy::Money;
use bts_sim::planning::production::{Concept, PersonId, StudioId, Subject, Tone};
use bts_sim::{AirMinute, SimTick};

#[test]
fn prepaid_host_dates_protect_the_role_and_reduce_the_later_booking_charge() {
    let mut planner = bts_web::scenario::planner().unwrap();
    let total = planner.state().economy().total_money();
    let concept = Concept {
        title: "Studio window".into(),
        host: PersonId(1),
        guests: [PersonId(4), PersonId(6)],
        subject: Subject::Science,
        tone: Tone::Calm,
        budget: Money(1000),
        record_at: SimTick(40),
    };
    planner
        .create_discussion_run(concept.clone(), AirMinute(1080), 0)
        .unwrap();
    let dates: Vec<_> = planner
        .discussion_runs()
        .next()
        .unwrap()
        .editions
        .iter()
        .map(|e| e.production)
        .collect();
    let conflict = planner.create_production(concept.clone()).unwrap();
    let mut other = concept.clone();
    other.host = PersonId(2);
    other.record_at = SimTick(48);
    let other = planner.create_production(other).unwrap();
    assert_eq!(
        planner
            .sign_host(PersonId(1), &[dates[0], conflict])
            .unwrap_err()
            .code,
        "host_date_unavailable"
    );
    planner.sign_host(PersonId(1), &dates).unwrap();
    assert_eq!(planner.state().economy().production_costs(), Money(660));
    let before = format!("{planner:?}");
    assert!(planner.sign_host(PersonId(1), &dates).is_err());
    assert_eq!(
        planner.book_production(conflict, None, 0).unwrap_err().code,
        "talent_conflict"
    );
    assert!(planner.discard_production(dates[0]).is_err());
    assert_eq!(format!("{planner:?}"), before);
    planner.sign_host(PersonId(2), &[other]).unwrap();
    planner
        .book_discussion_edition(dates[0], StudioId(1))
        .unwrap();
    assert_eq!(planner.state().economy().production_escrow(), 780);
    planner.advance_to(SimTick(52)).unwrap();
    assert_eq!(planner.state().economy().production_costs(), Money(1560));
    assert_eq!(planner.state().economy().total_money(), total);
}
