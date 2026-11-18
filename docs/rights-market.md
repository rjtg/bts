# Programme rights market

The browser **Rights market** sells exclusive, perpetual rights for finished
standalone works. Two archive titles open immediately. **Night Chase** arrives on
Day 3 at 00:00. Public notes distinguish sparse archive evidence from public
cinema reception. The art illustrates the listing and carries no simulation
attributes. A purchase adds the title to the station library immediately.
Repeat airings cost the published per-airing amount and can sell ad minutes.

## Offer, deadline and rival use

A release freezes its integer asking price. The provisional base is the authored
title fee plus `floor(estimated 18:00 viewers / 5)`. The base gains a 20% premium
rounded down when the rival can cover the premium price and one broadcast cost,
has at least 2,000 baseline forecast viewers, and has an eligible gap after the
deadline. At the deadline the same eligibility check runs at the listed price
using current finances and schedule. A successful rival purchase transfers cash
and rights, then books the earliest eligible gap. An unsold title closes.
All offers use an exclusive expiry tick. If time reaches that tick, the player
can no longer accept the quote.

The rival checks 14:00, 15:00, 18:00 and 21:00 across the remaining scenario.
Eligibility requires a complete free interval after the current tick, so any
existing airing or contract is protected. The fixture has afternoon
availability for the first two times and retains its preplanned evenings.
This single-step policy uses broad fit and affordability. It does not forecast
profits, manage a portfolio or reschedule the rival's existing commitments.

The player can request a resale bid on a market title they own. The rival
offers 70% of the original price, rounded down with a minimum of one credit,
when the same affordability and scheduling conditions apply. It expires after
24 simulated hours. Acceptance rechecks eligibility and can never interrupt an
airing or invalidate a future schedule or advertising commitment. If the bid
expires, the player retains the rights. Past airings and transactions remain
visible after resale. Initial sales pay the external rights seller account.
Resale transfers exact integer credits from the rival to the player.

## Evidence and Insight

A public title profile and execution estimate seed its projection. Each
group's benchmark reach is estimated population × 18:00 availability × 0.65
× `(0.65 × feature fit² + 0.35 × estimated execution)`. The browser uses the
latest published audience snapshots. This is a benchmark without named
competition, rather than an airing or advertiser forecast. An airing still
uses the same actual allocation, repeat history and ad settlement models as
other programmes.

A title assessment costs a chosen amount of the shared station Insight balance.
The spend is immediate and recorded by title. It publishes three simulated
hours later, after any audience poll due at that boundary. Cumulative completed
Insight `I` moves the authored public estimate towards the underlying
programme by `w = 0.80 × (1 − 0.83^I)`. A new assessment may raise or lower
reach, with diminishing changes. The illustrative uncertainty scale is
`0.12 + 0.25 × (1 − w/0.80)`. Players see qualitative uncertainty, feature
bands, and suggested viewer counts rounded to hundreds. Exact actual
execution and private model inputs stay inside Rust. Audience polls can also
move the benchmark subsequently. Assessment does not change a frozen price.

Private assumptions, prices and timings are provisional. The first market
omits auctions, windowed rights, territories and third-party resale. The
fixture does not establish economic balance. See the
[walkthrough and comparisons](../contrib/rights-market/README.md).
