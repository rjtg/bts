# Browser foundation

Status: implemented integration milestone; provisional content and reporting.
The browser is the intended primary player interface. The CLI remains a
headless developer tool. This is a functional test harness, not the full game,
a completed season loop, or evidence of economic balance.

## Boundaries and platform

`bts-fuzzy` remains dependency-free, independently extractable and unchanged.
`bts-sim` owns the existing mechanics, integer accounts, quote valuation,
commitments, deadline settlement, off-air handling and explicit simulated time.
Both compile for `wasm32-unknown-unknown` without platform-specific changes.
They use no filesystem, terminal, wall clock, browser API or external service.

`bts-web` owns one `Simulation` per `Session`. Its native Rust API is also used
in tests. The wasm-bindgen `WebSession` constructor creates one such session;
`dispatch(request: string): string` consumes and produces JSON. `api_version()`
returns 1. No simulation state, developer export or inspector is exported.
TypeScript handles initialization, controls and presentation. Rust calculates
quotes, delivery, money, settlement and all simulation effects.

The adapter uses clone/apply/observe/commit transactions. Malformed, invalid,
stale and locked commands leave simulation, plans, pending offers, counters and
results unchanged. A valid unfunded broadcast still follows the simulation's
off-air path, allowing the rival to continue and missed delivery to refund.
Reset rebuilds the fixed initial world and clears plans, bookings and results.
Opaque offer tokens have a process-local session namespace and monotonically
increasing counter retained across reset. Tokens prevent accidental cross-session
or reset reuse; they are not credentials. No time or randomness generates them.
Namespace differences do not affect numerical outcomes. Separate `WebSession`
objects own independent state, even within the same Wasm module.

## Fixed scenario and actions

Seven days have two decisions each: 18:00 and 21:00, fourteen slots in total.
Hours are absolute simulated hours: `24*day_index + slot_hour`. Advancing a
slot resolves exactly one action, including elapsed-time decay; it never uses
elapsed browser time. Completion requires reset for another run.

The fixture reuses the three audience groups, two stations and two advertisers
from the headless demo. Both stations start with 20,000 integer credits, and
both brands with finite 20,000-credit budgets and 4,000-credit offer caps. The
catalogue contains four original first episodes, a standalone film and two
further comedy episodes. Attributes and programme costs are fixed. The rival
always airs Evening Stories, without an advertisement. These are fixture
restrictions, not simulation validation restrictions. No rights, new purchases,
production, advance-listings mechanic or content-loading framework is added.

All future unplayed player slots may be planned or revised. Offers are requested
only for the next slot, with that slot as the inclusive settlement deadline.
Acceptance freezes the engine's audience-specific forecast, promises and payment
allocations. The adapter additionally locks that slot's programme and advertiser;
editing a booked broadcast returns `booking_locked`. There is no cancellation,
rebooking or makegood action. This stronger interface constraint avoids implying
that accepted delivery promises are freely editable. The engine retains its
more general delivery/refund semantics. A selected prospective advertiser does
not air unless its offer was accepted. Unadvertised broadcasting is valid.

Every successful plan, advance, acceptance or reset invalidates the pending
offer. A new quote replaces the previous quote. Rejected commands do not
invalidate a still-valid quote. Requesting a quote does not reserve budget;
acceptance does. Quote preview accepts on a disposable simulation copy to obtain
the exact Rust contractual allocation, never a TypeScript reconstruction.
A zero-price quote returns `no_payable_offer` without changing the session.

## Protocol v1

The authoritative serializable types are in
[`protocol.rs`](../crates/bts-web/src/protocol.rs); the harness's TypeScript
shapes are in [`protocol.ts`](../web/src/protocol.ts). This is a separate
browser protocol, not developer JSON schema 4 and not a save format. Changes
to this contract require explicit version review. Unknown fields, unknown
commands, invalid numeric types, invalid JSON and unsupported versions fail.
Optional `advertiser` may be omitted or null to select no advertisement.

