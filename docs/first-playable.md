# First playable proposal

Status: proposed scope. This bounds implementation; it does not remove later features from the design.

## Question to answer

Can a player observe an audience opportunity, make a production bet, schedule it effectively, earn advertising revenue, and understand the result well enough to improve?

## Scenario

Run a small station for one season. Remain solvent, fulfil explicit advertising commitments, and launch an original programme that earns returning viewers.

Season length, starting funds, contract thresholds, and the definition of a returning audience must be tuned experimentally. No board review system or universal campaign victory rule is assumed.

## Suggested content budget

- Six audience groups with distinct availability, tastes, purchasing power, and novelty preferences.
- Four broad scripted genres.
- Approximately twenty fictional people covering necessary creative roles and interview guests.
- One competing station following a simple audience-aware strategy.
- A small purchased-programme catalogue.
- One original scripted-series pipeline and a basic topical interview format.
- A handful of brands targeting different audience attributes.
- A small influence network, selected talent relationships, and a few topical events.

Counts are working limits, not requirements. Reduce content if necessary to finish a complete causal loop.

## Player actions

Inspect results; commission broad or targeted research; acquire broadcast rights; create a structured production concept; hire available talent; allocate resources; book an interview guest; schedule programmes and advertisements; accept contracts; advance time; review outcomes; renew, revise, or cancel.

Proposal: begin with daily planning and slot-based resolution. A headless version precedes a minimal TUI. Avoid free-text script interpretation in this milestone.

## Build sequence

### 1. Headless experiments

Implement audience choice and satisfaction, basic programme attributes, brand targeting, hype decay, and developer reports. Compare schedules and campaigns using identical starting states and seeds.

Current implementation: the [bounded broadcasting experiment](headless-experiment.md)
provides three audience groups, two stations, four programmes, two brands,
fixed schedules, deterministic expected shares, return willingness, and hype
traces. The [bounded advertising economy](advertising-economy.md) adds target
contracts, finite budgets, station cash, simple broadcast costs and a ledger. It supplies initial evidence for this stage, not a completed game or
balance validation. Randomness and schedule optimisation are not implemented.

Exit evidence: different audiences produce different outcomes; relevant reach affects hype; attraction differs from enjoyment; results are reproducible and explained.

### 2. Complete season loop

Prototype cash and delivery contracts now exist in the bounded economy. Rights, production lead times, talent commitments, and a research channel remain unimplemented. Run a full season with scripted actions before building the interface; the short economy comparisons do not complete this stage.

Exit evidence: obligations resolve correctly; production tradeoffs matter; at least one useful audience opportunity can be discovered; multiple plausible strategies survive under appropriate conditions.

### 3. Playable interface

Expose only legitimate player information through a minimal interface, preferably a TUI if it remains suitable. Provide scheduling, production, research, advertiser, and results views. Include save/load and a clear time-advance action.

Exit evidence: someone can play the scenario without developer tools, understand obligations, and explain a reasonable next decision from the reports.

## Scope exclusions

No full elections, awards circuit, demographic splitting/merging, detailed government economy, elaborate gossip publication system, every programme format, multiplayer, animated building, or AI pitch evaluation.

Audience and talent histories, event identities, and relationship types should leave room for those systems without implementing speculative infrastructure.

## Acceptance criteria

- Complete a season with a clear objective outcome and financial record.
- At least two audiences disagree about a programme in a way consistent with their preferences.
- A successful premiere can lose return viewers after poor satisfaction.
- A smaller relevant audience can be more valuable to a brand than a larger poorly matched one.
- Original-production choices create meaningful cost, timing, or audience tradeoffs.
- Research can change a sensible decision and improve outcomes across a batch of worlds.
- Repeating the same successful formula encounters diminishing returns where appropriate.
- Save/load preserves subsequent outcomes for the same actions.
- Developer inspection can explain outcomes; normal views do not reveal hidden state.
- Human playtesting finds the reports actionable, not merely numerically consistent.

Automated experiments establish mechanics; human playtesting establishes whether making these decisions is enjoyable.
