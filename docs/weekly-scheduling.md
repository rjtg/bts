# Weekly scheduling and audience flow

The Control Desk slice implements a 21-day browser calendar, recurring placement,
and weekday-aware viewing preference. Ads remain manual and viewing continues
to sample at the start of an airing. The 15-minute clock, exact accounting,
airing identity and player-information boundary still apply.

## Recurring slots

The browser calendar starts on Saturday and spans three weeks. Give an owned
library title a weekday, minute of day, default ad minutes and one to three dates.
The first date is the next matching time at or after the current minute.
Subsequent dates advance by seven days. The review shows exact programme-plus-ad
runtime, occupied interval, overlapping airings and out-of-calendar dates.
Before confirming, skip a date or give it a different day and time.

Confirming inserts the whole valid plan atomically. Each occurrence keeps its
own airing identity, schedule, ad inventory, bookings and results. The browser
holds a read-only preview and requires a fresh review after intervening commands
change the schedule or clock. For an existing airing, single-date move and remove
use normal planner controls. **Move future occurrences** previews one new time
of day across that airing and the later dates in its run. Dates and individual
ad minutes stay as they were. Protected and past airings retain their terms.
When an editable date conflicts, the entire edit is rejected.

Recurring slots are internal plans for a fixed library title. Automatic episode
selection, monthly recurrence and published listings remain future work.

## Optional autosell (proposal)

A recurring slot can opt into automatic sales with a minimum price per minute.
The policy uses the existing minute market on each eligible airing. Manual
bookings remain protected and count against its available minutes. Show what
sold, at what price and what stayed open before broadcast. Unsold time remains
listed unless the player chooses an explicit trim policy. Never shorten below
accepted sales. Exact occupancy and all escrow, budget, settlement and ad
exposure rules still apply.

Choose a predictable quote and acceptance time so players have a chance to
make manual deals. Fix the offer ordering and tie break for reproducible runs.
Keep weak offers visible for manual consideration where practical. Selling
automatically should leave the same ledger evidence as a manual sale. Timing,
sort policy and price floor units still need a bounded experiment.

## Preferred viewing times and habit

Availability still determines who can watch. Six authored preferences per
group cover weekdays and weekends, each split into 00:00–12:00, 12:00–18:00 and
18:00–24:00. A value of 0.5 is neutral. At a sampled interval start, the engine
multiplies the outside-option utility by `1.5 − preference`. This changes
the tendency to watch either station while preserving their relative appeal.
Zero availability still means zero viewers. At 23:45 the current day applies
even though the interval closes at midnight. Headless callers without a
configured viewing calendar retain neutral behaviour.

The shared survey publishes a plain-language watch-time note with the same
timestamp and uncertainty as the other audience estimates. The forecast engine
uses the latest published estimate when quoting ads; actual allocation uses
the authored preference. Estimates use the existing deterministic bias drift
with separate channels for the six new preferences. The note names the
estimated strongest band when the spread is at least 0.15. Raw values stay
inside Rust. [Controlled results](../contrib/weekly-planner/README.md)
compare the same 18:00 programme and competition with fresh histories.

### Slot habits (proposal)

A weekly habit belongs to an audience group, station, recurring show and
slot. Let it build gradually from actual viewing and good reception in that
slot. Missed weeks or disappointing airings weaken it. A habit helps explain
why players protect a regular appointment. Separate it from loyalty to a show,
which may follow the work to another time. Specify the tolerance for moves,
rate of buildup and decay, and how missed or skipped occurrences count before
implementation.

## Audience carryover between airings (proposal)

Some viewers already tuned to a station may give its next programme a chance.
At the handoff, use the prior viewing estimate by group to give staying a
bounded advantage alongside the choice to switch or stop watching. This should
be strongest for a short handoff and groups that also find the next programme
appealing. An off-air gap should weaken it. Extra advertising time should never
create more carryover just because a block is longer.

Combine carryover with new arrivals in one allocation. Cap returning viewers by
the previous cohort, current availability and resulting allocation. Reallocate
within each group across stations and not watching so the same viewer is counted
once. Use a consistent snapshot for simultaneous starts. The start-snapshot
model can show who gives the next show a chance. Estimating mid-show departures
requires a later viewing cadence. How to estimate the outgoing cohort, combine
it with slot habit and decay it across gaps remains open.

## Suggested validation

The implemented fixture tests recurring placement, exceptions, protected airings
and weekday-aware preference. Autosell remains a separate follow-up.
Use several simulated Saturdays to isolate habit from series loyalty and
repeat fatigue. Use the same following programme at the same time with
different lead-ins and gaps to isolate carryover. Check conservation and
reproducibility across those matched runs.
