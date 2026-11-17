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

`Simulation::step(&Slot)` reads common pre-slot loyalty, partitions each audience
between stations and not watching, updates satisfaction/return willingness and
hype, charges integer broadcast costs, records contract delivery, settles due
contracts and commits atomically. Failed actions leave all state untouched.
`State` inspection getters are developer tools, not player observations.

`models::Models` owns TV rules including two conditional interactions and
creative-gated advertising effectiveness. Complementary low/high rules preserve
gradual Sugeno responses. Effective weights are never confidence/probabilities.

`Simulation::{demand, quote}` expose actual target priorities, current/desired
hype, capped demand, forecast audience delivery and full fuzzy evaluations.
`accept_offer` revalidates a fresh quote, reserves finite marketing budget and
one station/hour ad slot. Deadlines settle actual delivery using exact integer
pro-rata payment and refunds. `State::economy()` exposes immutable contracts,
balances, escrow and the balanced transfer ledger. No credit or replenishment
is available. Broadcasts need cash before same-slot settlement.

`demo` keeps the original unpaid-ad experiments. `economy_demo` compares both
advertiser types under both schedules, followed by decay and a recovery quote.
`developer::{report, economy_report}` convert immutable causal evidence to
schema-version-2 JSON-ready DTOs. Internal state is not serializable. Simulation
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
