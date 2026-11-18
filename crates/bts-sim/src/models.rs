//! TV-specific, inspectable fuzzy models. No game rules live in bts-fuzzy.
use bts_fuzzy::{Condition, Degree, Evaluation, Membership, Rule, RuleSet};
use serde::Serialize;

/// Typed numerical features passed into TV rules.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub enum Feature {
    ProfileFit,
    WeakestFit,
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
fn paired(features: &[(Feature, f64)]) -> Vec<Rule<Feature>> {
    features
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
        .collect()
}

fn interaction(
    rules: &mut Vec<Rule<Feature>>,
    name: &str,
    condition: Condition<Feature>,
    weight: f64,
) {
    let weight = Degree::new(weight).unwrap();
    rules.push(
        Rule::new(
            format!("{name}: low"),
            condition.clone().negate(),
            weight,
            0.0,
        )
        .unwrap(),
    );
    rules.push(Rule::new(format!("{name}: high"), condition, weight, 1.0).unwrap());
}
impl Default for Models {
    fn default() -> Self {
        let mut attraction = paired(&[
            (Feature::ProfileFit, 0.35),
            (Feature::WeakestFit, 0.10),
            (Feature::Awareness, 0.30),
            (Feature::CastAppeal, 0.25),
        ]);
        interaction(
            &mut attraction,
            "cast with profile fit",
            high(Feature::ProfileFit).and(high(Feature::CastAppeal)),
            0.25,
        );
        let mut satisfaction = paired(&[
            (Feature::ProfileFit, 0.15),
            (Feature::WeakestFit, 0.15),
            (Feature::Execution, 0.70),
        ]);
        interaction(
            &mut satisfaction,
            "execution with profile fit",
            high(Feature::ProfileFit).and(high(Feature::Execution)),
            0.30,
        );
        let creative = high(Feature::ProductInterest)
            .and(high(Feature::Affordability).and(high(Feature::CreativeQuality)));
        let mut advertising = Vec::new();
        interaction(
            &mut advertising,
            "relevant creative",
            creative.clone(),
            0.75,
        );
        interaction(
            &mut advertising,
            "context supports creative",
            creative.and(high(Feature::Satisfaction)),
            0.25,
        );
        Self {
            attraction: RuleSet::new(attraction, 0.0).unwrap(),
            satisfaction: RuleSet::new(satisfaction, 0.0).unwrap(),
            advertising: RuleSet::new(advertising, 0.0).unwrap(),
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
        fit: f64,
        weakest: f64,
        awareness: f64,
        cast: f64,
    ) -> Result<ModelEvaluation, crate::Error> {
        evaluate(
            &self.attraction,
            vec![
                (Feature::ProfileFit, fit),
                (Feature::WeakestFit, weakest),
                (Feature::Awareness, awareness),
                (Feature::CastAppeal, cast),
            ],
        )
    }
    pub fn satisfaction(
        &self,
        fit: f64,
        weakest: f64,
        execution: f64,
    ) -> Result<ModelEvaluation, crate::Error> {
        evaluate(
            &self.satisfaction,
            vec![
                (Feature::ProfileFit, fit),
                (Feature::WeakestFit, weakest),
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
