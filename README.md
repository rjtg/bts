# Behind the Screens

A television station management game about understanding a changing audience and turning that understanding into successful television. Inspired by the management appeal of classic Mad TV, with deeper original production and a persistent simulated world.

## Direction

- Rust development; simulation mechanics come first.
- Simple presentation with an 80s retro / 8-bit identity; a TUI is a leading option.
- Hidden but learnable audience preferences, cultural trends, and talent careers.
- Multiple programme formats, original production, scheduling, and brand advertising.
- Scenario-specific ambitions rather than one universal victory condition; sandbox play is also intended.

## Core activity

Observe → acquire or produce → schedule and sell advertising → broadcast → interpret results.

The world supplies viewers, talent, stories, guests, and advertisers. Its detail must support decisions about television.

## Design documents

- [Game design](docs/design.md): scope, entities, relationships, formats, and scenarios.
- [Simulation](docs/simulation.md): causal mechanisms, hidden information, and verification principles.
- [First playable](docs/first-playable.md): a bounded proposal and acceptance criteria.
- [Open questions](docs/open-questions.md): unresolved decisions and experiments.
- [Architecture decisions](docs/architecture.md): crate boundaries and extraction plan.
- [Headless experiment](docs/headless-experiment.md): implemented provisional equations, demo commands, and evidence.
- [Contributor guidance](AGENTS.md): implementation principles for people and coding agents.

## Status

Design baseline and initial Cargo workspace, November 2026. The independent [`bts-fuzzy`](crates/bts-fuzzy/README.md) crate implements zero-order Sugeno inference, validated membership curves, nested conditions, traces, and an authoring macro. The first bounded headless broadcasting experiment is implemented in `bts-sim`, with a `bts-cli` comparison runner and developer JSON reports. The season loop and player interface are not implemented. Documents distinguish agreed direction from proposals and open questions; numerical examples are not balance commitments. No engine, UI framework, or AI model has been selected.

## Run the broadcasting experiment

```sh
cargo run -p bts-cli -- demo --json /tmp/bts-demo-results.json
cargo run -p bts-cli -- demo --case targeted --trace
cargo run -p bts-sim --example sugeno_semantics
```

The three fixed schedules start from identical state: broad popular programming,
a smaller specialist audience, and a promoted but disappointing series. Reports
are developer-only and include actual hidden inputs. Counts are expected
viewer-slot impressions, not cumulative unique reach. Coefficients are
provisional; hype does not represent sales or revenue.
