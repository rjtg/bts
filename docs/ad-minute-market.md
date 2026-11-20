# Ad-minute market

The teal **Airing** file lists ad space on one planned airing. A listing
can hold 0–30 whole minutes. A brand can buy part of the listing, and multiple
brands can pay for distinct portions. The current browser fixture has two brands.
The legacy delivery contract remains available on airings without a minute listing.

## Schedule and booking

Exact runtime is programme minutes plus listed minutes. Unsold space is real
schedule time with no paid exposure. Removing unsold minutes reclaims those
exact minutes. Rust rejects same-station overlaps and intervals outside the
scenario. A sale protects the airing's start, programme, and identity. The
station can resize a planned block around accepted sales and existing airings.
The legacy delivery contract uses separate inventory. See
[minute airtime](minute-airtime.md) for the station default and exact boundaries.

A quote batch may contain more requested minutes than the station has listed.
Accepting one offer reserves its exact credits and minutes and produces a fresh
batch for the remaining space. A brand buys at most once per airing. Quote
tokens are local to their session and airing, and any later state change
invalidates the old quote. Rust verifies the quote and budget again before
accepting. Money, inventory and schedule changes are atomic.

Eligible [recurring slots can opt into autosell](weekly-scheduling.md#optional-autosell)
using the same per-airing commitments and settlement rules.

## Provisional pricing

Let `U` be the existing uncapped target value for the airing: sum the relevant
viewing forecast per group weighted by brand targeting priority, expected
effectiveness, group hype deficit and the brand's rate per thousand. Browser
quotes use the latest published audience estimates. They use current history
and do not simulate intervening broadcasts before the prospective airing.

A brand asks for up to `ceil(1 + 5 × demand pressure)` whole minutes, capped at
six, the remaining inventory, and the number its budget can buy at the unit
rate. The provisional unit rate is `U / (5 × (1 + listed minutes / 15))`.
A brand with some budget but less than the full unit rate can bid its capped
amount for one minute. Total price is rounded down once to integer credits
after applying the brand maximum offer and remaining budget. Zero-credit
offers are omitted. A smaller listing may command more per minute, while
offering less inventory to competing brands. The figures are fixture
comparisons and have no claim to balance.

## Exposure and settlement

The recap samples actual viewing at the programme's start. Purchased spots run
after the programme in acceptance order. Split at spot and lineup boundaries
within quarter-hour intervals. For each purchased segment of `m` minutes,
record `viewer count × m` viewer-minutes and apply `m / 5` ad opportunities per
segment viewer to hype and repetition. Unsold minutes generate no exposure.
These numbers are expected viewer-minutes, distinct from unique reach and
measured impressions. A funded airing with no viewers generates no hype.

At the end of the reserved airing the station earns the frozen price for
each sale if the airing ran. If it went off air, every sale releases its
escrow to the original advertiser. The current airing model is all-or-nothing.
Low audience delivery has no effect on payment for minute sales. Each sale
settles once, and the ledger includes reserves, payments and releases.
Legacy delivery contracts retain their audience-specific refund rules.

Player reports show estimated target viewers, offers, agreed minutes and
prices, provisional audience counts and settled outcomes. Developer evidence
includes accepted quote inputs, timestamps and viewer-minute delivery. Measured
ratings, audience guarantees, ad-load effects on viewing and player-controlled in-break placement remain future work.

[Walkthrough, screenshots and comparisons](../contrib/ad-minute-market/README.md)

## Placement in the office interface

Ad sales belongs to scheduling. Set or review block length when placing a
programme, then open the scheduled airing's teal dossier to compare offers
and sign sales. An unsold-ad desk note is a shortcut to that file. The same
file gains provisional turnout and settlement after transmission. This is a
presentation change to the existing minute market and commitment rules.
See [the playable Station Office hub](station-office.md#playable-office-hub).
