# bts-web

Thin native/Wasm session adapter for the fixed seven-day browser experiment.
Owns a `bts-sim::Simulation` per session and constructs player-safe observations.
It does not add mechanics or serialize the developer export format.
Publication is disabled and licensing is unchanged.

- Native: `Session::new()`, `execute(Command)`, `dispatch_json(&str)`, `report()`.
- Wasm: `new WebSession()`, `session.dispatch(json)`, `session.free()`, `api_version()`.
- Protocol v1: strict typed commands, structured errors, whitelisted reports.
- Monetary values, contract IDs and integer delivery units are decimal strings.
- Rejected commands are atomic; accepted bookings lock the broadcast.

See [browser foundation](../../docs/browser-foundation.md) for the scenario,
protocol, observation exclusions, commands, build/test instructions and limits.
See [`protocol.rs`](src/protocol.rs) for the authoritative DTOs.

```sh
cargo test -p bts-web
cargo run -p bts-web --example protocol_fixture
```

The example is a native reference for browser parity tests, not a save or
simulation developer-export protocol. No developer inspector is exposed to JS.
Client-side observation filtering is not a security guarantee.
