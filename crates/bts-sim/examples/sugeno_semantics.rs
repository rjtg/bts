//! Focused regression: activation is a relative weight, not an output multiplier.
use bts_fuzzy::{Condition, Degree, Membership, Rule, RuleSet};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let high = Condition::is((), "high", Membership::rising(0.0, 1.0)?);
    let lone = RuleSet::new(
        vec![Rule::new("high only", high.clone(), Degree::ONE, 1.0)?],
        0.0,
    )?;
    let paired = RuleSet::new(
        vec![
            Rule::new("low", high.clone().negate(), Degree::ONE, 0.0)?,
            Rule::new("high", high, Degree::ONE, 1.0)?,
        ],
        0.0,
    )?;
    println!("input | high-only output | complementary low/high output");
    for x in [0.0, 0.25, 0.5, 0.75, 1.0] {
        let a = lone.evaluate(&[((), x)])?;
        let b = paired.evaluate(&[((), x)])?;
        println!("{x:.2}  | {:.2}             | {:.2}", a.output, b.output);
        assert_eq!(a.output, if x == 0.0 { 0.0 } else { 1.0 });
        assert_eq!(b.output, x);
    }
    Ok(())
}
