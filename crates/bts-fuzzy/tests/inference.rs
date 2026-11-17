use bts_fuzzy::{condition, Condition, ConditionTrace, Degree, Error, Membership, Rule, RuleSet};

#[derive(Clone, Debug, PartialEq)]
enum Input {
    Temperature,
    Humidity,
}

fn leaf(key: Input, label: &str) -> Condition<Input> {
    Condition::is(key, label, Membership::rising(0.0, 10.0).unwrap())
}
fn rule(name: &str, condition: Condition<Input>, weight: f64, output: f64) -> Rule<Input> {
    Rule::new(name, condition, Degree::new(weight).unwrap(), output).unwrap()
}
fn close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-12, "{actual} != {expected}");
}

#[test]
fn boundaries_shoulders_and_overlap() {
    let rising = Membership::rising(0.0, 10.0).unwrap();
    let falling = Membership::falling(0.0, 10.0).unwrap();
    for (input, expected) in [
        (-f64::MAX, 0.0),
        (0.0, 0.0),
        (2.5, 0.25),
        (5.0, 0.5),
        (10.0, 1.0),
        (f64::MAX, 1.0),
    ] {
        close(rising.degree(input).unwrap().get(), expected);
        close(falling.degree(input).unwrap().get(), 1.0 - expected);
    }
    let triangle = Membership::triangular(0.0, 5.0, 10.0).unwrap();
    let trapezoid = Membership::trapezoidal(0.0, 3.0, 7.0, 10.0).unwrap();
    for (input, expected) in [
        (-1.0, 0.0),
        (0.0, 0.0),
        (2.5, 0.5),
        (5.0, 1.0),
        (7.5, 0.5),
        (10.0, 0.0),
        (11.0, 0.0),
    ] {
        close(triangle.degree(input).unwrap().get(), expected);
    }
    for (input, expected) in [
        (-1.0, 0.0),
        (0.0, 0.0),
        (1.5, 0.5),
        (3.0, 1.0),
        (5.0, 1.0),
        (7.0, 1.0),
        (8.5, 0.5),
        (10.0, 0.0),
        (11.0, 0.0),
    ] {
        close(trapezoid.degree(input).unwrap().get(), expected);
    }
    let peaked = Membership::trapezoidal(0.0, 5.0, 5.0, 10.0).unwrap();
    for input in 0..=100 {
        let x = f64::from(input) / 10.0;
        assert_eq!(peaked.degree(x), triangle.degree(x));
        close(
            rising.degree(x).unwrap().get() + falling.degree(x).unwrap().get(),
            1.0,
        );
    }
}

#[test]
fn reject_invalid_degrees_curves_and_constants() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -0.01, 1.01] {
        assert!(Degree::new(value).is_err());
    }
    assert_eq!(Degree::new(0.0).unwrap(), Degree::ZERO);
    assert_eq!(Degree::new(1.0).unwrap(), Degree::ONE);
    for (a, b) in [
        (1.0, 1.0),
        (2.0, 1.0),
        (-f64::MAX, f64::MAX),
        (f64::NAN, 1.0),
        (0.0, f64::INFINITY),
    ] {
        assert!(Membership::rising(a, b).is_err());
        assert!(Membership::falling(a, b).is_err());
    }
    for (a, b, c) in [
        (0.0, 0.0, 1.0),
        (0.0, 1.0, 1.0),
        (2.0, 1.0, 3.0),
        (0.0, f64::NAN, 2.0),
    ] {
        assert!(Membership::triangular(a, b, c).is_err());
    }
    for (a, b, c, d) in [
        (0.0, 0.0, 1.0, 2.0),
        (0.0, 2.0, 1.0, 3.0),
        (0.0, 1.0, 2.0, 2.0),
        (0.0, 1.0, f64::NAN, 3.0),
    ] {
        assert!(Membership::trapezoidal(a, b, c, d).is_err());
    }
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(Rule::new("bad", leaf(Input::Temperature, "warm"), Degree::ONE, value).is_err());
        assert!(RuleSet::<Input>::new(vec![], value).is_err());
        assert!(Membership::rising(0.0, 1.0).unwrap().degree(value).is_err());
    }
}

#[test]
fn nested_conditions_preserve_every_membership_and_grouping() {
    let expression = leaf(Input::Temperature, "warm")
        .and(leaf(Input::Humidity, "humid").or(leaf(Input::Temperature, "warm again").negate()));
    let set = RuleSet::new(vec![rule("nested", expression, 1.0, 42.0)], 0.0).unwrap();
    let result = set
        .evaluate(&[(Input::Temperature, 8.0), (Input::Humidity, 1.0)])
        .unwrap();
    close(result.rules[0].activation.get(), 0.2); // min(.8, max(.1, 1-.8))
    let ConditionTrace::And(left, right, _) = &result.rules[0].condition else {
        panic!("AND trace missing")
    };
    close(left.degree().get(), 0.8);
    let ConditionTrace::Or(humid, complement, _) = right.as_ref() else {
        panic!("OR trace missing")
    };
    close(humid.degree().get(), 0.1);
    assert!(matches!(complement.as_ref(), ConditionTrace::Not(..)));
    let ConditionTrace::Is {
        key, label, input, ..
    } = left.as_ref()
    else {
        panic!("leaf trace missing")
    };
    assert_eq!(*key, Input::Temperature);
    assert_eq!(label, "warm");
    assert_eq!(*input, 8.0);
}

