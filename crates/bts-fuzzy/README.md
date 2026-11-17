# bts-fuzzy

A standalone fuzzy inference crate developed alongside Behind the Screens.

Status: scaffold only. No evaluator or authoring macro is implemented yet.
The package name is provisional; publication is disabled.

## Boundary

This crate owns generic membership functions, conditions, rule definitions,
inference, validation, and structured evaluation traces. It must not depend
on the TV simulation, its content, its entity types, a UI, or a game clock.

The consuming application maps its state into inputs, evaluates rules,
and applies the resulting outputs. The evaluator itself does not mutate
the application's world.

Game rules and balance values live in the game, not in this crate.
Public examples and future tests should use generic examples such as
temperature control, not audience or advertising concepts.

## Planned initial scope

- Finite numeric inputs and validated membership degrees.
- Rising/falling ramps, triangular and trapezoidal membership curves.
- Nested AND, OR, and NOT expressions.
- Named rules, weights, and numerical outputs.
- One documented zero-order Sugeno evaluator with an explicit no-rule fallback.
- Structured traces of condition membership, activation, and output contributions.
- A thin declarative Rust macro constructing inspectable rule data.

This is a proposed implementation scope, not an existing API.
Precise operators, aggregation semantics, validation, and macro syntax must
be specified and tested before implementing game rules.

Do not add a parser, scripting runtime, other inference families, or framework
integrations without a concrete use case.

## Standalone use

The manifest is self-contained: it inherits no package metadata, dependencies,
or lint settings from the workspace. The crate has no sibling dependencies.

With a Rust toolchain installed:

```sh
cargo test --manifest-path crates/bts-fuzzy/Cargo.toml
cargo doc --manifest-path crates/bts-fuzzy/Cargo.toml --no-deps
```

The directory can later move to a separate repository without changing its
dependency boundary. Choose a public name, license, and release policy before
enabling publication; no license has been selected for this project yet.
