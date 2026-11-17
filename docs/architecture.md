# Architecture decisions

## Fuzzy inference is an independent crate

Status: accepted boundary. Initial evaluator details remain proposed.
Date: 2026-11-17.

Develop the fuzzy machinery in `crates/bts-fuzzy`, within a Cargo workspace.
It may later be extracted into a separate project.

### Responsibilities

| Crate | Owns |
| --- | --- |
| bts-fuzzy | Generic membership curves, expressions, rules, inference, validation, traces, authoring macro |
| Future simulation crate | World entities, TV-specific rules, input mapping, time, state updates, player observations |
| Future front ends | Interaction, presentation, experiment orchestration |

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

### Initial implementation proposal

Implement a focused evaluator ourselves, with a thin `macro_rules!` authoring
layer over inspectable data. The initial target is zero-order Sugeno inference,
nested conditions, a few membership curves, and explicit fallback behaviour.
Do not implement multiple inference families speculatively.

Reuse candidates were briefly reviewed: rsfuzzy, velvetry, rust-fuzzylogic,
and fuzzy_logic_engine_rs. The recommendation favours direct control over
nested rule expressions, validation, and per-rule explanations. This was
a source/documentation review, not a compiled benchmark or numerical audit.

### Current state

The workspace and empty library scaffold exist. Evaluation, macros,
simulation crates, and front ends are not implemented. This document does
not select a UI framework or save format.
