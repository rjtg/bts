# bts-web

Thin native/Wasm session adapter for the seven-day quarter-hour planning workspace.
Owns a `bts-sim::planning::Planner` and its simulation per session and constructs player-safe observations.
The Rust simulation owns scheduling, runtime, pricing and accounting; this adapter
constructs player DTOs independently of the developer export format.
Publication is disabled and licensing is unchanged.

- Native: `Session::new()`, `execute(Command)`, `dispatch_json(&str)`, `report()`.
- Wasm: `new WebSession()`, `session.dispatch(json)`, `session.free()`, `api_version()`.
- Protocol v5: strict typed commands, structured errors, whitelisted reports.
- Monetary values, contract IDs and integer delivery units are decimal strings.
- Rejected commands are atomic; accepted bookings lock the airing, and all commands identify it explicitly.

See [browser foundation](../../docs/browser-foundation.md) for the build/test instructions. See [programme planning](../../docs/programme-planning.md)
for the current scenario, cadence, protocol and limits.
See [`protocol.rs`](src/protocol.rs) for the authoritative DTOs.

```sh
cargo test -p bts-web
cargo run -p bts-web --example protocol_fixture
cargo run -p bts-web --example planning_evening
cargo run -p bts-web --example production_experiment
```

The examples provide native parity and evening-workflow evidence, not a save or
simulation developer-export protocol. No developer inspector is exposed to JS.
Client-side observation filtering is not a security guarantee.

[Original production](../../docs/original-production.md) documents the studio commands,
budget escrow, talent commitments, hidden qualities and pooled Sway requests and spending.
