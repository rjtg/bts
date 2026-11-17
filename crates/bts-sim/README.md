# bts-sim

A bounded headless broadcasting economy, not a complete management game.
See the [broadcasting model](../../docs/headless-experiment.md) and
[economy specification](../../docs/advertising-economy.md) for provisional
rules, equations, units, rounding, update order, results and limits.

`Simulation::new(World, Parameters)` validates Rust configuration and initializes
private biological/economic state. Collection sizes are unrestricted; empty
collections are valid subject to references. Availability is a 24-hour array;
any strictly increasing integer simulated hour can be resolved. Demos retain
three groups, two stations, four programmes, two brands and 18:00/21:00 slots.

`Simulation::step(&Slot)` validates the entire plan, then resolves each station's
ability to pay. Unfunded programmes go off air independently with no cost,
viewing, learning or ad effects. Global time, passive decay and settlements
continue; available viewers choose among actual aired broadcasts. The engine
reads common pre-slot loyalty and partitions each audience
between stations and not watching, updates satisfaction/return willingness and
hype, charges integer broadcast costs, records contract delivery, settles due
contracts and commits atomically. Failed actions leave all state untouched.
`State` inspection getters are developer tools, not player observations.

`models::Models` owns TV rules including two conditional interactions and
creative-gated advertising effectiveness. Complementary low/high rules preserve
gradual Sugeno responses. Effective weights are never confidence/probabilities.

`Simulation::{demand, quote}` expose actual target priorities, current/desired
hype, audience-specific valuation, forecast delivery and full fuzzy evaluations.
Brand-wide pressure is diagnostic; each reached audience's own deficit prices
its relevant delivery. Valuation and delivery guarantees are separate.
`accept_offer` revalidates a fresh quote, reserves finite marketing budget and
one station/hour ad slot. Deadlines settle actual delivery using exact integer
pro-rata payment and refunds. `State::economy()` exposes immutable contracts,
balances, escrow and the balanced transfer ledger. No credit or replenishment
is available. Broadcasts need cash before same-slot settlement.

`demo` keeps the original unpaid-ad experiments. `economy_demo` compares both
advertiser types under both schedules, followed by decay and a recovery quote.
`advertiser_value` supplies matched-stock saturation/repetition/creative controls
and reports remaining pricing incentives; see [hardening](../../docs/advertiser-hardening.md).
`developer::{report, economy_report}` convert immutable causal evidence to
schema-version-3 JSON-ready DTOs. Internal state is not serializable. Simulation
uses no filesystem I/O, rendering, wall clock or randomness.

```sh
cargo test -p bts-sim
cargo run -p bts-sim --example sugeno_semantics
cargo run -p bts-cli -- economy --json /tmp/bts-economy.json
```

Viewers are fractional expectations, not unique individual reach. Hype is
bounded [0,100]; zero ad exposure implies zero campaign gain. Contract units
are floored weighted relevant impressions, not rounded raw people. Models and
costs are provisional; no balance or real-world validity is established.
Publication stays disabled; licensing is unchanged.