#[test]
fn weighted_average_and_trace_accounting() {
    let set = RuleSet::new(
        vec![
            rule("first", leaf(Input::Temperature, "warm"), 0.5, 10.0),
            rule("second", leaf(Input::Humidity, "humid"), 1.0, 30.0),
        ],
        -5.0,
    )
    .unwrap();
    let inputs = [(Input::Temperature, 5.0), (Input::Humidity, 7.5)];
    let trace = set.evaluate(&inputs).unwrap();
    assert_eq!(trace.output, 25.0);
    assert_eq!(trace.total_effective_weight, 1.0);
    assert_eq!(trace.total_contribution, 25.0);
    assert_eq!(trace.rules[0].effective_weight, 0.25);
    assert_eq!(trace.rules[0].contribution, 2.5);
    assert_eq!(trace.rules[1].contribution, 22.5);
    assert!(!trace.used_fallback);
    assert_eq!(trace, set.evaluate(&inputs).unwrap());
    assert_eq!(set.rules()[0].name(), "first");
    assert_eq!(set.rules()[0].weight(), Degree::new(0.5).unwrap());
    assert_eq!(set.rules()[0].consequent(), 10.0);
    assert_eq!(set.fallback(), -5.0);
}

#[test]
fn empty_inactive_and_zero_weight_sets_use_explicit_fallback() {
    let empty = RuleSet::<Input>::new(vec![], 17.0)
        .unwrap()
        .evaluate(&[])
        .unwrap();
    assert!(empty.used_fallback);
    assert_eq!(empty.output, 17.0);
    for (input, weight) in [(0.0, 1.0), (10.0, 0.0)] {
        let trace = RuleSet::new(
            vec![rule("off", leaf(Input::Temperature, "warm"), weight, 99.0)],
            -7.0,
        )
        .unwrap()
        .evaluate(&[(Input::Temperature, input)])
        .unwrap();
        assert!(trace.used_fallback);
        assert_eq!(trace.output, -7.0);
        assert_eq!(trace.total_effective_weight, 0.0);
    }
}

#[test]
fn inputs_are_validated_eagerly_with_useful_errors() {
    let set = RuleSet::new(
        vec![rule("zero", leaf(Input::Temperature, "warm"), 0.0, 1.0)],
        0.0,
    )
    .unwrap();
    assert_eq!(
        set.evaluate(&[]).unwrap_err(),
        Error::MissingInput("Temperature".into())
    );
    let error = set
        .evaluate(&[(Input::Temperature, 1.0), (Input::Humidity, f64::NAN)])
        .unwrap_err();
    assert!(error.to_string().contains("Humidity"));
    assert!(matches!(
        set.evaluate(&[(Input::Temperature, 1.0), (Input::Temperature, 2.0)]),
        Err(Error::DuplicateInput(_))
    ));
    for expression in [
        leaf(Input::Temperature, "cold").and(leaf(Input::Humidity, "missing")),
        leaf(Input::Temperature, "warm").or(leaf(Input::Humidity, "missing")),
    ] {
        let set = RuleSet::new(vec![rule("eager", expression, 1.0, 1.0)], 0.0).unwrap();
        for temperature in [0.0, 10.0] {
            assert!(matches!(
                set.evaluate(&[(Input::Temperature, temperature)]),
                Err(Error::MissingInput(_))
            ));
        }
    }
}

