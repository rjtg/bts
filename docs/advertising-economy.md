# Advertiser-funded headless economy

Status: provisional implementation specification. This is a small experiment,
not a market framework, calibrated economy, or finished player interface.

## Review corrections

All audience/station/programme/brand collection sizes are allowed, including
zero. An empty catalogue permits only idle stations; absent brands permit only
unadvertised broadcasts. Empty audiences give zero delivery, no demand, and no
paid offers. Targets must reference existing audiences, with unique IDs.
Availability has 24 hourly fractions. Any strictly increasing integer simulated
hour is valid; 18:00/21:00 and the small entity counts belong only to demo data.

Keep existing fuzzy features, high/low ramps and complementary 0/1 rule pairs.
Attraction adds a pair (weight .25) for high genre fit AND high cast appeal:
A = (previous linear A + .25*min(genre,cast))/1.25. A popular cast's appeal is
more useful where the audience likes the genre. Satisfaction adds a pair
(weight .30) for high genre fit AND high execution:
S = (previous linear S + .30*min(genre,execution))/1.30. Execution and fit must
work together; neither alone assures enjoyment. Tests compare equal linear
scores with different joint conditions and retain gradual-overlap checks.

Ad effectiveness = .75*min(interest,affordability,creative)
                 + .25*min(interest,affordability,creative,satisfaction).
Each term is a complementary fuzzy pair with the stated weight. Programme
satisfaction can support an effective creative, but cannot substitute for one:
creative=0 always gives effectiveness=0, even at satisfaction=1. S=0 still
allows the base creative response. Interest, affordability, creative, and
satisfaction have independent controlled tests. Sugeno denominators remain
normalization terms, never confidence or probabilities.

## Brand demand and forecasts

A brand has finite initial marketing budget, maximum offer, integer rate per
1,000 weighted relevant impressions, and explicit audience targets containing
priority p in [0,1] and desired hype H* in [0,100]. Empty/zero-priority targets
produce no demand. There is no budget replenishment or credit.

At quote time, per-audience deficit d = max(0,H*-H)/100.
Demand D = sum(p*d)/sum(p), or 0 for zero total priority. Priorities need not
sum to one. Demand uses current actual hype, not a player estimate. Future
hype decay is not speculated into demand; explicit time advancement updates it.
Reports disclose each priority, desired/current hype, deficit and contribution.

Forecast audience delivery uses current known programme attributes, loyalty,
availability, rival schedule and ad effectiveness, via a read-only preview of
the proposed slot. Relevant delivery F = sum(expected viewers*p*effectiveness).
Offer = floor(min(available budget, maximum offer, rate*F/1000*D)).
No budget or demand multiplier can exceed the independent caps. A zero-money
or zero-whole-delivery offer cannot be accepted. Forecasts are deterministic
model estimates, not guarantees: changing programmes, competitors or availability
of an actual broadcast may underdeliver. The actual action is supplied separately.
Quotes are recomputed on acceptance, including revision, inventory, budget and
forecast; stale quotes and inconsistent prices or delivery terms are rejected.

## Contract, money, deadlines, and update order

Money uses nonnegative integer credits (`Money(u64)`). Configured amounts are
capped at 10^12 credits for exact conversion to floating point in pricing.
Account additions/subtractions are checked; global conservation uses `u128`.
Prices are rounded down once after caps. No floating-point money is stored.

One contract reserves one station/hour ad slot, with a brand, an explicit
inclusive deadline at/after the broadcast hour, promised whole target delivery
T=floor(F), and a fixed maximum payment P. Acceptance transfers P from available
brand budget to escrow. A slot cannot be reserved twice, including by the same
brand. Acceptance and broadcast actions are atomic on failure.

