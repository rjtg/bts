//! Browser session adapter. Rust owns decisions, accounting and the observation boundary.
//! JSON protocol v1 is independent of bts-sim's developer export schema.
#![forbid(unsafe_code)]

pub mod protocol;
pub mod scenario;

use bts_sim::economy::{AudienceCommitment, Contract, ContractId, ContractStatus, Offer};
use bts_sim::{BrandId, Broadcast, Parameters, ProgrammeId, Simulation, Slot};
use protocol::*;
use scenario::{PLAYER, RIVAL, RIVAL_PROGRAMME, SLOT_COUNT};
use std::sync::atomic::{AtomicU32, Ordering};

// Namespaces prevent accidental cross-session token reuse. They are not security credentials.
static NEXT_SESSION: AtomicU32 = AtomicU32::new(1);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Plan {
    programme: ProgrammeId,
    advertiser: Option<BrandId>,
}
#[derive(Clone)]
struct Pending {
    token: String,
    offer: Offer,
    targets: Vec<Commitment>,
}

/// One simulation per independently owned session. No DOM, I/O, clock or RNG.
#[derive(Clone)]
pub struct Session {
    simulation: Simulation,
    namespace: u32,
    next_offer: u64,
    cursor: usize,
    plans: Vec<Option<Plan>>,
    bookings: Vec<Option<ContractId>>,
    pending: Option<Pending>,
    results: Vec<BroadcastReport>,
}

