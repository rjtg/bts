use bts_sim::economy::Money;
use bts_sim::planning::{
    film::FilmBooking,
    production::{PersonId, Subject},
};
use bts_sim::SimTick;

#[test]
fn paid_elastic_booking_uses_overlap_preserves_the_quote_and_checks_staff() {
    let mut quiet = bts_web::scenario::planner().unwrap();
    quiet.configure_rental_demand(vec![]).unwrap();
    let film = quiet
        .pitch_film("Midnight Crossing".into(), Subject::EverydayLife, [0.9; 7])
        .unwrap();
    quiet.write_film(film, PersonId(102), None).unwrap();
    quiet
        .advance_to(quiet.film(film).unwrap().due.unwrap())
        .unwrap();
    quiet.accept_script(film).unwrap();
    let second = quiet
        .pitch_film("One More Crossing".into(), Subject::EverydayLife, [0.9; 7])
        .unwrap();
    quiet.write_film(second, PersonId(102), None).unwrap();
    quiet
        .advance_to(quiet.film(second).unwrap().due.unwrap())
        .unwrap();
    quiet.accept_script(second).unwrap();

    let start = SimTick(quiet.now().0 + 4);
    let room = FilmBooking {
        cast: [PersonId(105), PersonId(106)],
        director: PersonId(104),
        studio: 3,
        start,
    };
    let mut busy = quiet.clone();
    busy.configure_rental_demand(vec![(SimTick(start.0 + 4), SimTick(start.0 + 12))])
        .unwrap();
    let quiet_quote = quiet.review_film(film, room).unwrap();
    let busy_quote = busy.review_film(film, room).unwrap();
    let better_room = quiet
        .review_film(film, FilmBooking { studio: 4, ..room })
        .unwrap();
    assert_eq!(quiet_quote.rent, Money(240));
    assert_eq!(busy_quote.rent, Money(300));
    assert_eq!(busy_quote.rental_pressure, Some(0.5));
    assert!(busy_quote.studio_shortfall);
    assert!(!better_room.studio_shortfall);
    assert_eq!(better_room.rent, Money(710));
    let economy = busy.state().economy();
    let conserved = economy.total_money();
    let cash = economy.cash(bts_web::scenario::PLAYER).unwrap();
    busy.book_film(film, room).unwrap();
    let ledger = busy.state().economy().ledger().to_vec();
    assert_eq!(
        busy.book_film(second, room).unwrap_err().code,
        "film_talent_conflict"
    );
    assert_eq!(busy.state().economy().ledger(), ledger);
    busy.configure_rental_demand(vec![]).unwrap();
    assert_eq!(busy.film(film).unwrap().quote.as_ref(), Some(&busy_quote));
    assert_eq!(
        busy.state().economy().cash(bts_web::scenario::PLAYER),
        Some(Money(cash.0 - busy_quote.total.0))
    );
    assert_eq!(busy.state().economy().total_money(), conserved);
    println!(
        "Same Edit Bay: quiet {} Funds; half-overlap {} Funds. Quiet Grand Floor: {} Funds.",
        quiet_quote.rent.0, busy_quote.rent.0, better_room.rent.0
    );
}
