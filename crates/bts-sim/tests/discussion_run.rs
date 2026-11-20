use bts_sim::demo::{world, MAIN};
use bts_sim::economy::Money;
use bts_sim::planning::production::{demo_roster, Concept, PersonId, StudioId, Subject, Tone};
use bts_sim::planning::Planner;
use bts_sim::{AirMinute, Parameters, ProgrammeKind, SimTick};

fn fixture() -> (Planner, Concept) {
    let mut planner =
        Planner::new(world(), Parameters::default(), MAIN, SimTick(2016), &[]).unwrap();
    let (mut roster, requests) = demo_roster();
    for person in &mut roster {
        if person.normal_until == SimTick(672) {
            person.normal_until = SimTick(2016);
        }
    }
    planner.configure_production(roster, requests).unwrap();
    let brief = Concept {
        title: "Studio Roundtable".into(),
        host: PersonId(1),
        guests: [PersonId(3), PersonId(4)],
        subject: Subject::Science,
        tone: Tone::Calm,
        budget: Money(1000),
        record_at: SimTick(40),
    };
    (planner, brief)
}

#[test]
fn editions_have_distinct_work_and_airings_and_protect_reserved_airtime() {
    let (mut planner, brief) = fixture();
    let total = planner.state().economy().total_money();
    let series = planner
        .create_discussion_run(brief, AirMinute(1081), 10)
        .unwrap();
    let ids: Vec<_> = planner
        .discussion_runs()
        .next()
        .unwrap()
        .editions
        .iter()
        .map(|edition| edition.production)
        .collect();
    let original = planner.world().programmes[0].id;
    assert_eq!(planner.state().economy().cash(MAIN), Some(Money(20_000)));
    planner
        .book_discussion_edition(ids[0], StudioId(1))
        .unwrap();
    assert_eq!(
        planner
            .place_exact(original, AirMinute(1083), 0)
            .unwrap_err()
            .code,
        "overlap"
    );
    planner.advance_exact(AirMinute(1136)).unwrap();
    let first = planner.discussion_runs().next().unwrap().editions[0]
        .airing
        .unwrap();
    assert!(planner.airing(first).unwrap().result.is_some());

    let mut updated = planner
        .production()
        .project(ids[1])
        .unwrap()
        .concept
        .clone();
    updated.guests = [PersonId(6), PersonId(4)];
    updated.subject = Subject::Entertainment;
    updated.tone = Tone::Lively;
    planner.revise_production(ids[1], updated).unwrap();
    planner
        .book_discussion_edition(ids[1], StudioId(1))
        .unwrap();
    let mut third = planner
        .production()
        .project(ids[2])
        .unwrap()
        .concept
        .clone();
    third.guests = [PersonId(6), PersonId(4)];
    planner.revise_production(ids[2], third).unwrap();
    planner
        .book_discussion_edition(ids[2], StudioId(1))
        .unwrap();
    let mut replay = planner.clone();
    let deadline = AirMinute(1136 + 14 * 1440);
    assert_eq!(
        planner.advance_exact(deadline).unwrap(),
        replay.advance_exact(deadline).unwrap()
    );
    assert_eq!(planner.state().economy().total_money(), total);
    assert_eq!(planner.state().economy().production_costs(), Money(3000));
    for (index, edition) in planner
        .discussion_runs()
        .next()
        .unwrap()
        .editions
        .iter()
        .enumerate()
    {
        let airing = planner.airing(edition.airing.unwrap()).unwrap();
        assert!(airing.result.is_some());
        let programme = planner
            .world()
            .programmes
            .iter()
            .find(|p| p.id == airing.programme)
            .unwrap();
        assert_eq!(
            programme.kind,
            ProgrammeKind::Episode {
                series,
                sequence: index as u32 + 1
            }
        );
    }
}

#[test]
fn invalid_booking_and_late_draft_leave_no_obligation() {
    let (mut planner, brief) = fixture();
    assert_eq!(
        planner
            .create_discussion_run(brief.clone(), AirMinute(700), 10)
            .unwrap_err()
            .code,
        "edition_delivery"
    );
    assert_eq!(planner.production().projects().count(), 0);
    planner
        .create_discussion_run(brief, AirMinute(1080), 10)
        .unwrap();
    let first = planner.discussion_runs().next().unwrap().editions[0].production;
    let before = format!("{planner:?}");
    assert_eq!(
        planner
            .book_discussion_edition(first, StudioId(0))
            .unwrap_err()
            .code,
        "studio_unavailable"
    );
    assert_eq!(format!("{planner:?}"), before);
    planner.advance_to(SimTick(41)).unwrap();
    assert!(
        planner.discussion_edition_missed(&planner.discussion_runs().next().unwrap().editions[0])
    );
    assert!(planner.book_discussion_edition(first, StudioId(1)).is_err());
    assert_eq!(planner.airings().count(), 0);
    assert!(planner.state().economy().ledger().is_empty());
}
