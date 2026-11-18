# Ad-minute market

The browser's **Sell ad time** panel lists space on one planned airing. A listing
can hold 0–30 whole minutes. A brand can buy part of the listing, and multiple
brands can pay for distinct portions. The current browser fixture has two brands.
The legacy delivery contract remains available on airings without a minute listing.

## Schedule and booking

Exact runtime is programme minutes plus listed minutes. Occupied ticks are
`ceil(exact runtime / 15)`. Unsold space and rounding padding are reserved schedule
time without paid exposure. The player can remove unsold minutes to reclaim
whole ticks. The planner rejects same-station overlaps and reservations beyond
the scenario. A sale protects the airing's start, programme, and identity.
The station can resize the block while it is still planned, subject to sold
minutes and schedule occupancy. Legacy contracts and minute sales cannot share
an airing. The model does not simulate the order of spots.

A quote batch may contain more requested minutes than the station has listed.
Accepting one offer reserves its exact credits and minutes and produces a fresh
batch for the remaining space. A brand buys at most once per airing. Quote
tokens are local to their session and airing, and any later state change
invalidates the old quote. Rust verifies the quote and budget again before
accepting. Money, inventory and schedule changes are atomic.

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

An airing that runs samples actual viewer allocation once, at programme start.
For a purchased block of `m` minutes, report `viewer count × m` viewer-minutes
and apply `m / 5` ad opportunities per sampled viewer to the existing hype
and repetition model. Unsold minutes generate no exposure. This dose is not
unique reach, actual time spent watching, or measured impressions. A funded
airing with no viewers generates no hype.

At the end of the reserved airing the station earns the frozen price for
each sale if the airing ran. If it went off air, every sale releases its
escrow to the original advertiser. The current airing model is all-or-nothing.
Low audience delivery has no effect on payment for minute sales. Each sale
settles once, and the ledger includes reserves, payments and releases.
Legacy delivery contracts retain their audience-specific refund rules.

Player reports show estimated target viewers, offers, agreed minutes and
prices, provisional audience counts and settled outcomes. Developer evidence
includes accepted quote inputs, timestamps and viewer-minute delivery. Measured
ratings, audience guarantees, ad-load effects on viewing, placement, and
station promos are future work.

[Walkthrough, screenshots and comparisons](../contrib/ad-minute-market/README.md)
