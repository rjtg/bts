//! Player protocol v1. These whitelisted DTOs deliberately do not serialize engine evidence.
use serde::{Deserialize, Serialize};

pub const API_VERSION: u32 = 1;
pub const REPORTING: &str = "Provisional exact expected audience counts; not measured ratings. Delivery units are millionths of a relevant impression.";

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
    Plan {
        slot: u32,
        programme: u16,
        advertiser: Option<u16>,
    },
    RequestOffer {},
    AcceptOffer {
        offer_id: String,
    },
    Advance {},
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
    pub next_slot: Option<usize>,
    pub cash: String,
    pub catalogue: Vec<CatalogueEntry>,
    pub advertisers: Vec<Advertiser>,
    pub schedule: Vec<ScheduleEntry>,
    pub offer: Option<Quote>,
    pub contracts: Vec<Booking>,
    pub results: Vec<BroadcastReport>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CatalogueEntry {
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
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Advertiser {
    pub id: u16,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScheduleEntry {
    pub slot: usize,
    pub day: usize,
    pub hour: u32,
    pub programme: Option<u16>,
    pub proposed_advertiser: Option<u16>,
    pub rival_programme: u16,
    pub locked: bool,
    pub completed: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Quote {
    pub offer_id: String,
    pub advertiser: u16,
    pub slot: usize,
    pub deadline_hour: u32,
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
    pub contract_id: String,
    pub advertiser: u16,
    pub deadline_hour: u32,
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
    pub slot: usize,
    pub at_hour: u32,
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
