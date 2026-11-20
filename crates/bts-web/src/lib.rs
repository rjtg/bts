//! Versioned browser commands and restricted player observations over the Rust planner.
#![forbid(unsafe_code)]
pub mod protocol;
mod save;
pub mod scenario;

use bts_sim::economy::{Account, AudienceCommitment, Contract, ContractStatus, LedgerReason};
use bts_sim::planning::production::{
    Concept, PersonId, ProductionId, RequestId, StudioId, SwayReason, PARTICIPATION_COST,
};
use bts_sim::planning::{AiringId, PlanError, Planner, PlanningOffer};
use bts_sim::{AirMinute, BrandId, ProgrammeId, SimTick};
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
#[derive(Clone)]
enum WeeklyDraft {
    Create(bts_sim::planning::weekly::WeeklyPlan),
    Edit(AiringId, u16),
}
/// Each game session exclusively owns its simulation. Rejections are atomic.
#[derive(Clone)]
pub struct Session {
    planner: Planner,
    journal: std::sync::Arc<Vec<Command>>,
    namespace: u32,
    ad_load_percent: u8,
    rights_epoch: u64,
    weekly_pending: Option<(WeeklyDraft, WeeklyReview)>,
    next_offer: u64,
    pending: Option<Pending>,
    minute_pending: Option<(
        AiringId,
        String,
        Vec<bts_sim::planning::ad_market::MinuteOffer>,
    )>,
    booking_review: Option<ProductionBookingReview>,
    film_review: Option<FilmReview>,
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
            journal: std::sync::Arc::default(),
            rights_epoch: 0,
            ad_load_percent: 0,
            weekly_pending: None,
            namespace,
            next_offer: 1,
            pending: None,
            minute_pending: None,
            booking_review: None,
            film_review: None,
        })
    }
    pub fn execute(&mut self, command: Command) -> Result<PlayerReport, PlayerError> {
        let logged = (!matches!(&command, Command::Inspect {})).then(|| command.clone());
        let mut candidate = self.clone();
        candidate.apply(command)?;
        let report = candidate.report();
        serde_json::to_string(&report).map_err(|_| {
            PlayerError::new("report_error", "Could not construct a player report.")
        })?;
        *self = candidate;
        if let Some(command) = logged {
            std::sync::Arc::make_mut(&mut self.journal).push(save::without_session_token(command));
        }
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
                "This adapter supports api_version 8.",
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
            Command::PlaceFiller {
                start_minute,
                teaser_for,
            } => {
                self.planner
                    .place_filler(AirMinute(start_minute), teaser_for.map(AiringId))
                    .map_err(player_error)?;
            }
            Command::SetAdLoad { percent } => {
                if percent > 50 {
                    return Err(PlayerError::new(
                        "ad_load",
                        "Choose a default ad load from 0 to 50 percent.",
                    ));
                }
                self.ad_load_percent = percent;
            }
            Command::PlaceExact {
                programme,
                start_minute,
                ad_minutes,
            } => {
                let programme = ProgrammeId(programme);
                let minutes = match ad_minutes {
                    Some(minutes) => minutes,
                    None => {
                        let duration = self
                            .planner
                            .world()
                            .programmes
                            .iter()
                            .find(|work| work.id == programme)
                            .ok_or_else(|| {
                                PlayerError::new(
                                    "unknown_programme",
                                    "Choose a tape from the library.",
                                )
                            })?
                            .duration_minutes;
                        ((u32::from(duration) * u32::from(self.ad_load_percent) + 50) / 100).min(30)
                            as u8
                    }
                };
                self.planner
                    .place_exact(programme, AirMinute(start_minute), minutes)
                    .map_err(player_error)?;
            }
            Command::MoveExact {
                airing_id,
                start_minute,
            } => {
                self.planner
                    .move_exact(AiringId(airing_id), AirMinute(start_minute))
                    .map_err(player_error)?;
            }
            Command::AdvanceExact { to_minute } => {
                self.planner
                    .advance_exact(AirMinute(to_minute))
                    .map_err(player_error)?;
            }
            Command::PreviewWeekly { plan } => {
                let weekday = bts_sim::planning::weekly::Weekday::new(plan.weekday)
                    .ok_or_else(|| PlayerError::new("weekly_plan", "Choose a weekday."))?;
                let plan = bts_sim::planning::weekly::WeeklyPlan {
                    programme: ProgrammeId(plan.programme),
                    weekday,
                    time: weekly_time(plan.time, plan.minute_of_day)?,
                    count: plan.count,
                    ad_minutes: plan.ad_minutes,
                    exceptions: plan
                        .exceptions
                        .into_iter()
                        .map(|exception| {
                            if exception.start_tick.is_some() && exception.start_minute.is_some() {
                                return Err(PlayerError::new(
                                    "weekly_exception",
                                    "Supply one start time per exception.",
                                ));
                            }
                            Ok(bts_sim::planning::weekly::Exception {
                                occurrence: exception.occurrence,
                                start_minute: exception.start_minute.or_else(|| {
                                    exception
                                        .start_tick
                                        .map(|tick| AirMinute::from_tick(SimTick(tick)).0)
                                }),
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                };
                let rows = self.planner.preview_weekly(&plan).map_err(player_error)?;
                self.weekly_pending = Some((
                    WeeklyDraft::Create(plan),
                    WeeklyReview {
                        edition: false,
                        rows,
                    },
                ));
                return Ok(());
            }
            Command::PreviewWeeklyEdit {
                airing_id,
                time,
                minute_of_day,
            } => {
                let time = weekly_time(time, minute_of_day)?;
                let id = AiringId(airing_id);
                let rows = self
                    .planner
                    .preview_weekly_edit(id, time)
                    .map_err(player_error)?;
                self.weekly_pending = Some((
                    WeeklyDraft::Edit(id, time),
                    WeeklyReview {
                        edition: true,
                        rows,
                    },
                ));
                return Ok(());
            }
            Command::ConfirmWeekly {} => {
                let (draft, review) = self.weekly_pending.as_ref().ok_or_else(|| {
                    PlayerError::new(
                        "weekly_review_required",
                        "Preview the dates again before confirming.",
                    )
                })?;
                let current = match draft {
                    WeeklyDraft::Create(plan) => self.planner.preview_weekly(plan),
                    WeeklyDraft::Edit(id, time) => self.planner.preview_weekly_edit(*id, *time),
                }
                .map_err(player_error)?;
                if current != review.rows {
                    return Err(PlayerError::new(
                        "weekly_review_required",
                        "The dates changed. Preview them again.",
                    ));
                }
                match draft {
                    WeeklyDraft::Create(plan) => {
                        self.planner.place_weekly(plan).map_err(player_error)?;
                    }
                    WeeklyDraft::Edit(id, time) => {
                        self.planner.edit_weekly(*id, *time).map_err(player_error)?
                    }
                }
            }

            Command::BuyRights { programme, token } => {
                let token = self.rights_token(&token)?;
                self.planner
                    .buy_rights(ProgrammeId(programme), token)
                    .map_err(player_error)?;
            }
            Command::AssessTitle { programme, insight } => {
                self.planner
                    .commission_title_assessment(ProgrammeId(programme), insight)
                    .map_err(player_error)?;
            }
            Command::ListResale { programme } => {
                self.planner
                    .list_for_resale(ProgrammeId(programme))
                    .map_err(player_error)?;
            }
            Command::AcceptResale { programme, token } => {
                let token = self.rights_token(&token)?;
                self.planner
                    .accept_resale(ProgrammeId(programme), token)
                    .map_err(player_error)?;
            }

            Command::ResizeAdBlock { airing_id, minutes } => {
                self.planner
                    .resize_ad_block(AiringId(airing_id), minutes)
                    .map_err(player_error)?;
            }
            Command::SetAutosell { airing_id, policy } => {
                let policy = policy
                    .map(|input| {
                        let floor = input
                            .floor
                            .parse::<u64>()
                            .ok()
                            .filter(|_| input.floor.bytes().all(|b| b.is_ascii_digit()))
                            .ok_or_else(|| {
                                PlayerError::new(
                                    "invalid_autosell_policy",
                                    "Use whole Funds for the price floor.",
                                )
                            })?;
                        Ok::<_, PlayerError>(bts_sim::planning::autosell::AutosellPolicy {
                            minimum_per_minute: bts_sim::economy::Money(floor),
                            max_minutes: input.max_minutes,
                        })
                    })
                    .transpose()?;
                self.planner
                    .set_autosell(AiringId(airing_id), policy)
                    .map_err(player_error)?;
            }
            Command::RequestMinuteOffers { airing_id } => {
                return self.issue_minute_offers(AiringId(airing_id));
            }
            Command::AcceptMinuteOffer {
                airing_id,
                advertiser,
                token,
            } => {
                let id = AiringId(airing_id);
                let quote = self
                    .minute_pending
                    .as_ref()
                    .filter(|(airing, issued, _)| *airing == id && *issued == token)
                    .and_then(|(_, _, offers)| offers.iter().find(|q| q.brand.0 == advertiser))
                    .ok_or_else(|| {
                        PlayerError::new("stale_minute_offer", "Refresh offers for this airing.")
                    })?;
                self.planner.accept_minutes(quote).map_err(player_error)?;
                self.pending = None;
                self.weekly_pending = None;
                return self.issue_minute_offers(id);
            }

            Command::PitchScriptedRun {
                title,
                subject,
                standing,
                tension,
            } => {
                self.planner
                    .pitch_scripted_run(title, subject, standing, tension)
                    .map_err(player_error)?;
            }
            Command::PitchFilm {
                title,
                subject,
                ambition,
            } => {
                self.planner
                    .pitch_film(title, subject, ambition)
                    .map_err(player_error)?;
            }
            Command::WriteFilm {
                film_id,
                writer,
                focus,
            } => {
                self.planner
                    .write_film(ProductionId(film_id), PersonId(writer), focus)
                    .map_err(player_error)?;
            }
            Command::AcceptScript { film_id } => self
                .planner
                .accept_script(ProductionId(film_id))
                .map_err(player_error)?,
            Command::ReviewFilm { film_id, booking } => {
                let booking = film_booking(booking);
                let quote = self
                    .planner
                    .review_film(ProductionId(film_id), booking)
                    .map_err(player_error)?;
                self.film_review = Some(film_review(film_id, booking, &quote));
                return Ok(());
            }
            Command::BookFilm { film_id, booking } => {
                self.planner
                    .book_film(ProductionId(film_id), film_booking(booking))
                    .map_err(player_error)?;
            }

            Command::SetPolling { polling } => {
                self.planner.set_polling(polling).map_err(player_error)?
            }
            Command::SpendInsight { audience, amount } => self
                .planner
                .spend_insight(bts_sim::AudienceId(audience), amount)
                .map_err(player_error)?,
            Command::Inspect {} => return Ok(()),
            Command::AcceptRequest { request_id } => self
                .planner
                .accept_request(RequestId(request_id))
                .map_err(player_error)?,
            Command::SignHost { host, productions } => {
                let productions = productions
                    .into_iter()
                    .map(ProductionId)
                    .collect::<Vec<_>>();
                self.planner
                    .sign_host(PersonId(host), &productions)
                    .map_err(player_error)?;
            }
            Command::StartNewsroom {
                title,
                first_day,
                minute_of_day,
                ad_minutes,
                staff,
            } => self
                .planner
                .start_newsroom(
                    title,
                    first_day,
                    minute_of_day,
                    ad_minutes,
                    news_staff(staff),
                )
                .map_err(player_error)?,
            Command::UpdateNewsroomStaff { staff } => self
                .planner
                .update_newsroom_staff(news_staff(staff))
                .map_err(player_error)?,
            Command::CreateDiscussionRun {
                brief,
                first_air_minute,
                ad_minutes,
            } => {
                self.planner
                    .create_discussion_run(
                        concept_input(brief)?,
                        AirMinute(first_air_minute),
                        ad_minutes,
                    )
                    .map_err(player_error)?;
            }
            Command::BookDiscussionEdition {
                production_id,
                studio,
            } => {
                self.planner
                    .book_discussion_edition(ProductionId(production_id), StudioId(studio))
                    .map_err(player_error)?;
            }
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
                    budget: self
                        .planner
                        .production_balance_due(ProductionId(production_id))
                        .map_err(player_error)?
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
                self.ad_load_percent = 0;
                self.rights_epoch = self
                    .rights_epoch
                    .checked_add(1)
                    .ok_or_else(|| PlayerError::new("capacity", "Reset capacity exhausted."))?;
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
        self.weekly_pending = None;
        self.minute_pending = None;
        self.booking_review = None;
        self.film_review = None;
        Ok(())
    }
    fn issue_minute_offers(&mut self, id: AiringId) -> Result<(), PlayerError> {
        let offers = self.planner.minute_offers(id).map_err(player_error)?;
        let token = format!("{}:{}", self.namespace, self.next_offer);
        self.next_offer = self
            .next_offer
            .checked_add(1)
            .ok_or_else(|| PlayerError::new("capacity", "Offer capacity exhausted."))?;
        self.minute_pending = Some((id, token, offers));
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
    fn booking(&self, contract: &Contract<AirMinute>) -> Booking {
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
            deadline_tick: contract.offer.deadline.tick_ceil().0,
            maximum_payment: contract.offer.payment.0.to_string(),
            targets: self.commitments(&contract.commitments),
            settlement,
        }
    }
    fn production_view(&self) -> ProductionView {
        let state = self.planner.production();
        ProductionView {
            host_agreements: self
                .planner
                .host_agreements()
                .map(|agreement| HostAgreementView {
                    id: agreement.id.0,
                    host: agreement.host.0,
                    total: agreement.total.0.to_string(),
                    dates: agreement
                        .engagements
                        .iter()
                        .map(|date| {
                            use bts_sim::planning::production::Phase;
                            let project = state
                                .project(date.production)
                                .expect("signed project exists");
                            let status = match project.phase {
                                Phase::Draft
                                    if AirMinute::from_tick(date.start)
                                        < self.planner.now_minute() =>
                                {
                                    "unused"
                                }
                                Phase::Draft => "reserved",
                                Phase::Booked | Phase::Recording => "booked",
                                _ => "fulfilled",
                            };
                            HostDateView {
                                production: date.production.0,
                                title: project.concept.title.clone(),
                                start_tick: date.start.0,
                                end_tick: date.end.0,
                                fee: date.fee.0.to_string(),
                                status,
                            }
                        })
                        .collect(),
                })
                .collect(),
            discussion_runs: self
                .planner
                .discussion_runs()
                .map(|run| DiscussionRunInfo {
                    series: run.series.0,
                    title: run.brief.title.clone(),
                    host: run.brief.host.0,
                    editions: run
                        .editions
                        .iter()
                        .map(|edition| DiscussionEditionInfo {
                            production_id: edition.production.0,
                            planned_air_minute: edition.air_at.0,
                            reserved_end_minute: edition.reserved_end().map(|end| end.0),
                            ad_minutes: edition.ad_minutes,
                            airing_id: edition
                                .airing
                                .filter(|id| self.planner.airing(*id).is_ok())
                                .map(|id| id.0),
                            missed: self.planner.discussion_edition_missed(edition),
                        })
                        .collect(),
                })
                .collect(),
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
                    role: p.role,
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
                        .chain(
                            self.planner
                                .host_agreements()
                                .filter(|hire| hire.host == p.id)
                                .flat_map(|hire| {
                                    hire.engagements.iter().filter_map(|date| {
                                        let project = state.project(date.production).ok()?;
                                        project.booked_at.is_none().then(|| TalentCommitment {
                                            production_id: project.id.0,
                                            title: format!(
                                                "{} (host agreement)",
                                                project.concept.title
                                            ),
                                            start_tick: date.start.0,
                                            end_tick: date.end.0,
                                        })
                                    })
                                }),
                        )
                        .chain(self.planner.newsroom().into_iter().flat_map(|room| {
                            room.editions.iter().filter_map(move |edition| {
                                use bts_sim::planning::newsroom::NewsPhase;
                                if matches!(edition.phase, NewsPhase::Planned | NewsPhase::Missed) {
                                    return None;
                                }
                                let (start, end) = if edition.reporter == p.id {
                                    (edition.reporting_at(), edition.recording_at())
                                } else if edition.anchor == p.id {
                                    (edition.recording_at(), edition.recording_end())
                                } else {
                                    return None;
                                };
                                Some(TalentCommitment {
                                    production_id: edition.id.0,
                                    title: room.title.clone(),
                                    start_tick: start.0,
                                    end_tick: end.0,
                                })
                            })
                        }))
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
                    ambition: qualities(p.concept.ambition()),
                    budget: p.concept.budget.0.to_string(),
                    host_prepaid: self
                        .planner
                        .host_engagement(p.id)
                        .map_or(0, |date| date.fee.0)
                        .to_string(),
                    balance_due: self
                        .planner
                        .production_balance_due(p.id)
                        .expect("existing production")
                        .0
                        .to_string(),
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
        let now_minute = self.planner.now_minute().0;
        let mut schedule = self
            .planner
            .airings()
            .map(|a| ScheduleEntry {
                airing_id: a.id.0,
                station: a.station.0,
                programme: a.programme.0,
                programme_name: match a.filler {
                    Some(bts_sim::planning::Filler::Teaser { .. })
                        if self.planner.teaser_is_current(a) =>
                    {
                        "Upcoming show teaser".into()
                    }
                    Some(_) => "Station ident".into(),
                    None => world
                        .programmes
                        .iter()
                        .find(|p| p.id == a.programme)
                        .unwrap()
                        .name
                        .clone(),
                },
                genre: world
                    .programmes
                    .iter()
                    .find(|p| p.id == a.programme)
                    .unwrap()
                    .profile
                    .genre()
                    .into(),
                duration_minutes: world
                    .programmes
                    .iter()
                    .find(|p| p.id == a.programme)
                    .unwrap()
                    .duration_minutes,
                start_tick: a.start.tick_floor().0,
                end_tick: a.end.tick_ceil().0,
                start_minute: a.start.0,
                end_minute: a.end.0,
                filler: a.filler.is_some(),
                teaser_for: match a.filler {
                    Some(bts_sim::planning::Filler::Teaser { airing, .. }) => Some(airing.0),
                    _ => None,
                },
                teaser_current: self.planner.teaser_is_current(a),
                proposed_advertiser: a.advertiser.map(|id| id.0),
                ad_minutes: a.ad_minutes,
                sold_minutes: self.planner.sold_minutes(a.id),
                weekly_id: a.weekly.map(|id| id.0),
                lead_in_airing_id: (a.station == PLAYER)
                    .then(|| self.planner.potential_lead_in(a))
                    .flatten()
                    .map(|previous| previous.id.0),
                prior_slot_airing: a.station == PLAYER && self.planner.has_prior_slot_airing(a),
                autosell_eligible: self.planner.autosell_eligible(a),
                autosell: self.planner.autosell_policy(a).map(|policy| AutosellView {
                    floor: policy.minimum_per_minute.0.to_string(),
                    max_minutes: policy.max_minutes,
                }),
                booked: a.contract.is_some() || self.planner.sold_minutes(a.id) > 0,
                status: if a.cancelled() {
                    "off_air"
                } else if a.result.is_some() {
                    "completed"
                } else if a.started() {
                    "on_air"
                } else {
                    "planned"
                },
                editable: a.station == PLAYER
                    && !a.started()
                    && a.contract.is_none()
                    && self.planner.sold_minutes(a.id) == 0,
            })
            .collect::<Vec<_>>();
        schedule.sort_by_key(|a| (a.station, a.start_minute, a.airing_id));
        let mut player = schedule
            .iter()
            .filter(|a| a.station == PLAYER.0)
            .collect::<Vec<_>>();
        player.sort_by_key(|a| (a.start_minute, a.airing_id));
        let next_airing = player
            .iter()
            .find(|a| a.start_minute >= now_minute)
            .map(|a| a.airing_id);
        let active_airing = player
            .iter()
            .find(|a| a.start_minute < now_minute && now_minute < a.end_minute)
            .map(|a| a.airing_id);
        let mut gaps = vec![];
        let mut cursor = 0;
        let boundary = self.planner.end().0 * 15;
        let mut occupied: Vec<_> = player
            .iter()
            .map(|a| (a.start_minute, a.end_minute))
            .chain(
                self.planner
                    .discussion_runs()
                    .flat_map(|run| &run.editions)
                    .filter_map(|edition| {
                        edition.reserved_end().map(|end| (edition.air_at.0, end.0))
                    }),
            )
            .chain(
                self.planner
                    .newsroom()
                    .into_iter()
                    .flat_map(|news| &news.editions)
                    .filter_map(|edition| {
                        edition.reserved_end().map(|end| (edition.air_at.0, end.0))
                    }),
            )
            .collect();
        occupied.sort_unstable();
        for (start, end) in occupied
            .into_iter()
            .chain(std::iter::once((boundary, boundary)))
        {
            if start > cursor {
                gaps.push(Gap {
                    start_tick: cursor / 15,
                    end_tick: start.div_ceil(15),
                    start_minute: cursor,
                    end_minute: start,
                });
            }
            cursor = end;
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
                        .find(|a| a.station == PLAYER && a.start < l.at && l.at <= a.end),
                    _ => match l.from {
                        Account::Escrow(id) => {
                            self.planner.airings().find(|a| a.contract == Some(id))
                        }
                        Account::MinuteEscrow(id) => economy
                            .minute_sales()
                            .find(|sale| sale.id == id)
                            .and_then(|sale| self.planner.airing(sale.offer.airing).ok()),
                        _ => None,
                    },
                };
                FinancialLine {
                    rights_title: l.rights_title.map(|id| id.0),
                    production_id: match (l.from, l.to) {
                        (Account::ProductionEscrow(id), _) | (_, Account::ProductionEscrow(id)) => {
                            Some(id.0)
                        }
                        _ => None,
                    },
                    tick: l.at.tick_ceil().0,
                    airing_id: airing.map(|a| a.id.0),
                    kind: match l.reason {
                        LedgerReason::RightsPurchase => "rights_purchase",
                        LedgerReason::RightsResale => "rights_resale",
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
            .filter(|l| l.to == Account::Station(PLAYER) && l.reason != LedgerReason::RightsResale)
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
                    start_tick: a.start.tick_floor().0,
                    end_tick: a.end.tick_ceil().0,
                    start_minute: a.start.0,
                    end_minute: a.end.0,
                    snapshot_tick: a.start.tick_floor().0,
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
                            film_reception: self
                                .planner
                                .films()
                                .any(|f| f.programme == Some(a.programme))
                                .then(|| {
                                    allocation
                                        .viewing
                                        .iter()
                                        .find(|v| v.station == PLAYER && v.viewers >= 1.0)
                                })
                                .flatten()
                                .map(|v| match v.satisfaction.trace.output {
                                    x if x >= 0.72 => "Strong",
                                    x if x >= 0.50 => "Mixed",
                                    _ => "Cool",
                                }),
                        })
                        .collect(),
                })
            })
            .collect::<Vec<_>>();
        results.sort_by_key(|r| (r.end_tick, r.airing_id));
        let transmitted = self.planner.airings().filter(|airing| {
            airing.station == PLAYER
                && airing.filler.is_none()
                && airing.result.is_some()
                && !airing.cancelled()
        });
        let mut active_weeks = std::collections::BTreeSet::new();
        let mut recurring = 0;
        for airing in transmitted {
            active_weeks.insert(airing.start.0 / (7 * 1440));
            recurring += u32::from(self.planner.autosell_eligible(airing));
        }
        let settled_blocks = economy
            .minute_sales()
            .filter(|sale| {
                sale.offer.station == PLAYER
                    && sale.settlement.as_ref().is_some_and(|s| s.payment.0 > 0)
            })
            .map(|sale| sale.offer.airing)
            .collect::<std::collections::BTreeSet<_>>()
            .len() as u32;
        PlayerReport {
            station_goals: vec![
                StationGoal {
                    label: "Broadcast in four different weeks",
                    progress: active_weeks.len() as u32,
                    target: 4,
                },
                StationGoal {
                    label: "Air three recurring programmes",
                    progress: recurring,
                    target: 3,
                },
                StationGoal {
                    label: "Earn paid ad-minute sales on three airings",
                    progress: settled_blocks,
                    target: 3,
                },
            ],
            minute_offers: self
                .minute_pending
                .as_ref()
                .map(|(id, token, offers)| MinuteOffers {
                    airing_id: id.0,
                    token: token.clone(),
                    offers: offers
                        .iter()
                        .map(|q| MinuteOfferView {
                            advertiser: q.brand.0,
                            minutes: q.minutes,
                            price: q.payment.0.to_string(),
                            expected_target_viewers: q.forecast_target_viewers,
                        })
                        .collect(),
                }),
            minute_sales: economy
                .minute_sales()
                .map(|sale| MinuteBooking {
                    automatic: self.planner.is_automatic_sale(sale.id),
                    airing_id: sale.offer.airing.0,
                    advertiser: sale.offer.brand.0,
                    minutes: sale.offer.minutes,
                    price: sale.offer.payment.0.to_string(),
                    settlement: sale.settlement.as_ref().map(|s| MinuteSettlementView {
                        aired_minutes: s.aired_minutes,
                        payment: s.payment.0.to_string(),
                        refund: s.refund.0.to_string(),
                        viewer_minutes: s.viewer_minutes,
                    }),
                })
                .collect(),

            reporting: REPORTING,
            now_tick: now,
            now_minute,
            ad_load_percent: self.ad_load_percent,
            end_tick: self.planner.end().0,
            first_weekday: self.planner.first_weekday().number(),
            weekly_review: self
                .weekly_pending
                .as_ref()
                .map(|(_, review)| review.clone()),
            tick_minutes: 15,
            next_airing,
            active_airing,
            cash: economy.cash(PLAYER).unwrap().0.to_string(),
            research: self.planner.research_view(),
            production: self.production_view(),
            newsroom: self.planner.newsroom().map(|news| NewsroomView {
                title: news.title.clone(),
                editions: news
                    .editions
                    .iter()
                    .map(|edition| NewsEditionView {
                        id: edition.id.0,
                        reporting_tick: edition.reporting_at().0,
                        recording_tick: edition.recording_at().0,
                        recording_end_tick: edition.recording_end().0,
                        delivery_tick: edition.delivery_at().0,
                        reporter: edition.reporter.0,
                        anchor: edition.anchor.0,
                        studio: edition.studio.0,
                        air_minute: edition.air_at.0,
                        reserved_end_minute: edition.reserved_end().map(|m| m.0),
                        ad_minutes: edition.ad_minutes,
                        cost: edition.cost.0.to_string(),
                        phase: edition.phase,
                        programme: edition.programme.map(|id| id.0),
                        airing_id: edition
                            .airing
                            .filter(|id| self.planner.airing(*id).is_ok())
                            .map(|id| id.0),
                        exception: edition.exception,
                    })
                    .collect(),
            }),
            film_desk: film_desk(&self.planner),
            rights_market: self.rights_market(),
            film_review: self.film_review.clone(),
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
                rights_spent: economy
                    .ledger()
                    .iter()
                    .filter(|l| {
                        l.from == Account::Station(PLAYER)
                            && l.reason == LedgerReason::RightsPurchase
                    })
                    .map(|l| u128::from(l.amount.0))
                    .sum::<u128>()
                    .to_string(),
                resale_receipts: economy
                    .ledger()
                    .iter()
                    .filter(|l| {
                        l.to == Account::Station(PLAYER) && l.reason == LedgerReason::RightsResale
                    })
                    .map(|l| u128::from(l.amount.0))
                    .sum::<u128>()
                    .to_string(),
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
                .filter(|p| {
                    self.planner.owns_programme(PLAYER, p.id)
                        && self.planner.filler_programme() != Some(p.id)
                })
                .map(|p| CatalogueEntry {
                    production_id: self
                        .planner
                        .production()
                        .projects()
                        .find(|project| project.programme == Some(p.id))
                        .map(|project| project.id.0),
                    id: p.id.0,
                    name: p.name.clone(),
                    genre: p.profile.genre().into(),
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
                    production_type: p.profile.production_type,
                    qualities: programme_qualities(p.profile),
                    awareness: p.awareness,
                    cast_appeal: p.cast_appeal,
                    broadcast_cost: p.broadcast_cost.0.to_string(),
                    duration_minutes: p.duration_minutes,
                })
                .collect(),
            brand_news: self
                .planner
                .brand_reviews()
                .iter()
                .map(|review| BrandNews {
                    advertiser: review.brand.0,
                    tick: review.at.tick_floor().0,
                    direction: review.direction,
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
                    start_tick: airing.start.tick_floor().0,
                    end_tick: airing.end.tick_ceil().0,
                    start_minute: airing.start.0,
                    end_minute: airing.end.0,
                    advertiser: p.quote.offer.brand.0,
                    deadline_tick: p.quote.offer.deadline.tick_ceil().0,
                    maximum_payment: p.quote.offer.payment.0.to_string(),
                    targets: p.targets.clone(),
                }
            }),
            contracts: economy.contracts().map(|c| self.booking(c)).collect(),
            results,
        }
    }
}
fn weekly_time(tick: Option<u8>, minute: Option<u16>) -> Result<u16, PlayerError> {
    match (tick, minute) {
        (Some(tick), None) => Ok(u16::from(tick) * 15),
        (None, Some(minute)) => Ok(minute),
        _ => Err(PlayerError::new(
            "weekly_time",
            "Supply one airtime for the regular slot.",
        )),
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
        pub fn save(&self) -> Result<String, JsValue> {
            self.session
                .save_game()
                .map_err(|error| JsValue::from_str(error.message))
        }
        pub fn restore(&mut self, text: &str) -> String {
            let response = match self.session.restore_game(text) {
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
    }
    #[wasm_bindgen]
    pub fn api_version() -> u32 {
        API_VERSION
    }
}
#[cfg(test)]
mod tests;

fn news_staff(input: NewsroomStaffInput) -> bts_sim::planning::newsroom::NewsStaff {
    bts_sim::planning::newsroom::NewsStaff {
        reporter: PersonId(input.reporter),
        anchor: PersonId(input.anchor),
        studio: StudioId(input.studio),
    }
}

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

fn qualities(profile: bts_sim::FeatureProfile) -> Vec<Quality> {
    bts_sim::FEATURE_NAMES
        .into_iter()
        .zip(profile.values)
        .filter_map(|(name, value)| {
            value.map(|value| Quality {
                name,
                level: bts_sim::feature_band(name, value),
            })
        })
        .collect()
}

fn programme_qualities(mut profile: bts_sim::FeatureProfile) -> Vec<Quality> {
    profile.values[bts_sim::EDITORIAL_RIGOUR] = None;
    qualities(profile)
}

fn film_booking(b: FilmBookingInput) -> bts_sim::planning::film::FilmBooking {
    bts_sim::planning::film::FilmBooking {
        cast: b.cast.map(PersonId),
        director: PersonId(b.director),
        studio: b.studio,
        start: SimTick(b.start),
    }
}
fn film_review(
    id: u32,
    b: bts_sim::planning::film::FilmBooking,
    q: &bts_sim::planning::film::FilmQuote,
) -> FilmReview {
    FilmReview {
        id,
        cast: b.cast.map(|p| p.0),
        director: b.director.0,
        studio: b.studio,
        start: b.start.0,
        recording_end: q.recording_end.0,
        ready: q.ready.0,
        fees: q.fees.0.to_string(),
        rent: q.rent.0.to_string(),
        total: q.total.0.to_string(),
        studio_shortfall: q.studio_shortfall,
    }
}
fn film_desk(planner: &Planner) -> FilmDesk {
    use bts_sim::planning::film::*;
    let now = planner.now().0;
    FilmDesk {
        scripted_runs: planner
            .scripted_runs()
            .map(|run| ScriptedRunView {
                series: run.series.0,
                title: run.title.clone(),
                brief: qualities(run.brief),
                episodes: run.episodes.map(|project| project.0),
                cast: run.cast.map(|cast| cast.map(|person| person.0)),
            })
            .collect(),
        roles: ROLES,
        writing_ticks: WRITING_TICKS,
        revision_ticks: REVISION_TICKS,
        projects: planner
            .films()
            .map(|film| FilmCard {
                id: film.id.0,
                title: film.title.clone(),
                subject: film.subject,
                phase: film.phase,
                episode: film.episode.map(|episode| ScriptedEpisodeView {
                    series: episode.series.0,
                    sequence: episode.sequence,
                    tension: film.ambition.values[2].expect("fiction tension target"),
                    demanding_shift: episode.strain >= 0.10,
                }),
                committed: film.committed.0.to_string(),
                writer: film.writer.map(|w| w.0),
                due: film.due.map(|tick| tick.0),
                revised: film.revision_focus.is_some(),
                programme: film.programme.map(|id| id.0),
                script: film.script.as_ref().map(|script| FilmScript {
                    qualities: qualities(script.profile),
                    weaknesses: script
                        .weaknesses
                        .iter()
                        .map(|&i| (i as u8, bts_sim::FEATURE_NAMES[i]))
                        .collect(),
                    studio_demand: if script.equipment_demand > 0.32 {
                        "Needs equipped staging"
                    } else {
                        "A compact room will do"
                    },
                }),
                booking: film
                    .booking
                    .zip(film.quote.as_ref())
                    .map(|(b, q)| film_review(film.id.0, b, q)),
            })
            .collect(),
        people: PEOPLE
            .iter()
            .map(|person| FilmPerson {
                id: person.id.0,
                name: person.name,
                job: person.job,
                fee: person.fee.0.to_string(),
                revision_fee: person.fee.0.div_ceil(2).to_string(),
                specialty: person.specialty,
                available_until: person.available_until,
                commitments: planner
                    .films()
                    .filter_map(|f| {
                        if matches!(f.phase, FilmPhase::Writing | FilmPhase::Revising)
                            && f.writer == Some(person.id)
                        {
                            return Some((f.title.clone(), f.writing_start?.0, f.due?.0));
                        }
                        let (b, q) = f.booking.zip(f.quote.as_ref())?;
                        (q.recording_end.0 > now
                            && (b.cast.contains(&person.id) || b.director == person.id))
                            .then(|| (f.title.clone(), b.start.0, q.recording_end.0))
                    })
                    .collect(),
            })
            .collect(),
        studios: RENTALS
            .iter()
            .map(|studio| FilmStudio {
                id: studio.id,
                name: studio.name,
                fee: studio.fee.0.to_string(),
                summary: studio.summary,
                commitments: planner
                    .films()
                    .filter_map(|f| {
                        let (b, q) = f.booking.zip(f.quote.as_ref())?;
                        (b.studio == studio.id && q.recording_end.0 > now)
                            .then(|| (f.title.clone(), b.start.0, q.recording_end.0))
                    })
                    .collect(),
            })
            .collect(),
    }
}

impl Session {
    fn rights_token(&self, token: &str) -> Result<u64, PlayerError> {
        token
            .strip_prefix(&format!("{}:{}:", self.namespace, self.rights_epoch))
            .and_then(|suffix| suffix.parse().ok())
            .ok_or_else(|| {
                PlayerError::new("stale_rights_offer", "Refresh the offer in this session.")
            })
    }

    fn rights_market(&self) -> RightsMarket {
        use bts_sim::planning::rights::{RightsOffer, TitleStatus, ASSESSMENT_TICKS};
        let offer = |q: &RightsOffer| MarketOffer {
            token: format!("{}:{}:{}", self.namespace, self.rights_epoch, q.token),
            price: q.price.0.to_string(),
            deadline: q.deadline.0,
        };
        let round = |n: f64| (n / 100.0).round() as u64 * 100;
        let titles = self
            .planner
            .titles()
            .filter(|title| title.status != TitleStatus::Unreleased)
            .map(|title| {
                let id = title.seed.programme.id;
                let f = self.planner.title_forecast(id).expect("published title");
                MarketTitle {
                    programme: id.0,
                    name: title.seed.programme.name.clone(),
                    status: title.status,
                    owner: title.owner.map(|id| id.0),
                    evidence: title.seed.evidence,
                    art: title.seed.art,
                    duration: title.seed.programme.duration_minutes,
                    broadcast_cost: title.seed.programme.broadcast_cost.0.to_string(),
                    acquisition: (title.status == TitleStatus::Listed)
                        .then(|| title.acquisition.as_ref().map(offer))
                        .flatten(),
                    resale: title.resale.as_ref().map(offer),
                    blocker: self.planner.resale_blocker(id).map(|id| id.0),
                    rival_airing: self
                        .planner
                        .airings()
                        .find(|a| {
                            a.programme == id
                                && a.station != PLAYER
                                && a.end > self.planner.now_minute()
                        })
                        .map(|a| a.id.0),
                    assessment_due: title.assessment_due.map(|at| at.0),
                    assessment_at: title.assessment_at.map(|at| at.0),
                    qualities: programme_qualities(f.qualities),
                    expected_viewers: round(f.expected_viewers),
                    uncertainty: if f.relative_uncertainty >= 0.28 {
                        "Rough read"
                    } else if f.relative_uncertainty >= 0.20 {
                        "Better informed"
                    } else {
                        "Well researched"
                    },
                    groups: f
                        .groups
                        .into_iter()
                        .map(|(id, reach)| (self.audience_name(id), round(reach)))
                        .collect(),
                }
            })
            .collect();
        let news = self
            .planner
            .rights_news()
            .iter()
            .map(|event| MarketNews {
                programme: event.programme.0,
                tick: event.at.0,
                kind: event.kind,
                station: event.station.map(|station| station.0),
            })
            .collect();
        let spending = self
            .planner
            .rights_spending()
            .iter()
            .map(|entry| MarketSpend {
                programme: entry.programme.0,
                tick: entry.at.0,
                amount: entry.amount,
                balance: entry.balance,
            })
            .collect();
        RightsMarket {
            titles,
            news,
            assessment_ticks: ASSESSMENT_TICKS,
            spending,
        }
    }
}
