# Programme-planning milestone

Status: implemented, bounded and provisional. This replaces the browser's
fourteen fixed slots with a seven-day schedule. The primary reference is
[screens](screens.md) and the [planning mockup](mockups/programme-planning.png).
This planning milestone added a playable scheduling workflow. The subsequent
[original-production milestone](original-production.md) adds one studio format;
a complete season game remains deferred.

The [Control Desk weekly planner](weekly-scheduling.md) now extends the browser calendar to three weeks, adds recurring slots and enables weekday viewing preferences. The [focused Control Desk](../contrib/focused-desks/README.md) separates Schedule, Weekly Run, Ad Sales and Broadcast Review. The original seven-day implementation below is the baseline.

The current planner reserves the programme plus listed ad time to the minute.
See [minute airtime](minute-airtime.md) and [ad-minute market](ad-minute-market.md)
for placement, segment processing, inventory and settlement. The original
quarter-hour planning implementation below provides historical context.

See [programme rights](rights-market.md) for ownership transfers, rival gaps and schedule protections.

## Workspace and actions

The [Planning Room design](station-office.md) describes the office CRT entry, calendar summaries and Today behaviour.

The browser has a full-day horizontal timeline with 96 quarter-hour cells,
a read-only competitor lane, a filterable programme catalogue and contextual
broadcast details. The initial view scrolls to the evening; 00:00, evening,
Now and day controls expose the entire day and all seven days. Programme blocks
span their actual runtime. Teal drafts, amber selection/booking marks, on-air
status and grey history distinguish states. The navy/cream/amber visual style
follows the concept; it does not implement the concept's publication or ad-share
controls. Navigation leads only to implemented planning, results and finances.

Choose a catalogue work and day/start time, then **Place broadcast**. Clicking
a gap sets that placement time. The next placement defaults to the new airing's
returned end. Select a block to move or remove it, choose its advertiser,
request/accept its offer or inspect its result. Moves preserve the airing ID;
removal does not reuse it. The programme determines runtime; it cannot be
shortened to bypass overlaps. Intervals are half-open `[start,end)`: adjacency
is valid, overlaps on one station are rejected, and cross-midnight works are
allowed if the whole work fits before the scenario end.

The selected airing is UI context, distinct from **next to air** and **currently
on air**, which are calculated in Rust. Selecting an airing performs no simulation
command. A quote/accept control names its exact airing ID and date/time. A pending
offer for another selection cannot be accepted from the wrong detail panel.
Global **Run next airing** finishes the current player airing, or advances through
the next player airing's completion. **Run through selected airing** explicitly
names its different target. **+15 min** processes one interval; **Finish current
day** advances to the next midnight. Every mode processes all intervening ticks,
competitors, gaps and due obligations. None uses wall-clock time.

Results appear at scheduled completion and remain linked to the airing, even
for a repeated work. Selecting an earlier result reopens its detail panel.
Station finances show exact opening cash, total broadcast costs, advertising
receipts, current cash and a ledger with links back to the affected airings.

## Explicit time, duration and identity

`SimTick(u32)` is an absolute 15-minute boundary; four ticks equal an hour,
96 a day. The browser scenario spans `[0,672]`. It starts before processing
`[0,1)`, so a programme may be placed at 00:00. At boundary t, starts at t are
still editable until that interval is processed; starts before t are not.
Programme `duration_minutes` must be positive and divisible by 15. Each
placement gets a stable `AiringId(u32)`, station, programme, start and end.

`bts-sim::planning::Planner` owns the timeline and a typed-clock instance of
the existing numerical/accounting engine. `Simulation` remains the whole-hour
alias used by native point experiments; those tests, monetary equations and
legacy developer-export integer elapsed hours are preserved. Shared engine,
state, ledger and contract types support `SimHour` and `SimTick`, so no tick
is silently passed off as an hour. Only these two clock implementations are
supported. `bts-fuzzy` and its self-contained manifest remain unchanged.

