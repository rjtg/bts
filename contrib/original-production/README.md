# Original-production evidence

Historical protocol-v3 snapshot. The personal-favour behaviour shown here has
been replaced by [pooled Sway](../pooled-sway/README.md); retained outputs are
original milestone evidence, not current protocol fixtures.

Implemented/verified locally 2026-11-18. These are outputs from the real Rust
session and built Wasm browser, not mockup values. No deployment was performed.

- [Public results JSON](results.json): two identical-start production comparisons
  and a separate complete promise → favour → later production → broadcast chain.
- [Actual Chromium production workspace](browser-production.png): two completed
  originals, recorded promise/fulfilment and consumed personal favour.
- [Model and provisional equations](../../docs/original-production.md).

Reproduce native reports from the repository root:

```sh
cargo run --locked -p bts-web --example production_experiment > /tmp/original-production.json
cargo run --locked -p bts-web --example production_experiment -- --sequence
```

The latter supplies the reference commands/reports for the Chromium native/Wasm
parity test, including rejected early scheduling and an unavailable guest without
a favour. It is test evidence, not a save format. All financial amounts remain
exact decimal strings. JSON/screenshot are player-whitelisted observations, not
hidden talent, audience, hype or fuzzy developer exports.

## Observed comparison

Both record day 1 10:00–11:00, finish 13:00 and air 18:00–18:45 against the same
competitor, with the same snack advertiser and initial cash 20,000:

| Programme choice | Production | Young | Households | Specialists | Ad income | Cash after airing |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Calm science, Elian/Nora/Felix | 1,000 | 664.69 | 1,521.51 | 661.96 | 469 | 19,419 |
| Lively culture, Mina/Jules/Rafi | 600 | 2,672.40 | 4,577.89 | 286.29 | 1,664 | 21,014 |

Each pays another 50 for its airing. Science wins specialist delivery; culture
wins broad reach and snack income in this fixture. Multiple creative choices
vary together, so this is not a claim about an isolated coefficient or general
balance. A separate test isolates host subject fit and verifies changed
specialist satisfaction at otherwise identical inputs.

In the favour chain, request #1 is promised at tick 0 for project #1, fulfilled
at tick 44, then favour #1 is called at tick 75 for project #2. Nora is otherwise
unavailable for its day-3 recording. The second work completes at 244 and airs
264–267. Final cash 20,315 = 20,000 − 2,000 production − 100 airing + 2,415 ads.
No promise/reward is generated implicitly by ordinary paid casting.

## Verification performed

Rust 1.100.0, Node 24, wasm-bindgen 0.2.130, Playwright 1.61.1/Chromium 149:

- Formatting, all **91 workspace tests**, **5 doctests**.
- Native all-target Clippy and Wasm adapter Clippy with warnings denied.
- Workspace Rustdoc with warnings denied; all three libraries build for Wasm.
- Ventilation/Sugeno examples; planning example; all four existing CLI exports.
- Production experiment runs twice with byte-identical JSON.
- Extracted `bts-fuzzy` outside workspace: formatting, Wasm build, 10 tests,
  3 doctests, Clippy, Rustdoc, runnable ventilation example.
- Release static browser bundle built with TypeScript checking.
- **7 real Chromium tests passed**: existing planner/mobile/init-failure coverage,
  full original/ad/broadcast workflow, favour earn/call and second broadcast,
  conflict rejection/draft edits/discard, both planning and production native/Wasm
  parity. Final suite: 1.1 minutes. The production parity test compares all fields
  (floating values within 10^-8; financial strings exact; opaque offer tokens excluded).
- Rust LSP diagnostics clean for touched engine/planner/production/adapter sources.
  TypeScript language server unavailable (`typescript-language-server` missing);
  `tsc --noEmit`, Vite and real-browser checks provide the fallback.

CI retains standalone fuzzy/native/Wasm checks, adds the production example, and
runs the expanded browser suite. Local results above are not a claim that a
particular future GitHub Actions run has passed.

Limitations: one standalone format, fixed three-hour lead time, no cancellation
of booked productions, no talent wallets, no dynamic careers/relationships or
pooled Sway, no unfinished-work pre-scheduling. Fixed fixtures and deterministic
expected counts are provisional, not economic balance or measured ratings.
