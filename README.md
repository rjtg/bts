# Behind the Screens

A television station management game about understanding a changing audience and turning that understanding into successful television. Inspired by the management appeal of classic Mad TV, with deeper original production and a persistent simulated world.

## Direction

- Rust development; simulation mechanics come first.
- Browser as the primary player interface; Rust/WebAssembly simulation with a late-1980s Station Office. The CLI remains a developer tool.
- Hidden but learnable audience preferences, cultural trends, and talent careers.
- Multiple programme formats, original production, scheduling, and brand advertising.
- Scenario-specific ambitions rather than one universal victory condition; sandbox play is also intended.

## Core activity

Observe → acquire or produce → schedule and sell advertising → broadcast → interpret results.

The world supplies viewers, talent, stories, guests, and advertisers. Its detail must support decisions about television.

## Design and status

Use the [design library](docs/README.md) to find current rules, mockups and
historical experiments. The [game design](docs/design.md) describes the
station and world. The [design guidelines](docs/design-guidelines.md)
record reusable decisions for future work. Start with the
[playable season](docs/first-playable.md) to review the current browser game.

The browser runs a bounded six-week Station Office scenario. The Rust
simulation owns scheduling, audience allocation, research, rights purchases,
production, ad-minute sales and accounting. The CLI supplies controlled
comparisons and developer traces. The independent [bts-fuzzy](crates/bts-fuzzy/README.md)
crate implements generic zero-order Sugeno inference.

## Run the broadcasting experiment

```sh
cargo run -p bts-cli -- episodes --json /tmp/bts-episodes.json
cargo run -p bts-cli -- advertiser-value --json /tmp/bts-advertiser-value.json
cargo run -p bts-cli -- economy --json /tmp/bts-economy.json
cargo run -p bts-cli -- demo --json /tmp/bts-demo-results.json
cargo run -p bts-cli -- demo --case targeted --trace
cargo run -p bts-sim --example sugeno_semantics
```

The three fixed schedules start from identical state: broad popular programming,
a smaller specialist audience, and a promoted but disappointing series. Reports
are developer-only and include actual hidden inputs. Counts are expected
viewer-slot impressions, not cumulative unique reach. Coefficients are
provisional; hype does not represent sales or revenue.

The `economy` comparisons pair mass-market/specialist advertisers with broad/
targeted programmes from identical starting conditions. Prices are capped by
finite budgets, accepted offers reserve escrow, actual target delivery settles
at a deadline, and broadcast costs reduce integer station cash. Inspect the
ledger and forecast/actual breakdown with `--trace`. Hype is internal developer
evidence; no finished player research interface is implied.

Quotes value each reached audience's own unmet hype demand. Valid plans that a
station cannot fund go off air for that slot while other stations continue;
malformed commands remain errors. `advertiser-value` isolates saturation and
repetition effects, including remaining incentives in delivery-based pricing.

Accepted contracts now freeze micro-impression commitments and allocated
payment per audience, preventing cross-audience delivery substitution.
`episodes` compares standalone movies, distinct series episodes, opening
reception, earlier repeats and continuity. Work familiarity is an expected
fraction of a group; it is not measured individual reach. Series attachment
and actual broadcast history are separate state.


## Run the browser planning workspace

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.130 --locked
cd web
npm ci
npm run wasm
npm run dev
```

`npm run build` produces the static `web/dist/` bundle. To serve that build
over the network, run `npm run preview -- --host 0.0.0.0` from `web/` and
open port 4173. Rebuild to update the preview. Use `npm run dev` for a
development server that reloads frontend changes. Run `npm run test:e2e`
after installing Chromium with `npx playwright install --with-deps chromium`.

In the office, open the **calendar** for Programming, the **shelf** for
owned tapes, and the **script file** for Ideas, Scripts and Production.
Audience surveys and title assessments spend or use their own dated evidence.
Airing actions use the exact airing's file. The player protocol exposes
provisional expected counts with restricted observations. See the
[browser foundation](docs/browser-foundation.md) for the adapter and money
serialization and [minute airtime](docs/minute-airtime.md) for current scheduling.
