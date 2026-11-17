//! Bounded deterministic TV experiments. See docs/headless-experiment.md.
//! State transitions are independent of CLI rendering and developer report conversion.
//!
//! ```
//! use bts_sim::{demo, Parameters, Simulation};
//! let mut simulation = Simulation::new(demo::world(), Parameters::default())?;
//! let actions = demo::schedule(demo::Case::Broad);
//! let evidence = simulation.step(&actions[0])?;
//! assert_eq!(simulation.state().now(), actions[0].at);
//! assert_eq!(evidence.allocations.len(), 3);
//! # Ok::<(), bts_sim::Error>(())
//! ```
#![forbid(unsafe_code)]

pub mod demo;
pub mod developer;
pub mod models;

use models::{ModelEvaluation, Models};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

macro_rules! id {
    ($($name:ident),*) => {$(
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
        pub struct $name(pub u16);
    )*};
}
id!(AudienceId, StationId, ProgrammeId, SeriesId, BrandId);

/// Absolute simulated hours since experiment origin, unrelated to wall time.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct SimHour(pub u32);
impl SimHour {
    fn availability_index(self) -> Result<usize, Error> {
        match self.0 % 24 {
            18 => Ok(0),
            21 => Ok(1),
            _ => Err(Error::Invalid(
                "broadcast slots must be at 18:00 or 21:00".into(),
            )),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub enum Genre {
    Comedy,
    Drama,
    Science,
}
impl Genre {
    fn index(self) -> usize {
        match self {
            Self::Comedy => 0,
            Self::Drama => 1,
            Self::Science => 2,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub enum Product {
    Snacks,
    Equipment,
}
impl Product {
    fn index(self) -> usize {
        match self {
            Self::Snacks => 0,
            Self::Equipment => 1,
        }
    }
}

/// All taste/availability/interest values are unitless in `[0,1]`.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Audience {
    pub id: AudienceId,
    pub name: String,
    pub population: u32,
    /// Comedy, drama, science affinities, in that order.
    pub genre_tastes: [f64; 3],
    pub preferred_tone: f64,
    /// 18:00 and 21:00 availability fractions.
    pub availability: [f64; 2],
    pub purchasing_power: f64,
    /// Snacks and equipment interests, in that order.
    pub product_interests: [f64; 2],
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Programme {
    pub id: ProgrammeId,
    pub series: SeriesId,
    pub name: String,
    pub genre: Genre,
    pub tone: f64,
    pub awareness: f64,
    pub cast_appeal: f64,
    pub execution: f64,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Station {
    pub id: StationId,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Brand {
    pub id: BrandId,
    pub name: String,
    pub product: Product,
    pub required_purchasing_power: f64,
    pub creative_quality: f64,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct World {
    pub audiences: Vec<Audience>,
    pub programmes: Vec<Programme>,
    pub stations: Vec<Station>,
    pub brands: Vec<Brand>,
}

/// Provisional coefficients; all numeric values validated at construction.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Parameters {
    pub outside_utility: f64,
    pub loyalty_learning: f64,
    pub hype_half_life_hours: f64,
    pub repetition_half_life_hours: f64,
    pub campaign_gain: f64,
}
impl Default for Parameters {
    fn default() -> Self {
        Self {
            outside_utility: 0.18,
            loyalty_learning: 0.9,
            hype_half_life_hours: 48.0,
            repetition_half_life_hours: 24.0,
            campaign_gain: 0.8,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Broadcast {
    pub station: StationId,
    pub programme: ProgrammeId,
    pub advertisement: Option<BrandId>,
}
/// Fixed ordered action. An empty broadcast list advances decay without exposure.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Slot {
    pub at: SimHour,
    pub broadcasts: Vec<Broadcast>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Invalid(String),
    Fuzzy(bts_fuzzy::Error),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(message) => f.write_str(message),
            Self::Fuzzy(error) => error.fmt(f),
        }
    }
}
impl std::error::Error for Error {}
impl From<bts_fuzzy::Error> for Error {
    fn from(value: bts_fuzzy::Error) -> Self {
        Self::Fuzzy(value)
    }
}
fn unit(value: f64, context: &str) -> Result<(), Error> {
    bounded(value, 0.0, 1.0, context)
}
fn bounded(value: f64, low: f64, high: f64, context: &str) -> Result<(), Error> {
    if !value.is_finite() || value < low || value > high {
        Err(Error::Invalid(format!(
            "{context}: expected finite value in [{low},{high}], got {value}"
        )))
    } else {
        Ok(())
    }
}
fn unique<T: Ord>(values: impl Iterator<Item = T>, context: &str) -> Result<(), Error> {
    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(value) {
            return Err(Error::Invalid(format!("duplicate {context} identifier")));
        }
    }
    Ok(())
}
impl World {
    fn validate(&self) -> Result<(), Error> {
        if self.audiences.len() != 3
            || self.stations.len() != 2
            || self.brands.len() != 2
            || self.programmes.is_empty()
        {
            return Err(Error::Invalid("experiment requires three audiences, two stations, two brands, and a nonempty catalogue".into()));
        }
        unique(self.audiences.iter().map(|x| x.id), "audience")?;
        unique(self.programmes.iter().map(|x| x.id), "programme")?;
        unique(self.stations.iter().map(|x| x.id), "station")?;
        unique(self.brands.iter().map(|x| x.id), "brand")?;
        for a in &self.audiences {
            if a.population == 0 {
                return Err(Error::Invalid(
                    "audience population must be positive".into(),
                ));
            }
            for value in a
                .genre_tastes
                .iter()
                .chain(&a.availability)
                .chain(&a.product_interests)
                .chain([&a.preferred_tone, &a.purchasing_power])
            {
                unit(*value, &format!("audience {:?}", a.id))?;
            }
        }
        for p in &self.programmes {
            for value in [p.tone, p.awareness, p.cast_appeal, p.execution] {
                unit(value, &format!("programme {:?}", p.id))?;
            }
        }
        for b in &self.brands {
            unit(b.creative_quality, "brand creative quality")?;
            bounded(
                b.required_purchasing_power,
                f64::MIN_POSITIVE,
                1.0,
                "required purchasing power",
            )?;
        }
        Ok(())
    }
}
impl Parameters {
    fn validate(self) -> Result<(), Error> {
        bounded(
            self.outside_utility,
            f64::MIN_POSITIVE,
            1.0,
            "outside utility",
        )?;
        unit(self.loyalty_learning, "loyalty learning")?;
        // Bounded away from zero to keep elapsed-hour divisions finite.
        bounded(self.hype_half_life_hours, 1.0, 1e6, "hype half-life hours")?;
        bounded(
            self.repetition_half_life_hours,
            1.0,
            1e6,
            "repetition half-life hours",
        )?;
        bounded(self.campaign_gain, 0.0, 5.0, "campaign gain")
    }
}

/// Actual world state. No serialization or player-facing view is provided.
#[derive(Clone, Debug, PartialEq)]
pub struct State {
    now: SimHour,
    loyalty: BTreeMap<(AudienceId, SeriesId), f64>,
    hype: BTreeMap<(BrandId, AudienceId), f64>,
    repetition: BTreeMap<(BrandId, AudienceId), f64>,
}
impl State {
    pub fn now(&self) -> SimHour {
        self.now
    }
    pub fn loyalty(&self, audience: AudienceId, series: SeriesId) -> Option<f64> {
        self.loyalty.get(&(audience, series)).copied()
    }
    pub fn hype(&self, brand: BrandId, audience: AudienceId) -> Option<f64> {
        self.hype.get(&(brand, audience)).copied()
    }
    pub fn repetition(&self, brand: BrandId, audience: AudienceId) -> Option<f64> {
        self.repetition.get(&(brand, audience)).copied()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Viewing {
    pub station: StationId,
    pub programme: ProgrammeId,
    pub series: SeriesId,
    pub attraction: ModelEvaluation,
    pub satisfaction: ModelEvaluation,
    pub utility: f64,
    pub viewers: f64,
    pub loyalty_before: f64,
    pub loyalty_after: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Allocation {
    pub audience: AudienceId,
    pub population: u32,
    pub available: f64,
    pub available_not_watching: f64,
    pub not_watching: f64,
    pub viewing: Vec<Viewing>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AdExposure {
    pub station: StationId,
    pub viewers: f64,
    pub effectiveness: ModelEvaluation,
}
#[derive(Clone, Debug, PartialEq)]
pub struct HypeChange {
    pub brand: BrandId,
    pub audience: AudienceId,
    pub before: f64,
    pub decayed: f64,
    pub repetition_before: f64,
    pub repetition_decayed: f64,
    pub exposures: f64,
    /// Effectiveness-weighted impressions per person.
    pub relevant_exposure_per_person: f64,
    pub campaign_generated: f64,
    pub after: f64,
    pub repetition_after: f64,
    pub advertisements: Vec<AdExposure>,
}
/// Immutable developer evidence from a committed transition, independent of rendering.
#[derive(Clone, Debug, PartialEq)]
pub struct SlotEvent {
    pub action: Slot,
    pub elapsed_hours: u32,
    pub allocations: Vec<Allocation>,
    pub hype: Vec<HypeChange>,
}

#[derive(Clone, Debug)]
pub struct Simulation {
    world: World,
    parameters: Parameters,
    models: Models,
    state: State,
}
impl Simulation {
    pub fn new(world: World, parameters: Parameters) -> Result<Self, Error> {
        world.validate()?;
        parameters.validate()?;
        let mut state = State {
            now: SimHour(0),
            loyalty: BTreeMap::new(),
            hype: BTreeMap::new(),
            repetition: BTreeMap::new(),
        };
        for a in &world.audiences {
            for p in &world.programmes {
                state.loyalty.insert((a.id, p.series), 0.5);
            }
            for b in &world.brands {
                state.hype.insert((b.id, a.id), 5.0);
                state.repetition.insert((b.id, a.id), 0.0);
            }
        }
        Ok(Self {
            world,
            parameters,
            models: Models::default(),
            state,
        })
    }
    pub fn world(&self) -> &World {
        &self.world
    }
    pub fn parameters(&self) -> Parameters {
        self.parameters
    }
    pub fn state(&self) -> &State {
        &self.state
    }

    /// Validate and apply one slot atomically. All viewing uses pre-slot loyalty.
    pub fn step(&mut self, action: &Slot) -> Result<SlotEvent, Error> {
        let period = action.at.availability_index()?;
        if action.at <= self.state.now {
            return Err(Error::Invalid("slot time must strictly increase".into()));
        }
        unique(
            action.broadcasts.iter().map(|b| b.station),
            "scheduled station",
        )?;
        for broadcast in &action.broadcasts {
            if !self
                .world
                .stations
                .iter()
                .any(|s| s.id == broadcast.station)
            {
                return Err(Error::Invalid(format!(
                    "unknown station {:?}",
                    broadcast.station
                )));
            }
            if !self
                .world
                .programmes
                .iter()
                .any(|p| p.id == broadcast.programme)
            {
                return Err(Error::Invalid(format!(
                    "unknown programme {:?}",
                    broadcast.programme
                )));
            }
            if broadcast
                .advertisement
                .is_some_and(|id| !self.world.brands.iter().any(|b| b.id == id))
            {
                return Err(Error::Invalid("unknown advertised brand".into()));
            }
        }
        let elapsed_hours = action.at.0 - self.state.now.0;
        let mut next = self.state.clone();
        let mut allocations = Vec::new();
        // Station configuration order, not action-list order, determines summation.
        for audience in &self.world.audiences {
            let population = f64::from(audience.population);
            let available = population * audience.availability[period];
            let mut viewing = Vec::new();
            for station in &self.world.stations {
                let Some(broadcast) = action.broadcasts.iter().find(|b| b.station == station.id)
                else {
                    continue;
                };
                let programme = self
                    .world
                    .programmes
                    .iter()
                    .find(|p| p.id == broadcast.programme)
                    .expect("validated programme");
                let genre = audience.genre_tastes[programme.genre.index()];
                let tone = 1.0 - (audience.preferred_tone - programme.tone).abs();
                let attraction = self.models.attraction(
                    genre,
                    tone,
                    programme.awareness,
                    programme.cast_appeal,
                )?;
                let satisfaction = self.models.satisfaction(genre, tone, programme.execution)?;
                let loyalty_before = self.state.loyalty[&(audience.id, programme.series)];
                let utility = attraction.trace.output.powi(2) * (0.2 + 1.6 * loyalty_before);
                viewing.push(Viewing {
                    station: station.id,
                    programme: programme.id,
                    series: programme.series,
                    attraction,
                    satisfaction,
                    utility,
                    viewers: 0.0,
                    loyalty_before,
                    loyalty_after: loyalty_before,
                });
            }
            let denominator =
                self.parameters.outside_utility + viewing.iter().map(|v| v.utility).sum::<f64>();
            let mut series_totals: BTreeMap<SeriesId, (f64, f64)> = BTreeMap::new();
            for v in &mut viewing {
                v.viewers = available * (v.utility / denominator);
                let total = series_totals.entry(v.series).or_default();
                total.0 += v.viewers / population;
                total.1 += (v.viewers / population) * v.satisfaction.trace.output;
            }
            for (series, (fraction, satisfaction_sum)) in series_totals {
                let old = self.state.loyalty[&(audience.id, series)];
                let updated =
                    old + self.parameters.loyalty_learning * (satisfaction_sum - fraction * old);
                unit(updated, "updated loyalty")?;
                next.loyalty.insert((audience.id, series), updated);
            }
            for v in &mut viewing {
                v.loyalty_after = next.loyalty[&(audience.id, v.series)];
            }
            let watched = viewing.iter().map(|v| v.viewers).sum::<f64>();
            bounded(watched, 0.0, available, "allocated viewers")?;
            allocations.push(Allocation {
                audience: audience.id,
                population: audience.population,
                available,
                available_not_watching: available - watched,
                not_watching: population - watched,
                viewing,
            });
        }
        let mut hype = Vec::new();
        for brand in &self.world.brands {
            for (audience, allocation) in self.world.audiences.iter().zip(&allocations) {
                let key = (brand.id, audience.id);
                let before = self.state.hype[&key];
                let repetition_before = self.state.repetition[&key];
                let decayed = before
                    * 2.0_f64
                        .powf(-f64::from(elapsed_hours) / self.parameters.hype_half_life_hours);
                let repetition_decayed = repetition_before
                    * 2.0_f64.powf(
                        -f64::from(elapsed_hours) / self.parameters.repetition_half_life_hours,
                    );
                let mut advertisements = Vec::new();
                for v in &allocation.viewing {
                    let broadcast = action
                        .broadcasts
                        .iter()
                        .find(|b| b.station == v.station)
                        .unwrap();
                    if broadcast.advertisement != Some(brand.id) {
                        continue;
                    }
                    let interest = audience.product_interests[brand.product.index()];
                    let affordability =
                        (audience.purchasing_power / brand.required_purchasing_power).min(1.0);
                    let effectiveness = self.models.advertising(
                        interest,
                        affordability,
                        brand.creative_quality,
                        v.satisfaction.trace.output,
                    )?;
                    advertisements.push(AdExposure {
                        station: v.station,
                        viewers: v.viewers,
                        effectiveness,
                    });
                }
                let exposures = advertisements.iter().map(|a| a.viewers).sum::<f64>();
                let relevant_exposure_per_person = advertisements
                    .iter()
                    .map(|a| {
                        a.viewers / f64::from(audience.population) * a.effectiveness.trace.output
                    })
                    .sum::<f64>();
                let dose = self.parameters.campaign_gain * relevant_exposure_per_person
                    / (1.0 + repetition_decayed);
                let campaign_generated = (100.0 - decayed) * (-(-dose).exp_m1());
                let after = decayed + campaign_generated;
                let repetition_after =
                    repetition_decayed + exposures / f64::from(audience.population);
                bounded(after, 0.0, 100.0, "hype")?;
                bounded(repetition_after, 0.0, 1e6, "repetition stock")?;
                next.hype.insert(key, after);
                next.repetition.insert(key, repetition_after);
                hype.push(HypeChange {
                    brand: brand.id,
                    audience: audience.id,
                    before,
                    decayed,
                    repetition_before,
                    repetition_decayed,
                    exposures,
                    relevant_exposure_per_person,
                    campaign_generated,
                    after,
                    repetition_after,
                    advertisements,
                });
            }
        }
        next.now = action.at;
        self.state = next;
        Ok(SlotEvent {
            action: action.clone(),
            elapsed_hours,
            allocations,
            hype,
        })
    }
}