The browser fixture retains three audiences, two stations and two brands with
20,000-credit opening station balances and brand budgets. Runtime is 30 minutes
for comedy episodes, 45 for science, 60 for drama and the promoted premiere,
and 90 for the standalone film. The fixed rival fills 16:00–24:00 daily with
eight works, using the same catalogue durations. Player scheduling is available
for the full day, not only the evening. Hidden fixture availability now extends
from 16:00 through 23:00: relative to the earlier 18:00/21:00 fractions, 16:00
is .35 times the early fraction, 17:00 .65, 19:00/20:00 linearly interpolate,
22:00 is .8 times the late fraction and 23:00 .5. Other hours remain zero.
These are provisional content parameters, not engine count/hour restrictions.

## Original processing cadence and units

The quarter-hour milestone deliberately retains **one provisional start snapshot per airing**,
rather than adding an interval exposure or viewer-retention model. Runtime
consumes airtime and changes which programmes compete; it does not multiply
an airing's cost or its single implicit advertising opportunity.

For interval `[t,t+1)`, in deterministic station/ID order:

1. Identify scheduled active works and fresh starts. An airing cancelled for
   lack of funds remains off air for its whole reserved duration. Continuing
   funded airings are not checked or charged again.
2. Check starting stations against pre-interval cash. Unfunded starts go off
   air independently. Recompute expected allocation across actually active
   works plus not watching, using availability for `floor(t/4) mod 24`.
   Each interval conserves every group's population across these alternatives.
3. Decay hype/repetition by 0.25 hours. Only a fresh funded start generates its
   one full advertising exposure opportunity, and only when an ad is booked.
   There are no additional ad pulses on continuation ticks. Store the starting
   audience/satisfaction snapshot for later reception and result construction.
4. Charge each funded start's full integer cost once, record booked delivery
   once, and settle contracts due at the ending boundary. The financial event
   for a start at t is timestamped t+1: the end of its first processed interval.
   Later intervals can decay state and settle deadlines without another cost,
   impression, ledger payment or full viewing consequence.
5. At completion, use stored reception samples once to update expected work
   familiarity, series attachment and episode progression. Append one actual
   broadcast-history record; cancelled works append none. All completions at
   the boundary use one pre-completion history state. The next interval's
   starts can then respond to that reception.
6. Commit the interval; an advance command commits all its intervals atomically.

There is **no sum of tick audiences labelled as unique viewers or delivery**.
Past player results show the stored start snapshot and its time, not a mean
across the duration. Continuing programmes still compete in each interval's
allocation, but later arrivals/departures do not alter the sampled programme's
ad delivery or reception. One full-airing delivery opportunity is the duration
normalization: its weight is 1 once, not 1 on every tick. Thus a 90-minute work
has the same single opportunity as a 15-minute version in the controlled
constant-input test. Minute-integrated exposure remains unimplemented.

Completion snapshots may describe different instants when staggered works end
together. Their combined learning mass is conservatively scaled by
`min(1,population/sum(sample viewers))` per audience (1 for zero samples), before
applying the existing bounded history equations. Fresh learning uses current
familiarity at completion. This cap is an explicit approximation, not an
estimate of unique reach; each reported starting sample remains unchanged.

Passive stocks retain elapsed-time units: `H *= 2^(-.25/half_life_hours)` and
likewise repetition on each tick. Original hype response, fuzzy models, weighted
average semantics, audience-specific deficits and exact settlement equations
are unchanged. Numerical time advancement is identical whether called tick
by tick, to the next airing, or in one batch through the day.

## Gaps, funding and contracts

Unfilled schedule intervals are explicitly off air, have no cost or ad exposure,
and do not update programme reception. The rival continues and available viewers
redistribute to other aired options or not watching. Passive decay and due
settlement still run. A valid unaffordable plan succeeds as an off-air outcome;
an overlap, unknown identifier or past edit is a rejected command. A cancelled
work cannot resume part-way through after funds change; its result arrives at
its scheduled end. No filler, loans, penalties or bankruptcy subsystem is added.

