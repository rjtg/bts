//! Fixed seven-day content. Timing and durations belong to Rust, not the UI.
use bts_sim::planning::{CompetitorAiring, PlanError, Planner};
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
    let world = world();
    let mut competitors = vec![];
    for day in 0..DAYS {
        let mut tick = day * 96 + 16 * 4;
        for id in [1, 2, 4, 5, 4, 3, 2, 5] {
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
    let (roster, request) = bts_sim::planning::production::demo_roster();
    planner.configure_production(roster, Some(request))?;
    Ok(planner)
}
