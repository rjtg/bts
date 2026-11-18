# Pooled Sway evidence

Historical protocol-v4 milestone snapshot: the cooperation checkbox and
production-specific promises shown here have been superseded by the
[casting desk and timed quests](../casting-desk/README.md).

Verified locally 2026-11-18. This replaces the personal-favour implementation;
[the previous evidence](../original-production/README.md) remains a historical
protocol-v3 snapshot. Current player protocol is v4.

- [Public results JSON](results.json): original comparison controls and the new
  three-production pooled-Sway sequence.
- [Actual Chromium screenshot](browser-sway.png): Nora's fulfilled request,
  station pool/history, Jules's exceptional participation and applied cooperation.
- [Rules, units and provisional equations](../../docs/pooled-sway.md).

Reproduce from the root:

```sh
cargo run --locked -p bts-web --example production_experiment > /tmp/sway-results.json
cargo run --locked -p bts-web --example production_experiment -- --sequence
```

The sequence is a native reference for actual Wasm parity, not a save format.
Normal player observations exclude exact talent skills, execution, audience tastes,
hype/repetition and fuzzy traces. Money is decimal strings; Sway is exact bounded
integer units.

## Observed sequence

| Tick | Achievement/action | Change | Balance |
| ---: | --- | ---: | ---: |
| 0 | Promise Nora's co-guest request and book project #1 | 0 | 0 |
| 44 | Nora and Felix complete recording | +4 | 4 |
| 75 | Negotiate Jules's day-3 participation for project #2 | −2 | 2 |
| 75 | Smooth cooperation for project #2 | −2 | 0 |
| 292 | Complete project #3: Rafi, everyday life, calm tone | +3 | 3 |

Nora proposes the first request, but Jules benefits from the spending. Booking
without the exceptional-participation option is rejected without changing state.
Sway does not pay anyone's fees. The three projects consume 3,000 cash credits;
two airings cost 100 and settle 2,488 advertising credits. Final cash is
**19,388 = 20,000 − 3,000 − 100 + 2,488**; final Sway is **3**.

A separate browser flow spends Rafi's reward on cooperation for a different
production without Rafi, leaving 1 Sway. A controlled Rust test changes only
cooperation treatment: execution improves within 0.05 while craft/topic limitations,
recognition and financial state remain unchanged. No universal balance claim follows.

## Checks performed

Rust 1.100.0, Node 24, wasm-bindgen 0.2.130, Playwright 1.61.1/Chromium 149:

- Formatting; **96 workspace tests**, **5 doctests**.
- Native all-target Clippy and Wasm adapter Clippy, warnings denied.
- Workspace Rustdoc with warnings denied; fuzzy/sim/web Wasm target builds.
- Release static browser bundle including `tsc --noEmit`.
- **8 real Chromium tests**, all passed (1.1 minutes): planning, mobile controls,
  initialization errors, full Sway earn/spend/broadcast, second quest, independent
  cooperation purchase, draft editing/conflicts, native/Wasm parity for both flows.
- Repeated native production experiment JSON is byte-identical.
- Existing planning/Sugeno examples and all four CLI experiment exports.
- Copied standalone fuzzy crate: formatting, Wasm build, 10 tests, 3 doctests,
  Clippy, Rustdoc and ventilation example.
- Clean Rust LSP diagnostics for changed production/adapter sources and tests.
  TypeScript language server unavailable; TypeScript/build/browser checks provided
  the fallback. Local Markdown links and whitespace checked.

CI retains the standalone/native/Wasm checks and automatically runs the updated
native example and expanded browser suite. No deployment is performed. Amounts,
cooperation effect, windows and fixed fixtures are provisional; no staged writing,
awards, careers or broader quest infrastructure is included.
