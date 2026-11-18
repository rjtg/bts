//! Audience-facing qualities shared by programmes, research and production.
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum ProductionType {
    Fiction,
    Nonfiction,
}

pub const EDITORIAL_RIGOUR: usize = 9;

pub const FEATURE_NAMES: [&str; 10] = [
    "Humour",
    "Playfulness",
    "Tension",
    "Warmth",
    "Intimacy",
    "Spectacle",
    "Pace",
    "Information depth",
    "Topical urgency",
    "Editorial rigour",
];

/// Values are unitless. Zero playfulness means a serious register.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct FeatureProfile {
    pub production_type: ProductionType,
    pub values: [Option<f64>; 10],
}

impl FeatureProfile {
    pub fn fiction(creative: [f64; 7]) -> Self {
        Self {
            production_type: ProductionType::Fiction,
            values: std::array::from_fn(|i| creative.get(i).copied()),
        }
    }

    pub fn nonfiction(values: [Option<f64>; 10]) -> Self {
        Self {
            production_type: ProductionType::Nonfiction,
            values,
        }
    }

    pub(crate) fn is_valid(self) -> bool {
        self.values.iter().any(Option::is_some)
            && self.values.iter().enumerate().all(|(i, value)| {
                value.is_none_or(|v| {
                    (i < 7 || self.production_type == ProductionType::Nonfiction)
                        && (0.0..=1.0).contains(&v)
                })
            })
    }

    pub fn fit(self, preferences: &[f64; 10]) -> f64 {
        let mut sum = 0.0;
        let mut count = 0;
        for (value, preferred) in self.values.iter().zip(preferences) {
            if let Some(value) = value {
                sum += 1.0 - (value - preferred).abs();
                count += 1;
            }
        }
        if count == 0 {
            0.0
        } else {
            sum / f64::from(count)
        }
    }

    pub(crate) fn weakest_fit(self, preferences: &[f64; 10]) -> f64 {
        self.values
            .iter()
            .zip(preferences)
            .filter_map(|(v, p)| v.map(|v| 1.0 - (v - p).abs()))
            .fold(1.0, f64::min)
    }

    pub fn genre(self) -> &'static str {
        let high = |i: usize| self.values[i].is_some_and(|v| v >= 0.65);
        match self.production_type {
            ProductionType::Fiction if high(0) => "Comedy",
            ProductionType::Fiction if high(2) => "Thriller",
            ProductionType::Fiction => "Drama",
            ProductionType::Nonfiction if high(8) && high(0) => "Late-night",
            ProductionType::Nonfiction if high(8) => "News",
            ProductionType::Nonfiction if high(7) => "Explainer",
            ProductionType::Nonfiction => "Magazine",
        }
    }
}

/// Skill in delivering each feature in an assigned role. Absent values mean
/// the role has no contribution on that dimension.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Capabilities(pub [Option<f64>; 10]);

impl Capabilities {
    pub fn discussion([humour, register, warmth, pace, depth]: [f64; 5]) -> Self {
        Self([
            Some(humour),
            Some(register),
            None,
            Some(warmth),
            None,
            None,
            Some(pace),
            Some(depth),
            None,
            None,
        ])
    }
}

/// Coarse player-facing description shared by observations and the catalogue.
pub fn feature_band(name: &str, value: f64) -> &'static str {
    match (name, value) {
        ("Playfulness", v) if v < 0.34 => "Serious",
        ("Playfulness", v) if v >= 0.67 => "Playful",
        ("Playfulness", _) => "Balanced",
        (_, v) if v < 0.34 => "Low",
        (_, v) if v >= 0.67 => "High",
        _ => "Moderate",
    }
}