Every request is an envelope, for example:

```json
{"api_version":1,"command":{"type":"plan","slot":0,"programme":1,"advertiser":1}}
```

| Command type | Fields | Effect |
| --- | --- | --- |
| `inspect` | none | Observe catalogue, finances, schedule, terms and past results |
| `plan` | `slot`, `programme`, optional `advertiser` | Plan an unplayed, unbooked slot; programme/brand IDs are u16 numbers |
| `request_offer` | none | Quote the next planned slot and retain an opaque offer token |
| `accept_offer` | `offer_id` string | Reserve the quoted budget in escrow; freeze terms and lock the broadcast |
| `advance` | none | Air the next planned slot, resolve its contract and move forward |
| `reset` | none | Restore initial conditions and invalidate all previous offers |

Success is `{ "status":"ok", "api_version":1, "report":{...} }`.
Failure is `{ "status":"error", "api_version":1,
"error":{"code":"plan_required","message":"Choose and plan a programme for the next slot."} }`.
Useful errors distinguish malformed commands, unsupported versions, unknown
programmes/advertisers, invalid slots, absent plans/advertisers, already-booked
slots, locked broadcasts, stale offers, no payable offer and completed scenarios.
Unexpected engine failures use a controlled message, not an internal debug dump.

All money, u64 contract IDs and integer micro-impression values are **decimal
strings**, including cash, broadcast costs, price, allocations, payments and
refunds. They never become JSON numbers or JavaScript `Number`s. Presentation
uses `BigInt` only to format credits, with no accounting in JavaScript. Tests
cover the full u64 maximum `18446744073709551615`. Small IDs, hours, indices and
populations fit exact JavaScript integers. Audience expectations and forecast
relevant impressions remain finite floating numbers. Unit-bearing field names
separate those quantities from integer contractual micro-units.

## Player observation boundary

Player reports deliberately construct whitelisted fields rather than serializing
`World`, `State`, `Offer`, `SlotEvent`, or developer DTOs. They expose:

- Known catalogue names, kinds, series identity/sequence, genre, tone, awareness,
  cast appeal and per-airing cost; programme execution remains hidden.
- Advertiser names/IDs, own station cash, fixed rival schedule, own plans,
  booking locks and scenario progress.
- Quoted and accepted contractual audience forecasts, promised micro-units,
  allocated payments, deadline, settlement credits and refunds per audience.
- Past aired work/advertisement, off-air status, own cost/cash and group counts
  at each station or not watching.

They exclude exact audience affinities, availability profiles, purchasing power,
product interests, internal hype/desired-hype/deficits/repetition, creative quality,
private execution, familiarity/attachment stocks, fuzzy models, traces, rules,
activation/normalization totals, and other accounts' balances. Quoted terms
necessarily reveal commercial willingness to pay, not the underlying demand
stocks or causal calculation. No developer inspector is enabled or shipped.
Internal data exists in downloaded Wasm and client memory: this is a deliberate
normal-observation boundary, **not client-side secrecy or a security guarantee**.

Audience counts are explicitly labelled **provisional exact expected counts,
not measured ratings**. Contract forecasts are exact model forecasts for the
unchanged fixed scenario; they are not a finished player prediction/research
service. Settlement still uses actual Rust expected relevant delivery, not a
measurement sample. No arbitrary noise is added. Measured ratings, delayed or
imperfect research, public beliefs, restricted information sources and
measurement-based settlement remain future work. Player reports do not expose
experienced satisfaction directly or explain every hidden cause.

## Local build and smoke tests

