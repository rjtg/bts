use bts_sim::demo::{self, MAIN, RIVAL, YOUNG};
use bts_sim::planning::weekly::{Exception, Weekday, WeeklyPlan};
use bts_sim::planning::{CompetitorAiring, Planner};
use bts_sim::{Parameters, ProgrammeId, SimTick};

fn planner() -> Planner {
    Planner::new(
        demo::world(),
        Parameters::default(),
        MAIN,
        SimTick(2016),
        &[],
    )
    .unwrap()
}

#[test]
fn weekly_dates_conflicts_exceptions_protection_and_atomic_edits() {
    let mut p = planner();
    let ordinary = p.place(ProgrammeId(2), SimTick(744), None).unwrap();
    let mut plan = WeeklyPlan {
        programme: ProgrammeId(1),
        weekday: Weekday::SATURDAY,
        time: 72 * 15,
        count: 3,
        ad_minutes: 10,
        exceptions: vec![],
    };
    let rows = p.preview_weekly(&plan).unwrap();
    assert_eq!(
        rows.iter()
            .map(|r| r.start_tick.unwrap())
            .collect::<Vec<_>>(),
        [72, 744, 1416]
    );
    assert_eq!(rows[0].runtime_minutes, 70);
    assert_eq!(rows[0].end_tick, Some(77));
    assert_eq!(rows[1].conflicts, [ordinary]);
    let before = format!("{p:?}");
    assert!(p.place_weekly(&plan).is_err());
    assert_eq!(format!("{p:?}"), before);
    let mut skip = plan.clone();
    skip.exceptions.push(Exception {
        occurrence: 1,
        start_minute: None,
    });
    assert_eq!(p.clone().place_weekly(&skip).unwrap().len(), 2);
    plan.exceptions.push(Exception {
        occurrence: 1,
        start_minute: Some((752) * 15),
    });
    let ids = p.place_weekly(&plan).unwrap();
    assert_eq!(p.airing(ids[1]).unwrap().start.tick_floor().0, 752);
    assert!(ids
        .iter()
        .all(|&id| p.airing(id).unwrap().weekly == Some(ids[0])
            && p.airing(id).unwrap().ad_minutes == 10));
    let offer = p.minute_offers(ids[0]).unwrap().remove(0);
    p.accept_minutes(&offer).unwrap();
    let review = p.preview_weekly_edit(ids[0], 80 * 15).unwrap();
    assert!(review[0].protected);
    let before = format!("{p:?}");
    assert!(p.edit_weekly(ids[0], 72 * 15).is_err());
    assert_eq!(format!("{p:?}"), before);
    p.edit_weekly(ids[0], 80 * 15).unwrap();
    assert_eq!(p.airing(ids[0]).unwrap().start.tick_floor().0, 72);
    assert_eq!(p.airing(ids[2]).unwrap().start.tick_floor().0, 1424);
    p.move_airing(ids[2], SimTick(1428)).unwrap();
    assert_eq!(p.airing(ids[2]).unwrap().weekly, Some(ids[0]));
    p.advance_to(SimTick(97)).unwrap();
    let upcoming = p.preview_weekly(&plan).unwrap();
    assert_eq!(upcoming[0].regular_tick, 744);
    assert_eq!(p.airing(ids[0]).unwrap().start.tick_floor().0, 72);
}

#[test]
fn local_day_and_preference_change_watch_choice_without_creating_availability() {
    let run = |start: u32| {
        let mut world = demo::world();
        for group in &mut world.audiences {
            group.availability = if group.id == YOUNG {
                [0.8; 24]
            } else {
                [0.0; 24]
            };
        }
        let profiles = world
            .audiences
            .iter()
            .map(|group| (group.id, [[0.05; 3], [0.95; 3]]))
            .collect();
        let rival = [CompetitorAiring {
            station: RIVAL,
            programme: ProgrammeId(4),
            start: SimTick(start),
        }];
        let mut planner =
            Planner::new(world, Parameters::default(), MAIN, SimTick(2016), &rival).unwrap();
        planner
            .configure_viewing_calendar(Weekday::SATURDAY, profiles)
            .unwrap();
        let airing = planner.place(ProgrammeId(1), SimTick(start), None).unwrap();
        planner.advance_to(SimTick(start + 4)).unwrap();
        let snapshot = planner.airing(airing).unwrap().result.as_ref().unwrap();
        let mut young = None;
        for group in &snapshot.allocations {
            let watched = group.viewing.iter().map(|v| v.viewers).sum::<f64>();
            assert!((watched + group.not_watching - f64::from(group.population)).abs() < 1e-7);
            if group.audience != YOUNG {
                assert_eq!(watched, 0.0);
            } else {
                young = Some((
                    group.available,
                    group
                        .viewing
                        .iter()
                        .find(|v| v.station == MAIN)
                        .unwrap()
                        .viewers,
                ));
            }
        }
        young.unwrap()
    };
    let saturday = run(72);
    let monday = run(192 + 72);
    assert_eq!(saturday.0, monday.0);
    assert!(saturday.1 > monday.1);
    assert_eq!(saturday, run(72));
    // Sunday-to-Monday boundary samples 23:45 as Sunday, even though that
    // interval's closing timestamp is Monday 00:00.
    assert_eq!(run(96 + 95), saturday);
    assert_eq!(run(192), monday);
}
