#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::fmt;

/// A validation or evaluation failure, with its numerical or input context.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// A value or arithmetic intermediate was not finite.
    NonFinite(String),
    /// A degree was outside the closed unit interval.
    InvalidDegree(f64),
    /// Curve endpoints were unordered or their span overflowed.
    InvalidCurve(&'static str),
    /// A referenced input was absent (debug representation of its key).
    MissingInput(String),
    /// A supplied input key occurred more than once.
    DuplicateInput(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite(context) => write!(f, "non-finite value: {context}"),
            Self::InvalidDegree(value) => write!(f, "degree must be in [0, 1], got {value}"),
            Self::InvalidCurve(context) => write!(f, "invalid membership curve: {context}"),
            Self::MissingInput(key) => write!(f, "missing input: {key}"),
            Self::DuplicateInput(key) => write!(f, "duplicate input: {key}"),
        }
    }
}
impl std::error::Error for Error {}

fn finite(value: f64, context: impl FnOnce() -> String) -> Result<f64, Error> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(Error::NonFinite(context()))
    }
}

/// A finite membership degree or rule weight in `[0, 1]`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Degree(f64);

impl Degree {
    /// No membership.
    pub const ZERO: Self = Self(0.0);
    /// Full membership.
    pub const ONE: Self = Self(1.0);

    /// Validate a degree; NaN, infinities, and out-of-range values are errors.
    pub fn new(value: f64) -> Result<Self, Error> {
        finite(value, || "degree".into())?;
        if (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(Error::InvalidDegree(value))
        }
    }

    /// Return the validated numerical value.
    pub fn get(self) -> f64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Shape {
    Rising(f64, f64),
    Falling(f64, f64),
    Triangle(f64, f64, f64),
    Trapezoid(f64, f64, f64, f64),
}

/// A validated piecewise-linear membership function.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Membership(Shape);

fn span(start: f64, end: f64) -> Result<(), Error> {
    finite(start, || "curve endpoint".into())?;
    finite(end, || "curve endpoint".into())?;
    if start >= end || !(end - start).is_finite() {
        return Err(Error::InvalidCurve(
            "endpoints must increase with a finite positive span",
        ));
    }
    Ok(())
}

impl Membership {
    /// A rising ramp, with `start < end`.
    pub fn rising(start: f64, end: f64) -> Result<Self, Error> {
        span(start, end)?;
        Ok(Self(Shape::Rising(start, end)))
    }

    /// A falling ramp, with `start < end`.
    pub fn falling(start: f64, end: f64) -> Result<Self, Error> {
        span(start, end)?;
        Ok(Self(Shape::Falling(start, end)))
    }

    /// A triangle, with `left < peak < right`.
    pub fn triangular(left: f64, peak: f64, right: f64) -> Result<Self, Error> {
        span(left, peak)?;
        span(peak, right)?;
        Ok(Self(Shape::Triangle(left, peak, right)))
    }

    /// A trapezoid, with `left < plateau_start <= plateau_end < right`.
    pub fn trapezoidal(
        left: f64,
        plateau_start: f64,
        plateau_end: f64,
        right: f64,
    ) -> Result<Self, Error> {
        span(left, plateau_start)?;
        span(plateau_end, right)?;
        if plateau_start > plateau_end {
            return Err(Error::InvalidCurve("plateau endpoints must not decrease"));
        }
        Ok(Self(Shape::Trapezoid(
            left,
            plateau_start,
            plateau_end,
            right,
        )))
    }

