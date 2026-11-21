//! Standalone fiction production, with authored freelancers and fixed rental rooms.
use super::production::{PersonId, ProductionId, Subject};
use super::*;
use bts_fuzzy::{Condition, Degree, Membership, Rule, RuleSet};

pub const WRITING_TICKS: u32 = 48;
pub const REVISION_TICKS: u32 = 32;
pub const RECORDING_TICKS: u32 = 8;
pub const POST_TICKS: u32 = 16;
pub const ROLES: [&str; 2] = ["The Lead", "The Confidant"];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FilmJob {
    Writer,
    Director,
    Actor,
}

#[derive(Clone, Copy, Debug)]
pub struct Freelancer {
    pub id: PersonId,
    pub name: &'static str,
    pub job: FilmJob,
    pub fee: Money,
    pub specialty: &'static str,
    pub available_until: u32,
    craft: f64,
    capability: [f64; 7],
    recognition: f64,
}

pub const PEOPLE: [Freelancer; 7] = [
    Freelancer {
        id: PersonId(101),
        name: "Ada Mercer",
        job: FilmJob::Writer,
        fee: Money(360),
        specialty: "Warm character stories and small casts",
        available_until: 672,
        craft: 0.60,
        capability: [0.62, 0.72, 0.44, 0.88, 0.80, 0.34, 0.62],
        recognition: 0.3,
    },
    Freelancer {
        id: PersonId(102),
        name: "Leon Vale",
        job: FilmJob::Writer,
        fee: Money(680),
        specialty: "Ambitious suspense and elaborate action",
        available_until: 580,
        craft: 0.96,
        capability: [0.85, 0.92, 0.98, 0.83, 0.89, 0.99, 0.95],
        recognition: 0.55,
    },
    Freelancer {
        id: PersonId(103),
        name: "Inez Park",
        job: FilmJob::Director,
        fee: Money(430),
        specialty: "Intimate performances and gentle pacing",
        available_until: 672,
        craft: 0.73,
        capability: [0.65, 0.70, 0.67, 0.92, 0.95, 0.39, 0.66],
        recognition: 0.4,
    },
    Freelancer {
        id: PersonId(104),
        name: "Dev Rowan",
        job: FilmJob::Director,
        fee: Money(720),
        specialty: "Action staging and tight suspense",
        available_until: 590,
        craft: 0.88,
        capability: [0.63, 0.78, 0.93, 0.66, 0.70, 0.97, 0.95],
        recognition: 0.65,
    },
    Freelancer {
        id: PersonId(105),
        name: "Nina Bell",
        job: FilmJob::Actor,
        fee: Money(390),
        specialty: "Dry humour and close character work",
        available_until: 672,
        craft: 0.83,
        capability: [0.91, 0.83, 0.63, 0.79, 0.91, 0.57, 0.72],
        recognition: 0.60,
    },
    Freelancer {
        id: PersonId(106),
        name: "Sam Ives",
        job: FilmJob::Actor,
        fee: Money(340),
        specialty: "Warm performances and quick dialogue",
        available_until: 672,
        craft: 0.78,
        capability: [0.83, 0.80, 0.58, 0.92, 0.84, 0.54, 0.85],
        recognition: 0.48,
    },
    Freelancer {
        id: PersonId(107),
        name: "Rosa Dane",
        job: FilmJob::Actor,
        fee: Money(570),
        specialty: "Physical performances and intensity",
        available_until: 620,
        craft: 0.87,
        capability: [0.57, 0.73, 0.93, 0.58, 0.74, 0.93, 0.89],
        recognition: 0.75,
    },
];

