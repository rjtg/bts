//! Player protocol v8. These whitelisted DTOs deliberately do not serialize engine evidence.
use bts_sim::planning::production::{Subject, Tone};
use serde::{Deserialize, Serialize};

pub const API_VERSION: u32 = 8;
pub const REPORTING: &str = "Provisional start-of-airing expected counts, not measured ratings or unique reach. Contract forecasts are provisional. One ad exposure opportunity per airing; delivery units are millionths of a relevant impression.";

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub api_version: u32,
    pub command: Command,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Command {
    PreviewWeekly {
        plan: WeeklyInput,
    },
    PreviewWeeklyEdit {
        airing_id: u32,
        time: u8,
    },
    ConfirmWeekly {},
    BuyRights {
        programme: u16,
        token: String,
    },
    AssessTitle {
        programme: u16,
        insight: u32,
    },
    ListResale {
        programme: u16,
    },
    AcceptResale {
        programme: u16,
        token: String,
    },
    ResizeAdBlock {
        airing_id: u32,
        minutes: u8,
    },
    RequestMinuteOffers {
        airing_id: u32,
    },
    AcceptMinuteOffer {
        airing_id: u32,
        advertiser: u16,
        token: String,
    },
    PitchFilm {
        title: String,
        subject: Subject,
        ambition: [f64; 7],
    },
    WriteFilm {
        film_id: u32,
        writer: u16,
        focus: Option<u8>,
    },
    AcceptScript {
        film_id: u32,
    },
    ReviewFilm {
        film_id: u32,
        booking: FilmBookingInput,
    },
    BookFilm {
        film_id: u32,
        booking: FilmBookingInput,
    },
    SetPolling {
        polling: bts_sim::planning::research::Polling,
    },
    SpendInsight {
        audience: u16,
        amount: u32,
    },
    Inspect {},
    AcceptRequest {
        request_id: u16,
    },
    CreateProduction {
        concept: ProductionConceptInput,
    },
    ReviseProduction {
        production_id: u32,
        concept: ProductionConceptInput,
    },
    DiscardProduction {
        production_id: u32,
    },
    ReviewProduction {
        production_id: u32,
        exceptional_person: Option<u16>,
        cooperation_sway: u32,
    },
    BookProduction {
        production_id: u32,
        exceptional_person: Option<u16>,
        cooperation_sway: u32,
    },
    Place {
        start_tick: u32,
        programme: u16,
        advertiser: Option<u16>,
    },
    Move {
        airing_id: u32,
        start_tick: u32,
    },
    Remove {
        airing_id: u32,
    },
    SetAdvertiser {
        airing_id: u32,
        advertiser: Option<u16>,
    },
    RequestOffer {
        airing_id: u32,
    },
    AcceptOffer {
        airing_id: u32,
        offer_id: String,
    },
    Advance {
        to_tick: u32,
    },
    Reset {},
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PlayerError {
    pub code: &'static str,
    pub message: &'static str,
}
impl PlayerError {
    pub(crate) fn new(code: &'static str, message: &'static str) -> Self {
        Self { code, message }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Response {
    Ok {
        api_version: u32,
        report: Box<PlayerReport>,
    },
    Error {
        api_version: u32,
        error: PlayerError,
    },
}

/// All money, contract IDs and integer delivery units are decimal strings.
/// No u64 crosses the JavaScript boundary as an IEEE-754 number.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PlayerReport {
    pub reporting: &'static str,
    pub now_tick: u32,
    pub end_tick: u32,
    pub first_weekday: u8,
    pub weekly_review: Option<WeeklyReview>,
    pub tick_minutes: u16,
    pub next_airing: Option<u32>,
    pub active_airing: Option<u32>,
    pub stations: Vec<StationInfo>,
    pub gaps: Vec<Gap>,
    pub finances: Finances,
    pub cash: String,
    pub research: Option<bts_sim::planning::research::ResearchView>,
    pub production: ProductionView,
    pub film_desk: FilmDesk,
    pub rights_market: RightsMarket,
    pub film_review: Option<FilmReview>,
    pub booking_review: Option<ProductionBookingReview>,
    pub catalogue: Vec<CatalogueEntry>,
    pub advertisers: Vec<Advertiser>,
    pub schedule: Vec<ScheduleEntry>,
    pub offer: Option<Quote>,
    pub minute_offers: Option<MinuteOffers>,
    pub minute_sales: Vec<MinuteBooking>,
    pub contracts: Vec<Booking>,
    pub results: Vec<BroadcastReport>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Quality {
    pub name: &'static str,
    pub level: &'static str,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CatalogueEntry {
    pub production_id: Option<u32>,
    pub id: u16,
    pub name: String,
    pub genre: String,
    pub kind: String,
    pub series_name: Option<String>,
    pub episode: Option<u32>,
    pub production_type: bts_sim::ProductionType,
    pub qualities: Vec<Quality>,
    pub awareness: f64,
    pub cast_appeal: f64,
    pub broadcast_cost: String,
    pub duration_minutes: u16,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Advertiser {
    pub id: u16,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScheduleEntry {
    pub airing_id: u32,
    pub station: u16,
    pub programme: u16,
    pub programme_name: String,
    pub genre: String,
    pub duration_minutes: u16,
    pub start_tick: u32,
    pub end_tick: u32,
    pub proposed_advertiser: Option<u16>,
    pub ad_minutes: u8,
    pub sold_minutes: u8,
    pub weekly_id: Option<u32>,
    pub booked: bool,
    pub status: &'static str,
    pub editable: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct StationInfo {
    pub id: u16,
    pub name: String,
    pub is_player: bool,
    pub staff: Vec<String>,
    pub owned_studios: Option<u16>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Gap {
    pub start_tick: u32,
    pub end_tick: u32,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Finances {
    pub opening_cash: String,
    pub costs_paid: String,
    pub revenue: String,
    pub rights_spent: String,
    pub resale_receipts: String,
    pub production_committed: String,
    pub production_in_escrow: String,
    pub production_spent: String,
    pub ledger: Vec<FinancialLine>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FinancialLine {
    pub rights_title: Option<u16>,
    pub production_id: Option<u32>,
    pub tick: u32,
    pub airing_id: Option<u32>,
    pub kind: &'static str,
    pub amount: String,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Quote {
    pub offer_id: String,
    pub advertiser: u16,
    pub airing_id: u32,
    pub start_tick: u32,
    pub end_tick: u32,
    pub deadline_tick: u32,
    pub maximum_payment: String,
    pub targets: Vec<Commitment>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Commitment {
    pub audience: String,
    pub forecast_relevant_delivery: f64,
    pub promised_units: String,
    pub allocated_payment: String,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Booking {
    pub airing_id: u32,
    pub contract_id: String,
    pub advertiser: u16,
    pub deadline_tick: u32,
    pub maximum_payment: String,
    pub targets: Vec<Commitment>,
    pub settlement: Option<ContractResult>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ContractResult {
    pub payment: String,
    pub refund: String,
    pub targets: Vec<TargetResult>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TargetResult {
    pub audience: String,
    pub promised_units: String,
    pub credited_units: String,
    pub allocated_payment: String,
    pub payment: String,
    pub refund: String,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BroadcastReport {
    pub airing_id: u32,
    pub start_tick: u32,
    pub end_tick: u32,
    pub snapshot_tick: u32,
    pub aired_programme: Option<u16>,
    pub aired_advertiser: Option<u16>,
    pub off_air: bool,
    pub broadcast_cost: String,
    pub cash_after: String,
    pub audiences: Vec<AudienceReport>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AudienceReport {
    pub audience: String,
    pub population: u32,
    pub player_viewers: f64,
    pub rival_viewers: f64,
    pub not_watching: f64,
    pub film_reception: Option<&'static str>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductionConceptInput {
    pub title: String,
    pub host: u16,
    pub guests: [u16; 2],
    pub subject: Subject,
    pub tone: Tone,
    pub budget: String,
    pub record_at: u32,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductionView {
    pub format: &'static str,
    pub duration_minutes: u16,
    pub recording_ticks: u32,
    pub post_ticks: u32,
    pub people: Vec<PersonInfo>,
    pub projects: Vec<ProductionInfo>,
    pub requests: Vec<RequestInfo>,
    pub sway_balance: u32,
    pub participation_cost: u32,
    pub known_concerns: Vec<KnownConcern>,
    pub sway_history: Vec<SwayHistoryInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PersonInfo {
    pub id: u16,
    pub name: String,
    pub role: bts_sim::planning::production::Role,
    pub fee: String,
    pub description: String,
    pub normal_until_tick: u32,
    pub negotiable_from_tick: Option<u32>,
    pub station_id: Option<u16>,
    pub commitments: Vec<TalentCommitment>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductionInfo {
    pub id: u32,
    pub title: String,
    pub host: u16,
    pub guests: [u16; 2],
    pub subject: Subject,
    pub tone: Tone,
    pub ambition: Vec<Quality>,
    pub budget: String,
    pub fees: String,
    pub record_at: u32,
    pub recording_end: u32,
    pub ready_at: u32,
    pub phase: &'static str,
    pub programme: Option<u16>,
    pub booked_at: Option<u32>,
    pub studio: Option<u16>,
    pub quest_hints: Vec<QuestHint>,
    pub exceptional_person: Option<u16>,
    pub cooperation_sway: u32,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RequestInfo {
    pub id: u16,
    pub proposer: u16,
    pub reward: u32,
    pub achievement: &'static str,
    pub statement: String,
    pub time_limit_ticks: u32,
    pub status: &'static str,
    pub acceptance: Option<QuestAcceptanceInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct QuestAcceptanceInfo {
    pub production_id: Option<u32>,
    pub accepted_at: u32,
    pub deadline_tick: u32,
    pub fulfilled_at: Option<u32>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SwayHistoryInfo {
    pub tick: u32,
    pub production_id: u32,
    pub change: i32,
    pub balance: u32,
    pub kind: &'static str,
    pub request_id: Option<u16>,
    pub person_id: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct KnownConcern {
    pub people: [u16; 2],
    pub note: String,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TalentCommitment {
    pub production_id: u32,
    pub title: String,
    pub start_tick: u32,
    pub end_tick: u32,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductionBookingReview {
    pub production_id: u32,
    pub exceptional_person: Option<u16>,
    pub cooperation_sway: u32,
    pub studio: u16,
    pub total_sway: u32,
    pub sway_remaining: u32,
    pub budget: String,
    pub cash_remaining: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct QuestHint {
    pub request_id: u16,
    pub matches_requirements: bool,
    pub can_finish_in_time: bool,
    pub achievement_tick: u32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FilmBookingInput {
    pub cast: [u16; 2],
    pub director: u16,
    pub studio: u8,
    pub start: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FilmReview {
    pub id: u32,
    pub cast: [u16; 2],
    pub director: u16,
    pub studio: u8,
    pub start: u32,
    pub recording_end: u32,
    pub ready: u32,
    pub fees: String,
    pub rent: String,
    pub total: String,
    pub studio_shortfall: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FilmDesk {
    pub projects: Vec<FilmCard>,
    pub people: Vec<FilmPerson>,
    pub studios: Vec<FilmStudio>,
    pub roles: [&'static str; 2],
    pub writing_ticks: u32,
    pub revision_ticks: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FilmCard {
    pub id: u32,
    pub title: String,
    pub subject: Subject,
    pub phase: bts_sim::planning::film::FilmPhase,
    pub committed: String,
    pub writer: Option<u16>,
    pub due: Option<u32>,
    pub script: Option<FilmScript>,
    pub revised: bool,
    pub booking: Option<FilmReview>,
    pub programme: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FilmScript {
    pub qualities: Vec<Quality>,
    pub weaknesses: Vec<(u8, &'static str)>,
    pub studio_demand: &'static str,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FilmPerson {
    pub id: u16,
    pub name: &'static str,
    pub job: bts_sim::planning::film::FilmJob,
    pub fee: String,
    pub revision_fee: String,
    pub specialty: &'static str,
    pub available_until: u32,
    pub commitments: Vec<(String, u32, u32)>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FilmStudio {
    pub id: u8,
    pub name: &'static str,
    pub fee: String,
    pub summary: &'static str,
    pub commitments: Vec<(String, u32, u32)>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MinuteOffers {
    pub airing_id: u32,
    pub token: String,
    pub offers: Vec<MinuteOfferView>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MinuteOfferView {
    pub advertiser: u16,
    pub minutes: u8,
    pub price: String,
    pub expected_target_viewers: f64,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MinuteBooking {
    pub airing_id: u32,
    pub advertiser: u16,
    pub minutes: u8,
    pub price: String,
    pub settlement: Option<MinuteSettlementView>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MinuteSettlementView {
    pub aired_minutes: u8,
    pub payment: String,
    pub refund: String,
    pub viewer_minutes: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RightsMarket {
    pub titles: Vec<MarketTitle>,
    pub news: Vec<MarketNews>,
    pub assessment_ticks: u32,
    pub spending: Vec<MarketSpend>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MarketTitle {
    pub programme: u16,
    pub name: String,
    pub status: bts_sim::planning::rights::TitleStatus,
    pub owner: Option<u16>,
    pub evidence: &'static str,
    pub art: &'static str,
    pub duration: u16,
    pub broadcast_cost: String,
    pub acquisition: Option<MarketOffer>,
    pub resale: Option<MarketOffer>,
    pub blocker: Option<u32>,
    pub rival_airing: Option<u32>,
    pub assessment_due: Option<u32>,
    pub assessment_at: Option<u32>,
    pub qualities: Vec<Quality>,
    pub expected_viewers: u64,
    pub uncertainty: &'static str,
    pub groups: Vec<(String, u64)>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MarketOffer {
    pub token: String,
    pub price: String,
    pub deadline: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MarketNews {
    pub programme: u16,
    pub tick: u32,
    pub kind: &'static str,
    pub station: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MarketSpend {
    pub programme: u16,
    pub tick: u32,
    pub amount: u32,
    pub balance: u32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WeeklyInput {
    pub programme: u16,
    pub weekday: u8,
    pub time: u8,
    pub count: u8,
    pub ad_minutes: u8,
    pub exceptions: Vec<bts_sim::planning::weekly::Exception>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WeeklyReview {
    pub edition: bool,
    pub rows: Vec<bts_sim::planning::weekly::Placement>,
}
