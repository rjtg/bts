//! Player protocol v4. These whitelisted DTOs deliberately do not serialize engine evidence.
use bts_sim::planning::production::{Subject, Tone};
use serde::{Deserialize, Serialize};

pub const API_VERSION: u32 = 4;
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
    Inspect {},
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
    BookProduction {
        production_id: u32,
        promise_request: Option<u16>,
        exceptional_person: Option<u16>,
        smooth_cooperation: bool,
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
    pub tick_minutes: u16,
    pub next_airing: Option<u32>,
    pub active_airing: Option<u32>,
    pub stations: Vec<StationInfo>,
    pub gaps: Vec<Gap>,
    pub finances: Finances,
    pub cash: String,
    pub production: ProductionView,
    pub catalogue: Vec<CatalogueEntry>,
    pub advertisers: Vec<Advertiser>,
    pub schedule: Vec<ScheduleEntry>,
    pub offer: Option<Quote>,
    pub contracts: Vec<Booking>,
    pub results: Vec<BroadcastReport>,
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
    pub tone: f64,
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
    pub start_tick: u32,
    pub end_tick: u32,
    pub proposed_advertiser: Option<u16>,
    pub booked: bool,
    pub status: &'static str,
    pub editable: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct StationInfo {
    pub id: u16,
    pub name: String,
    pub is_player: bool,
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
    pub production_committed: String,
    pub production_in_escrow: String,
    pub production_spent: String,
    pub ledger: Vec<FinancialLine>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FinancialLine {
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
    pub cooperation_cost: u32,
    pub sway_history: Vec<SwayHistoryInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PersonInfo {
    pub id: u16,
    pub name: String,
    pub role: &'static str,
    pub fee: String,
    pub description: String,
    pub normal_until_tick: u32,
    pub negotiable_from_tick: Option<u32>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductionInfo {
    pub id: u32,
    pub title: String,
    pub host: u16,
    pub guests: [u16; 2],
    pub subject: Subject,
    pub tone: Tone,
    pub budget: String,
    pub fees: String,
    pub record_at: u32,
    pub recording_end: u32,
    pub ready_at: u32,
    pub phase: &'static str,
    pub programme: Option<u16>,
    pub booked_at: Option<u32>,
    pub promised_request: Option<u16>,
    pub exceptional_person: Option<u16>,
    pub smooth_cooperation: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RequestInfo {
    pub id: u16,
    pub proposer: u16,
    pub reward: u32,
    pub achievement: &'static str,
    pub statement: String,
    pub promise: Option<PromiseInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PromiseInfo {
    pub production_id: u32,
    pub promised_at: u32,
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
