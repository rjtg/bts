//! Versioned browser commands and restricted player observations over the Rust planner.
#![forbid(unsafe_code)]
pub mod protocol;
pub mod scenario;

use bts_sim::economy::{Account, AudienceCommitment, Contract, ContractStatus, LedgerReason};
use bts_sim::planning::production::{
    Concept, PersonId, ProductionId, RequestId, Role, SwayReason, PARTICIPATION_COST,
};
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
    booking_review: Option<ProductionBookingReview>,
}
impl Session {
    pub fn new() -> Result<Self, PlayerError> {
        // Checked allocation without the fetch_update API deprecated by newer Rust.
        let mut namespace = NEXT_SESSION.load(Ordering::Relaxed);
        loop {
            let next = namespace.checked_add(1).ok_or_else(|| {
                PlayerError::new("session_limit", "Session identifier capacity exhausted.")
            })?;
            match NEXT_SESSION.compare_exchange_weak(
                namespace,
                next,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(observed) => namespace = observed,
            }
        }
        Ok(Self {
            planner: scenario::planner().map_err(player_error)?,
            namespace,
            next_offer: 1,
            pending: None,
            booking_review: None,
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
                "This adapter supports api_version 5.",
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
            Command::SetPolling { audience, polling } => self
                .planner
                .set_polling(bts_sim::AudienceId(audience), polling)
                .map_err(player_error)?,
            Command::SpendInsight { audience, amount } => self
                .planner
                .spend_insight(bts_sim::AudienceId(audience), amount)
                .map_err(player_error)?,
            Command::Inspect {} => return Ok(()),
            Command::AcceptRequest { request_id } => self
                .planner
                .accept_request(RequestId(request_id))
                .map_err(player_error)?,
            Command::CreateProduction { concept } => {
                self.planner
                    .create_production(concept_input(concept)?)
                    .map_err(player_error)?;
            }
            Command::ReviseProduction {
                production_id,
                concept,
            } => self
                .planner
                .revise_production(ProductionId(production_id), concept_input(concept)?)
                .map_err(player_error)?,
            Command::DiscardProduction { production_id } => self
                .planner
                .discard_production(ProductionId(production_id))
                .map_err(player_error)?,
            Command::ReviewProduction {
                production_id,
                exceptional_person,
                cooperation_sway,
            } => {
                let mut preview = self.planner.clone();
                preview
                    .book_production(
                        ProductionId(production_id),
                        exceptional_person.map(PersonId),
                        cooperation_sway,
                    )
                    .map_err(player_error)?;
                self.booking_review = Some(ProductionBookingReview {
                    production_id,
                    exceptional_person,
                    cooperation_sway,
                    studio: preview
                        .production()
                        .project(ProductionId(production_id))
                        .map_err(player_error)?
                        .studio
                        .unwrap()
                        .0,
                    total_sway: self.planner.production().sway() - preview.production().sway(),
                    sway_remaining: preview.production().sway(),
                    budget: preview
                        .production()
                        .project(ProductionId(production_id))
                        .map_err(player_error)?
                        .concept
                        .budget
                        .0
                        .to_string(),
                    cash_remaining: preview
                        .state()
                        .economy()
                        .cash(PLAYER)
                        .unwrap()
                        .0
                        .to_string(),
                });
                return Ok(());
            }
            Command::BookProduction {
                production_id,
                exceptional_person,
                cooperation_sway,
            } => self
                .planner
                .book_production(
                    ProductionId(production_id),
                    exceptional_person.map(PersonId),
                    cooperation_sway,
                )
                .map_err(player_error)?,
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
        self.booking_review = None;
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
    fn production_view(&self) -> ProductionView {
        let state = self.planner.production();
        ProductionView {
            format: "standalone_interview_discussion",
            duration_minutes: 45,
            recording_ticks: 4,
            post_ticks: 8,
            people: state
                .roster()
                .filter(|person| {
                    self.planner
                        .staff_owner(person.id)
                        .is_none_or(|station| station == PLAYER)
                })
                .map(|p| PersonInfo {
                    id: p.id.0,
                    name: p.name.clone(),
                    role: match p.role {
                        Role::Host => "host",
                        Role::Guest => "guest",
                    },
                    fee: p.fee.0.to_string(),
                    description: p.description.clone(),
                    normal_until_tick: p.normal_until.0,
                    negotiable_from_tick: p.negotiable_from.map(|t| t.0),
                    station_id: self.planner.staff_owner(p.id).map(|s| s.0),
                    commitments: state
                        .projects()
                        .filter(|project| {
                            project.booked_at.is_some()
                                && [
                                    project.concept.host,
                                    project.concept.guests[0],
                                    project.concept.guests[1],
                                ]
                                .contains(&p.id)
                        })
                        .map(|project| TalentCommitment {
                            production_id: project.id.0,
                            title: project.concept.title.clone(),
                            start_tick: project.concept.record_at.0,
                            end_tick: project.recording_end.0,
                        })
                        .collect(),
                })
                .collect(),
            projects: state
                .projects()
                .map(|p| ProductionInfo {
                    id: p.id.0,
                    title: p.concept.title.clone(),
                    host: p.concept.host.0,
                    guests: p.concept.guests.map(|g| g.0),
                    subject: p.concept.subject,
                    tone: p.concept.tone,
                    budget: p.concept.budget.0.to_string(),
                    fees: p.fees.0.to_string(),
                    record_at: p.concept.record_at.0,
                    recording_end: p.recording_end.0,
                    ready_at: p.ready_at.0,
                    phase: p.phase.label(),
                    programme: p.programme.map(|id| id.0),
                    booked_at: p.booked_at.map(|t| t.0),
                    studio: p.studio.map(|s| s.0),
                    quest_hints: state
                        .requests()
                        .filter_map(|r| {
                            let a = r.acceptance.as_ref()?;
                            if a.fulfilled_at.is_some() || self.planner.now() >= a.deadline {
                                return None;
                            }
                            let at = match r.condition {
                                bts_sim::planning::production::RequestCondition::CoGuests {
                                    ..
                                } => p.recording_end,
                                _ => p.ready_at,
                            };
                            Some(QuestHint {
                                request_id: r.id.0,
                                matches_requirements: r.condition.matches(&p.concept),
                                can_finish_in_time: p.concept.record_at >= a.accepted_at
                                    && (p.phase != bts_sim::planning::production::Phase::Draft
                                        || p.concept.record_at >= self.planner.now())
                                    && at <= a.deadline,
                                achievement_tick: at.0,
                            })
                        })
                        .collect(),
                    exceptional_person: p.exceptional_person.map(|id| id.0),
                    cooperation_sway: p.cooperation_sway,
                })
                .collect(),
            requests: state
                .requests()
                .map(|r| RequestInfo {
                    id: r.id.0,
                    proposer: r.proposer.0,
                    reward: r.reward,
                    achievement: r.condition.achievement(),
                    statement: r.statement.clone(),
                    time_limit_ticks: r.time_limit_ticks,
                    status: match &r.acceptance {
                        None => "available",
                        Some(a) if a.fulfilled_at.is_some() => "completed",
                        Some(a) if self.planner.now() >= a.deadline => "expired",
                        Some(_) => "open",
                    },
                    acceptance: r.acceptance.as_ref().map(|a| QuestAcceptanceInfo {
                        production_id: a.production.map(|p| p.0),
                        accepted_at: a.accepted_at.0,
                        deadline_tick: a.deadline.0,
                        fulfilled_at: a.fulfilled_at.map(|t| t.0),
                    }),
                })
                .collect(),
            sway_balance: state.sway(),
            participation_cost: PARTICIPATION_COST,
            known_concerns: state
                .cooperation_issues()
                .iter()
                .filter_map(|issue| {
                    issue.known_note.as_ref().map(|note| KnownConcern {
                        people: issue.people.map(|p| p.0),
                        note: note.clone(),
                    })
                })
                .collect(),
            sway_history: state
                .sway_history()
                .iter()
                .map(|e| SwayHistoryInfo {
                    tick: e.at.0,
                    production_id: e.production.0,
                    change: e.change,
                    balance: e.balance,
                    kind: match e.reason {
                        SwayReason::RequestReward(_) => "request_reward",
                        SwayReason::ExceptionalParticipation(_) => "exceptional_participation",
                        SwayReason::Cooperation => "cooperation",
                    },
                    request_id: match e.reason {
                        SwayReason::RequestReward(id) => Some(id.0),
                        _ => None,
                    },
                    person_id: match e.reason {
                        SwayReason::ExceptionalParticipation(id) => Some(id.0),
                        _ => None,
                    },
                })
                .collect(),
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
                programme_name: world
                    .programmes
                    .iter()
                    .find(|p| p.id == a.programme)
                    .unwrap()
                    .name
                    .clone(),
                genre: format!(
                    "{:?}",
                    world
                        .programmes
                        .iter()
                        .find(|p| p.id == a.programme)
                        .unwrap()
                        .genre
                ),
                duration_minutes: world
                    .programmes
                    .iter()
                    .find(|p| p.id == a.programme)
                    .unwrap()
                    .duration_minutes,
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
            .filter(|l| {
                l.from == Account::Station(PLAYER)
                    || l.to == Account::Station(PLAYER)
                    || matches!(l.from, Account::ProductionEscrow(_))
            })
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
                    production_id: match (l.from, l.to) {
                        (Account::ProductionEscrow(id), _) | (_, Account::ProductionEscrow(id)) => {
                            Some(id.0)
                        }
                        _ => None,
                    },
                    tick: l.at.0,
                    airing_id: airing.map(|a| a.id.0),
                    kind: match l.reason {
                        LedgerReason::BroadcastCost => "broadcast_cost",
                        LedgerReason::AudiencePoll => "audience_poll",
                        LedgerReason::ProductionReserve => "production_reserve",
                        LedgerReason::ProductionSpend => "production_spend",
                        _ => "advertising_payment",
                    },
                    amount: l.amount.0.to_string(),
                }
            })
            .collect::<Vec<_>>();
        let costs_paid = economy
            .ledger()
            .iter()
            .filter(|l| {
                l.from == Account::Station(PLAYER) && l.reason == LedgerReason::BroadcastCost
            })
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
            research: self.planner.research_view(),
            production: self.production_view(),
            booking_review: self.booking_review.clone(),
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
                production_committed: economy
                    .ledger()
                    .iter()
                    .filter(|l| {
                        l.from == Account::Station(PLAYER)
                            && l.reason == LedgerReason::ProductionReserve
                    })
                    .map(|l| u128::from(l.amount.0))
                    .sum::<u128>()
                    .to_string(),
                production_in_escrow: economy.production_escrow().to_string(),
                production_spent: economy.production_costs().0.to_string(),
                ledger,
            },
            stations: world
                .stations
                .iter()
                .map(|s| StationInfo {
                    id: s.id.0,
                    name: s.name.clone(),
                    is_player: s.id == PLAYER,
                    staff: self
                        .planner
                        .station_staff(s.id)
                        .filter_map(|id| {
                            self.planner
                                .production()
                                .roster()
                                .find(|p| p.id == id)
                                .map(|p| p.name.clone())
                        })
                        .collect(),
                    owned_studios: (s.id == PLAYER).then(|| self.planner.studio_count(s.id)),
                })
                .collect(),
            gaps,
            catalogue: world
                .programmes
                .iter()
                .filter(|p| self.planner.owns_programme(PLAYER, p.id))
                .map(|p| CatalogueEntry {
                    production_id: self
                        .planner
                        .production()
                        .projects()
                        .find(|project| project.programme == Some(p.id))
                        .map(|project| project.id.0),
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

fn concept_input(input: ProductionConceptInput) -> Result<Concept, PlayerError> {
    if input.budget.is_empty() || !input.budget.bytes().all(|b| b.is_ascii_digit()) {
        return Err(PlayerError::new(
            "invalid_budget",
            "Use a decimal string of integer credits for the budget.",
        ));
    }
    let budget = input.budget.parse::<u64>().map_err(|_| {
        PlayerError::new(
            "invalid_budget",
            "Budget is outside the supported integer range.",
        )
    })?;
    Ok(Concept {
        title: input.title,
        host: PersonId(input.host),
        guests: input.guests.map(PersonId),
        subject: input.subject,
        tone: input.tone,
        budget: bts_sim::economy::Money(budget),
        record_at: SimTick(input.record_at),
    })
}
