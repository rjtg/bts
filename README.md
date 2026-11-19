# Behind the Screens

A television station management game about understanding a changing audience and turning that understanding into successful television. Inspired by the management appeal of classic Mad TV, with deeper original production and a persistent simulated world.

## Direction

- Rust development; simulation mechanics come first.
- Browser as the primary player interface; Rust/WebAssembly integration first, with an 80s retro / 8-bit identity later. The CLI remains a developer tool.
- Hidden but learnable audience preferences, cultural trends, and talent careers.
- Multiple programme formats, original production, scheduling, and brand advertising.
- Scenario-specific ambitions rather than one universal victory condition; sandbox play is also intended.

## Core activity

Observe → acquire or produce → schedule and sell advertising → broadcast → interpret results.

The world supplies viewers, talent, stories, guests, and advertisers. Its detail must support decisions about television.

## Design documents

- [Game design](docs/design.md): scope, entities, relationships, formats, and scenarios.
- [Station Office widget guide](docs/office-widget-style.md) maps desk props to current and planned workspaces.
- [Screens and interaction design](docs/screens.md): proposed workspaces, shared production and mockup brief.
- [UI concept mockups](docs/mockups/README.md): station overview, programme planning and production editorial profile.
- [Weekly scheduling](docs/weekly-scheduling.md): recurring slots, weekday preferences and deferred follow-ups.
- [Minute airtime and the planning desk](docs/minute-airtime.md): exact placement, ad-load defaults, dossiers and one-minute fillers.
- [Playable Control Desk](contrib/focused-desks/README.md): Schedule, Weekly Run, Ad Sales, Broadcast Review and pause.
- [Control Desk redesign](docs/mockups/control-desk/README.md): connected UI mockups, audit and visual guide.
- [Resource economies](docs/resource-economies-idea.md): pooled Sway, award-driven Glamour and the Credibility–Backing–loyalty loop.
- [Simulation](docs/simulation.md): causal mechanisms, hidden information, and verification principles.
- [First playable](docs/first-playable.md): a bounded proposal and acceptance criteria.
- [Open questions](docs/open-questions.md): unresolved decisions and experiments.
- [Architecture decisions](docs/architecture.md): crate boundaries and extraction plan.
- [Headless experiment](docs/headless-experiment.md): broadcasting equations and developer reports.
- [Advertising economy](docs/advertising-economy.md): provisional demand, contracts, accounting, and comparisons.
- [Advertiser hardening](docs/advertiser-hardening.md): audience-specific value, off-air handling and pricing incentives.
- [Commitments and episodes](docs/episodes-and-commitments.md): audience-specific settlement, standalone works, familiarity and continuity.
- [Browser foundation](docs/browser-foundation.md): Wasm sessions, player protocol and local build.
- [Programme planning](docs/programme-planning.md): playable timeline, airing identities, cadence, controls and evidence.
- [Character portraits](docs/portraits.md): initial varied fictional roster artwork and generation briefs.
- [Talent and contracts](docs/talent-and-contracts.md): proposed hiring market, obligations, recurring roles and rewriting boundaries.
- [Production ambition draft](docs/production-ambition-draft.md): staged work, creative ambition, delivery shortfalls and downstream demands.
- [Production logistics](docs/production-logistics.md): implemented station ownership/capacity and future setup/prop economies.
- [Audience research](docs/audience-research.md) covers dated polls, Insight spending and ad forecasts.
- [Programme feature profiles](docs/feature-profiles.md) covers ambition, achieved qualities, audience fit and catalogue labels.
- [Programme rights market](docs/rights-market.md) covers acquisitions, assessments, rival purchases, resale and accounting.
- [Ad-minute market](docs/ad-minute-market.md) covers the sell-by-the-minute workflow, booking and settlement.
- [Original films](docs/original-film.md) covers script commissions, revisions, casting, studio bookings and airing results.
- [Casting desk](docs/casting-desk.md): portrait casting, booking reviews and adjustable cooperation support.
- [Sway quests](docs/quest-opportunities.md): separate acceptance, deadlines and production hints.
- [Pooled Sway](docs/pooled-sway.md): finite requests, station-wide social capital and bounded production uses.
- [Original production](docs/original-production.md): bounded discussion shows, talent commitments, budgets and pooled Sway requests.
- [Contributor guidance](AGENTS.md): implementation principles for people and coding agents.

## Status

Design baseline and initial Cargo workspace, November 2026. The independent [`bts-fuzzy`](crates/bts-fuzzy/README.md) crate implements zero-order Sugeno inference, validated membership curves, nested conditions, traces, and an authoring macro. A bounded headless broadcasting economy is implemented in `bts-sim`, with audience choice, hype-driven advertiser demand, delivery contracts and integer station finances. `bts-cli` provides controlled comparisons and developer JSON reports. A thin `bts-web` adapter and TypeScript/Vite browser harness now provide a playable three-week programme-planning workspace with 15-minute ticks, real runtimes, editable broadcasts, Rust-calculated ad contracts and restricted player reports. A bounded original-production workflow now books a host and two guests, reserves cash and recording time, completes a standalone discussion show and adds it to the schedulable library. Accepted timed quests earn pooled Sway, spendable on another person’s negotiable participation or on bounded cooperation smoothing. The complete season loop and full game interface remain unimplemented. Documents distinguish agreed direction from proposals and open questions; numerical examples are not balance commitments. The browser harness uses plain TypeScript, HTML and CSS; no game engine or AI model is selected.

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

`npm run build` produces the static `web/dist/` bundle. `npm run test:e2e`
runs real Chromium tests after `npx playwright install --with-deps chromium`.
Player protocol v8 excludes internal preferences, hype, execution and fuzzy
traces. Audience delivery uses explicitly provisional start-of-airing expected counts;
measured ratings and research remain future work. See the [browser guide](docs/browser-foundation.md)
for contractual edit constraints, JSON money strings and verification results.

The planner supports placing, moving and removing future broadcasts across seven
full days. Durations occupy real airtime; gaps are off air. Offers and results
name an airing ID, and accepted bookings lock its programme/time/advertiser.
Selected-airing actions are separate from next-to-air controls. Use
`cargo run -p bts-web --example planning_evening` for the reproducible native evening.

Use the **Production** casting desk to select portrait cards, draft and review a booking, run through recording,
then select **Show in programme library** to schedule it and sell advertising.
Nora Vale’s co-guest request and Rafi Linden’s calm discussion request earn
station-wide Sway, spendable across people and productions. See the [production rules and comparison](docs/original-production.md);
`cargo run -p bts-web --example production_experiment` reproduces the public reports.