impl Session {
    pub fn new() -> Result<Self, PlayerError> {
        let namespace = NEXT_SESSION
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| v.checked_add(1))
            .map_err(|_| {
                PlayerError::new("session_limit", "Session identifier capacity exhausted.")
            })?;
        Ok(Self {
            simulation: Self::initial_simulation()?,
            namespace,
            next_offer: 1,
            cursor: 0,
            plans: vec![None; SLOT_COUNT],
            bookings: vec![None; SLOT_COUNT],
            pending: None,
            results: vec![],
        })
    }
    fn initial_simulation() -> Result<Simulation, PlayerError> {
        Simulation::new(scenario::world(), Parameters::default()).map_err(|_| engine_error())
    }
    /// Rejected commands leave the entire session unchanged, including pending offers.
    pub fn execute(&mut self, command: Command) -> Result<PlayerReport, PlayerError> {
        let mut candidate = self.clone();
        candidate.apply(command)?;
        let report = candidate.report();
        // Build the serializable observation before committing the transition.
        serde_json::to_string(&report).map_err(|_| engine_error())?;
        *self = candidate;
        Ok(report)
    }
    pub fn dispatch_json(&mut self, json: &str) -> String {
        let outcome = match serde_json::from_str::<Request>(json) {
            Err(_) => Err(PlayerError::new(
                "malformed_command",
                "Expected a versioned command with valid fields and identifier types.",
            )),
            Ok(request) if request.api_version != API_VERSION => Err(PlayerError::new(
                "unsupported_version",
                "This adapter supports api_version 1.",
            )),
            Ok(request) => self.execute(request.command),
        };
        let response = match outcome {
            Ok(report) => Response::Ok {
                api_version: API_VERSION,
                report: Box::new(report),
            },
            Err(error) => Response::Error {
                api_version: API_VERSION,
                error,
            },
        };
        serde_json::to_string(&response).expect("player DTOs contain only serializable values")
    }
    fn apply(&mut self, command: Command) -> Result<(), PlayerError> {
        match command {
            Command::Inspect {} => (),
            Command::Reset {} => {
                self.simulation = Self::initial_simulation()?;
                self.cursor = 0;
                self.plans.fill(None);
                self.bookings.fill(None);
                self.pending = None;
                self.results.clear();
                // Keep the token counter: reset must not make an old offer usable again.
            }
            Command::Plan {
                slot,
                programme,
                advertiser,
            } => {
                let slot = slot as usize;
                if slot >= SLOT_COUNT || slot < self.cursor {
                    return Err(PlayerError::new(
                        "invalid_slot",
                        "Choose an unplayed slot within the seven-day scenario.",
                    ));
                }
                if self.bookings[slot].is_some() {
                    return Err(PlayerError::new(
                        "booking_locked",
                        "An accepted booking locks this programme and advertiser until broadcast.",
                    ));
                }
                if !self
                    .simulation
                    .world()
                    .programmes
                    .iter()
                    .any(|p| p.id.0 == programme)
                {
                    return Err(PlayerError::new(
                        "unknown_programme",
                        "Choose a programme from the catalogue.",
                    ));
                }
                if advertiser
                    .is_some_and(|id| !self.simulation.world().brands.iter().any(|b| b.id.0 == id))
                {
                    return Err(PlayerError::new(
                        "unknown_advertiser",
                        "Choose a listed advertiser or no advertisement.",
                    ));
                }
                self.plans[slot] = Some(Plan {
                    programme: ProgrammeId(programme),
                    advertiser: advertiser.map(BrandId),
                });
                self.pending = None;
            }
            Command::RequestOffer {} => {
                self.require_current()?;
                if self.bookings[self.cursor].is_some() {
                    return Err(PlayerError::new(
                        "already_booked",
                        "This broadcast already has an accepted advertising contract.",
                    ));
                }
                let plan = self.plan()?;
                let brand = plan.advertiser.ok_or_else(|| {
                    PlayerError::new(
                        "no_advertiser",
                        "Plan a broadcast with an advertiser before requesting an offer.",
                    )
                })?;
                let slot = self.slot(true)?;
                let offer = self
                    .simulation
                    .quote(&slot, PLAYER, brand, slot.at)
                    .map_err(|_| engine_error())?;
                if offer.payment.0 == 0 {
                    return Err(PlayerError::new("no_payable_offer", "This advertiser has no payable offer for the planned broadcast. Try another programme or advertiser."));
                }
                // Preview acceptance on an isolated copy to use exactly the engine's rounding/terms.
                let mut preview = self.simulation.clone();
                let id = preview.accept_offer(&offer).map_err(|_| engine_error())?;
                let contract = preview
                    .state()
                    .economy()
                    .contract(id)
                    .ok_or_else(engine_error)?;
                let targets = self.commitments(&contract.commitments);
                let token = format!("{}:{}", self.namespace, self.next_offer);
                self.next_offer = self.next_offer.checked_add(1).ok_or_else(engine_error)?;
                self.pending = Some(Pending {
                    token,
                    offer,
                    targets,
                });
            }
            Command::AcceptOffer { offer_id } => {
                let pending = self
                    .pending
                    .clone()
                    .filter(|p| p.token == offer_id)
                    .ok_or_else(|| {
                        PlayerError::new(
                            "stale_offer",
                            "Request a fresh offer for this session and current plan.",
                        )
                    })?;
                let id = self.simulation.accept_offer(&pending.offer).map_err(|_| {
                    PlayerError::new(
                        "stale_offer",
                        "The offer is no longer valid. Request a fresh offer.",
                    )
                })?;
                self.bookings[self.cursor] = Some(id);
                self.pending = None;
            }
            Command::Advance {} => {
                let action =
                    self.slot(self.bookings.get(self.cursor).is_some_and(Option::is_some))?;
                let event = self.simulation.step(&action).map_err(|_| engine_error())?;
                let aired = event.aired.broadcasts.iter().find(|b| b.station == PLAYER);
                let audiences = event
                    .allocations
                    .iter()
                    .map(|a| AudienceReport {
                        audience: self.audience_name(a.audience),
                        population: a.population,
                        player_viewers: a
                            .viewing
                            .iter()
                            .filter(|v| v.station == PLAYER)
                            .map(|v| v.viewers)
                            .sum(),
                        rival_viewers: a
                            .viewing
                            .iter()
                            .filter(|v| v.station == RIVAL)
                            .map(|v| v.viewers)
                            .sum(),
                        not_watching: a.not_watching,
                    })
                    .collect();
                let cost = aired
                    .and_then(|b| {
                        self.simulation
                            .world()
                            .programmes
                            .iter()
                            .find(|p| p.id == b.programme)
                    })
                    .map_or(0, |p| p.broadcast_cost.0);
                self.results.push(BroadcastReport {
                    slot: self.cursor,
                    at_hour: action.at.0,
                    aired_programme: aired.map(|b| b.programme.0),
                    aired_advertiser: aired.and_then(|b| b.advertisement.map(|a| a.0)),
                    off_air: aired.is_none(),
                    broadcast_cost: cost.to_string(),
                    cash_after: self.cash(),
                    audiences,
                });
                self.cursor += 1;
                self.pending = None;
            }
        }
        Ok(())
    }
    fn require_current(&self) -> Result<(), PlayerError> {
        if self.cursor >= SLOT_COUNT {
            Err(PlayerError::new(
                "scenario_complete",
                "All fourteen slots are complete. Reset to start again.",
            ))
        } else {
            Ok(())
        }
    }
    fn plan(&self) -> Result<&Plan, PlayerError> {
        self.require_current()?;
        self.plans[self.cursor].as_ref().ok_or_else(|| {
            PlayerError::new(
                "plan_required",
                "Choose and plan a programme for the next slot.",
            )
        })
    }
    fn slot(&self, with_ad: bool) -> Result<Slot, PlayerError> {
        let plan = self.plan()?;
        Ok(Slot {
            at: scenario::hour(self.cursor),
            broadcasts: vec![
                Broadcast {
                    station: PLAYER,
                    programme: plan.programme,
                    advertisement: if with_ad { plan.advertiser } else { None },
                },
                Broadcast {
                    station: RIVAL,
                    programme: RIVAL_PROGRAMME,
                    advertisement: None,
                },
            ],
        })
    }
    fn cash(&self) -> String {
        self.simulation
            .state()
            .economy()
            .cash(PLAYER)
            .expect("fixed player")
            .0
            .to_string()
    }
    fn audience_name(&self, id: bts_sim::AudienceId) -> String {
        self.simulation
            .world()
            .audiences
            .iter()
            .find(|a| a.id == id)
            .expect("validated audience")
            .name
            .clone()
    }
    fn commitments(&self, rows: &[AudienceCommitment]) -> Vec<Commitment> {
        rows.iter()
            .map(|r| Commitment {
                audience: self.audience_name(r.audience),
                forecast_relevant_delivery: r.forecast_delivery,
                promised_units: r.promised_units.to_string(),
                allocated_payment: r.allocated_payment.0.to_string(),
            })
            .collect()
    }
    fn booking(&self, contract: &Contract) -> Booking {
        let settlement = match &contract.status {
            ContractStatus::Reserved => None,
            ContractStatus::Settled(s) => Some(ContractResult {
                payment: s.payment.0.to_string(),
                refund: s.refund.0.to_string(),
                targets: s
                    .audiences
                    .iter()
                    .map(|r| TargetResult {
                        audience: self.audience_name(r.audience),
                        promised_units: r.promised_units.to_string(),
                        credited_units: r.credited_units.to_string(),
                        allocated_payment: r.allocated_payment.0.to_string(),
                        payment: r.payment.0.to_string(),
                        refund: r.refund.0.to_string(),
                    })
                    .collect(),
            }),
        };
        Booking {
            contract_id: contract.id.0.to_string(),
            advertiser: contract.offer.brand.0,
            deadline_hour: contract.offer.deadline.0,
            maximum_payment: contract.offer.payment.0.to_string(),
            targets: self.commitments(&contract.commitments),
            settlement,
        }
    }
    /// This whitelist is the only normal observation path. No developer inspector is exported.
    pub fn report(&self) -> PlayerReport {
        let world = self.simulation.world();
        PlayerReport {
            reporting: REPORTING,
            next_slot: (self.cursor < SLOT_COUNT).then_some(self.cursor),
            cash: self.cash(),
            catalogue: world
                .programmes
                .iter()
                .map(|p| CatalogueEntry {
                    id: p.id.0,
                    name: p.name.clone(),
                    genre: format!("{:?}", p.genre),
                    kind: if p.series().is_some() {
                        "episode"
                    } else {
                        "standalone"
                    }
                    .into(),
                    series_name: p.series().map(|id| {
                        world
                            .series
                            .iter()
                            .find(|s| s.id == id)
                            .expect("validated series")
                            .name
                            .clone()
                    }),
                    episode: p.sequence(),
                    tone: p.tone,
                    awareness: p.awareness,
                    cast_appeal: p.cast_appeal,
                    broadcast_cost: p.broadcast_cost.0.to_string(),
                })
                .collect(),
            advertisers: world
                .brands
                .iter()
                .map(|b| Advertiser {
                    id: b.id.0,
                    name: b.name.clone(),
                })
                .collect(),
            schedule: (0..SLOT_COUNT)
                .map(|i| ScheduleEntry {
                    slot: i,
                    day: i / 2 + 1,
                    hour: scenario::hour(i).0 % 24,
                    programme: self.plans[i].as_ref().map(|p| p.programme.0),
                    proposed_advertiser: self.plans[i]
                        .as_ref()
                        .and_then(|p| p.advertiser.map(|a| a.0)),
                    rival_programme: RIVAL_PROGRAMME.0,
                    locked: self.bookings[i].is_some(),
                    completed: i < self.cursor,
                })
                .collect(),
            offer: self.pending.as_ref().map(|p| Quote {
                offer_id: p.token.clone(),
                advertiser: p.offer.brand.0,
                slot: self.cursor,
                deadline_hour: p.offer.deadline.0,
                maximum_payment: p.offer.payment.0.to_string(),
                targets: p.targets.clone(),
            }),
            contracts: self
                .simulation
                .state()
                .economy()
                .contracts()
                .map(|c| self.booking(c))
                .collect(),
            results: self.results.clone(),
        }
    }
}
fn engine_error() -> PlayerError {
    PlayerError::new(
        "simulation_rejected",
        "The simulation could not apply this action. Reset the scenario if the problem persists.",
    )
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct WebSession {
        session: Session,
    }
    #[wasm_bindgen]
    impl WebSession {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Result<WebSession, JsValue> {
            Session::new()
                .map(|session| Self { session })
                .map_err(|e| JsValue::from_str(e.message))
        }
        pub fn dispatch(&mut self, request: &str) -> String {
            self.session.dispatch_json(request)
        }
    }
    #[wasm_bindgen]
    pub fn api_version() -> u32 {
        API_VERSION
    }
}

#[cfg(test)]
mod tests;