Actual delivery X=sum(actual ad viewers*p*actual effectiveness) is recorded
only for that station, brand, and exact booked hour; credited units U=floor(X)
are rounded once after aggregation. Missing broadcasts or omitted ads deliver
zero, including when time skips the booking. A conflicting brand on reserved
inventory is invalid; unpaid/unreserved ads remain allowed for the original
hype-only experiments. No late makegoods or cancellation mechanism is provided.

At each action: validate action; compute audience/loyalty/hype from a common
snapshot; debit each airing's programme broadcast cost from station cash to a
cost sink; record booked delivery; settle all contracts whose deadline <= now;
commit the complete state. Costs require pre-settlement cash (no implicit loan).
An error at any stage commits nothing, including money, time, hype and ledger.

At the first action at/after deadline, pay floor(P*min(U,T)/T) to station cash
using exact u128 intermediate arithmetic. Return P-payment to brand budget.
Overdelivery earns no bonus. Underdelivery loses the corresponding fraction of
payment, with no extra penalty. Settlement status and retained slot reservations
prevent repeat payment, repeat delivery, and resale. Deadlines crossed by an
idle action also settle; no wall clock is consulted. Payment plus refund always
equals escrow. Brand available funds + unsettled escrow + station cash + cost
sink equals initial total money. Initial balances are configuration, not ledger
transfers; every later transfer has source, destination, amount, time and reason.

Higher actual hype reduces subsequent demand through the deficit equation.
Decay can restore demand, but cannot restore spent budget. Repeated campaigns
also have saturation and repetition effects from the broadcasting model. A brand
can remain interested but unable to pay. Sales/profits of advertisers are not
modelled; this is their exogenous finite marketing budget.

## Comparisons

`cargo run -p bts-cli -- economy --json /tmp/bts-economy.json` compares mass-market
and specialist advertisers under both broad and targeted schedules. Every run
starts with identical world, money and parameters; within each same-brand pair
only the first station's programme differs. The rival and dates remain fixed.
Each broadcast is quoted, optionally accepted, then aired independently. The
last campaign is followed by 48 hours idle and an unaccepted recovery quote
for the next 18:00 broadcast. Demand is measured at the end of the idle period;
the forecast uses the future broadcast's availability, not late-night availability.
Compare forecast versus actual delivery, brand spending/demand, total audience,
station cost/revenue/profit, and post-decay demand subject to remaining funds.
The fixtures use provisional costs and rates; no optimizer chooses a winner.
Reports expose actual internal state for developers; player research remains
unimplemented. No production, general market, politics, demographic evolution,
AI, save files or TUI is added. Publication/licensing are unchanged.

## Default experiment parameters and evidence

Both stations start with 20,000 credits; both brands start with 20,000 credits
and a maximum offer of 4,000 credits. No funds are replenished. Broadcast costs
are 150 credits for broad comedy, 50 for specialist science, 400 for the
promoted disappointment and 100 for rival drama. These invented costs are
independent of viewers and charged per actual airing, including unwatched ones.

| Brand | Rate per 1,000 relevant impressions | Priorities and desired hype |
| --- | ---: | --- |
| Everyday Snacks | 800 credits | Young: priority 1, desired 40; households: 1, 40; specialists: .1, 15 |
| Precision Equipment | 3,500 credits | Specialists: priority 1, desired 45; young and households: .02, 5 each |

Default six-day results for the first station, before the idle period:

| Advertiser / programming | Viewer-slot impressions | Actual weighted relevant delivery | Revenue | Broadcast costs | Profit |
| --- | ---: | ---: | ---: | ---: | ---: |
| Mass / broad | 89,570 | 68,145.51 | 2,462 | 1,800 | 662 |
| Mass / targeted | 25,196 | 11,691.84 | 2,272 | 600 | 1,672 |
| Specialist / broad | 89,570 | 3,426.01 | 2,652 | 1,800 | 852 |
| Specialist / targeted | 25,196 | 7,585.68 | 2,661 | 600 | 2,061 |