#[test]
fn numerical_extremes_overflow_underflow_and_signed_outputs() {
    let tiny = f64::from_bits(1);
    let tiny_curve = Membership::rising(0.0, tiny).unwrap();
    assert_eq!(tiny_curve.degree(tiny).unwrap(), Degree::ONE);
    let negative = RuleSet::new(
        vec![rule(
            "negative",
            leaf(Input::Temperature, "warm"),
            1.0,
            -f64::MAX,
        )],
        0.0,
    )
    .unwrap();
    assert_eq!(
        negative
            .evaluate(&[(Input::Temperature, 10.0)])
            .unwrap()
            .output,
        -f64::MAX
    );
    let overflow = RuleSet::new(
        vec![
            rule("a", leaf(Input::Temperature, "warm"), 1.0, f64::MAX),
            rule("b", leaf(Input::Temperature, "warm"), 1.0, f64::MAX),
        ],
        0.0,
    )
    .unwrap();
    let error = overflow
        .evaluate(&[(Input::Temperature, 10.0)])
        .unwrap_err();
    assert!(matches!(error, Error::NonFinite(_)));
    assert!(error.to_string().contains("contribution sum"));
    let underflow = RuleSet::new(
        vec![rule("tiny", leaf(Input::Temperature, "warm"), tiny, 1.0)],
        9.0,
    )
    .unwrap();
    assert!(
        underflow
            .evaluate(&[(Input::Temperature, 5.0)])
            .unwrap()
            .used_fallback
    );
    let cancellation = RuleSet::new(
        vec![
            rule("positive", leaf(Input::Temperature, "warm"), 1.0, 10.0),
            rule("negative", leaf(Input::Temperature, "warm"), 1.0, -10.0),
        ],
        9.0,
    )
    .unwrap();
    let result = cancellation
        .evaluate(&[(Input::Temperature, 10.0)])
        .unwrap();
    assert_eq!(result.output, 0.0);
    assert!(!result.used_fallback);
}

// Integration tests compile as a separate crate: this also checks exported macro
// hygiene without importing Condition at the expansion site.
mod external_macro {
    #[test]
    fn equivalent_to_ordinary_api() {
        use super::{leaf, rule, Input};
        use bts_fuzzy as renamed;
        let curve = renamed::Membership::rising(0.0, 10.0).unwrap();
        let authored = renamed::condition!(and(
            is(Input::Temperature, "warm", curve),
            or(
                is(Input::Humidity, "humid", curve),
                not(is(Input::Temperature, "cool", curve))
            )
        ));
        let ordinary = leaf(Input::Temperature, "warm")
            .and(leaf(Input::Humidity, "humid").or(leaf(Input::Temperature, "cool").negate()));
        assert_eq!(authored, ordinary);
        let a = renamed::RuleSet::new(vec![rule("same", authored, 0.5, 25.0)], 3.0).unwrap();
        let b = renamed::RuleSet::new(vec![rule("same", ordinary, 0.5, 25.0)], 3.0).unwrap();
        for (t, h) in [(0.0, 0.0), (2.0, 8.0), (10.0, 5.0)] {
            let inputs = [(Input::Temperature, t), (Input::Humidity, h)];
            assert_eq!(a.evaluate(&inputs), b.evaluate(&inputs));
        }
    }
}

#[test]
fn macro_accepts_expressions_and_evaluates_them_once() {
    let mut calls = 0;
    let expression = condition!(is(
        Input::Temperature,
        {
            calls += 1;
            "warm"
        },
        Membership::rising(0.0, 10.0).unwrap()
    ));
    assert_eq!(calls, 1);
    assert_eq!(expression, leaf(Input::Temperature, "warm"));
}

#[test]
fn external_rule_macro_matches_api_and_validation() {
    use bts_fuzzy as fuzzy;
    let curve = fuzzy::Membership::rising(0.0, 10.0).unwrap();
    let authored = fuzzy::rule_set! {
        fallback: 3.0;
        "nested" => and(is(Input::Temperature,"warm",curve),not(is(Input::Humidity,"humid",curve))) => 25.0, weight: Degree::new(0.5).unwrap();
        "other" => is(Input::Humidity,"humid",curve) => 50.0, weight: Degree::ONE;
    }.unwrap();
    let ordinary = RuleSet::new(
        vec![
            rule(
                "nested",
                leaf(Input::Temperature, "warm").and(leaf(Input::Humidity, "humid").negate()),
                0.5,
                25.0,
            ),
            rule("other", leaf(Input::Humidity, "humid"), 1.0, 50.0),
        ],
        3.0,
    )
    .unwrap();
    assert_eq!(authored, ordinary);
    for inputs in [
        [(Input::Temperature, 5.0), (Input::Humidity, 2.0)],
        [(Input::Temperature, 0.0), (Input::Humidity, 0.0)],
    ] {
        assert_eq!(authored.evaluate(&inputs), ordinary.evaluate(&inputs));
    }
    let invalid = fuzzy::rule_set! {
        fallback: 0.0;
        "invalid" => is(Input::Temperature,"warm",curve) => f64::NAN, weight: Degree::ONE;
    };
    assert!(matches!(invalid, Err(Error::NonFinite(_))));
    let invalid_fallback: Result<RuleSet<Input>, _> = fuzzy::rule_set! { fallback: f64::INFINITY; };
    assert!(matches!(invalid_fallback, Err(Error::NonFinite(_))));
    let empty: RuleSet<Input> = fuzzy::rule_set! { fallback: 7.0; }.unwrap();
    assert_eq!(empty.evaluate(&[]).unwrap().output, 7.0);
}
