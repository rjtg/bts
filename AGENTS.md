# Contributor guidance

## Project intent

Read [README.md](README.md), [design](docs/design.md), and the relevant simulation and milestone sections before changing behaviour. Behind the Screens is a Rust TV station management game. World simulation must support television decisions.

## Design status

- Preserve the distinction between agreed direction, proposed mechanics, and open questions.
- Do not silently resolve major open design questions in implementation.
- Keep documentation aligned with implemented behaviour. Mark experiments and provisional parameters.
- Implement the bounded first-playable scope before expanding the society simulation.

## Implementation principles

- Keep `bts-fuzzy` independent of game types and sibling crates. Its manifest must remain self-contained so the crate can be extracted. Keep TV rules in the consuming simulation. See [architecture](docs/architecture.md).
- Keep the simulation independent of rendering, terminal input, and wall-clock time.
- Support reproducible runs from an initial state, seed, and ordered player actions. Store the state needed to continue a save consistently.
- Separate actual world state, public beliefs, and player-visible observations. UI code must not accidentally expose hidden values.
- Keep stable entity identifiers and explicit, typed relationships. Shared entity concepts do not require a universal untyped attribute bag.
- Use exact accounting for money and commitments; use continuous values, selected fuzzy rules, and probabilistic choices where they serve the design.
- Record causal contributions and significant events for developer inspection. Player explanations use only legitimately available evidence.
- Prefer small, inspectable models over speculative framework work. Avoid exhaustive relationships between all entities.
- Keep content and balance parameters easy to change; choose a data format when an implementation needs one.
- An AI model is optional. Core mechanics and structured concepts must work without network access or model inference.

## Verification

Test meaningful properties: population and audience accounting, financial obligations, bounded state, reproducibility, distinct viewing and satisfaction, targeting effects, and sensitivity to changed inputs. Use multiple seeds for balance experiments; one successful season is not evidence of general balance.

Document relevant checks and limitations. Do not introduce an engine, external service, or additional simulation subsystem solely because it might be useful later.
