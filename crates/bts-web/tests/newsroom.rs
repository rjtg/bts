use bts_sim::economy::Money;
use bts_sim::planning::newsroom::{NewsPhase, NewsStaff};
use bts_sim::planning::production::{Concept, PersonId, StudioId, Subject, Tone};
use bts_sim::{AirMinute, ProgrammeId, SimTick};

fn staff(reporter: u16, anchor: u16) -> NewsStaff {
    NewsStaff {
        reporter: PersonId(reporter),
        anchor: PersonId(anchor),
        studio: StudioId(1),
    }
}

#[test]
fn reporting_locks_resources_and_later_editions_can_change_without_rewriting_history() {
    let mut planner = bts_web::scenario::planner().unwrap();
    let total = planner.state().economy().total_money();
    assert_eq!(
        planner
            .start_newsroom("Bulletin".into(), 0, 830, 5, staff(20, 22))
            .unwrap_err()
            .code,
        "news_freshness"
    );
    assert!(planner.newsroom().is_none());
    planner
        .start_newsroom("Bulletin".into(), 0, 1080, 5, staff(20, 22))
        .unwrap();
    // Block edition two before the newsroom attempts to commit its resources.
    planner
        .place_exact(ProgrammeId(2), AirMinute(1440 + 1080), 0)
        .unwrap();
    planner.advance_to(SimTick(36)).unwrap();
    assert_eq!(
        planner.newsroom().unwrap().editions[0].phase,
        NewsPhase::Reporting
    );
    assert_eq!(planner.state().economy().production_costs(), Money(350));
    assert_eq!(
        planner
            .place_exact(ProgrammeId(2), AirMinute(1080), 0)
            .unwrap_err()
            .code,
        "overlap"
    );

    let draft = planner
        .create_production(Concept {
            title: "Same studio".into(),
            host: PersonId(1),
            guests: [PersonId(3), PersonId(4)],
            subject: Subject::Science,
            tone: Tone::Calm,
            budget: Money(1000),
            record_at: SimTick(52),
        })
        .unwrap();
    assert_eq!(
        planner.book_production(draft, None, 0).unwrap_err().code,
        "studio_unavailable"
    );
    planner.update_newsroom_staff(staff(21, 23)).unwrap();
    planner.advance_to(SimTick(56)).unwrap();
    let first = &planner.newsroom().unwrap().editions[0];
    assert_eq!(first.reporter, PersonId(20));
    assert_eq!(first.phase, NewsPhase::Delivered);
    let airing = first.airing.unwrap();
    assert_eq!(
        planner
            .move_exact(airing, AirMinute(1440 + 720))
            .unwrap_err()
            .code,
        "news_expired"
    );
    assert_eq!(
        planner
            .move_exact(airing, AirMinute(1430))
            .unwrap_err()
            .code,
        "news_expired"
    );

    let mut replay = planner.clone();
    planner.advance_to(SimTick(4 * 96 + 76)).unwrap();
    replay.advance_to(SimTick(2 * 96)).unwrap();
    replay.advance_to(SimTick(4 * 96 + 76)).unwrap();
    let editions = &planner.newsroom().unwrap().editions;
    assert_eq!(editions[1].exception, Some("Airtime already occupied"));
    assert_eq!(editions[1].reporter, PersonId(21));
    assert_eq!(editions[1].phase, NewsPhase::Missed);
    assert!(editions[1].airing.is_none());
    assert!(editions[2..]
        .iter()
        .all(|e| e.phase == NewsPhase::Delivered && e.reporter == PersonId(21)));
    assert_eq!(planner.state().economy().production_costs(), Money(1400));
    assert_eq!(planner.state().economy().total_money(), total);
    assert_eq!(
        planner.state().economy().ledger(),
        replay.state().economy().ledger()
    );
}
