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

pub mod advertiser_value;
pub mod demo;
pub mod developer;
pub mod economy;
pub mod economy_demo;
pub mod episodes_demo;
pub mod models;
pub mod planning;
pub mod programmes;

use economy::{EconomyState, FinancialEvent, Money, Target};
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
/// Clock units are explicit; the legacy experiments and planning share mechanics.
mod clock_sealed {
    pub trait Sealed {}
    impl Sealed for super::SimHour {}
    impl Sealed for super::SimTick {}
}
pub trait Clock: clock_sealed::Sealed + Copy + std::fmt::Debug + Eq + Ord + Serialize {
    fn zero() -> Self;
    fn availability_index(self) -> usize;
    fn elapsed_hours(self, before: Self) -> f64;
}
impl Clock for SimHour {
    fn zero() -> Self {
        Self(0)
    }
    fn availability_index(self) -> usize {
        (self.0 % 24) as usize
    }
    fn elapsed_hours(self, before: Self) -> f64 {
        f64::from(self.0 - before.0)
    }
}
/// Absolute 15-minute simulated ticks. Intervals are [start,end).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct SimTick(pub u32);
impl Clock for SimTick {
    fn zero() -> Self {
        Self(0)
    }
    fn availability_index(self) -> usize {
        ((self.0 / 4) % 24) as usize
    }
    fn elapsed_hours(self, before: Self) -> f64 {
        f64::from(self.0 - before.0) / 4.0
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
    /// Availability fraction for each hour of the day.
    pub availability: [f64; 24],
    pub purchasing_power: f64,
    /// Snacks and equipment interests, in that order.
    pub product_interests: [f64; 2],
}
/// Stable series identity; work-specific content lives on programmes.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Series {
    pub id: SeriesId,
    pub name: String,
    pub continuity_strength: f64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum ProgrammeKind {
    Standalone,
    Episode { series: SeriesId, sequence: u32 },
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Programme {
    pub id: ProgrammeId,
    pub kind: ProgrammeKind,
    pub name: String,
    pub genre: Genre,
    pub tone: f64,
    pub awareness: f64,
    pub cast_appeal: f64,
    pub execution: f64,
    pub broadcast_cost: Money,
    /// Positive runtime, in whole 15-minute ticks expressed as minutes.
    pub duration_minutes: u16,
}
impl Programme {
    pub fn series(&self) -> Option<SeriesId> {
        match self.kind {
            ProgrammeKind::Standalone => None,
            ProgrammeKind::Episode { series, .. } => Some(series),
        }
    }
    pub fn sequence(&self) -> Option<u32> {
        match self.kind {
            ProgrammeKind::Standalone => None,
            ProgrammeKind::Episode { sequence, .. } => Some(sequence),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Station {
    pub id: StationId,
    pub name: String,
    pub initial_cash: Money,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Brand {
    pub id: BrandId,
    pub name: String,
    pub product: Product,
    pub required_purchasing_power: f64,
    pub creative_quality: f64,
    pub marketing_budget: Money,
    pub maximum_offer: Money,
    pub rate_per_thousand: Money,
    pub targets: Vec<Target>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct World {
    pub series: Vec<Series>,
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
pub struct Slot<T: Clock = SimHour> {
    pub at: T,
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
        unique(self.audiences.iter().map(|x| x.id), "audience")?;
        unique(self.programmes.iter().map(|x| x.id), "programme")?;
        unique(self.series.iter().map(|x| x.id), "series")?;
        let mut episode_sequences = BTreeSet::new();
        for series in &self.series {
            unit(series.continuity_strength, "series continuity strength")?;
        }
        for programme in &self.programmes {
            if let ProgrammeKind::Episode { series, sequence } = programme.kind {
                if sequence == 0 {
                    return Err(Error::Invalid("episode sequence must be positive".into()));
                }
                if !self.series.iter().any(|s| s.id == series) {
                    return Err(Error::Invalid("unknown programme series".into()));
                }
                if !episode_sequences.insert((series, sequence)) {
                    return Err(Error::Invalid(
                        "duplicate episode sequence within series".into(),
                    ));
                }
            }
        }
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
        for station in &self.stations {
            station.initial_cash.validate()?;
        }
        for p in &self.programmes {
            p.broadcast_cost.validate()?;
            if p.duration_minutes == 0 || !p.duration_minutes.is_multiple_of(15) {
                return Err(Error::Invalid(
                    "programme duration must be a positive multiple of 15 minutes".into(),
                ));
            }
            for value in [p.tone, p.awareness, p.cast_appeal, p.execution] {
                unit(value, &format!("programme {:?}", p.id))?;
            }
        }
        for b in &self.brands {
            economy::validate_brand(b, &self.audiences)?;
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
pub struct State<T: Clock = SimHour> {
    now: T,
    loyalty: BTreeMap<(AudienceId, SeriesId), f64>,
    hype: BTreeMap<(BrandId, AudienceId), f64>,
    repetition: BTreeMap<(BrandId, AudienceId), f64>,
    economy: EconomyState<T>,
    familiarity: BTreeMap<(AudienceId, ProgrammeId), f64>,
    reception: BTreeMap<(AudienceId, ProgrammeId), f64>,
    progress: BTreeMap<(AudienceId, SeriesId), u32>,
    broadcasts: Vec<programmes::BroadcastRecord<T>>,
}
impl<T: Clock> State<T> {
    pub fn economy(&self) -> &EconomyState<T> {
        &self.economy
    }
    pub fn now(&self) -> T {
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
    pub series: Option<SeriesId>,
    pub kind: ProgrammeKind,
    pub history: programmes::SelectionHistory,
    pub fresh_viewers: f64,
    pub familiarity_after: f64,
    pub attraction: ModelEvaluation,
    pub satisfaction: ModelEvaluation,
    pub utility: f64,
    pub viewers: f64,
    pub loyalty_before: Option<f64>,
    pub loyalty_after: Option<f64>,
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
/// Deterministic funding failure of a valid station plan.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OffAir {
    pub planned: Broadcast,
    pub available_cash: Money,
    pub required_cost: Money,
}
/// Immutable developer evidence from a committed transition, independent of rendering.
#[derive(Clone, Debug, PartialEq)]
pub struct SlotEvent<T: Clock = SimHour> {
    pub action: Slot<T>,
    pub aired: Slot<T>,
    pub off_air: Vec<OffAir>,
    pub elapsed_hours: f64,
    /// Stations starting a fresh airing; continuations do not repeat full costs/effects.
    pub starting: Vec<StationId>,
    pub allocations: Vec<Allocation>,
    pub hype: Vec<HypeChange>,
    pub financial: FinancialEvent<T>,
}

/// Legacy whole-hour point experiments. The planner uses the same engine with ticks.
pub type Simulation = Engine<SimHour>;

#[derive(Clone, Debug)]
pub struct Engine<T: Clock> {
    world: World,
    parameters: Parameters,
    models: Models,
    state: State<T>,
}
impl<T: Clock> Engine<T> {
    pub fn new(world: World, parameters: Parameters) -> Result<Self, Error> {
        world.validate()?;
        parameters.validate()?;
        let mut state = State {
            now: T::zero(),
            loyalty: BTreeMap::new(),
            hype: BTreeMap::new(),
            repetition: BTreeMap::new(),
            economy: EconomyState::new(&world),
            familiarity: BTreeMap::new(),
            reception: BTreeMap::new(),
            progress: BTreeMap::new(),
            broadcasts: Vec::new(),
        };
        for a in &world.audiences {
            for p in &world.programmes {
                state.familiarity.insert((a.id, p.id), 0.0);
            }
            for series in &world.series {
                state.loyalty.insert((a.id, series.id), 0.5);
                state.progress.insert((a.id, series.id), 0);
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
    pub fn state(&self) -> &State<T> {
        &self.state
    }

    /// Validate and apply one slot atomically. All viewing uses pre-slot loyalty.
    pub fn step(&mut self, action: &Slot<T>) -> Result<SlotEvent<T>, Error> {
        let (mut next, mut event) = self.evaluate_slot(action)?;
        event.financial = economy::apply_slot(&self.world, &mut next.economy, &event)?;
        next.economy.advance_revision()?;
        self.state = next;
        Ok(event)
    }

    fn evaluate_slot(&self, action: &Slot<T>) -> Result<(State<T>, SlotEvent<T>), Error> {
        self.evaluate_cadence(action, None, action.at.availability_index())
    }

    fn evaluate_cadence(
        &self,
        action: &Slot<T>,
        starts: Option<&BTreeSet<StationId>>,
        period: usize,
    ) -> Result<(State<T>, SlotEvent<T>), Error> {
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
        economy::validate_inventory(&self.state.economy, action)?;
        let mut aired = Slot {
            at: action.at,
            broadcasts: Vec::new(),
        };
        let mut off_air = Vec::new();
        for station in &self.world.stations {
            if let Some(broadcast) = action.broadcasts.iter().find(|b| b.station == station.id) {
                let programme = self
                    .world
                    .programmes
                    .iter()
                    .find(|p| p.id == broadcast.programme)
                    .expect("validated programme");
                let available_cash = self
                    .state
                    .economy
                    .cash(station.id)
                    .expect("configured station");
                if starts.is_none_or(|set| set.contains(&station.id))
                    && available_cash < programme.broadcast_cost
                {
                    off_air.push(OffAir {
                        planned: broadcast.clone(),
                        available_cash,
                        required_cost: programme.broadcast_cost,
                    });
                } else {
                    aired.broadcasts.push(broadcast.clone());
                }
            }
        }
        let elapsed_hours = action.at.elapsed_hours(self.state.now);
        let mut next = self.state.clone();
        let mut allocations = Vec::new();
        // Station configuration order, not action-list order, determines summation.
        for audience in &self.world.audiences {
            let population = f64::from(audience.population);
            let available = population * audience.availability[period];
            let mut viewing = Vec::new();
            for station in &self.world.stations {
                let Some(broadcast) = aired.broadcasts.iter().find(|b| b.station == station.id)
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
                let history = self.selection_history(audience.id, programme)?;
                let loyalty_before = programme
                    .series()
                    .and_then(|s| self.state.loyalty(audience.id, s));
                let utility = attraction.trace.output.powi(2)
                    * history.attachment_factor
                    * history.reception_factor
                    * history.continuity_factor
                    * history.repeat_factor;
                bounded(utility, 0.0, 3.0, "programme selection utility")?;
                viewing.push(Viewing {
                    station: station.id,
                    programme: programme.id,
                    series: programme.series(),
                    kind: programme.kind,
                    fresh_viewers: 0.0,
                    familiarity_after: history.familiarity,
                    history,
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
            for v in &mut viewing {
                v.viewers = available * (v.utility / denominator);
                v.fresh_viewers = v.viewers * (1.0 - v.history.familiarity);
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
        if starts.is_none() {
            self.update_programme_history(&mut next, &mut allocations)?;
            for broadcast in &aired.broadcasts {
                next.broadcasts.push(programmes::BroadcastRecord {
                    at: action.at,
                    station: broadcast.station,
                    programme: broadcast.programme,
                });
            }
        }
        let mut hype = Vec::new();
        for brand in &self.world.brands {
            for (audience, allocation) in self.world.audiences.iter().zip(&allocations) {
                let key = (brand.id, audience.id);
                let before = self.state.hype[&key];
                let repetition_before = self.state.repetition[&key];
                let decayed =
                    before * 2.0_f64.powf(-elapsed_hours / self.parameters.hype_half_life_hours);
                let repetition_decayed = repetition_before
                    * 2.0_f64.powf(-elapsed_hours / self.parameters.repetition_half_life_hours);
                let mut advertisements = Vec::new();
                for v in &allocation.viewing {
                    let broadcast = aired
                        .broadcasts
                        .iter()
                        .find(|b| b.station == v.station)
                        .unwrap();
                    if broadcast.advertisement != Some(brand.id)
                        || starts.is_some_and(|set| !set.contains(&v.station))
                    {
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
                bounded(
                    repetition_after,
                    0.0,
                    f64::from(u32::MAX),
                    "repetition stock",
                )?;
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
        Ok((
            next,
            SlotEvent {
                action: action.clone(),
                aired,
                off_air,
                elapsed_hours,
                starting: self
                    .world
                    .stations
                    .iter()
                    .filter(|s| {
                        action.broadcasts.iter().any(|b| b.station == s.id)
                            && starts.is_none_or(|set| set.contains(&s.id))
                    })
                    .map(|s| s.id)
                    .collect(),
                allocations,
                hype,
                financial: FinancialEvent::default(),
            },
        ))
    }
}
