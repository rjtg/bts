//! Small paired experiments; content and controls stay out of the engine.
use crate::*;
pub const MOVIE: ProgrammeId = ProgrammeId(10);
pub const FIRST: ProgrammeId = ProgrammeId(11);
pub const SECOND: ProgrammeId = ProgrammeId(12);
pub const THIRD: ProgrammeId = ProgrammeId(13);
pub const ARC: SeriesId = SeriesId(10);
fn world(opening: f64, continuity: f64) -> World {
    let mut w = demo::world();
    w.audiences.truncate(1);
    w.stations.truncate(1);
    w.brands.clear();
    let a = &mut w.audiences[0];
    a.population = 10000;
    a.genre_tastes = [1.0; 3];
    a.preferred_tone = 0.8;
    a.availability = [0.8; 24];
    w.stations[0].initial_cash = Money(100000);
    let template = w.programmes[0].clone();
    w.programmes = vec![MOVIE, FIRST, SECOND, THIRD]
        .into_iter()
        .map(|id| {
            let mut p = template.clone();
            p.id = id;
            p.name = if id == MOVIE {
                "Standalone movie".into()
            } else {
                format!("Arc episode {}", id.0 - 10)
            };
            p.kind = if id == MOVIE {
                ProgrammeKind::Standalone
            } else {
                ProgrammeKind::Episode {
                    series: ARC,
                    sequence: u32::from(id.0 - 10),
                }
            };
            p.genre = Genre::Drama;
            p.tone = 0.8;
            p.awareness = 0.8;
            p.cast_appeal = 0.8;
            p.execution = if id == FIRST { opening } else { 0.9 };
            p.broadcast_cost = Money(10);
            p
        })
        .collect();
    w.series = vec![Series {
        id: ARC,
        name: "The Arc".into(),
        continuity_strength: continuity,
    }];
    w
}
/// Eight controlled runs. Matched pairs differ only in the named intervention.
pub fn run() -> Result<Vec<demo::ExperimentRun>, Error> {
    let mut runs = Vec::new();
    for (name, opening, continuity, learning, sequence) in [
        ("standalone-movie", 0.9, 0.7, 0.9, vec![MOVIE]),
        ("strong-opening", 0.9, 0.7, 0.9, vec![FIRST, SECOND, THIRD]),
        (
            "disappointing-opening",
            0.05,
            0.7,
            0.9,
            vec![FIRST, SECOND, THIRD],
        ),
        (
            "earlier-repeat-control",
            0.9,
            0.0,
            0.0,
            vec![FIRST, SECOND, FIRST, THIRD],
        ),
        (
            "fresh-movie-control",
            0.9,
            0.0,
            0.0,
            vec![FIRST, SECOND, MOVIE, THIRD],
        ),
        (
            "out-of-order-low",
            0.9,
            0.0,
            0.9,
            vec![THIRD, FIRST, SECOND],
        ),
        (
            "out-of-order-high",
            0.9,
            1.0,
            0.9,
            vec![THIRD, FIRST, SECOND],
        ),
        (
            "ordered-high-control",
            0.9,
            1.0,
            0.9,
            vec![FIRST, SECOND, THIRD],
        ),
    ] {
        let mut world = world(opening, continuity);
        if name == "standalone-movie" {
            world.series.clear();
            world.programmes.retain(|p| p.id == MOVIE);
        }
        let parameters = Parameters {
            loyalty_learning: learning,
            ..Parameters::default()
        };
        let mut simulation = Simulation::new(world.clone(), parameters)?;
        let mut events = Vec::new();
        for (day, programme) in sequence.into_iter().enumerate() {
            events.push(simulation.step(&Slot {
                at: SimHour(day as u32 * 24 + 18),
                broadcasts: vec![Broadcast {
                    station: world.stations[0].id,
                    programme,
                    advertisement: None,
                }],
            })?);
        }
        runs.push(demo::ExperimentRun {
            case: name.into(),
            world,
            parameters,
            events,
            final_state: simulation.state().clone(),
        });
    }
    Ok(runs)
}
