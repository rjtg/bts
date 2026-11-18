//! Fixed seven-day content. Timing and durations belong to Rust, not the UI.
use bts_sim::planning::{CompetitorAiring, PlanError, Planner, StationResources};
use bts_sim::{demo, Parameters, ProgrammeId, ProgrammeKind, SimTick, World};

pub const DAYS: u32 = 7;
pub const END_TICK: u32 = DAYS * 96;
pub const PLAYER: bts_sim::StationId = demo::MAIN;
pub const RIVAL: bts_sim::StationId = demo::RIVAL;

pub fn world() -> World {
    let mut world = demo::world();
    let mut movie = world.programmes[3].clone();
    movie.id = ProgrammeId(5);
    movie.kind = ProgrammeKind::Standalone;
    movie.name = "The Long Weekend".into();
    world.programmes.push(movie);
    for sequence in [2, 3] {
        let mut episode = world.programmes[0].clone();
        episode.id = ProgrammeId(4 + sequence as u16);
        episode.kind = ProgrammeKind::Episode {
            series: bts_sim::SeriesId(1),
            sequence,
        };
        episode.name = format!("Neighbourhood Laughs: episode {sequence}");
        world.programmes.push(episode);
    }
    for p in &mut world.programmes {
        p.duration_minutes = match p.id.0 {
            1 | 6 | 7 => 30,
            2 => 45,
            5 => 90,
            _ => 60,
        };
    }
    // Rival-only standalone works: separate identities and rights, without a new content loader.
    for (id, source, name) in [
        (8, 1, "After Hours"),
        (9, 2, "Tomorrow's Workshop"),
        (10, 4, "City Stories"),
        (11, 5, "The Last Tram"),
    ] {
        let mut work = world
            .programmes
            .iter()
            .find(|p| p.id == ProgrammeId(source))
            .unwrap()
            .clone();
        work.id = ProgrammeId(id);
        work.name = name.into();
        work.kind = ProgrammeKind::Standalone;
        world.programmes.push(work);
    }
    // Fixed provisional availability extends the earlier two-snapshot fixture
    // to the evening. No affinity/availability profile is sent to the browser.
    for audience in &mut world.audiences {
        let early = audience.availability[18];
        let late = audience.availability[21];
        for (hour, availability) in audience.availability.iter_mut().enumerate().skip(16) {
            *availability = match hour {
                16 => early * 0.35,
                17 => early * 0.65,
                18 => early,
                19 => early + (late - early) / 3.0,
                20 => early + 2.0 * (late - early) / 3.0,
                21 => late,
                22 => late * 0.8,
                _ => late * 0.5,
            };
        }
    }
    world
}

pub fn planner() -> Result<Planner, PlanError> {
    planner_from_world(world())
}

pub(crate) fn planner_from_world(world: World) -> Result<Planner, PlanError> {
    let mut competitors = vec![];
    for day in 0..DAYS {
        let mut tick = day * 96 + 16 * 4;
        for id in [8, 9, 10, 11, 10, 10, 9, 11] {
            let duration = u32::from(
                world
                    .programmes
                    .iter()
                    .find(|p| p.id == ProgrammeId(id))
                    .unwrap()
                    .duration_minutes,
            ) / 15;
            competitors.push(CompetitorAiring {
                station: RIVAL,
                programme: ProgrammeId(id),
                start: SimTick(tick),
            });
            tick += duration;
        }
    }
    let mut planner = Planner::new(
        world,
        Parameters::default(),
        PLAYER,
        SimTick(END_TICK),
        &competitors,
    )?;
    let (roster, request) = production_roster();
    planner.configure_production(roster, request)?;
    planner.configure_cooperation(bts_sim::planning::production::demo_cooperation())?;
    planner.configure_station_resources(vec![
        StationResources {
            station: PLAYER,
            studios: 1,
            programmes: (1..=7).map(ProgrammeId).collect(),
            staff: [
                bts_sim::planning::production::PersonId(1),
                bts_sim::planning::production::PersonId(2),
            ]
            .into_iter()
            .collect(),
        },
        StationResources {
            station: RIVAL,
            studios: 1,
            programmes: (8..=11).map(ProgrammeId).collect(),
            staff: [bts_sim::planning::production::PersonId(7)]
                .into_iter()
                .collect(),
        },
    ])?;
    let ages = planner
        .world()
        .audiences
        .iter()
        .map(|a| {
            (
                a.id,
                match a.id.0 {
                    1 => 22,
                    2 => 42,
                    _ => 56,
                },
            )
        })
        .collect();
    planner.configure_research(719, 12, ages)?;
    Ok(planner)
}

/// Scenario staff content, kept outside the generic production mechanics.
pub fn production_roster() -> (
    Vec<bts_sim::planning::production::Talent>,
    Vec<bts_sim::planning::production::ProductionRequest>,
) {
    use bts_sim::planning::production::*;
    let (mut people, requests) = demo_roster();
    let mut rival = people[1].clone();
    rival.id = PersonId(7);
    rival.name = "Vera Quinn".into();
    rival.description = "Channel Two's regular studio host.".into();
    people.push(rival);
    (people, requests)
}