Relevant delivery in this table retains fractional expectations for inspection;
contract promises/credits are separately floored whole units. Forecast and
actual delivery coincide in these known, unchanged schedules. A regression
changes the actual programme after acceptance and verifies partial payment;
missing ads/broadcasts refund everything, while overdelivery earns no bonus.
These are not evidence of prediction accuracy under uncertainty.

Broad mass programming supports a mass advertiser with substantially more
relevant delivery and higher final population-weighted snack hype (44.36 vs
18.44). Specialist programming supports the equipment advertiser with fewer
total viewers but more than twice the weighted relevant delivery. Both broad
and targeted schedules are profitable for the first station here. The rival
is deliberately unadvertised in every case and spends 1,200 credits; it is a
fixed competitive control, not an economically optimized station.

Campaign success reduces immediate demand: mass/broad pressure falls from
.3381 to .0045; specialist/targeted falls from .3846 to .0008. After 48 hours
idle, pressure recovers to .1437 and .2016 respectively. Quotes for the exact
same future 18:00 slot before/after decay rise 23 -> 759 credits for mass/broad
and 1 -> 431 for specialist/targeted. Forecast delivery is exactly unchanged,
and remaining budgets stay 17,538 and 17,339 credits respectively. The recovery
quotes are not accepted or included in revenue. An exhausted-budget test shows
positive recovering demand still yields an unpayable zero-credit offer.

Audience and station-profit rankings differ under the default costs. A
controlled sensitivity test changes only broad comedy cost from 150 to zero:
its mass-market profit becomes 2,462 credits, exceeding targeted programming's
1,672, with identical allocations, delivery, hype and advertising revenue.
Thus the model does not establish one universal optimal schedule. Cost and
rate calibration are unproven. Lower-reach schedules can preserve urgency and
change repeated-campaign incentives; no advertiser optimization or empirical
claim justifies those incentives yet.

Every action preserves 80,000 total credits across available budgets, station
cash, outstanding escrow and the cost sink. Ledger replay reconstructs every
account with nonnegative balances. End-of-run escrow is zero. All forecasts,
priorities, deficits, actual audience delivery, settlements, and transfers are
available in developer JSON schema 2 and the verbose CLI.

## Validation and remaining limits

Local checks on Rust/Cargo 1.98.1 passed 37 integration tests and five doctests,
formatting, Clippy with warnings denied, and Rustdoc with warnings denied.
Both CLI modes and both examples ran successfully. All fuzzy-crate checks also
passed offline after copying that crate outside the workspace. Fuzzy source
and manifest are unchanged; its README only drops obsolete CI status wording.
The [comparison snapshot](../contrib/advertising-economy/README.md) records the
readable output and compact developer metrics/ledger. Original schema-1
broadcasting artifacts remain explicitly labelled historical.

Checks include formatting, workspace tests/doctests, Clippy and Rustdoc with
warnings denied, both runnable examples, both CLI JSON exports, and extraction
checks for a copied standalone `bts-fuzzy`. CI retains all standalone checks
and runs the new economy export. Existing conservation/reproducibility tests
remain; new tests cover varied and empty collections, independent ad factors,
conditional interactions, inventory, budgets/escrow, integer refunds,
deadlines, ledger replay, failed-action atomicity, target priorities, demand
feedback/recovery and economic sensitivity. JSON exports reproduce byte for
byte on the same platform. Compiler/Cargo checks backstop stale LSP feedback.

This is a weighted-aggregate target-delivery contract, not separate minimum
quotas per audience. There are no auctions, credit, cancellations, late
makegoods, inventory rescheduling, replenishing budgets, advertiser sales,
ongoing station overhead, tax, inflation or insolvency proceedings. Integer
cash cannot go negative: an unaffordable broadcast is rejected. Programme
attributes and tastes are fixed; forecasts know proposed schedules and do not
predict intervening changes. Reports disclose true state; fair player evidence,
research costs and information uncertainty remain open. The fixtures establish
causal/accounting properties, not realistic economics, long-run balance or a
complete season game.
