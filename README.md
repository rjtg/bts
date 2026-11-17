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
- [Contributor guidance](AGENTS.md): implementation principles for people and coding agents.

## Status

Design baseline and initial Cargo workspace, November 2026. The independent [`bts-fuzzy`](crates/bts-fuzzy/README.md) crate implements zero-order Sugeno inference, validated membership curves, nested conditions, traces, and an authoring macro. No game implementation exists yet. Documents distinguish agreed direction from proposals and open questions; numerical examples are not balance commitments. No engine, UI framework, or AI model has been selected.
