# Minute airtime and the planning desk

## Agreed direction

Schedule half-open intervals in whole minutes. A programme beginning at 18:00
with 45 minutes of content and nine advertising minutes ends at 18:54. The next
programme may start then. Clock controls can still advance by a quarter hour.
Within that quarter, split the lineup at exact boundaries. Use the quarter's
audience inputs for each lineup and weight exposure by the segment's minutes.
Charge each airing once and settle its purchases once. Promotional fillers and
teasers use [their own intervals](open-questions.md#one-minute-fillers-and-teasers).

Keep the shelf for selection. Put titles vertically inside cassette spines and
filter by programme runtime. **Fill Next Gap** is a temporary view ranked by
estimated turnout at the gap. Define the next gap from the selected planner day
and position, never from a past minute. Use ad-inclusive runtime to assess
whether a tape fits, and keep unranked works visible with an evidence label.
Only published competitor information belongs in [rival comparisons](programme-planning.md#future-public-rival-schedules).

A selected tape opens its dossier alongside it. Put **Next Gap**, **Best Slot**
and **Regular Slot** on the dossier. Show the proposed time before committing.
Best Slot compares upcoming openings using player-visible survey evidence.
Regular Slot uses a date preview. Monthly recurrence remains a proposal even though Month can summarize the
current calendar.

The ad-load control sets a default ratio for new airings. Show the resulting
whole minutes and total airtime before booking. Existing ad inventory and
signed contracts keep their own terms. The scheduled-airing dossier includes
ad offers and accepted terms. Signing commits the exact contract. Striking
through an offer dismisses it without selling minutes. Open the same Airing file for past results. Put actionable reminders for gaps
and unsold inventory in the station notebook and link to the relevant paper.

## First implementation

Broadcast intervals, moves, gaps, recurring starts and exceptions, and
contract settlement use whole minutes. Legacy browser requests that use
quarter-hour ticks still decode. The player report adds authoritative minute
fields alongside the older tick fields. The timeline ruler remains marked at
quarter hours. Drops use minute coordinates, with a two-minute magnetic region
near quarter marks and adjacent airing boundaries. Shift+Left or Shift+Right
on the timeline adjusts the candidate by a minute.

The engine steps through the next quarter boundary, airing start or end, or
purchased spot boundary, whichever occurs first. Availability and research
retain their existing cadence. Each segment conserves its audience allocation.
Purchased spots are consecutive after the programme, in acceptance order; the
unused part of the ad block stays at the end. Sum viewers × elapsed minutes
during each purchased spot. An airing that was unfunded releases its escrow
and records no purchased impressions. Legacy delivery contracts still use a
single opening sample. Programme recap ratings also use an opening sample
and do not add viewers across sequential segments.

The default ad-load dial accepts 0–50% of content minutes. Round to the nearest
whole minute, with halves rounded up, and cap at 30 minutes. The default is
initially zero and changes only future placements. Weekly plans copy it into
the editable preview. The six-week season supports weekly recurrence. Month is a browsing view,
and monthly recurrence remains proposed.

The desk shows one selected title or airing dossier. An unsold inventory
note opens the scheduled airing and a completed airing opens its ledger.
Striking out an offer dismisses it locally, with **Restore offer** available.
Only signing an offer sends an acceptance to Rust.

Fill Next Gap uses ad-inclusive runtime and a provisional ranking from dated
survey tastes, group size and preferred daypart. For each suitable group,
weight its surveyed population by its mean fit on observed strong or moderate
qualities, then multiply by 1 on its published preferred daypart or 0.65
otherwise. Add those contributions as a comparative score. Works with no
applicable survey fit remain unranked. Best Slot compares the same score at
gap starts and hourly boundaries within seven days, breaking ties in favour
of the earlier time. These scores are decision aids. The Audience paper
retains its existing title assessment and historical counts where available.
Public rival schedule uncertainty and a calibrated slot forecast remain open.

Short openings of up to five minutes offer a one-minute ident or an authored
teaser for the next scheduled show. Fillers have no ad inventory or
commercial receipts and no automatic lead-in bonus. If the teaser target
was moved or removed before the teaser runs, the visible plan and completed
broadcast show an ident. Viewer allocations are still made for its own
one-minute segment. Anticipation and promotion effects are deferred.
