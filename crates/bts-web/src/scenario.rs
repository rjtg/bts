//! Fixed six-week content. Timing and durations belong to Rust, not the UI.
use bts_sim::planning::{CompetitorAiring, PlanError, Planner, StationResources};
use bts_sim::{demo, Parameters, ProgrammeId, ProgrammeKind, SimTick, World};

pub const DAYS: u32 = bts_sim::planning::weekly::MAX_DAYS;
pub const END_TICK: u32 = DAYS * 96;
pub const PLAYER: bts_sim::StationId = demo::MAIN;
const IDENT: ProgrammeId = ProgrammeId(0);
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
    let mut ident = world.programmes[0].clone();
    ident.id = IDENT;
    ident.name = "Station ident".into();
    ident.kind = ProgrammeKind::Standalone;
    ident.profile = bts_sim::FeatureProfile::fiction([0.1; 7]);
    ident.duration_minutes = 1;
    ident.broadcast_cost = bts_sim::economy::Money(0);
    ident.awareness = 0.05;
    ident.cast_appeal = 0.0;
    world.programmes.push(ident);
    // Fixed provisional availability extends the earlier two-snapshot fixture
    // to the evening. No affinity/availability profile is sent to the browser.
    for audience in &mut world.audiences {
        let early = audience.availability[18];
        let late = audience.availability[21];
        audience.availability[14] = early * 0.23;
        audience.availability[15] = early * 0.30;
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
            programmes: (1..=7).map(ProgrammeId).chain([IDENT]).collect(),
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
    planner.configure_filler(IDENT)?;
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
    planner.configure_viewing_calendar(
        bts_sim::planning::weekly::Weekday::SATURDAY,
        [
            (
                bts_sim::demo::YOUNG,
                [[0.27, 0.41, 0.52], [0.36, 0.63, 0.94]],
            ),
            (
                bts_sim::demo::HOUSEHOLDS,
                [[0.31, 0.52, 0.87], [0.49, 0.58, 0.67]],
            ),
            (
                bts_sim::demo::SPECIALISTS,
                [[0.35, 0.47, 0.57], [0.50, 0.93, 0.43]],
            ),
        ]
        .into_iter()
        .collect(),
    )?;
    planner.configure_research(719, 12, ages)?;
    planner.configure_rights(RIVAL, rights_titles(planner.world()))?;
    Ok(planner)
}

/// Scenario staff content, kept outside the generic production mechanics.
pub fn production_roster() -> (
    Vec<bts_sim::planning::production::Talent>,
    Vec<bts_sim::planning::production::ProductionRequest>,
) {
    use bts_sim::planning::production::*;
    let (mut people, requests) = demo_roster();
    for person in &mut people {
        if person.normal_until == SimTick(672) {
            person.normal_until = SimTick(END_TICK);
        }
    }
    let mut rival = people[1].clone();
    rival.id = PersonId(7);
    rival.name = "Vera Quinn".into();
    rival.description = "Channel Two's regular studio host.".into();
    people.push(rival);
    (people, requests)
}

pub(crate) fn rights_titles(world: &World) -> Vec<bts_sim::planning::rights::TitleSeed> {
    use bts_sim::{economy::Money, planning::rights::TitleSeed, FeatureProfile};
    vec![
        TitleSeed {
            programme: bts_sim::Programme {
                id: ProgrammeId(60000), kind: ProgrammeKind::Standalone,
                name: "The Corner Shop".into(), profile: FeatureProfile::fiction([0.84,0.74,0.27,0.83,0.59,0.33,0.67]),
                awareness: 0.66, cast_appeal: 0.6, execution: 0.50,
                broadcast_cost: Money(100), duration_minutes: 90,
            },
            public_profile: FeatureProfile::fiction([0.84,0.78,0.24,0.88,0.56,0.44,0.72]),
            public_execution: 0.81, release: SimTick(0), deadline: SimTick(144),
            base_price: Money(350), evidence: "A familiar neighbourhood comedy. An old television guide praises its cast. No recent audience figures.",
            art: "corner",
        },
        TitleSeed {
            programme: bts_sim::Programme {
                id: ProgrammeId(60001), kind: ProgrammeKind::Standalone,
                name: "Beyond the Observatory".into(), profile: world.programmes[1].profile,
                awareness: 0.45, cast_appeal: 0.35, execution: 0.86,
                broadcast_cost: Money(70), duration_minutes: 60,
            },
            public_profile: world.programmes[1].profile,
            public_execution: 0.58, release: SimTick(0), deadline: SimTick(216),
            base_price: Money(220), evidence: "An archive astronomy special. A specialist magazine remembers its clear explanations. Public reception is sparse.",
            art: "observatory",
        },
        TitleSeed {
            programme: bts_sim::Programme {
                id: ProgrammeId(60002), kind: ProgrammeKind::Standalone,
                name: "Night Chase".into(), profile: FeatureProfile::fiction([0.26,0.3,0.88,0.30,0.60,0.86,0.89]),
                awareness: 0.80, cast_appeal: 0.77, execution: 0.78,
                broadcast_cost: Money(170), duration_minutes: 90,
            },
            public_profile: FeatureProfile::fiction([0.22,0.27,0.94,0.29,0.57,0.91,0.86]),
            public_execution: 0.73, release: SimTick(192), deadline: SimTick(336),
            base_price: Money(1350), evidence: "Fresh from cinemas. A strong opening weekend and favourable press for the chase sequences. Press reactions to the story were mixed.",
            art: "chase",
        },
    ]
}