Prerequisites: stable Rust with `wasm32-unknown-unknown`, Node 24+ and npm,
wasm-bindgen CLI exactly matching the pinned crate version 0.2.130.
From the repository root:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.130 --locked
cargo build --locked --target wasm32-unknown-unknown -p bts-fuzzy -p bts-sim -p bts-web
cd web
npm ci
npm run wasm
npm run dev
```

Use the printed localhost HTTP URL. `file://` is not supported. After changing
Rust, rerun `npm run wasm`; Vite handles TypeScript/CSS changes. A custom CLI
path may be supplied with `WASM_BINDGEN=/path/to/wasm-bindgen`.

```sh
# In web/
npm run build
npx playwright install --with-deps chromium
npm run test:e2e
npm run preview
```

`npm run build` builds release Wasm, generates bindings, checks TypeScript and
creates the static **`web/dist/`** directory. Copy its contents as a unit to a
static HTTP host serving `.wasm` as `application/wasm`; relative asset URLs
support a subdirectory. Generated bindings, node_modules, test artifacts and
dist are ignored; package-lock.json and Cargo.lock are committed. No backend,
accounts or runtime external/network services are needed. Deployment is outside
this milestone. CI builds and uploads a static bundle artifact without deploying.

## Validation evidence

Local verification on 2026-11-17 used Rust/Cargo 1.98.1, Node 26.10.0,
TypeScript 6.0.3, Vite 8.3.8 and Playwright 1.61.1 with real headless Chromium
149.0.7827.55. All three browser tests passed against the production static
bundle:

1. Loading through plan, quote, accept, locked-edit rejection, airing, settlement
   and reset, with no uncaught page errors.
2. Actual Wasm commands versus the equivalent native sequence, comparing every
   player-report field (opaque offer tokens differ by namespace), exact financial
   strings/contract units, and floating expectations within 10^-8. The generated
   test-loaded Wasm is byte-equal to the binary in the built static bundle.
   Invalid/stale/version commands, reset and same-module session isolation also
   run in Chromium. Native tests separately compare complete simulation state
   against direct `bts-sim` calls for the same booking and broadcast.
3. Failed Wasm loading leaves controls unavailable and reports a reload action.

The reference first comedy broadcast earns 1,697 credits with a 150-credit cost,
ending at 21,547 credits. Frozen audience allocations are 620 / 1,076 / 1 credits;
all are fulfilled and refunds total zero. Expected player viewing is 2,772.20 /
4,819.83 / 346.36 across young viewers / households / specialists. These numbers
are integration evidence, not economic balance claims.

All 70 native workspace tests and five doctests passed. The seven new adapter
tests cover malformed/invalid commands without mutation, stale
quotes, reset, isolated sessions, booking locks, native equivalence, conservation,
unbooked ads not airing, the fourteen-slot boundary, reproducibility, player
report leakage and lossless financial serialization. Workspace tests/doctests,
formatting, native/Wasm Clippy and Rustdoc checks, CLI experiments, and a fresh
outside-workspace fuzzy copy retain the existing checks. The CI workflow adds
Wasm target builds and browser build/smoke tests while preserving native and
standalone checks. Local success is not a claim that this newly edited CI has
already run. LSP feedback was stale for some Rust revisions and the TypeScript
server was unavailable; Cargo and `tsc` supplied semantic verification.

## Remaining limits and next UI milestone

This small fixed scenario has no acquisition, research, uncertainty, save/load,
production or open-ended calendar. Whole-session copying is acceptable for
fourteen slots, not a commitment to a large-game transaction strategy. History
and reports are retained only in memory. Browser smoke coverage currently uses
Chromium; Firefox, WebKit, mobile devices and assistive-technology playtesting
remain unverified. Static-host deployment has not been performed. Existing
repetition-blind advertiser pricing, ordered allocation rounding bias and the
provisional expected-group mechanics are unchanged and remain limitations.

Next UI work should improve the daily schedule/contract/results workflow,
clarify information sources and obligations, and test whether a person can
make and explain a next decision without developer traces. The polished retro
control-room presentation can follow that usability evidence. A complete season,
research/measurement design and save protocol remain separate milestones; this
integration does not silently implement them.
