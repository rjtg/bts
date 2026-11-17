use bts_fuzzy::{condition, ConditionTrace, Degree, Membership, Rule, RuleSet};

#[derive(Clone, Debug, PartialEq)]
enum Input {
    Temperature,
    Humidity,
}

fn print_condition(trace: &ConditionTrace<Input>, depth: usize) {
    let indent = "  ".repeat(depth);
    match trace {
        ConditionTrace::Is {
            key,
            label,
            input,
            degree,
            ..
        } => println!("{indent}{key:?}={input:.1} is {label}: {:.3}", degree.get()),
        ConditionTrace::And(a, b, degree) | ConditionTrace::Or(a, b, degree) => {
            let operator = if matches!(trace, ConditionTrace::And(..)) {
                "AND"
            } else {
                "OR"
            };
            println!("{indent}{operator}: {:.3}", degree.get());
            print_condition(a, depth + 1);
            print_condition(b, depth + 1);
        }
        ConditionTrace::Not(child, degree) => {
            println!("{indent}NOT: {:.3}", degree.get());
            print_condition(child, depth + 1);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let warm = Membership::rising(18.0, 30.0)?;
    let comfortable = Membership::triangular(15.0, 22.0, 29.0)?;
    let humid = Membership::rising(40.0, 80.0)?;
    let dry = Membership::falling(30.0, 60.0)?;
    let rules = RuleSet::new(
        vec![
            Rule::new(
                "boost",
                condition!(and(
                    is(Input::Temperature, "warm", warm),
                    or(
                        is(Input::Humidity, "humid", humid),
                        not(is(Input::Humidity, "dry", dry))
                    )
                )),
                Degree::ONE,
                90.0,
            )?,
            Rule::new(
                "gentle",
                condition!(is(Input::Temperature, "comfortable", comfortable)),
                Degree::new(0.7)?,
                30.0,
            )?,
        ],
        10.0,
    )?;

    // Both temperature memberships overlap at 25°C. At 5°C neither rule fires.
    for temperature in [25.0, 5.0] {
        let result =
            rules.evaluate(&[(Input::Temperature, temperature), (Input::Humidity, 65.0)])?;
        println!("\nTemperature {temperature}°C, humidity 65%:");
        for rule in &result.rules {
            println!("{}: activation={:.3}, weight={:.3}, effective={:.3}, consequent={:.1}, contribution={:.3}",rule.name,rule.activation.get(),rule.weight.get(),rule.effective_weight,rule.consequent,rule.contribution);
            print_condition(&rule.condition, 1);
        }
        println!(
            "Ventilation={:.2}% (numerator={:.3}, denominator={:.3}, fallback={})",
            result.output,
            result.total_contribution,
            result.total_effective_weight,
            result.used_fallback
        );
    }
    Ok(())
}
