//! TV-specific, inspectable fuzzy models. No game rules live in bts-fuzzy.
use bts_fuzzy::{Condition, Degree, Evaluation, Membership, Rule, RuleSet};
use serde::Serialize;

/// Typed numerical features passed into TV rules.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub enum Feature {
    GenreFit,
    ToneFit,
    Awareness,
    CastAppeal,
    Execution,
    ProductInterest,
    Affordability,
    CreativeQuality,
    Satisfaction,
}

/// Validated TV model definitions, shared across broadcasts.
#[derive(Clone, Debug)]
pub struct Models {
    attraction: RuleSet<Feature>,
    satisfaction: RuleSet<Feature>,
    advertising: RuleSet<Feature>,
}

fn high(key: Feature) -> Condition<Feature> {
    Condition::is(
        key,
        "high",
        Membership::rising(0.0, 1.0).expect("fixed valid curve"),
    )
}
fn paired(features: &[(Feature, f64)]) -> RuleSet<Feature> {
    let rules = features
        .iter()
        .flat_map(|(feature, weight)| {
            let weight = Degree::new(*weight).expect("fixed valid weight");
            [
                Rule::new(
                    format!("{feature:?}: low"),
                    high(*feature).negate(),
                    weight,
                    0.0,
                )
                .unwrap(),
                Rule::new(format!("{feature:?}: high"), high(*feature), weight, 1.0).unwrap(),
            ]
        })
        .collect();
    RuleSet::new(rules, 0.0).expect("finite fallback")
}

impl Default for Models {
    fn default() -> Self {
        let relevant = high(Feature::ProductInterest).and(
            high(Feature::Affordability)
                .and(high(Feature::CreativeQuality).or(high(Feature::Satisfaction))),
        );
        Self {
            attraction: paired(&[
                (Feature::GenreFit, 0.35),
                (Feature::ToneFit, 0.10),
                (Feature::Awareness, 0.30),
                (Feature::CastAppeal, 0.25),
            ]),
            satisfaction: paired(&[
                (Feature::GenreFit, 0.15),
                (Feature::ToneFit, 0.15),
                (Feature::Execution, 0.70),
            ]),
            advertising: RuleSet::new(
                vec![
                    Rule::new("ineffective", relevant.clone().negate(), Degree::ONE, 0.0).unwrap(),
                    Rule::new("relevant and effective", relevant, Degree::ONE, 1.0).unwrap(),
                ],
                0.0,
            )
            .unwrap(),
        }
    }
}

/// Model inputs alongside a complete fuzzy explanation. Weight sums are not confidence.
#[derive(Clone, Debug, PartialEq)]
pub struct ModelEvaluation {
    pub inputs: Vec<(Feature, f64)>,
    pub trace: Evaluation<Feature>,
}
fn evaluate(
    rules: &RuleSet<Feature>,
    inputs: Vec<(Feature, f64)>,
) -> Result<ModelEvaluation, crate::Error> {
    for (feature, value) in &inputs {
        crate::unit(*value, &format!("model input {feature:?}"))?;
    }
    let trace = rules.evaluate(&inputs)?;
    crate::unit(trace.output, "model output")?;
    Ok(ModelEvaluation { inputs, trace })
}
impl Models {
    pub fn attraction(
        &self,
        genre: f64,
        tone: f64,
        awareness: f64,
        cast: f64,
    ) -> Result<ModelEvaluation, crate::Error> {
        evaluate(
            &self.attraction,
            vec![
                (Feature::GenreFit, genre),
                (Feature::ToneFit, tone),
                (Feature::Awareness, awareness),
                (Feature::CastAppeal, cast),
            ],
        )
    }
    pub fn satisfaction(
        &self,
        genre: f64,
        tone: f64,
        execution: f64,
    ) -> Result<ModelEvaluation, crate::Error> {
        evaluate(
            &self.satisfaction,
            vec![
                (Feature::GenreFit, genre),
                (Feature::ToneFit, tone),
                (Feature::Execution, execution),
            ],
        )
    }
    pub fn advertising(
        &self,
        interest: f64,
        affordability: f64,
        creative: f64,
        satisfaction: f64,
    ) -> Result<ModelEvaluation, crate::Error> {
        evaluate(
            &self.advertising,
            vec![
                (Feature::ProductInterest, interest),
                (Feature::Affordability, affordability),
                (Feature::CreativeQuality, creative),
                (Feature::Satisfaction, satisfaction),
            ],
        )
    }
}
