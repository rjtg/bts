//! Versioned browser commands and restricted player observations over the Rust planner.
#![forbid(unsafe_code)]
pub mod protocol;
pub mod scenario;

use bts_sim::economy::{Account, AudienceCommitment, Contract, ContractStatus, LedgerReason};
use bts_sim::planning::{AiringId, PlanError, Planner, PlanningOffer};
use bts_sim::{BrandId, ProgrammeId, SimTick};
use protocol::*;
use scenario::PLAYER;
use std::sync::atomic::{AtomicU32, Ordering};
static NEXT_SESSION: AtomicU32 = AtomicU32::new(1);

#[derive(Clone)]
struct Pending {
    token: String,
    quote: PlanningOffer,
    targets: Vec<Commitment>,
}
/// Each game session exclusively owns its simulation. Rejections are atomic.
#[derive(Clone)]
pub struct Session {
    planner: Planner,
    namespace: u32,
    next_offer: u64,
    pending: Option<Pending>,
}
impl Session {
    pub fn new() -> Result<Self, PlayerError> {
        let namespace = NEXT_SESSION
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| v.checked_add(1))
            .map_err(|_| {
                PlayerError::new("session_limit", "Session identifier capacity exhausted.")
            })?;
        Ok(Self {
            planner: scenario::planner().map_err(player_error)?,
            namespace,
            next_offer: 1,
            pending: None,
        })
    }
    pub fn execute(&mut self, command: Command) -> Result<PlayerReport, PlayerError> {
        let mut candidate = self.clone();
        candidate.apply(command)?;
        let report = candidate.report();
        serde_json::to_string(&report).map_err(|_| {
            PlayerError::new("report_error", "Could not construct a player report.")
        })?;
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
                "This adapter supports api_version 2.",
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
        serde_json::to_string(&response).expect("player DTOs are serializable")
    }
    fn apply(&mut self, command: Command) -> Result<(), PlayerError> {
        match command {
            Command::Inspect {} => return Ok(()),
            Command::Reset {} => {
                let mut fresh = scenario::planner().map_err(player_error)?;
                fresh.retain_id_floor(&self.planner);
                self.planner = fresh;
            }
            Command::Place {
                start_tick,
                programme,
                advertiser,
            } => {
                self.planner
                    .place(
                        ProgrammeId(programme),
                        SimTick(start_tick),
                        advertiser.map(BrandId),
                    )
                    .map_err(player_error)?;
            }
            Command::Move {
                airing_id,
                start_tick,
            } => self
                .planner
                .move_airing(AiringId(airing_id), SimTick(start_tick))
                .map_err(player_error)?,
            Command::Remove { airing_id } => self
                .planner
                .remove(AiringId(airing_id))
                .map_err(player_error)?,
            Command::SetAdvertiser {
                airing_id,
                advertiser,
            } => self
                .planner
                .set_advertiser(AiringId(airing_id), advertiser.map(BrandId))
                .map_err(player_error)?,
            Command::RequestOffer { airing_id } => {
                let quote = self
                    .planner
                    .quote(AiringId(airing_id))
                    .map_err(player_error)?;
                let mut preview = self.planner.clone();
                let contract = preview.accept(&quote).map_err(player_error)?;
                let targets = self.commitments(
                    &preview
                        .state()
                        .economy()
                        .contract(contract)
                        .expect("accepted contract")
                        .commitments,
                );
                let token = format!("{}:{}", self.namespace, self.next_offer);
                self.next_offer = self
                    .next_offer
                    .checked_add(1)
                    .ok_or_else(|| PlayerError::new("capacity", "Offer capacity exhausted."))?;
                self.pending = Some(Pending {
                    token,
                    quote,
                    targets,
                });
                return Ok(());
            }
            Command::AcceptOffer {
                airing_id,
                offer_id,
            } => {
                let pending = self
                    .pending
                    .as_ref()
                    .filter(|p| p.token == offer_id && p.quote.airing_id.0 == airing_id)
                    .ok_or_else(|| {
                        PlayerError::new(
                            "stale_offer",
                            "Request a fresh offer for this exact airing and session.",
                        )
                    })?;
                self.planner.accept(&pending.quote).map_err(player_error)?;
            }
            Command::Advance { to_tick } => {
                self.planner
                    .advance_to(SimTick(to_tick))
                    .map_err(player_error)?;
            }
        }
        self.pending = None;
        Ok(())
    }
    fn audience_name(&self, id: bts_sim::AudienceId) -> String {
        self.planner
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
    fn booking(&self, contract: &Contract<SimTick>) -> Booking {
        let airing = self
            .planner
            .airings()
            .find(|a| a.contract == Some(contract.id))
            .expect("airing contract");
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
            airing_id: airing.id.0,
            contract_id: contract.id.0.to_string(),
            advertiser: contract.offer.brand.0,
            deadline_tick: contract.offer.deadline.0,
            maximum_payment: contract.offer.payment.0.to_string(),
            targets: self.commitments(&contract.commitments),
            settlement,
        }
    }
    pub fn report(&self) -> PlayerReport {
        let world = self.planner.world();
        let now = self.planner.now().0;
        let mut schedule = self
            .planner
            .airings()
            .map(|a| ScheduleEntry {
                airing_id: a.id.0,
                station: a.station.0,
                programme: a.programme.0,
                start_tick: a.start.0,
                end_tick: a.end.0,
                proposed_advertiser: a.advertiser.map(|id| id.0),
                booked: a.contract.is_some(),
                status: if a.cancelled() {
                    "off_air"
                } else if a.result.is_some() {
                    "completed"
                } else if a.started() {
                    "on_air"
                } else {
                    "planned"
                },
                editable: a.station == PLAYER && !a.started() && a.contract.is_none(),
            })
            .collect::<Vec<_>>();
        schedule.sort_by_key(|a| (a.station, a.start_tick, a.airing_id));
        let mut player = schedule
            .iter()
            .filter(|a| a.station == PLAYER.0)
            .collect::<Vec<_>>();
        player.sort_by_key(|a| (a.start_tick, a.airing_id));
        let next_airing = player
            .iter()
            .find(|a| a.start_tick >= now)
            .map(|a| a.airing_id);
        let active_airing = player
            .iter()
            .find(|a| a.start_tick < now && now < a.end_tick)
            .map(|a| a.airing_id);
        let mut gaps = vec![];
        let mut cursor = 0;
        for a in &player {
            if a.start_tick > cursor {
                gaps.push(Gap {
                    start_tick: cursor,
                    end_tick: a.start_tick,
                });
            }
            cursor = a.end_tick;
        }
        if cursor < self.planner.end().0 {
            gaps.push(Gap {
                start_tick: cursor,
                end_tick: self.planner.end().0,
            });
        }
        let economy = self.planner.state().economy();
        let ledger = economy
            .ledger()
            .iter()
            .filter(|l| l.from == Account::Station(PLAYER) || l.to == Account::Station(PLAYER))
            .map(|l| {
                let airing = match l.reason {
                    LedgerReason::BroadcastCost => self
                        .planner
                        .airings()
                        .find(|a| a.station == PLAYER && a.start.0 + 1 == l.at.0),
                    _ => match l.from {
                        Account::Escrow(id) => {
                            self.planner.airings().find(|a| a.contract == Some(id))
                        }
                        _ => None,
                    },
                };
                FinancialLine {
                    tick: l.at.0,
                    airing_id: airing.map(|a| a.id.0),
                    kind: if l.reason == LedgerReason::BroadcastCost {
                        "broadcast_cost"
                    } else {
                        "advertising_payment"
                    },
                    amount: l.amount.0.to_string(),
                }
            })
            .collect::<Vec<_>>();
        let costs_paid = economy
            .ledger()
            .iter()
            .filter(|l| l.from == Account::Station(PLAYER))
            .map(|l| u128::from(l.amount.0))
            .sum::<u128>()
            .to_string();
        let revenue = economy
            .ledger()
            .iter()
            .filter(|l| l.to == Account::Station(PLAYER))
            .map(|l| u128::from(l.amount.0))
            .sum::<u128>()
            .to_string();
        let mut results = self
            .planner
            .airings()
            .filter(|a| a.station == PLAYER)
            .filter_map(|a| {
                a.result.as_ref().map(|r| BroadcastReport {
                    airing_id: a.id.0,
                    start_tick: a.start.0,
                    end_tick: a.end.0,
                    snapshot_tick: a.start.0,
                    aired_programme: (!r.off_air).then_some(a.programme.0),
                    aired_advertiser: if r.off_air || a.contract.is_none() {
                        None
                    } else {
                        a.advertiser.map(|b| b.0)
                    },
                    off_air: r.off_air,
                    broadcast_cost: r.cost.0.to_string(),
                    cash_after: r.cash_after.0.to_string(),
                    audiences: r
                        .allocations
                        .iter()
                        .map(|allocation| AudienceReport {
                            audience: self.audience_name(allocation.audience),
                            population: allocation.population,
                            player_viewers: allocation
                                .viewing
                                .iter()
                                .filter(|v| v.station == PLAYER)
                                .map(|v| v.viewers)
                                .sum(),
                            rival_viewers: allocation
                                .viewing
                                .iter()
                                .filter(|v| v.station != PLAYER)
                                .map(|v| v.viewers)
                                .sum(),
                            not_watching: allocation.not_watching,
                        })
                        .collect(),
                })
            })
            .collect::<Vec<_>>();
        results.sort_by_key(|r| (r.end_tick, r.airing_id));
        PlayerReport {
            reporting: REPORTING,
            now_tick: now,
            end_tick: self.planner.end().0,
            tick_minutes: 15,
            next_airing,
            active_airing,
            cash: economy.cash(PLAYER).unwrap().0.to_string(),
            finances: Finances {
                opening_cash: world
                    .stations
                    .iter()
                    .find(|s| s.id == PLAYER)
                    .unwrap()
                    .initial_cash
                    .0
                    .to_string(),
                costs_paid,
                revenue,
                ledger,
            },
            stations: world
                .stations
                .iter()
                .map(|s| StationInfo {
                    id: s.id.0,
                    name: s.name.clone(),
                    is_player: s.id == PLAYER,
                })
                .collect(),
            gaps,
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
                            .unwrap()
                            .name
                            .clone()
                    }),
                    episode: p.sequence(),
                    tone: p.tone,
                    awareness: p.awareness,
                    cast_appeal: p.cast_appeal,
                    broadcast_cost: p.broadcast_cost.0.to_string(),
                    duration_minutes: p.duration_minutes,
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
            schedule,
            offer: self.pending.as_ref().map(|p| {
                let airing = self.planner.airing(p.quote.airing_id).unwrap();
                Quote {
                    offer_id: p.token.clone(),
                    airing_id: airing.id.0,
                    start_tick: airing.start.0,
                    end_tick: airing.end.0,
                    advertiser: p.quote.offer.brand.0,
                    deadline_tick: p.quote.offer.deadline.0,
                    maximum_payment: p.quote.offer.payment.0.to_string(),
                    targets: p.targets.clone(),
                }
            }),
            contracts: economy.contracts().map(|c| self.booking(c)).collect(),
            results,
        }
    }
}
fn player_error(error: PlanError) -> PlayerError {
    PlayerError::new(error.code, error.message)
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
