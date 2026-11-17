# Architecture decisions

## Fuzzy inference is an independent crate

Status: accepted boundary; initial zero-order Sugeno evaluator implemented.
Date: 2026-11-17.

Develop the fuzzy machinery in `crates/bts-fuzzy`, within a Cargo workspace.
It may later be extracted into a separate project.

### Responsibilities

| Crate | Owns |
| --- | --- |
| bts-fuzzy | Generic membership curves, expressions, rules, inference, validation, traces, authoring macro |
| bts-sim | Typed entities, TV-specific rules, explicit time, deterministic state transitions, developer evidence |
| bts-cli | Fixed experiment orchestration, readable developer summaries and JSON export |
| bts-web | Per-session simulation ownership, versioned commands, player observation whitelist, wasm-bindgen ABI |
| web/ | TypeScript/HTML/CSS harness, Wasm initialization, controls and presentation |

Dependency direction is from the simulation to bts-fuzzy. The fuzzy crate
must never import the simulation. Its outputs describe values or pressures;
the simulation decides what those mean and how they affect the world.

### Extraction requirements

- Self-contained Cargo manifest; no inherited workspace metadata or sibling dependencies.
- Generic public types and examples; no TV-specific vocabulary in the evaluator.
- No UI, filesystem, global randomness, or wall-clock requirements in evaluation.
- Caller-controlled ordering and explicit numerical semantics.
- Structured traces rather than logging as the only explanation channel.
- Library-level tests that run without game content.
- Keep public API small while experiments establish the required semantics.

The initial name is provisional. Publication is disabled until naming,
licensing, API stability, and release policy are decided.

### Initial implementation

The focused evaluator uses a thin `macro_rules!` authoring layer over
inspectable data. It implements zero-order Sugeno inference, nested conditions,
ramp/triangle/trapezoid membership curves, and an explicit finite fallback.
AND uses minimum, OR maximum, NOT complement. Rule activation times rule weight
weights each constant consequent in the final weighted average. Zero total
weight uses the fallback. Inputs and arithmetic must be finite. See the
[crate semantics](../crates/bts-fuzzy/README.md) for the precise contract.
Other inference families remain outside scope.

Reuse candidates were briefly reviewed: rsfuzzy, velvetry, rust-fuzzylogic,
and fuzzy_logic_engine_rs. The recommendation favours direct control over
nested rule expressions, validation, and per-rule explanations. This was
a source/documentation review, not a compiled benchmark or numerical audit.

### Current state

The standalone library and the first bounded broadcasting experiment are
implemented. `bts-sim` depends on `bts-fuzzy`; `bts-cli` depends on `bts-sim`.
The fuzzy crate and its self-contained manifest are unchanged by game code.

Simulation state is private and not serializable. Slot transitions emit owned
causal evidence; the separate `developer` module borrows that evidence to build
serializable reports. Serde supports experiment configuration/report data;
serde_json supports CLI developer exports and the separate browser adapter protocol.
Developer reports are not player observations or a save format. `bts-web` constructs
whitelisted player DTOs without serializing developer reports or internal state.
Public beliefs and measurement-based research are not implemented. Simulation never renders, reads files,
uses wall time, or invokes randomness.

The [experiment specification](headless-experiment.md) documents provisional
allocation, return and hype equations. The [advertising economy](advertising-economy.md)
adds finite budgets, explicit target priorities, demand, quotes, escrowed
delivery contracts, integer cash, costs and a balanced financial ledger.
Economy state participates in the same atomic slot transitions; quote/accept
actions are deterministic and independently inspectable. Entity counts and
broadcast hours are configured data, not fixed engine restrictions. Schema-4
reports separate delivery forecasts from audience-specific valuation and
preserve planned/actual airing plus funding failures. Unfunded broadcasts
resolve off air before audience allocation; invalid plans still fail atomically.
The [hardening controls](advertiser-hardening.md) keep remaining incentive
questions explicit. [Audience commitments and programme progression](episodes-and-commitments.md)
now add frozen per-audience payments and micro-impression settlement, explicit
standalone/episode kinds, series definitions, observed work familiarity,
predecessor reception and actual broadcast history. `programmes` owns the
bounded history factors; `episodes_demo` owns only controlled fixture content.
State remains separate from developer DTOs.
The browser is now the intended primary player interface; the CLI remains a
developer tool. `bts-web` depends on `bts-sim`, owns a simulation per session and
exposes versioned JSON commands through wasm-bindgen. The plain TypeScript,
HTML and CSS Vite harness is a minimal integration page. Financial u64 values
cross the boundary as decimal strings. Exact expected counts are labelled
provisional reporting; research and measurement-based settlement remain future
work. Accepted bookings lock that broadcast's programme and advertiser in the
adapter. Rejected commands are atomic. No developer inspector is exported.
See [browser foundation](browser-foundation.md) for the implemented protocol,
seven-day fixture, observation whitelist, real browser/native parity evidence,
build instructions and limitations. Neither fuzzy nor simulation imports the
adapter or browser dependencies. Native and standalone checks remain, alongside
Wasm target and browser smoke checks.

Production, full season play, research, save/load and the full game interface
remain unimplemented. A larger UI framework and save format remain open.
