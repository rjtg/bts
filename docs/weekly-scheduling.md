# Weekly scheduling and audience flow

The browser calendar covers six weeks and supports recurring placement,
weekday-aware viewing preferences and optional autosell. Viewing continues to
sample at the start of an airing. The 15-minute clock, exact accounting,
airing identity and player-information boundary still apply.

## Recurring slots

The browser calendar starts on Saturday and spans six weeks. Give an owned
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

## Optional autosell

A fixed-title weekly run or a produced discussion run can opt in from any of
its Airing files. The player chooses a positive whole-Funds floor per minute
and a cap on total sold minutes per airing, including manual sales. Turning
the policy off affects future decisions and preserves signed deals.

Each eligible airing gets one check at the first processed quarter-hour
boundary within the final hour before airtime. An exact quarter-hour start
can use that boundary if it is the first opportunity. If a tape is not yet
published, it becomes eligible after delivery. At a check, the engine
quotes the ordinary minute market. It takes whole offers with sufficient
price and room under the cap, selecting highest price per minute first and
breaking ties by brand ID. Each accepted deal triggers a fresh quote.
The policy applies independently to each airing. A resize or move after an
unsuccessful check does not retry the sale.

The ordinary price, cash, escrow and settlement rules apply. The Airing
file labels automatically signed deals. Unsold time remains available for
manual selling or trimming.

## Station goals

The office notebook tracks three six-week goals using completed,
successfully aired items: broadcast in four different weeks, air three
recurring programmes, and earn paid ad-minute sales on three distinct
airings. These goals are progress markers and carry no reward or automatic
season-ending condition. The initial fixture's programming and finances
extend under the same rules. Persistence is planned for a later slice.

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

### Slot habits

A weekly appointment belongs to a group, station, recurring run and
15-minute weekly slot. Moves within the same slot retain the appointment.
Crossing its boundary uses the destination slot's own history. A different
run of the same programme has its own appointments.

At completion, a funded airing updates strength to 55% of prior strength
and 45% of sampled audience share of total population times satisfaction.
For each week without a successful edition in that slot, multiply the
stored contribution by 0.75 before reuse. Starting utility is multiplied
by `1 + 0.30 × strength` throughout the airing. All choices share the
same allocation denominator and remain subject to audience availability.

The tracker is internal. Programming and Reception indicate a prior
successful airing of the same run in the slot. They do not imply a
measured increase. Quote forecasts omit the realized habit multiplier.
In a controlled fixture with constant all-day availability, the stable
second airing draws about 3,864 young viewers versus 3,812 one quarter
later. The comparison does not calibrate actual viewing behaviour.

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
and weekday-aware preference. Autosell and slot habits have separate controlled tests.
Use several simulated Saturdays to isolate habit from series loyalty and
repeat fatigue. Use the same following programme at the same time with
different lead-ins and gaps to isolate carryover. Check conservation and
reproducibility across those matched runs.