An offer is for one explicit airing. Current true demand values a provisional
start snapshot at its scheduled time, against the fixed competing schedule and
current knowledge/state, without replaying intervening reception or guaranteeing
future solvency. Any future unstarted player airing can be quoted. Earlier
broadcasts can change actual delivery or exhaust cash before that airing.
Forecast, promised units, actual units and allocated money remain distinct.

Rust previews acceptance on a copy to expose the exact frozen per-audience
terms. Acceptance reserves the existing capped integer amount in advertiser
escrow. It locks that airing's **programme, start, duration and advertiser**;
move/remove/advertiser changes are rejected. There is no cancellation action.
Every audience retains its own forecast, promised micro-impressions and allocated
payment, with the existing ordered rounding and no substitution across groups.

Actual delivery is recorded from the start sample at t+1; the inclusive deadline
is the airing's **end**. Escrow remains reserved until that boundary. Underdelivery
or an unfunded airing refunds the unpaid allocation under the existing exact
integer rule. Settlement happens once. A later hype change cannot reprice the
contract. One selected prospective advertiser airs no ad until accepted.

Any successful schedule edit, production command, acceptance, advance or reset invalidates a pending
offer; a new quote replaces it. Rejected commands preserve a still-valid offer.
Player airing IDs are not reused after removal or reset; the fixed competitor
schedule is recreated on reset. Opaque offer tokens also retain
session namespaces/counters across reset, preventing accidental stale reuse.

## Browser protocol and observation boundary

The planning revision replaced slot indices with airing IDs and explicit tick
boundaries. Current protocol **v8** includes production commands and pooled Sway; old versions are rejected. Authoritative
Rust DTOs live in [`protocol.rs`](../crates/bts-web/src/protocol.rs); TypeScript
mirrors only that player contract. There is no developer-export or save schema reuse.

```json
{"api_version":8,"command":{"type":"place","programme":1,"start_tick":72,"advertiser":null}}
```

| Command | Fields | Target |
| --- | --- | --- |
| `inspect` | none | Current player observations |
| `place` | programme, start_tick, optional advertiser | New airing, ID assigned by Rust |
| `move` | airing_id, start_tick | Same airing; runtime is retained |
| `remove` | airing_id | Remove an editable airing |
| `set_advertiser` | airing_id, optional advertiser | Prospective advertiser for that airing |
| `request_offer` | airing_id | Quote that airing, regardless of which is next |
| `accept_offer` | airing_id, offer_id | Both must match the pending Rust quote |
| `advance` | to_tick | All intervening intervals through the explicit boundary |
| `reset` | none | Fresh initial simulation, plans and reports |

Response envelopes retain `status`, `api_version` and either `report` or a
structured `error {code,message}`. Useful codes distinguish overlap, past/started
edits, protected bookings, competitor edits, invalid time, missing/unknown
advertisers/airings, unpayable or stale quotes, malformed JSON/types and versions.

The player-report whitelist still excludes exact audience preferences,
availability, purchasing power, interests, hype/desired hype/deficits, repetition,
hidden execution, fuzzy rules/traces and familiarity/attachment stocks. It adds
known runtimes, airing IDs/intervals/status, explicit draft gaps, own financial
transactions and airing-linked terms/results. It exposes no other station's
balances or brand budgets. Quotes and counts are labelled **provisional**;
there are no measured ratings, research estimates or arbitrary noise. Client-side
state filtering is not a security guarantee and no developer inspector is exported.

All financial amounts, u64 contract IDs and integer micro-impressions cross JSON
as decimal strings. Cumulative money totals use exact integer accumulation and
also remain strings. TypeScript uses `BigInt` only for formatting, never pricing,
accounting or settlement. Safe u16/u32 entity/airing IDs and tick numbers use
JSON numbers; expectations remain finite floating numbers.

## Running and evidence

Build/run instructions remain in [browser foundation](browser-foundation.md).
From the root, `npm --prefix web run preview -- --host 0.0.0.0` serves the already
built bundle on port 4173. The browser tests use separate port 4174 and leave a
manual preview alone. Rebuild after Rust/TypeScript changes.

