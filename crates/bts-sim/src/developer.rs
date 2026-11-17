//! Developer-only report construction: never a player observation or save format.
//! Conversions borrow immutable events and never touch simulation state.
use crate::{
    demo::ExperimentRun,
    models::{Feature, ModelEvaluation},
    *,
};
use bts_fuzzy::ConditionTrace;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "operator")]
pub enum ConditionReport {
    Membership {
        key: Feature,
        label: String,
        input: f64,
        degree: f64,
    },
    And {
        degree: f64,
        children: Vec<Self>,
    },
    Or {
        degree: f64,
        children: Vec<Self>,
    },
    Not {
        degree: f64,
        child: Box<Self>,
    },
}
impl From<&ConditionTrace<Feature>> for ConditionReport {
    fn from(value: &ConditionTrace<Feature>) -> Self {
        match value {
            ConditionTrace::Is {
                key,
                label,
                input,
                degree,
                ..
            } => Self::Membership {
                key: *key,
                label: label.clone(),
                input: *input,
                degree: degree.get(),
            },
            ConditionTrace::And(a, b, d) => Self::And {
                degree: d.get(),
                children: vec![Self::from(a.as_ref()), Self::from(b.as_ref())],
            },
            ConditionTrace::Or(a, b, d) => Self::Or {
                degree: d.get(),
                children: vec![Self::from(a.as_ref()), Self::from(b.as_ref())],
            },
            ConditionTrace::Not(c, d) => Self::Not {
                degree: d.get(),
                child: Box::new(Self::from(c.as_ref())),
            },
        }
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct RuleReport {
    pub name: String,
    pub condition: ConditionReport,
    pub activation: f64,
    pub configured_weight: f64,
    pub effective_weight: f64,
    pub consequent: f64,
    pub contribution: f64,
}
#[derive(Clone, Debug, Serialize)]
pub struct FuzzyReport {
    pub inputs: Vec<(Feature, f64)>,
    pub rules: Vec<RuleReport>,
    pub total_effective_weight: f64,
    pub total_contribution: f64,
    pub output: f64,
    pub used_fallback: bool,
}
impl From<&ModelEvaluation> for FuzzyReport {
    fn from(model: &ModelEvaluation) -> Self {
        Self {
            inputs: model.inputs.clone(),
            rules: model
                .trace
                .rules
                .iter()
                .map(|r| RuleReport {
                    name: r.name.clone(),
                    condition: ConditionReport::from(&r.condition),
                    activation: r.activation.get(),
                    configured_weight: r.weight.get(),
                    effective_weight: r.effective_weight,
                    consequent: r.consequent,
                    contribution: r.contribution,
                })
                .collect(),
            total_effective_weight: model.trace.total_effective_weight,
            total_contribution: model.trace.total_contribution,
            output: model.trace.output,
            used_fallback: model.trace.used_fallback,
        }
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct ViewingReport {
    pub station: StationId,
    pub programme: ProgrammeId,
    pub series: SeriesId,
    pub attraction: FuzzyReport,
    pub satisfaction: FuzzyReport,
    pub utility: f64,
    pub expected_viewers: f64,
    pub willingness_before: f64,
    pub willingness_after: f64,
}
#[derive(Clone, Debug, Serialize)]
pub struct AllocationReport {
    pub audience: AudienceId,
    pub population: u32,
    pub available: f64,
    pub available_not_watching: f64,
    pub total_not_watching: f64,
    pub viewing: Vec<ViewingReport>,
}
#[derive(Clone, Debug, Serialize)]
pub struct AdvertisingReport {
    pub station: StationId,
    pub expected_exposures: f64,
    pub effectiveness: FuzzyReport,
}
#[derive(Clone, Debug, Serialize)]
pub struct HypeReport {
    pub brand: BrandId,
    pub audience: AudienceId,
    pub before: f64,
    pub after_decay: f64,
    pub repetition_before: f64,
    pub repetition_after_decay: f64,
    pub expected_exposures: f64,
    pub relevant_exposure_per_person: f64,
    pub campaign_generated: f64,
    pub after: f64,
    pub repetition_after: f64,
    pub advertisements: Vec<AdvertisingReport>,
}
#[derive(Clone, Debug, Serialize)]
pub struct SlotReport {
    pub action: Slot,
    pub aired: Slot,
    pub off_air: Vec<OffAir>,
    pub elapsed_hours: u32,
    pub allocations: Vec<AllocationReport>,
    pub hype: Vec<HypeReport>,
    pub financial: crate::economy::FinancialEvent,
}
impl From<&SlotEvent> for SlotReport {
    fn from(event: &SlotEvent) -> Self {
        Self {
            action: event.action.clone(),
            aired: event.aired.clone(),
            off_air: event.off_air.clone(),
            elapsed_hours: event.elapsed_hours,
            financial: event.financial.clone(),
            allocations: event
                .allocations
                .iter()
                .map(|a| AllocationReport {
                    audience: a.audience,
                    population: a.population,
                    available: a.available,
                    available_not_watching: a.available_not_watching,
                    total_not_watching: a.not_watching,
                    viewing: a
                        .viewing
                        .iter()
                        .map(|v| ViewingReport {
                            station: v.station,
                            programme: v.programme,
                            series: v.series,
                            attraction: (&v.attraction).into(),
                            satisfaction: (&v.satisfaction).into(),
                            utility: v.utility,
                            expected_viewers: v.viewers,
                            willingness_before: v.loyalty_before,
                            willingness_after: v.loyalty_after,
                        })
                        .collect(),
                })
                .collect(),
            hype: event
                .hype
                .iter()
                .map(|h| HypeReport {
                    brand: h.brand,
                    audience: h.audience,
                    before: h.before,
                    after_decay: h.decayed,
                    repetition_before: h.repetition_before,
                    repetition_after_decay: h.repetition_decayed,
                    expected_exposures: h.exposures,
                    relevant_exposure_per_person: h.relevant_exposure_per_person,
                    campaign_generated: h.campaign_generated,
                    after: h.after,
                    repetition_after: h.repetition_after,
                    advertisements: h
                        .advertisements
                        .iter()
                        .map(|a| AdvertisingReport {
                            station: a.station,
                            expected_exposures: a.viewers,
                            effectiveness: (&a.effectiveness).into(),
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct StationSummary {
    pub station: StationId,
    pub viewer_slot_impressions: f64,
    pub mean_satisfaction: Option<f64>,
}
#[derive(Clone, Debug, Serialize)]
pub struct ReturnSummary {
    pub audience: AudienceId,
    pub first_early_viewers: f64,
    pub last_early_viewers: f64,
    pub initial_willingness: f64,
    pub final_willingness: f64,
}
#[derive(Clone, Debug, Serialize)]
pub struct BrandSummary {
    pub brand: BrandId,
    pub mean_hype_after_broadcasts: f64,
    pub mean_hype_after_cooling: f64,
    pub specialist_hype_after_broadcasts: f64,
}
#[derive(Clone, Debug, Serialize)]
pub struct Summary {
    pub stations: Vec<StationSummary>,
    pub first_station_return: Vec<ReturnSummary>,
    pub brands: Vec<BrandSummary>,
}
#[derive(Clone, Debug, Serialize)]
pub struct DeveloperReport {
    pub schema_version: u32,
    pub audience_scope: String,
    pub case: String,
    pub initial_world: World,
    pub parameters: Parameters,
    pub initial_hype: f64,
    pub initial_willingness: f64,
    pub initial_repetition: f64,
    pub initial_hour: SimHour,
    pub summary: Summary,
    pub slots: Vec<SlotReport>,
}

fn viewing(event: &SlotEvent, audience: AudienceId, station: StationId) -> Option<&Viewing> {
    event
        .allocations
        .iter()
        .find(|a| a.audience == audience)
        .and_then(|a| a.viewing.iter().find(|v| v.station == station))
}

/// Summarize the bounded demo. Does not serialize internal state or imply unique reach.
pub fn report(run: &ExperimentRun) -> DeveloperReport {
    let active: Vec<_> = run
        .events
        .iter()
        .filter(|e| !e.action.broadcasts.is_empty())
        .collect();
    let stations = run
        .world
        .stations
        .iter()
        .map(|s| {
            let views: Vec<_> = active
                .iter()
                .flat_map(|e| &e.allocations)
                .flat_map(|a| &a.viewing)
                .filter(|v| v.station == s.id)
                .collect();
            let total = views.iter().map(|v| v.viewers).sum::<f64>();
            StationSummary {
                station: s.id,
                viewer_slot_impressions: total,
                mean_satisfaction: if total > 0.0 {
                    Some(
                        views
                            .iter()
                            .map(|v| v.viewers * v.satisfaction.trace.output)
                            .sum::<f64>()
                            / total,
                    )
                } else {
                    None
                },
            }
        })
        .collect();
    let early: Vec<_> = active.iter().filter(|e| e.action.at.0 % 24 == 18).collect();
    let mut returns = Vec::new();
    if let (Some(first), Some(last), Some(final_active), Some(station)) = (
        early.first(),
        early.last(),
        active.last(),
        run.world.stations.first(),
    ) {
        for audience in &run.world.audiences {
            if let (Some(a), Some(b), Some(final_view)) = (
                viewing(first, audience.id, station.id),
                viewing(last, audience.id, station.id),
                viewing(final_active, audience.id, station.id),
            ) {
                returns.push(ReturnSummary {
                    audience: audience.id,
                    first_early_viewers: a.viewers,
                    last_early_viewers: b.viewers,
                    initial_willingness: a.loyalty_before,
                    final_willingness: final_view.loyalty_after,
                });
            }
        }
    }
    let total_population = run
        .world
        .audiences
        .iter()
        .map(|a| f64::from(a.population))
        .sum::<f64>();
    let brands = run
        .world
        .brands
        .iter()
        .map(|brand| {
            let mean = |event: Option<&&SlotEvent>| {
                event
                    .map(|event| {
                        event
                            .hype
                            .iter()
                            .filter(|h| h.brand == brand.id)
                            .map(|h| {
                                let pop = run
                                    .world
                                    .audiences
                                    .iter()
                                    .find(|a| a.id == h.audience)
                                    .unwrap()
                                    .population;
                                h.after * f64::from(pop) / total_population
                            })
                            .sum::<f64>()
                    })
                    .unwrap_or(5.0)
            };
            BrandSummary {
                brand: brand.id,
                mean_hype_after_broadcasts: mean(active.last()),
                mean_hype_after_cooling: run
                    .world
                    .audiences
                    .iter()
                    .map(|a| {
                        run.final_state.hype(brand.id, a.id).unwrap() * f64::from(a.population)
                            / total_population
                    })
                    .sum(),
                specialist_hype_after_broadcasts: active
                    .last()
                    .and_then(|e| {
                        e.hype
                            .iter()
                            .find(|h| h.brand == brand.id && h.audience == demo::SPECIALISTS)
                    })
                    .map(|h| h.after)
                    .unwrap_or(5.0),
            }
        })
        .collect();
    DeveloperReport {schema_version:3,audience_scope:"Developer only: actual inputs and internal causal evidence; viewer-slot impressions are not unique reach".into(),case:run.case.clone(),initial_world:run.world.clone(),parameters:run.parameters,initial_hype:5.0,initial_willingness:0.5,initial_repetition:0.0,initial_hour:SimHour(0),summary:Summary {stations,first_station_return:returns,brands},slots:run.events.iter().map(SlotReport::from).collect()}
}

#[derive(Clone, Debug, Serialize)]
pub struct ForecastAudienceReport {
    pub audience: AudienceId,
    pub priority: f64,
    pub expected_viewers: f64,
    pub effectiveness: Option<FuzzyReport>,
    pub relevant_delivery: f64,
}
#[derive(Clone, Debug, Serialize)]
pub struct OfferReport {
    pub brand: BrandId,
    pub station: StationId,
    pub proposal: Slot,
    pub deadline: SimHour,
    pub demand: crate::economy::Demand,
    pub audiences: Vec<ForecastAudienceReport>,
    pub forecast_aired: Slot,
    pub forecast_off_air: Vec<OffAir>,
    pub forecast_relevant_delivery: f64,
    pub value_weighted_delivery: f64,
    pub valuation: Vec<crate::economy::AudienceValuation>,
    pub promised_whole_delivery: u64,
    pub rate_per_thousand: crate::economy::Money,
    pub maximum_offer: crate::economy::Money,
    pub uncapped_price: f64,
    pub payment: crate::economy::Money,
}
impl From<&crate::economy::Offer> for OfferReport {
    fn from(offer: &crate::economy::Offer) -> Self {
        Self {
            brand: offer.brand,
            station: offer.station,
            proposal: offer.proposal().clone(),
            deadline: offer.deadline,
            demand: offer.demand.clone(),
            audiences: offer
                .forecast
                .audiences
                .iter()
                .map(|a| ForecastAudienceReport {
                    audience: a.audience,
                    priority: a.priority,
                    expected_viewers: a.viewers,
                    effectiveness: a.effectiveness.as_ref().map(FuzzyReport::from),

                    relevant_delivery: a.relevant_delivery,
                })
                .collect(),
            forecast_aired: offer.forecast.aired.clone(),
            forecast_off_air: offer.forecast.off_air.clone(),
            value_weighted_delivery: offer.value_weighted_delivery,
            valuation: offer.valuation.clone(),
            forecast_relevant_delivery: offer.forecast.relevant_delivery,
            promised_whole_delivery: offer.promised_delivery,
            rate_per_thousand: offer.rate_per_thousand,
            maximum_offer: offer.maximum_offer,
            uncapped_price: offer.uncapped_price,
            payment: offer.payment,
        }
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct ContractReport {
    pub id: crate::economy::ContractId,
    pub offer: OfferReport,
    pub actual: crate::economy::ActualDelivery,
    pub delivered_at: Option<SimHour>,
    pub status: crate::economy::ContractStatus,
}
#[derive(Clone, Debug, Serialize)]
pub struct StationFinance {
    pub station: StationId,
    pub initial_cash: crate::economy::Money,
    pub cash: crate::economy::Money,
    pub revenue: u128,
    pub broadcast_costs: u128,
    pub profit: i128,
}
#[derive(Clone, Debug, Serialize)]
pub struct BrandFinance {
    pub brand: BrandId,
    pub initial_budget: crate::economy::Money,
    pub remaining_budget: crate::economy::Money,
    pub unsettled_escrow: u128,
    pub spent: u128,
}
#[derive(Clone, Debug, Serialize)]
pub struct FinancialReport {
    pub stations: Vec<StationFinance>,
    pub brands: Vec<BrandFinance>,
    pub cost_sink: crate::economy::Money,
    pub total_money: u128,
    pub ledger: Vec<crate::economy::LedgerEntry>,
}
#[derive(Clone, Debug, Serialize)]
pub struct EconomyReport {
    pub schema_version: u32,
    pub kind: String,
    pub brand: BrandId,
    pub broadcasts: DeveloperReport,
    pub offers: Vec<OfferReport>,
    pub accepted: Vec<Option<crate::economy::ContractId>>,
    pub recovery_quote: OfferReport,
    pub before_cooling_quote: OfferReport,
    pub demand_after_campaigns: crate::economy::Demand,
    pub contracts: Vec<ContractReport>,
    pub finances: FinancialReport,
}
pub fn economy_report(run: &crate::economy_demo::Run) -> EconomyReport {
    use crate::economy::{Account, ContractStatus};
    let book = run.experiment.final_state.economy();
    let stations = run
        .experiment
        .world
        .stations
        .iter()
        .map(|s| {
            let cash = book.cash(s.id).unwrap();
            StationFinance {
                station: s.id,
                initial_cash: s.initial_cash,
                cash,
                revenue: book
                    .ledger()
                    .iter()
                    .filter(|e| e.to == Account::Station(s.id))
                    .map(|e| u128::from(e.amount.0))
                    .sum(),
                broadcast_costs: book
                    .ledger()
                    .iter()
                    .filter(|e| e.from == Account::Station(s.id))
                    .map(|e| u128::from(e.amount.0))
                    .sum(),
                profit: i128::from(cash.0) - i128::from(s.initial_cash.0),
            }
        })
        .collect();
    let brands = run
        .experiment
        .world
        .brands
        .iter()
        .map(|b| {
            let remaining_budget = book.budget(b.id).unwrap();
            let unsettled_escrow = book
                .contracts()
                .filter(|c| c.offer.brand == b.id && matches!(c.status, ContractStatus::Reserved))
                .map(|c| u128::from(c.offer.payment.0))
                .sum::<u128>();
            BrandFinance {
                brand: b.id,
                initial_budget: b.marketing_budget,
                remaining_budget,
                unsettled_escrow,
                spent: u128::from(b.marketing_budget.0)
                    - u128::from(remaining_budget.0)
                    - unsettled_escrow,
            }
        })
        .collect();
    EconomyReport {
        schema_version: 3,
        kind: "advertising_economy_developer_only".into(),
        brand: run.brand,
        broadcasts: report(&run.experiment),
        offers: run.rounds.iter().map(|r| (&r.offer).into()).collect(),
        accepted: run.rounds.iter().map(|r| r.accepted).collect(),
        recovery_quote: (&run.recovery).into(),
        before_cooling_quote: (&run.before_cooling_quote).into(),
        demand_after_campaigns: run.demand_after_campaigns.clone(),
        contracts: book
            .contracts()
            .map(|c| ContractReport {
                id: c.id,
                offer: (&c.offer).into(),
                actual: c.actual.clone(),
                delivered_at: c.delivered_at,
                status: c.status.clone(),
            })
            .collect(),
        finances: FinancialReport {
            stations,
            brands,
            cost_sink: book.cost_sink(),
            total_money: book.total_money(),
            ledger: book.ledger().to_vec(),
        },
    }
}

/// Developer-only matched-state valuation counterfactual, not an empirical estimate.
#[derive(Clone, Debug, Serialize)]
pub struct AdvertiserValueReport {
    pub schema_version: u32,
    pub kind: String,
    pub case: String,
    pub initial_world: World,
    pub parameters: Parameters,
    pub initial_hype: f64,
    pub initial_repetition: f64,
    pub offer: OfferReport,
    pub slot: SlotReport,
    pub campaign_generated_hype: f64,
    pub payment: crate::economy::Money,
    pub credits_per_hype_point: Option<f64>,
    pub post_campaign_demand: crate::economy::Demand,
    pub next_quote: OfferReport,
    pub ledger: Vec<crate::economy::LedgerEntry>,
    pub total_money: u128,
}
pub fn advertiser_value_report(
    case: &crate::advertiser_value::CaseResult,
) -> AdvertiserValueReport {
    let campaign_generated_hype = case
        .event
        .hype
        .iter()
        .map(|h| h.campaign_generated)
        .sum::<f64>();
    let payment = case.event.financial.settlements[0].payment;
    AdvertiserValueReport {
        schema_version: 3,
        kind: "advertiser_value_developer_counterfactual".into(),
        case: case.name.clone(),
        initial_world: case.world.clone(),
        parameters: case.parameters,
        initial_hype: case.initial_hype,
        initial_repetition: case.initial_repetition,
        offer: (&case.offer).into(),
        slot: (&case.event).into(),
        campaign_generated_hype,
        payment,
        credits_per_hype_point: if campaign_generated_hype > 0.0 {
            Some(payment.0 as f64 / campaign_generated_hype)
        } else {
            None
        },
        post_campaign_demand: case.post_campaign_demand.clone(),
        next_quote: (&case.next_quote).into(),
        ledger: case.final_state.economy().ledger().to_vec(),
        total_money: case.final_state.economy().total_money(),
    }
}
