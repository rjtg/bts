# bts-fuzzy

Small, dependency-free, independently reusable zero-order Sugeno inference.
Implemented: validated degrees, finite inputs, four membership curves, nested
conditions, named weighted rules, structured traces, and `condition!`/`rule_set!` authoring.
Publication remains disabled; licensing and the provisional name are unchanged.

## Precise semantics

- Degrees and rule weights are finite `f64` values in `[0, 1]`.
- Inputs, curve parameters, constant consequents, and the required fallback are
  finite `f64` values. Every supplied input is checked, including unused ones;
  duplicate keys and missing referenced keys are errors.
- Rising ramps are 0 at/below `start`, 1 at/above `end`, linear between;
  falling ramps are their complement. Require `start < end`.
- Triangles require `left < peak < right`: 0 outside/at the outer endpoints,
  1 at the peak, linear on either side. Trapezoids require
  `left < plateau_start <= plateau_end < right`: 1 on the closed plateau,
  0 outside/at the outer endpoints, linear on the shoulders. Degenerate
  shoulders are rejected; equal plateau endpoints are allowed.
- AND = minimum, OR = maximum, NOT = `1 - degree`. Binary expressions use
  explicit grouping; there is no implicit precedence. Every branch is evaluated
  in order, including inactive rules and branches; missing inputs never hide
  behind short-circuiting or zero weights.
- Effective rule weight = activation × rule weight. Contribution = effective
  weight × constant consequent. Output = sum(contributions) / sum(effective
  weights), in supplied rule order. If the total effective weight is exactly
  zero (including floating-point underflow), return the explicit fallback.
- All arithmetic intermediates and the result must remain finite. Overflow is
  an error, even if a rearranged mathematical expression could be finite.
  In particular, curve spans must be finite and positive. Normal IEEE-754
  rounding and underflow apply; no epsilon threshold or output clamping is used.

Evaluation is read-only and deterministic for the same ordered rules/inputs on
the same floating-point platform. It uses no time, randomness, or filesystem I/O.
It returns owned traces, including the entire condition tree, leaf memberships,
activation, effective weight, raw contribution, totals, output, and fallback use.
Keys need only `Clone + PartialEq + Debug`; enums work without string attributes.
Input lookup is linear, intended for small rule sets.

## Ordinary API and macro

```rust
use bts_fuzzy::{condition, Condition, Degree, Membership, Rule, RuleSet};
#[derive(Clone, Debug, PartialEq)]
enum Input { Temperature }
let warm = Membership::rising(18.0, 28.0)?;
let ordinary = Condition::is(Input::Temperature, "warm", warm);
let authored = condition!(is(Input::Temperature, "warm", warm));
assert_eq!(ordinary, authored);
let rules = RuleSet::new(vec![
    Rule::new("ventilate", ordinary, Degree::new(0.8)?, 80.0)?,
], 10.0)?;
let trace = rules.evaluate(&[(Input::Temperature, 23.0)])?;
assert_eq!(trace.output, 80.0);
# Ok::<(), bts_fuzzy::Error>(())
```

`condition!` is a thin data constructor. Its grammar is recursively
`is(key, label, curve)`, `and(condition, condition)`, `or(condition, condition)`,
`not(condition)`. For example:
`condition!(and(is(key, "warm", curve), not(is(key, "cool", other_curve))))`.
`rule_set!` additionally constructs named rules and the rule set, returning
`Result<RuleSet<_>, Error>` through the same validated constructors:

```rust
use bts_fuzzy::{rule_set, Degree, Membership};
let warm = Membership::rising(18.0, 28.0)?;
let rules = rule_set! {
    fallback: 10.0;
    "ventilate" => is(0, "warm", warm) => 80.0, weight: Degree::ONE;
}?;
assert_eq!(rules.evaluate(&[(0, 23.0)])?.output, 80.0);
# Ok::<(), bts_fuzzy::Error>(())
```
See `cargo run --example ventilation` for a complete example with overlapping
memberships, nested conditions, multiple rules, fallback, and readable traces.

## Verification and standalone use

From this directory run:

```sh
cargo fmt --all -- --check
cargo test
cargo clippy --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
cargo run --example ventilation
```

The manifest inherits nothing from the workspace and has no dependencies.
Copy this directory outside the workspace and run the same commands unchanged.
CI checks both workspace and copied-crate use, including external macro use.

Local validation on Rust/Cargo 1.98.1 passed 10 integration tests and 3 doctests,
formatting, Clippy with warnings denied, documentation with warnings denied,
and the runnable example. The same checks passed offline after copying the
crate outside the workspace. LSP diagnostics were unavailable (server exit,
then timeout); compiler and Cargo checks supplied semantic verification.
The GitHub Actions workflow is added but has not yet run on GitHub.

## Limits and future proposals

This is a small first API, not a stability or publication commitment. It has
no parser, serialization, inference families beyond zero-order Sugeno,
application state updates, or framework integrations. Recursive condition trees
and allocated traces suit small models; very deep/untrusted trees and large
rule sets are not resource-bounded. Those extensions need concrete use cases.
Game-specific rules belong in consumers; no game implementation is provided.
