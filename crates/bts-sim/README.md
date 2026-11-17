# bts-sim

The first bounded headless broadcasting experiment. This is not a complete
station-management game. The authoritative provisional equations, units,
parameters, update order, and evidence are in
[the experiment specification](../../docs/headless-experiment.md).

`Simulation::new(World, Parameters)` validates ordinary Rust configuration and
initializes private state. `Simulation::step(&Slot)` validates an ordered fixed
action, computes all choices from one pre-broadcast snapshot, then commits
loyalty, hype, repetition and explicit `SimHour`. Failed actions do not mutate
state. `State` getters are developer inspection, not player observations.

`models::Models` owns TV-specific attraction, satisfaction, and ad-effectiveness
rules. Every gradual response uses complementary low/high rules because a
single active Sugeno rule always returns its constant consequent.

`demo::{world, schedule, run}` supplies three groups, two stations, four
programmes, two brands and the three comparison cases. `developer::report`
converts immutable causal events to explicit serializable developer DTOs;
there is no state serialization, file I/O, wall clock, or randomness here.

```sh
cargo test -p bts-sim
cargo run -p bts-sim --example sugeno_semantics
cargo run -p bts-cli -- demo --json /tmp/bts-demo-results.json
```

Expected viewers are fractional and conserve each group's population per
slot. Series willingness is group-level, not identified returning people.
Hype points stay in [0,100]; zero actual ad viewers imply zero campaign gain.
All input traits are finite unit intervals; parameters and actions are checked.
Configuration currently requires exactly three groups, two stations and two
brands. The two daily broadcast timestamps are 18:00 and 21:00. These limits
are deliberate; there is no general content-loading framework.

Publication remains disabled and licensing is unchanged.