```sh
cargo run -p bts-web --example planning_evening > /tmp/planning-evening.json
```

The example plans comedy at 18:00–18:30, science at 18:30–19:15 and a film at
20:00–21:30, leaving a 45-minute explicit gap. It books Everyday Snacks on the
film before advancing the day. All offers, settlement and results are Rust-owned.

| Evidence from this fixed evening | Result |
| --- | ---: |
| Broadcast costs: 150 + 50 + 100 | 300 credits |
| Film contract maximum | 1,046 credits |
| Paid at 21:30 / refunded | 996 / 50 credits |
| Cash at midnight | 20,696 credits |
| Player costs recorded | 3, one per airing |

The provisional forecast underdelivers after intervening viewing changes
history, so it does not receive its full maximum. The 50-credit refund returns
to the advertiser; it is not a station expense. The controls establish linked
planning/accounting behaviour, not balance or prediction calibration. See
[committed evidence](../contrib/programme-planning/README.md).

Local validation passed 81 native workspace tests, five doctests, Rustfmt,
native/Wasm Clippy and native Rustdoc with warnings denied, all four legacy CLI modes,
both original examples, the new planning example, native/Wasm target builds and
the full outside-workspace `bts-fuzzy` extraction checks. Four real Chromium tests
exercise a varied evening, overlapping edits, move/removal, selected-versus-next
booking, gap progression, results/finances, mobile controls, initialization failure,
reset/session isolation/stale commands and native/Wasm player-protocol parity.
Financial strings and contractual integer units compare exactly; floating
expectations compare within 10^-8. Some Rust LSP feedback was stale and the
TypeScript server unavailable; Cargo and `tsc` provided semantic verification.
CI uses current stable Rust (1.100 at this milestone), newer than local 1.98.1.
The complete native tests/doctests, Clippy, docs, standalone extraction and
browser checks were also rerun with 1.100. Its deprecation of the original atomic
session-ID helper was addressed with
a checked compare-and-exchange loop, preserving namespace isolation and overflow
handling without depending on the renamed API. The fixture availability update
uses iteration to satisfy current Clippy, and wasm-bindgen is a Wasm-only
dependency so native builds do not report an unused dependency.

## Future public rival schedules

The daily planner should support multiple competitors with one optional
comparison lane. That lane shows publicly observable past broadcasts and
future airings the rival has both committed to and published. Private drafts,
unannounced commitments, and internal changes remain hidden. Where no future
listing is public, show **No listing published**. Reserve **Off air** for
confirmed information. Exact rival schedules remain available to the
simulation and developer tools. Player-facing forecasts should use public
listings and express uncertainty where information is missing.

Publication timing and the handling of public revisions or cancellations
remain open. The current prototype still shows its fixed rival schedule.

## Limits and next milestone

The initial-snapshot model does not measure retention, duration-weighted exposure,
late joiners, mid-programme satisfaction changes or unique reach. Continuations
can lose/gain an instantaneous audience without changing that airing's fixed
reception sample. Shorter works permit more implicit ad opportunities per hour;
this is not a balanced commercial-load model. Repetition-blind advertiser pricing,
ordered payment-rounding bias and fixed fixture parameters remain provisional.

Publication/listings promises, promotion, campaign shares/ad breaks,
research, broader society, save/load and a wall-clock-continuous simulation are deferred; browser Play/×2 now drives explicit ticks.
The Control Desk now provides three-week navigation, a narrow agenda and a weekly placement form. See [weekly scheduling](weekly-scheduling.md) for recurring slots and occurrence-level protections. A separate full-week grid remains open. History is in memory; advancement copies bounded
state for atomicity. Chromium desktop/narrow-screen smoke tests are not full
cross-browser, mobile-device or assistive-technology validation. No deployment
is performed. Next UI work should use human planning sessions to improve the
day/catalogue/details/results flow and clarify information sources alongside the bounded production workflow.
