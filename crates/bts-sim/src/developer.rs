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
    pub elapsed_hours: u32,
    pub allocations: Vec<AllocationReport>,
    pub hype: Vec<HypeReport>,
}
impl From<&SlotEvent> for SlotReport {
    fn from(event: &SlotEvent) -> Self {
        Self {
            action: event.action.clone(),
            elapsed_hours: event.elapsed_hours,
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
    DeveloperReport {schema_version:1,audience_scope:"Developer only: actual inputs and internal causal evidence; viewer-slot impressions are not unique reach".into(),case:run.case.clone(),initial_world:run.world.clone(),parameters:run.parameters,initial_hype:5.0,initial_willingness:0.5,initial_repetition:0.0,initial_hour:SimHour(0),summary:Summary {stations,first_station_return:returns,brands},slots:run.events.iter().map(SlotReport::from).collect()}
}
