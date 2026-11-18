//! Bounded duration-aware planning. See docs/programme-planning.md for provisional cadence.
//! No presentation, player observations, filesystem or wall clock.
pub mod film;
pub mod production;
pub mod research;

use crate::economy::{ContractId, Offer};
use crate::*;
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct AiringId(pub u32);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlanError {
    pub code: &'static str,
    pub message: &'static str,
}
impl fmt::Display for PlanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message)
    }
}
impl std::error::Error for PlanError {}
fn error(code: &'static str, message: &'static str) -> PlanError {
    PlanError { code, message }
}
fn engine_error(_: Error) -> PlanError {
    error(
        "simulation_rejected",
        "The simulation could not apply this action.",
    )
}

#[derive(Clone, Debug, PartialEq)]
pub struct AiringResult {
    pub off_air: bool,
    pub cost: Money,
    pub cash_after: Money,
    /// One shared audience snapshot at this airing's start, not cumulative reach.
    pub allocations: Vec<Allocation>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Airing {
    pub id: AiringId,
    pub station: StationId,
    pub programme: ProgrammeId,
    pub start: SimTick,
    pub end: SimTick,
    pub advertiser: Option<BrandId>,
    pub contract: Option<ContractId>,
    pub result: Option<AiringResult>,
    snapshot: Option<AiringResult>,
}
impl Airing {
    pub fn started(&self) -> bool {
        self.snapshot.is_some()
    }
    pub fn cancelled(&self) -> bool {
        self.snapshot.as_ref().is_some_and(|s| s.off_air)
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PlanningOffer {
    pub airing_id: AiringId,
    pub offer: Offer<SimTick>,
    revision: u64,
}
#[derive(Clone, Copy, Debug)]
pub struct CompetitorAiring {
    pub station: StationId,
    pub programme: ProgrammeId,
    pub start: SimTick,
}

/// Explicit station rights and roster assignment. Guest freelancers may remain unassigned.
#[derive(Clone, Debug, PartialEq)]
pub struct StationResources {
    pub station: StationId,
    pub programmes: BTreeSet<ProgrammeId>,
    pub staff: BTreeSet<production::PersonId>,
    pub studios: u16,
}

/// Owns the schedule, active reception samples and the shared numerical/accounting engine.
#[derive(Clone, Debug)]
pub struct Planner {
    engine: Engine<SimTick>,
    player: StationId,
    end: SimTick,
    airings: BTreeMap<AiringId, Airing>,
    next_id: u32,
    revision: u64,
    production: production::ProductionState,
    films: BTreeMap<production::ProductionId, film::Film>,
    research: Option<research::Research>,
    libraries: BTreeMap<StationId, BTreeSet<ProgrammeId>>,
    staff: BTreeMap<StationId, BTreeSet<production::PersonId>>,
    studio_counts: BTreeMap<StationId, u16>,
}
impl Planner {
    pub fn new(
        world: World,
        parameters: Parameters,
        player: StationId,
        end: SimTick,
        competitors: &[CompetitorAiring],
    ) -> Result<Self, PlanError> {
        if end.0 == 0 || end.0 > 7 * 96 {
            return Err(error(
                "invalid_horizon",
                "The planner supports up to seven days.",
            ));
        }
        if !world.stations.iter().any(|s| s.id == player) {
            return Err(error("unknown_station", "The player station must exist."));
        }
        // Legacy callers may intentionally license a shared catalogue. The browser
        // supplies explicit disjoint inventories before play begins.
        let libraries = world
            .stations
            .iter()
            .map(|station| (station.id, world.programmes.iter().map(|p| p.id).collect()))
            .collect();
        let studio_counts = world
            .stations
            .iter()
            .map(|station| (station.id, 1))
            .collect();
        let mut planner = Self {
            engine: Engine::new(world, parameters).map_err(engine_error)?,
            player,
            end,
            airings: BTreeMap::new(),
            next_id: 1,
            revision: 0,
            production: production::ProductionState::default(),
            films: BTreeMap::new(),
            research: None,
            libraries,
            staff: BTreeMap::new(),
            studio_counts,
        };
        for c in competitors {
            if c.station == player
                || !planner
                    .engine
                    .world
                    .stations
                    .iter()
                    .any(|s| s.id == c.station)
            {
                return Err(error(
                    "invalid_competitor",
                    "Competitor schedules must reference another existing station.",
                ));
            }
            planner.insert(c.station, c.programme, c.start, None)?;
        }
        Ok(planner)
    }
    pub fn studio_count(&self, station: StationId) -> u16 {
        self.studio_counts.get(&station).copied().unwrap_or(0)
    }
    pub fn owns_programme(&self, station: StationId, programme: ProgrammeId) -> bool {
        self.libraries
            .get(&station)
            .is_some_and(|library| library.contains(&programme))
    }
    pub fn library(&self, station: StationId) -> impl Iterator<Item = ProgrammeId> + '_ {
        self.libraries
            .get(&station)
            .into_iter()
            .flat_map(|library| library.iter().copied())
    }
    pub fn station_staff(
        &self,
        station: StationId,
    ) -> impl Iterator<Item = production::PersonId> + '_ {
        self.staff
            .get(&station)
            .into_iter()
            .flat_map(|staff| staff.iter().copied())
    }
    pub fn staff_owner(&self, person: production::PersonId) -> Option<StationId> {
        self.staff
            .iter()
            .find_map(|(station, staff)| staff.contains(&person).then_some(*station))
    }
    pub fn configure_station_resources(
        &mut self,
        resources: Vec<StationResources>,
    ) -> Result<(), PlanError> {
        if self.now().0 != 0
            || self.airings.values().any(|a| a.station == self.player)
            || self.production.projects().next().is_some()
            || self.production.requests().any(|r| r.acceptance.is_some())
        {
            return Err(error(
                "station_resources_locked",
                "Set station libraries and staff before play starts.",
            ));
        }
        let mut libraries = BTreeMap::new();
        let mut studios = BTreeMap::new();
        let mut staff = BTreeMap::new();
        let mut assigned = BTreeSet::new();
        for resource in resources {
            if !self
                .world()
                .stations
                .iter()
                .any(|s| s.id == resource.station)
                || libraries.contains_key(&resource.station)
                || resource
                    .programmes
                    .iter()
                    .any(|id| !self.world().programmes.iter().any(|p| p.id == *id))
                || resource.staff.iter().any(|id| {
                    !self.production.roster().any(|p| p.id == *id) || !assigned.insert(*id)
                })
            {
                return Err(error(
                    "invalid_station_resources",
                    "Use existing stations, works and people; staff cannot belong to two stations.",
                ));
            }
            studios.insert(resource.station, resource.studios);
            libraries.insert(resource.station, resource.programmes);
            staff.insert(resource.station, resource.staff);
        }
        if libraries.len() != self.world().stations.len()
            || self.airings.values().any(|a| {
                !libraries
                    .get(&a.station)
                    .is_some_and(|l| l.contains(&a.programme))
            })
        {
            return Err(error(
                "invalid_station_resources",
                "Configure every station and retain rights for all fixed rival broadcasts.",
            ));
        }
        self.studio_counts = studios;
        self.libraries = libraries;
        self.staff = staff;
        Ok(())
    }
    pub fn world(&self) -> &World {
        self.engine.world()
    }
    pub fn state(&self) -> &State<SimTick> {
        self.engine.state()
    }
    pub fn now(&self) -> SimTick {
        self.engine.state.now
    }
    pub fn end(&self) -> SimTick {
        self.end
    }
    pub fn player(&self) -> StationId {
        self.player
    }
    pub fn airings(&self) -> impl Iterator<Item = &Airing> {
        self.airings.values()
    }
    pub fn airing(&self, id: AiringId) -> Result<&Airing, PlanError> {
        self.airings.get(&id).ok_or_else(|| {
            error(
                "unknown_airing",
                "Select an existing airing in this session.",
            )
        })
    }
    /// Reset uses this to avoid reusing removed player-airing identities.
    pub fn retain_id_floor(&mut self, other: &Self) {
        self.next_id = self.next_id.max(other.next_id);
        self.production.retain_ids(&other.production);
    }
    fn bump(&mut self) -> Result<(), PlanError> {
        self.revision = self
            .revision
            .checked_add(1)
            .ok_or_else(|| error("capacity", "Schedule revision capacity exhausted."))?;
        Ok(())
    }
    fn validate_placement(
        &self,
        station: StationId,
        programme: ProgrammeId,
        start: SimTick,
        except: Option<AiringId>,
    ) -> Result<SimTick, PlanError> {
        if self.production.unfinished_programme(programme) {
            return Err(error("production_not_ready", "This original programme is not complete. Advance through post-production before scheduling it."));
        }
        let programme = self
            .engine
            .world
            .programmes
            .iter()
            .find(|p| p.id == programme)
            .ok_or_else(|| {
                error(
                    "unknown_programme",
                    "Choose a programme from the catalogue.",
                )
            })?;
        if !self.owns_programme(station, programme.id) {
            return Err(error(
                "programme_not_owned",
                "This station does not have this programme in its library.",
            ));
        }
        let end = SimTick(
            start
                .0
                .checked_add(u32::from(programme.duration_minutes) / 15)
                .ok_or_else(|| error("invalid_time", "The broadcast time exceeds the scenario."))?,
        );
        if start < self.now() || end > self.end {
            return Err(error(
                "invalid_time",
                "Place the whole programme in unplayed time within the seven-day schedule.",
            ));
        }
        if self
            .airings
            .values()
            .any(|a| a.station == station && Some(a.id) != except && start < a.end && end > a.start)
        {
            return Err(error("overlap", "This broadcast overlaps another programme on your channel. Choose a gap or move the other airing."));
        }
        Ok(end)
    }
    fn validate_advertiser(&self, advertiser: Option<BrandId>) -> Result<(), PlanError> {
        if advertiser.is_some_and(|id| !self.engine.world.brands.iter().any(|b| b.id == id)) {
            return Err(error(
                "unknown_advertiser",
                "Choose a listed advertiser or no advertisement.",
            ));
        }
        Ok(())
    }
    fn insert(
        &mut self,
        station: StationId,
        programme: ProgrammeId,
        start: SimTick,
        advertiser: Option<BrandId>,
    ) -> Result<AiringId, PlanError> {
        let end = self.validate_placement(station, programme, start, None)?;
        self.validate_advertiser(advertiser)?;
        let id = AiringId(self.next_id);
        let next_id = self
            .next_id
            .checked_add(1)
            .ok_or_else(|| error("capacity", "Airing identifier capacity exhausted."))?;
        self.bump()?;
        self.airings.insert(
            id,
            Airing {
                id,
                station,
                programme,
                start,
                end,
                advertiser,
                contract: None,
                result: None,
                snapshot: None,
            },
        );
        self.next_id = next_id;
        Ok(id)
    }
    pub fn place(
        &mut self,
        programme: ProgrammeId,
        start: SimTick,
        advertiser: Option<BrandId>,
    ) -> Result<AiringId, PlanError> {
        self.insert(self.player, programme, start, advertiser)
    }
    fn editable(&self, id: AiringId) -> Result<&Airing, PlanError> {
        let airing = self.airing(id)?;
        if airing.station != self.player {
            return Err(error(
                "competitor_read_only",
                "The competitor schedule is fixed for this experiment.",
            ));
        }
        if airing.started() || airing.start < self.now() {
            return Err(error(
                "already_started",
                "An airing that has started or finished cannot be edited.",
            ));
        }
        if airing.contract.is_some() {
            return Err(error("booking_locked", "An accepted commitment locks this airing's programme, start, duration and advertiser."));
        }
        Ok(airing)
    }
    pub fn move_airing(&mut self, id: AiringId, start: SimTick) -> Result<(), PlanError> {
        let airing = self.editable(id)?;
        let end = self.validate_placement(self.player, airing.programme, start, Some(id))?;
        self.bump()?;
        let airing = self.airings.get_mut(&id).unwrap();
        airing.start = start;
        airing.end = end;
        Ok(())
    }
    pub fn remove(&mut self, id: AiringId) -> Result<(), PlanError> {
        self.editable(id)?;
        self.bump()?;
        self.airings.remove(&id);
        Ok(())
    }
    pub fn set_advertiser(
        &mut self,
        id: AiringId,
        advertiser: Option<BrandId>,
    ) -> Result<(), PlanError> {
        self.editable(id)?;
        self.validate_advertiser(advertiser)?;
        self.bump()?;
        self.airings.get_mut(&id).unwrap().advertiser = advertiser;
        Ok(())
    }
    fn interval_action(
        &self,
        start: SimTick,
        quoted: Option<(AiringId, BrandId)>,
    ) -> (Slot<SimTick>, BTreeSet<StationId>) {
        let active = self
            .airings
            .values()
            .filter(|a| a.start <= start && start < a.end && !a.cancelled())
            .collect::<Vec<_>>();
        let starts = active
            .iter()
            .filter(|a| a.start == start)
            .map(|a| a.station)
            .collect();
        let broadcasts = active
            .iter()
            .map(|a| Broadcast {
                station: a.station,
                programme: a.programme,
                advertisement: if a.start == start {
                    quoted
                        .filter(|(id, _)| *id == a.id)
                        .map(|(_, brand)| brand)
                        .or_else(|| a.contract.and(a.advertiser))
                } else {
                    None
                },
            })
            .collect();
        (
            Slot {
                at: SimTick(start.0 + 1),
                broadcasts,
            },
            starts,
        )
    }
    pub fn quote(&self, id: AiringId) -> Result<PlanningOffer, PlanError> {
        let airing = self.editable(id)?;
        let brand = airing.advertiser.ok_or_else(|| {
            error(
                "no_advertiser",
                "Select an advertiser for this airing before requesting an offer.",
            )
        })?;
        let (proposal, starts) = self.interval_action(airing.start, Some((id, brand)));
        let forecast = self.forecast_engine();
        let offer = forecast
            .as_ref()
            .unwrap_or(&self.engine)
            .quote_cadence(
                &proposal,
                self.player,
                brand,
                airing.end,
                Some((&starts, airing.start.availability_index())),
            )
            .map_err(engine_error)?;
        if offer.payment.0 == 0 {
            return Err(error(
                "no_payable_offer",
                "No payable offer for this airing. Try a different programme, time or advertiser.",
            ));
        }
        Ok(PlanningOffer {
            airing_id: id,
            offer,
            revision: self.revision,
        })
    }
    pub fn accept(&mut self, quoted: &PlanningOffer) -> Result<ContractId, PlanError> {
        if &self.quote(quoted.airing_id)? != quoted {
            return Err(error(
                "stale_offer",
                "The schedule or simulation changed. Request a fresh offer for this airing.",
            ));
        }
        // quote() above revalidates the full typed proposal and price inside Rust.
        let mut candidate = self.clone();
        let contract = candidate
            .engine
            .reserve_verified_offer(&quoted.offer)
            .map_err(engine_error)?;
        candidate
            .airings
            .get_mut(&quoted.airing_id)
            .unwrap()
            .contract = Some(contract);
        candidate.bump()?;
        *self = candidate;
        Ok(contract)
    }
    /// Atomic advancement through *every* intervening interval. Returned evidence is developer-only.
    pub fn advance_to(&mut self, target: SimTick) -> Result<Vec<SlotEvent<SimTick>>, PlanError> {
        if target <= self.now() || target > self.end {
            return Err(error(
                "invalid_advance",
                "Advance to a later tick within the seven-day scenario.",
            ));
        }
        let mut candidate = self.clone();
        let mut events = Vec::new();
        while candidate.now() < target {
            events.push(candidate.tick()?);
        }
        candidate.bump()?;
        *self = candidate;
        Ok(events)
    }
    fn tick(&mut self) -> Result<SlotEvent<SimTick>, PlanError> {
        let start = self.now();
        self.start_recordings(start)?;
        let (action, starts) = self.interval_action(start, None);
        let (mut next, mut event) = self
            .engine
            .evaluate_cadence(&action, Some(&starts), start.availability_index())
            .map_err(engine_error)?;
        event.financial = economy::apply_slot(&self.engine.world, &mut next.economy, &event)
            .map_err(engine_error)?;
        next.economy.advance_revision().map_err(engine_error)?;
        self.engine.state = next;
        for airing in self.airings.values_mut().filter(|a| a.start == start) {
            let off_air = !event
                .aired
                .broadcasts
                .iter()
                .any(|b| b.station == airing.station);
            let cost = if off_air {
                Money(0)
            } else {
                self.engine
                    .world
                    .programmes
                    .iter()
                    .find(|p| p.id == airing.programme)
                    .unwrap()
                    .broadcast_cost
            };
            airing.snapshot = Some(AiringResult {
                off_air,
                cost,
                cash_after: self.engine.state.economy.cash(airing.station).unwrap(),
                allocations: event.allocations.clone(),
            });
        }
        // All completions share a consistent pre-completion state and update reception once.
        let ending = self
            .airings
            .values()
            .filter(|a| a.end == action.at)
            .cloned()
            .collect::<Vec<_>>();
        let mut completion = event.allocations.clone();
        for allocation in &mut completion {
            allocation.viewing.clear();
            for airing in &ending {
                if let Some(snapshot) = &airing.snapshot {
                    if let Some(viewing) = snapshot
                        .allocations
                        .iter()
                        .find(|a| a.audience == allocation.audience)
                        .and_then(|a| a.viewing.iter().find(|v| v.station == airing.station))
                    {
                        let mut viewing = viewing.clone();
                        let familiar =
                            self.engine.state.familiarity[&(allocation.audience, airing.programme)];
                        viewing.fresh_viewers = viewing.viewers * (1.0 - familiar);
                        allocation.viewing.push(viewing);
                    }
                }
            }
        }
        // Start samples can describe different instants. Conservatively cap their
        // combined learning mass at one group, rather than claiming unique reach.
        for allocation in &mut completion {
            let total = allocation.viewing.iter().map(|v| v.viewers).sum::<f64>();
            let scale = if total > f64::from(allocation.population) {
                f64::from(allocation.population) / total
            } else {
                1.0
            };
            for viewing in &mut allocation.viewing {
                viewing.viewers *= scale;
                viewing.fresh_viewers *= scale;
            }
        }
        let mut completed_state = self.engine.state.clone();
        self.engine
            .update_programme_history(&mut completed_state, &mut completion)
            .map_err(engine_error)?;
        for airing in ending {
            if !airing.cancelled() {
                completed_state
                    .broadcasts
                    .push(programmes::BroadcastRecord {
                        at: action.at,
                        station: airing.station,
                        programme: airing.programme,
                    });
            }
            let mut result = airing.snapshot;
            if let Some(r) = &mut result {
                r.cash_after = self.engine.state.economy.cash(airing.station).unwrap();
            }
            self.airings.get_mut(&airing.id).unwrap().result = result;
        }
        self.engine.state = completed_state;
        self.finish_productions(action.at)?;
        self.advance_films(action.at)?;
        self.poll_boundary(action.at)?;
        Ok(event)
    }
}
