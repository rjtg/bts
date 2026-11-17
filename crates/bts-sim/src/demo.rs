//! Fixed fixtures and schedules, not a general content loader.
use crate::*;

pub const MAIN: StationId = StationId(1);
pub const RIVAL: StationId = StationId(2);
pub const YOUNG: AudienceId = AudienceId(1);
pub const HOUSEHOLDS: AudienceId = AudienceId(2);
pub const SPECIALISTS: AudienceId = AudienceId(3);
pub const SNACKS: BrandId = BrandId(1);
pub const EQUIPMENT: BrandId = BrandId(2);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Case {
    Broad,
    Targeted,
    Disappointing,
}
impl Case {
    pub const ALL: [Self; 3] = [Self::Broad, Self::Targeted, Self::Disappointing];
    pub fn name(self) -> &'static str {
        match self {
            Self::Broad => "broad",
            Self::Targeted => "targeted",
            Self::Disappointing => "disappointing",
        }
    }
    pub fn programme(self) -> ProgrammeId {
        ProgrammeId(match self {
            Self::Broad => 1,
            Self::Targeted => 2,
            Self::Disappointing => 3,
        })
    }
}

fn availability(early: f64, late: f64) -> [f64; 24] {
    let mut hours = [0.0; 24];
    hours[18] = early;
    hours[21] = late;
    hours
}
pub fn world() -> World {
    World {
        series: (1..=4)
            .map(|id| Series {
                id: SeriesId(id),
                name: format!("Demo series {id}"),
                continuity_strength: 0.0,
            })
            .collect(),
        audiences: vec![
            Audience {
                id: YOUNG,
                name: "Young viewers".into(),
                population: 6000,
                genre_tastes: [1.0, 0.55, 0.08],
                preferred_tone: 0.9,
                availability: availability(0.8, 0.9),
                purchasing_power: 0.25,
                product_interests: [1.0, 0.05],
            },
            Audience {
                id: HOUSEHOLDS,
                name: "Households".into(),
                population: 11000,
                genre_tastes: [0.8, 0.8, 0.15],
                preferred_tone: 0.65,
                availability: availability(0.9, 0.45),
                purchasing_power: 0.55,
                product_interests: [0.8, 0.12],
            },
            Audience {
                id: SPECIALISTS,
                name: "Specialists".into(),
                population: 2500,
                genre_tastes: [0.1, 0.65, 1.0],
                preferred_tone: 0.15,
                availability: availability(0.6, 0.8),
                purchasing_power: 0.95,
                product_interests: [0.1, 1.0],
            },
        ],
        programmes: vec![
            Programme {
                duration_minutes: 60,
                id: ProgrammeId(1),
                kind: ProgrammeKind::Episode {
                    series: SeriesId(1),
                    sequence: 1,
                },
                name: "Neighbourhood Laughs".into(),
                genre: Genre::Comedy,
                tone: 0.8,
                awareness: 0.8,
                cast_appeal: 0.75,
                execution: 0.78,
                broadcast_cost: Money(150),
            },
            Programme {
                duration_minutes: 60,
                id: ProgrammeId(2),
                kind: ProgrammeKind::Episode {
                    series: SeriesId(2),
                    sequence: 1,
                },
                name: "Workshop Science".into(),
                genre: Genre::Science,
                tone: 0.15,
                awareness: 0.5,
                cast_appeal: 0.35,
                execution: 0.9,
                broadcast_cost: Money(50),
            },
            Programme {
                duration_minutes: 60,
                id: ProgrammeId(3),
                kind: ProgrammeKind::Episode {
                    series: SeriesId(3),
                    sequence: 1,
                },
                name: "The Big Premiere".into(),
                genre: Genre::Comedy,
                tone: 0.8,
                awareness: 1.0,
                cast_appeal: 1.0,
                execution: 0.05,
                broadcast_cost: Money(400),
            },
            Programme {
                duration_minutes: 60,
                id: ProgrammeId(4),
                kind: ProgrammeKind::Episode {
                    series: SeriesId(4),
                    sequence: 1,
                },
                name: "Evening Stories".into(),
                genre: Genre::Drama,
                tone: 0.45,
                awareness: 0.65,
                cast_appeal: 0.6,
                execution: 0.72,
                broadcast_cost: Money(100),
            },
        ],
        stations: vec![
            Station {
                id: MAIN,
                name: "Channel One".into(),
                initial_cash: Money(20000),
            },
            Station {
                id: RIVAL,
                name: "Channel Two".into(),
                initial_cash: Money(20000),
            },
        ],
        brands: vec![
            Brand {
                id: SNACKS,
                name: "Everyday Snacks".into(),
                product: Product::Snacks,
                required_purchasing_power: 0.2,
                creative_quality: 0.8,
                marketing_budget: Money(20000),
                maximum_offer: Money(4000),
                rate_per_thousand: Money(800),
                targets: vec![
                    Target {
                        audience: YOUNG,
                        priority: 1.0,
                        desired_hype: 40.0,
                    },
                    Target {
                        audience: HOUSEHOLDS,
                        priority: 1.0,
                        desired_hype: 40.0,
                    },
                    Target {
                        audience: SPECIALISTS,
                        priority: 0.1,
                        desired_hype: 15.0,
                    },
                ],
            },
            Brand {
                id: EQUIPMENT,
                name: "Precision Equipment".into(),
                product: Product::Equipment,
                required_purchasing_power: 0.8,
                creative_quality: 0.85,
                marketing_budget: Money(20000),
                maximum_offer: Money(4000),
                rate_per_thousand: Money(3500),
                targets: vec![
                    Target {
                        audience: SPECIALISTS,
                        priority: 1.0,
                        desired_hype: 45.0,
                    },
                    Target {
                        audience: YOUNG,
                        priority: 0.02,
                        desired_hype: 5.0,
                    },
                    Target {
                        audience: HOUSEHOLDS,
                        priority: 0.02,
                        desired_hype: 5.0,
                    },
                ],
            },
        ],
    }
}

pub fn schedule(case: Case) -> Vec<Slot> {
    let mut slots = Vec::new();
    for day in 0..6 {
        for hour in [18, 21] {
            slots.push(Slot {
                at: SimHour(day * 24 + hour),
                broadcasts: vec![
                    Broadcast {
                        station: MAIN,
                        programme: case.programme(),
                        advertisement: Some(if hour == 18 { EQUIPMENT } else { SNACKS }),
                    },
                    Broadcast {
                        station: RIVAL,
                        programme: ProgrammeId(4),
                        advertisement: None,
                    },
                ],
            });
        }
    }
    slots.push(Slot {
        at: SimHour(5 * 24 + 21 + 48),
        broadcasts: vec![],
    });
    slots
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExperimentRun {
    pub case: String,
    pub world: World,
    pub parameters: Parameters,
    pub events: Vec<SlotEvent>,
    pub final_state: State,
}
pub fn run(case: Case) -> Result<ExperimentRun, Error> {
    let world = world();
    let parameters = Parameters::default();
    let mut simulation = Simulation::new(world.clone(), parameters)?;
    let events = schedule(case)
        .iter()
        .map(|slot| simulation.step(slot))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ExperimentRun {
        case: case.name().into(),
        world,
        parameters,
        events,
        final_state: simulation.state().clone(),
    })
}