    /// Evaluate a finite input, rejecting non-finite arithmetic intermediates.
    pub fn degree(self, input: f64) -> Result<Degree, Error> {
        finite(input, || "membership input".into())?;
        fn rising(x: f64, a: f64, b: f64) -> Result<f64, Error> {
            if x <= a {
                return Ok(0.0);
            }
            if x >= b {
                return Ok(1.0);
            }
            let numerator = finite(x - a, || "ramp numerator".into())?;
            finite(numerator / (b - a), || "ramp interpolation".into())
        }
        let value = match self.0 {
            Shape::Rising(a, b) => rising(input, a, b)?,
            Shape::Falling(a, b) => 1.0 - rising(input, a, b)?,
            Shape::Triangle(a, b, c) => {
                if input <= b {
                    rising(input, a, b)?
                } else {
                    1.0 - rising(input, b, c)?
                }
            }
            Shape::Trapezoid(a, b, c, d) => {
                if input < b {
                    rising(input, a, b)?
                } else if input <= c {
                    1.0
                } else {
                    1.0 - rising(input, c, d)?
                }
            }
        };
        Degree::new(value)
    }
}

/// Inspectable, explicitly grouped condition data. Every branch is evaluated.
#[derive(Clone, Debug, PartialEq)]
pub enum Condition<K> {
    /// A labelled membership test on a typed input key.
    Is {
        /// Input key.
        key: K,
        /// Human-readable membership label.
        label: String,
        /// Validated curve.
        membership: Membership,
    },
    /// Minimum of two child degrees.
    And(Box<Self>, Box<Self>),
    /// Maximum of two child degrees.
    Or(Box<Self>, Box<Self>),
    /// Complement of a child degree.
    Not(Box<Self>),
}

impl<K> Condition<K> {
    /// Construct a labelled leaf membership test.
    pub fn is(key: K, label: impl Into<String>, membership: Membership) -> Self {
        Self::Is {
            key,
            label: label.into(),
            membership,
        }
    }
    /// Explicitly group an AND expression.
    pub fn and(self, other: Self) -> Self {
        Self::And(Box::new(self), Box::new(other))
    }
    /// Explicitly group an OR expression.
    pub fn or(self, other: Self) -> Self {
        Self::Or(Box::new(self), Box::new(other))
    }
    /// Explicitly group a NOT expression.
    pub fn negate(self) -> Self {
        Self::Not(Box::new(self))
    }
}

/// A named rule with a validated weight and finite constant consequent.
#[derive(Clone, Debug, PartialEq)]
pub struct Rule<K> {
    name: String,
    condition: Condition<K>,
    weight: Degree,
    consequent: f64,
}
impl<K> Rule<K> {
    /// Construct a rule, rejecting a non-finite consequent.
    pub fn new(
        name: impl Into<String>,
        condition: Condition<K>,
        weight: Degree,
        consequent: f64,
    ) -> Result<Self, Error> {
        let name = name.into();
        finite(consequent, || format!("consequent of rule {name:?}"))?;
        Ok(Self {
            name,
            condition,
            weight,
            consequent,
        })
    }
    /// Rule name; names are labels and need not be unique.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Inspect the expression tree.
    pub fn condition(&self) -> &Condition<K> {
        &self.condition
    }
    /// Configured rule weight.
    pub fn weight(&self) -> Degree {
        self.weight
    }
    /// Constant numerical consequent.
    pub fn consequent(&self) -> f64 {
        self.consequent
    }
}

/// A complete evaluated expression tree, preserving child order.
#[derive(Clone, Debug, PartialEq)]
pub enum ConditionTrace<K> {
    /// A membership observation.
    Is {
        /// Input key.
        key: K,
        /// Membership label.
        label: String,
        /// Observed finite input.
        input: f64,
        /// Curve used.
        membership: Membership,
        /// Resulting degree.
        degree: Degree,
    },
    /// AND children and their minimum.
    And(Box<Self>, Box<Self>, Degree),
    /// OR children and their maximum.
    Or(Box<Self>, Box<Self>, Degree),
    /// NOT child and its complement.
    Not(Box<Self>, Degree),
}
impl<K> ConditionTrace<K> {
    /// The evaluated degree at this node.
    pub fn degree(&self) -> Degree {
        match self {
            Self::Is { degree, .. }
            | Self::And(_, _, degree)
            | Self::Or(_, _, degree)
            | Self::Not(_, degree) => *degree,
        }
    }
}

/// Per-rule explanation of the weighted-average calculation.
#[derive(Clone, Debug, PartialEq)]
pub struct RuleTrace<K> {
    /// Rule label.
    pub name: String,
    /// Complete condition evaluation.
    pub condition: ConditionTrace<K>,
    /// Root condition degree.
    pub activation: Degree,
    /// Configured rule weight.
    pub weight: Degree,
    /// Activation times configured weight.
    pub effective_weight: f64,
    /// Constant consequent.
    pub consequent: f64,
    /// Effective weight times consequent (unnormalized numerator term).
    pub contribution: f64,
}

/// Evaluation result and sufficient data to inspect its calculation.
#[derive(Clone, Debug, PartialEq)]
pub struct Evaluation<K> {
    /// Ordered rule traces.
    pub rules: Vec<RuleTrace<K>>,
    /// Sum of effective weights.
    pub total_effective_weight: f64,
    /// Sum of raw contributions.
    pub total_contribution: f64,
    /// Final weighted average or explicit fallback.
    pub output: f64,
    /// Whether total effective weight was exactly zero.
    pub used_fallback: bool,
}

/// An ordered zero-order Sugeno rule set with an explicit finite fallback.
#[derive(Clone, Debug, PartialEq)]
pub struct RuleSet<K> {
    rules: Vec<Rule<K>>,
    fallback: f64,
}
impl<K> RuleSet<K> {
    /// Construct a rule set. An empty set is valid and always uses fallback.
    pub fn new(rules: Vec<Rule<K>>, fallback: f64) -> Result<Self, Error> {
        finite(fallback, || "fallback".into())?;
        Ok(Self { rules, fallback })
    }
    /// Inspect rules in evaluation order.
    pub fn rules(&self) -> &[Rule<K>] {
        &self.rules
    }
    /// Configured fallback.
    pub fn fallback(&self) -> f64 {
        self.fallback
    }
}

impl<K: Clone + PartialEq + fmt::Debug> RuleSet<K> {
    /// Evaluate without modifying inputs or application state.
    /// All supplied values are checked before any rule is evaluated.
    pub fn evaluate(&self, inputs: &[(K, f64)]) -> Result<Evaluation<K>, Error> {
        for (index, (key, value)) in inputs.iter().enumerate() {
            finite(*value, || format!("input {key:?}"))?;
            if inputs[..index].iter().any(|(previous, _)| previous == key) {
                return Err(Error::DuplicateInput(format!("{key:?}")));
            }
        }
        let mut result = Evaluation {
            rules: Vec::with_capacity(self.rules.len()),
            total_effective_weight: 0.0,
            total_contribution: 0.0,
            output: self.fallback,
            used_fallback: false,
        };
        for rule in &self.rules {
            let condition = evaluate_condition(&rule.condition, inputs)?;
            let activation = condition.degree();
            let effective_weight = finite(activation.get() * rule.weight.get(), || {
                format!("effective weight of rule {:?}", rule.name)
            })?;
            let contribution = finite(effective_weight * rule.consequent, || {
                format!("contribution of rule {:?}", rule.name)
            })?;
            result.total_effective_weight =
                finite(result.total_effective_weight + effective_weight, || {
                    format!("weight sum at rule {:?}", rule.name)
                })?;
            result.total_contribution = finite(result.total_contribution + contribution, || {
                format!("contribution sum at rule {:?}", rule.name)
            })?;
            result.rules.push(RuleTrace {
                name: rule.name.clone(),
                condition,
                activation,
                weight: rule.weight,
                effective_weight,
                consequent: rule.consequent,
                contribution,
            });
        }
        result.used_fallback = result.total_effective_weight == 0.0;
        if !result.used_fallback {
            result.output = finite(
                result.total_contribution / result.total_effective_weight,
                || "weighted-average output".into(),
            )?;
        }
        Ok(result)
    }
}

fn evaluate_condition<K: Clone + PartialEq + fmt::Debug>(
    condition: &Condition<K>,
    inputs: &[(K, f64)],
) -> Result<ConditionTrace<K>, Error> {
    Ok(match condition {
        Condition::Is {
            key,
            label,
            membership,
        } => {
            let input = inputs
                .iter()
                .find(|(candidate, _)| candidate == key)
                .map(|(_, value)| *value)
                .ok_or_else(|| Error::MissingInput(format!("{key:?}")))?;
            ConditionTrace::Is {
                key: key.clone(),
                label: label.clone(),
                input,
                membership: *membership,
                degree: membership.degree(input)?,
            }
        }
        Condition::And(left, right) | Condition::Or(left, right) => {
            let left = Box::new(evaluate_condition(left, inputs)?);
            let right = Box::new(evaluate_condition(right, inputs)?);
            let a = left.degree().get();
            let b = right.degree().get();
            if matches!(condition, Condition::And(..)) {
                ConditionTrace::And(left, right, Degree::new(a.min(b))?)
            } else {
                ConditionTrace::Or(left, right, Degree::new(a.max(b))?)
            }
        }
        Condition::Not(child) => {
            let child = Box::new(evaluate_condition(child, inputs)?);
            let degree = Degree::new(1.0 - child.degree().get())?;
            ConditionTrace::Not(child, degree)
        }
    })
}

/// Construct explicitly grouped condition data using the ordinary public API.
/// See the crate documentation for examples and the recursive grammar.
#[macro_export]
macro_rules! condition {
    (is($key:expr, $label:expr, $membership:expr)) => {
        $crate::Condition::is($key, $label, $membership)
    };
    (and($left:ident($($a:tt)*), $right:ident($($b:tt)*))) => {
        $crate::condition!($left($($a)*)).and($crate::condition!($right($($b)*)))
    };
    (or($left:ident($($a:tt)*), $right:ident($($b:tt)*))) => {
        $crate::condition!($left($($a)*)).or($crate::condition!($right($($b)*)))
    };
    (not($child:ident($($args:tt)*))) => {
        $crate::condition!($child($($args)*)).negate()
    };
}

/// Construct a validated rule set, returning `Result<RuleSet<_>, Error>`.
/// Weights are [`Degree`] values; the macro delegates all validation to constructors.
///
/// ```
/// use bts_fuzzy::{rule_set, Degree, Membership};
/// let warm = Membership::rising(18.0, 28.0)?;
/// let rules = rule_set! {
///     fallback: 10.0;
///     "ventilate" => is(0, "warm", warm) => 80.0, weight: Degree::ONE;
/// }?;
/// assert_eq!(rules.evaluate(&[(0, 23.0)])?.output, 80.0);
/// # Ok::<(), bts_fuzzy::Error>(())
/// ```
#[macro_export]
macro_rules! rule_set {
    (fallback: $fallback:expr; $(
        $name:expr => $condition:ident($($args:tt)*) => $consequent:expr, weight: $weight:expr;
    )*) => {{
        let fallback = $fallback;
        let rules = ::std::vec![$(
            $crate::Rule::new($name, $crate::condition!($condition($($args)*)), $weight, $consequent)
        ),*];
        match rules.into_iter().collect::<::std::result::Result<::std::vec::Vec<_>, $crate::Error>>() {
            ::std::result::Result::Ok(rules) => $crate::RuleSet::new(rules, fallback),
            ::std::result::Result::Err(error) => ::std::result::Result::Err(error),
        }
    }};
}