#[derive(Clone, Copy, Debug)]
pub struct Rental {
    pub id: u8,
    pub name: &'static str,
    pub fee: Money,
    pub summary: &'static str,
    equipment: f64,
}
pub const RENTALS: [Rental; 2] = [
    Rental {
        id: 1,
        name: "Backlot Room",
        fee: Money(280),
        summary: "A compact room for dialogue and small scenes.",
        equipment: 0.32,
    },
    Rental {
        id: 2,
        name: "Soundstage B",
        fee: Money(930),
        summary: "Space, rigs and lighting for ambitious action.",
        equipment: 0.94,
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FilmPhase {
    DevelopingIdea,
    Pitch,
    Writing,
    ScriptReview,
    Revising,
    Casting,
    Booked,
    Recording,
    Post,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PitchPace {
    Rush,
    Steady,
    TakeTime,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FilmIdea {
    pub description: String,
    pace: PitchPace,
}

fn film_idea_pressure(ambition: [f64; 7]) -> Option<f64> {
    FeatureProfile::fiction(ambition).is_valid().then(|| {
        let mut demands = ambition;
        demands[1] = 2.0 * (demands[1] - 0.5).abs();
        let peak = demands.into_iter().fold(0.0, f64::max);
        (peak + demands.iter().sum::<f64>() / 7.0) / 2.0
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct Script {
    pub profile: FeatureProfile,
    pub weaknesses: Vec<usize>,
    pub equipment_demand: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FilmBooking {
    pub cast: [PersonId; 2],
    pub director: PersonId,
    pub studio: u8,
    pub start: SimTick,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FilmQuote {
    pub fees: Money,
    pub rent: Money,
    pub total: Money,
    pub recording_end: SimTick,
    pub ready: SimTick,
    pub studio_shortfall: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Film {
    pub id: ProductionId,
    pub title: String,
    pub subject: Subject,
    pub ambition: FeatureProfile,
    pub idea: Option<FilmIdea>,
    pub phase: FilmPhase,
    pub committed: Money,
    pub writer: Option<PersonId>,
    pub writing_start: Option<SimTick>,
    pub due: Option<SimTick>,
    pub script: Option<Script>,
    pub revision_focus: Option<usize>,
    pub booking: Option<FilmBooking>,
    pub quote: Option<FilmQuote>,
    pub programme: Option<ProgrammeId>,
    pub evidence: Option<FilmEvidence>,
    pub episode: Option<super::scripted_run::ScriptedEpisode>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FilmEvidence {
    pub profile: FeatureProfile,
    pub craft: f64,
    pub cooperation: f64,
    pub equipment: f64,
    pub spectacle_support: f64,
    pub execution: f64,
}

pub(super) fn audition_fit(
    script: &Script,
    person: PersonId,
    role: usize,
) -> Result<f64, PlanError> {
    let actor = talent(person, FilmJob::Actor)?;
    let indexes = if role == 0 { [0, 2, 6] } else { [1, 3, 4] };
    let capability = indexes
        .into_iter()
        .map(|i| 1.0 - (actor.capability[i] - script.profile.values[i].unwrap_or(0.0)).abs())
        .sum::<f64>()
        / 3.0;
    Ok(capability * 0.7 + actor.craft * 0.3)
}

fn talent(id: PersonId, job: FilmJob) -> Result<&'static Freelancer, PlanError> {
    PEOPLE
        .iter()
        .find(|p| p.id == id && p.job == job)
        .ok_or_else(|| error("film_talent", "Choose a person suited to this assignment."))
}
fn overlap(start: SimTick, end: SimTick, other_start: SimTick, other_end: SimTick) -> bool {
    start < other_end && other_start < end
}

impl Planner {
    pub fn films(&self) -> impl Iterator<Item = &Film> {
        self.films.values()
    }

    pub fn film(&self, id: ProductionId) -> Result<&Film, PlanError> {
        self.films
            .get(&id)
            .ok_or_else(|| error("unknown_film", "Choose an existing film project."))
    }

    pub fn pitch_film(
        &mut self,
        title: String,
        subject: Subject,
        ambition: [f64; 7],
    ) -> Result<ProductionId, PlanError> {
        let title = title.trim().to_owned();
        let ambition = FeatureProfile::fiction(ambition);
        if title.is_empty() || title.chars().count() > 60 || !ambition.is_valid() {
            return Err(error(
                "film_pitch",
                "Enter a title up to 60 characters and valid creative targets.",
            ));
        }
        let id = ProductionId(self.production.next_project);
        let next =
            id.0.checked_add(1)
                .ok_or_else(|| error("capacity", "Project identifiers exhausted."))?;
        self.bump()?;
        self.production.next_project = next;
        self.films.insert(
            id,
            Film {
                id,
                title,
                subject,
                ambition,
                idea: None,
                phase: FilmPhase::Pitch,
                committed: Money(0),
                writer: None,
                writing_start: None,
                due: None,
                script: None,
                revision_focus: None,
                booking: None,
                quote: None,
                programme: None,
                evidence: None,
                episode: None,
            },
        );
        Ok(id)
    }

    pub fn develop_film_idea(
        &mut self,
        title: String,
        description: String,
        ambition: [f64; 7],
        pace: PitchPace,
    ) -> Result<ProductionId, PlanError> {
        let description = description.trim().to_owned();
        if description.is_empty() || description.chars().count() > 800 {
            return Err(error(
                "film_description",
                "Write a description of up to 800 characters.",
            ));
        }
        let pressure = film_idea_pressure(ambition)
            .ok_or_else(|| error("film_pitch", "Choose valid creative targets."))?;
        let standard_ticks = 4 + (12.0 * pressure).round() as u32;
        let ticks = match pace {
            PitchPace::Rush => standard_ticks.div_ceil(2),
            PitchPace::Steady => standard_ticks,
            PitchPace::TakeTime => (3 * standard_ticks).div_ceil(2),
        };
        let due = self
            .now()
            .0
            .checked_add(ticks)
            .filter(|due| *due <= self.end.0)
            .ok_or_else(|| {
                error(
                    "film_idea_deadline",
                    "This idea would be ready after the station season ends.",
                )
            })?;
        let id = self.pitch_film(title, Subject::EverydayLife, ambition)?;
        let film = self.films.get_mut(&id).unwrap();
        film.idea = Some(FilmIdea { description, pace });
        film.phase = FilmPhase::DevelopingIdea;
        film.due = Some(SimTick(due));
        Ok(id)
    }

    pub fn write_film(
        &mut self,
        id: ProductionId,
        writer: PersonId,
        focus: Option<u8>,
    ) -> Result<(), PlanError> {
        let film = self.film(id)?;
        let revise = focus.is_some();
        if (!revise && film.phase != FilmPhase::Pitch)
            || (revise && (film.phase != FilmPhase::ScriptReview || film.revision_focus.is_some()))
        {
            return Err(error(
                "film_stage",
                "This film cannot commission writing at its current stage.",
            ));
        }
        let focus = focus.map(usize::from);
        if focus.is_some_and(|i| !film.script.as_ref().unwrap().weaknesses.contains(&i)) {
            return Err(error(
                "film_revision",
                "Choose a shortcoming noted in the script review.",
            ));
        }
        let writer = talent(writer, FilmJob::Writer)?;
        let start = self.now();
        let due = SimTick(
            start.0
                + if revise {
                    REVISION_TICKS
                } else {
                    WRITING_TICKS
                },
        );
        if due > self.end
            || due.0 > writer.available_until
            || self.films().any(|other| {
                other.writer == Some(writer.id)
                    && matches!(other.phase, FilmPhase::Writing | FilmPhase::Revising)
            })
        {
            return Err(error(
                "film_writer_busy",
                "This writer is unavailable for the full writing block.",
            ));
        }
        let price = Money(if revise {
            writer.fee.0.div_ceil(2)
        } else {
            writer.fee.0
        });
        let committed = film.committed.add(price).map_err(engine_error)?;
        let mut candidate = self.clone();
        candidate.pay_for_film(id, price, true)?;
        let film = candidate.films.get_mut(&id).unwrap();
        film.phase = if revise {
            FilmPhase::Revising
        } else {
            FilmPhase::Writing
        };
        film.writer = Some(writer.id);
        film.writing_start = Some(start);
        film.due = Some(due);
        film.revision_focus = focus;
        film.committed = committed;
        *self = candidate;
        Ok(())
    }

    pub fn accept_script(&mut self, id: ProductionId) -> Result<(), PlanError> {
        if self.film(id)?.phase != FilmPhase::ScriptReview {
            return Err(error("film_stage", "The script is not ready to accept."));
        }
        self.bump()?;
        self.films.get_mut(&id).unwrap().phase = FilmPhase::Casting;
        Ok(())
    }

    fn pay_for_film(
        &mut self,
        id: ProductionId,
        price: Money,
        immediately: bool,
    ) -> Result<(), PlanError> {
        let now = self.now();
        let economy = &mut self.engine.state.economy;
        if economy.cash(self.player).unwrap() < price {
            return Err(error(
                "insufficient_production_funds",
                "Station cash cannot cover this commitment.",
            ));
        }
        economy
            .reserve_production(AirMinute::from_tick(now), self.player, id, price)
            .map_err(engine_error)?;
        if immediately {
            economy
                .spend_production(AirMinute::from_tick(now), id)
                .map_err(engine_error)?;
        }
        self.bump()
    }

    pub fn review_film(
        &self,
        id: ProductionId,
        booking: FilmBooking,
    ) -> Result<FilmQuote, PlanError> {
        let film = self.film(id)?;
        if film.phase != FilmPhase::Casting {
            return Err(error(
                "film_stage",
                "Accept the script before booking filming.",
            ));
        }
        if let Some(episode) = film.episode {
            self.validate_series_cast(episode, booking.cast)?;
        }
        let start = booking.start;
        if start <= self.now() || start > self.end {
            return Err(error(
                "film_date",
                "Choose a future recording time within this week.",
            ));
        }
        let end = SimTick(start.0 + RECORDING_TICKS);
        let ready = SimTick(end.0 + POST_TICKS);
        let airtime_ticks = if film.episode.is_some() { 3 } else { 6 };
        if ready.0 + airtime_ticks > self.end.0 {
            return Err(error(
                "film_date",
                "Allow time for filming, post-production and a feature-length airing.",
            ));
        }
        let [lead, second] = booking.cast;
        if lead == second {
            return Err(error(
                "film_duplicate_cast",
                "Each role needs a different actor.",
            ));
        }
        let crew = [
            talent(lead, FilmJob::Actor)?,
            talent(second, FilmJob::Actor)?,
            talent(booking.director, FilmJob::Director)?,
        ];
        let rental = RENTALS
            .iter()
            .find(|r| r.id == booking.studio)
            .ok_or_else(|| error("film_studio", "Choose a listed studio."))?;
        for person in crew {
            if end.0 > person.available_until
                || self.films().any(|other| {
                    other
                        .booking
                        .zip(other.quote.as_ref())
                        .is_some_and(|(b, q)| {
                            (b.cast.contains(&person.id) || b.director == person.id)
                                && overlap(start, end, b.start, q.recording_end)
                        })
                })
            {
                return Err(error(
                    "film_talent_conflict",
                    "A selected cast or crew member is unavailable during this recording.",
                ));
            }
        }
        if self.films().any(|other| {
            other
                .booking
                .zip(other.quote.as_ref())
                .is_some_and(|(b, q)| {
                    b.studio == booking.studio && overlap(start, end, b.start, q.recording_end)
                })
        }) {
            return Err(error(
                "film_studio_conflict",
                "This studio is reserved during part of the selected time.",
            ));
        }
        let fees = Money(crew.iter().map(|p| p.fee.0).sum());
        let total = fees.add(rental.fee).map_err(engine_error)?;
        if self.state().economy().cash(self.player).unwrap() < total {
            return Err(error(
                "insufficient_production_funds",
                "Station cash cannot cover this commitment.",
            ));
        }
        Ok(FilmQuote {
            fees,
            rent: rental.fee,
            total,
            recording_end: end,
            ready,
            studio_shortfall: rental.equipment < film.script.as_ref().unwrap().equipment_demand,
        })
    }

    pub fn book_film(&mut self, id: ProductionId, booking: FilmBooking) -> Result<(), PlanError> {
        let quote = self.review_film(id, booking)?;
        let committed = self
            .film(id)?
            .committed
            .add(quote.total)
            .map_err(engine_error)?;
        let mut candidate = self.clone();
        candidate.pay_for_film(id, quote.total, false)?;
        let film = candidate.films.get_mut(&id).unwrap();
        film.phase = FilmPhase::Booked;
        film.due = Some(quote.ready);
        film.committed = committed;
        film.booking = Some(booking);
        film.quote = Some(quote);
        if let Some(episode) = film.episode {
            candidate
                .scripted_runs
                .get_mut(&episode.series)
                .unwrap()
                .cast
                .get_or_insert(booking.cast);
        }
        *self = candidate;
        Ok(())
    }

    pub(super) fn advance_films(&mut self, at: SimTick) -> Result<(), PlanError> {
        for film in self.films.values_mut() {
            match film.phase {
                FilmPhase::DevelopingIdea if film.due == Some(at) => {
                    film.phase = FilmPhase::Pitch;
                    film.due = None;
                }
                FilmPhase::Writing | FilmPhase::Revising if film.due == Some(at) => {
                    let writer = talent(film.writer.unwrap(), FilmJob::Writer)?;
                    film.script = Some(write_script(film, writer));
                    film.phase = FilmPhase::ScriptReview;
                }
                FilmPhase::Booked if film.booking.is_some_and(|b| b.start == at) => {
                    self.engine
                        .state
                        .economy
                        .spend_production(AirMinute::from_tick(at), film.id)
                        .map_err(engine_error)?;
                    film.phase = FilmPhase::Recording;
                }
                FilmPhase::Recording
                    if film.quote.as_ref().is_some_and(|q| q.recording_end == at) =>
                {
                    film.phase = FilmPhase::Post;
                }
                FilmPhase::Post if film.due == Some(at) => {
                    let next = super::rights::original_programme_number(
                        &self.engine.world.programmes,
                        self.rights.as_ref(),
                        self.production.next_programme,
                    )?;
                    self.production.next_programme = next
                        .checked_add(1)
                        .ok_or_else(|| error("capacity", "Programme identifiers exhausted."))?;
                    let id = ProgrammeId(next);
                    let (work, evidence) = film_work(film, id)?;
                    self.engine.world.programmes.push(work);
                    self.engine.world.validate().map_err(engine_error)?;
                    self.libraries.entry(self.player).or_default().insert(id);
                    for a in &self.engine.world.audiences {
                        self.engine.state.familiarity.insert((a.id, id), 0.0);
                    }
                    film.programme = Some(id);
                    film.evidence = Some(evidence);
                    film.phase = FilmPhase::Complete;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

fn write_script(film: &Film, writer: &Freelancer) -> Script {
    let targets: [f64; 7] = std::array::from_fn(|i| film.ambition.values[i].unwrap());
    let previous = film.script.as_ref();
    let rush = film.idea.as_ref().map_or(0.0, |idea| match idea.pace {
        PitchPace::Rush => 0.12,
        PitchPace::Steady => 0.0,
        PitchPace::TakeTime => -0.12,
    });
    let delivered = std::array::from_fn(|i| {
        let target = targets[i];
        let baseline = if i == 1 { 0.5 } else { 0.0 };
        if let Some(previous) = previous {
            let old = previous.profile.values[i].unwrap();
            return if film.revision_focus == Some(i) {
                old + (target - old) * (0.25 + 0.40 * writer.capability[i])
            } else {
                old
            };
        }
        let complexity = if i == 1 {
            2.0 * (target - 0.5).abs()
        } else {
            target
        };
        let subject_fit = if writer.id.0 == 101 {
            [0.65, 1.0, 0.84]
        } else {
            [0.94, 0.80, 1.0]
        }[film.subject.index()];
        let capacity = 0.30 * writer.craft + 0.60 * writer.capability[i] + 0.10 * subject_fit;
        let strain = if i == 2 {
            film.episode.map_or(0.0, |episode| episode.strain)
        } else {
            0.0
        };
        baseline
            + (target - baseline)
                * (1.0 - 0.8 * (complexity + rush - capacity).max(0.0))
                * (1.0 - strain)
    });
    let weaknesses = (0..7)
        .filter(|i| (targets[*i] - delivered[*i]).abs() >= 0.075)
        .collect();
    Script {
        profile: FeatureProfile::fiction(delivered),
        weaknesses,
        equipment_demand: 0.2 + 0.75 * delivered[5],
    }
}

fn film_work(film: &Film, id: ProgrammeId) -> Result<(Programme, FilmEvidence), PlanError> {
    let booking = film.booking.unwrap();
    let studio = RENTALS.iter().find(|r| r.id == booking.studio).unwrap();
    let director = talent(booking.director, FilmJob::Director)?;
    let lead = talent(booking.cast[0], FilmJob::Actor)?;
    let second = talent(booking.cast[1], FilmJob::Actor)?;
    let script = film.script.as_ref().unwrap();
    let craft = 0.5 * director.craft + 0.3 * lead.craft + 0.2 * second.craft;
    let cooperation = if [lead.id.0, second.id.0].contains(&107) && director.id.0 == 103 {
        0.80
    } else {
        1.0
    };
    let equipment = (studio.equipment / script.equipment_demand).min(1.0);
    let high = |key| Condition::is(key, "high", Membership::rising(0.0, 1.0).unwrap());
    let joint = high(0).and(high(1));
    let weight = Degree::new(1.0).unwrap();
    let rules = RuleSet::new(
        vec![
            Rule::new("direction and equipment", joint.clone(), weight, 1.0).unwrap(),
            Rule::new("staging limitations", joint.negate(), weight, 0.0).unwrap(),
        ],
        0.0,
    )
    .unwrap();
    let support = rules
        .evaluate(&[(0, director.capability[5]), (1, equipment)])
        .map_err(|_| {
            error(
                "film_model",
                "The staging model could not evaluate this film.",
            )
        })?
        .output;
    let profile = FeatureProfile::fiction(std::array::from_fn(|i| {
        let base = if i == 1 { 0.5 } else { 0.0 };
        let capability = if i == 5 {
            support
        } else {
            0.50 * director.capability[i] + 0.30 * lead.capability[i] + 0.20 * second.capability[i]
        };
        base + (script.profile.values[i].unwrap() - base)
            * (0.40 + 0.45 * capability + 0.15 * craft)
            * cooperation
    }));
    let shortfall = film
        .ambition
        .values
        .iter()
        .zip(script.profile.values)
        .filter_map(|(a, b)| a.zip(b).map(|(a, b)| (a - b).abs()))
        .sum::<f64>()
        / 7.0;
    let strain = film.episode.map_or(0.0, |episode| episode.strain);
    let execution = (0.20 + 0.65 * craft + 0.15 * equipment)
        * cooperation
        * (1.0 - shortfall)
        * (1.0 - 0.5 * strain);
    let evidence = FilmEvidence {
        profile,
        craft,
        cooperation,
        equipment,
        spectacle_support: support,
        execution,
    };
    Ok((
        Programme {
            id,
            kind: film.episode.map_or(ProgrammeKind::Standalone, |episode| {
                ProgrammeKind::Episode {
                    series: episode.series,
                    sequence: episode.sequence,
                }
            }),
            name: film.title.clone(),
            profile,
            awareness: 0.52,
            cast_appeal: 0.6 * lead.recognition + 0.4 * second.recognition,
            execution,
            broadcast_cost: Money(120),
            duration_minutes: if film.episode.is_some() { 45 } else { 90 },
        },
        evidence,
    ))
}
