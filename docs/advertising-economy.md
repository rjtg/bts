# Advertiser-funded headless economy

Status: current schema-3 provisional implementation specification. This is a small experiment,
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
Diagnostic pressure D = sum(p*d)/sum(p), or 0 for zero total priority. This
brand-wide summary is never multiplied by delivery to price offers. Priorities
need not sum to one. Valuation uses current actual hype, not a player estimate. Future
hype decay is not speculated into demand; explicit time advancement updates it.
Reports disclose each priority, desired/current hype, deficit and contribution.

Forecast audience delivery uses current known programme attributes, loyalty,
availability, rival schedule and ad effectiveness, via a read-only preview of
the proposed slot after resolving current cash availability. Per-audience
relevant delivery F_i=expected viewers_i*p_i*effectiveness_i; total F=sum(F_i).
Valuation V_i=F_i*d_i. Uncapped price=sum(rate/1000*V_i), in target order.
Offer=floor(min(available budget, maximum offer, uncapped price)). Valuation
rows are separate from delivery forecasts. A satisfied reached target has
zero value even if another, unreached audience has unmet demand. No caps can
be exceeded by deficit-driven valuation. See [hardening controls](advertiser-hardening.md). A zero-money
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

At each action: validate the whole plan, then resolve each station's ability
to pay from pre-slot cash. Unaffordable planned programmes go off air with an
inspectable funding record. Resolve audience allocation, satisfaction/loyalty
and ad hype against only the aired broadcasts. Charge those broadcasts' costs
to the cost sink; record booked delivery; settle all contracts due at/before
now; commit. A cancelled programme has no viewing, learning, cost, ad exposure
or campaign effects. Other stations continue; time and passive decay advance.
Missing contracted delivery follows the same refund rule. Same-slot receipts
can fund later broadcasts, but cannot retroactively fund this slot. Malformed
plans remain atomic errors, including unknown IDs or conflicting booked ads
on an unfunded station. Valid but unfunded plans succeed with off-air events.
Quotes use the same current-cash resolution and never assume loans or future
receipts; an already unfunded station has zero forecast delivery/value.

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

Current six-day results for the first station, before the idle period:

| Advertiser / programming | Viewer-slot impressions | Actual weighted relevant delivery | Revenue | Broadcast costs | Profit |
| --- | ---: | ---: | ---: | ---: | ---: |
| Mass / broad | 89,570 | 45,143.84 | 2,955 | 1,800 | 1,155 |
| Mass / targeted | 25,196 | 11,691.84 | 2,351 | 600 | 1,751 |
| Specialist / broad | 89,570 | 3,426.01 | 2,640 | 1,800 | 840 |
| Specialist / targeted | 25,196 | 6,975.60 | 2,954 | 600 | 2,354 |

Relevant delivery in this table retains fractional expectations for inspection;
contract promises/credits are separately floored whole units. Forecast and
actual delivery coincide in these known, unchanged schedules. A regression
changes the actual programme after acceptance and verifies partial payment;
missing ads/broadcasts refund everything, while overdelivery earns no bonus.
These are not evidence of prediction accuracy under uncertainty.

Broad mass programming supports a mass advertiser with substantially more
relevant delivery and higher final population-weighted snack hype (35.00 vs
18.44). Specialist programming supports the equipment advertiser with fewer
total viewers but more than twice the weighted relevant delivery. Both broad
and targeted schedules are profitable for the first station here. The rival
is deliberately unadvertised in every case and spends 1,200 credits; it is a
fixed competitive control, not an economically optimized station.

Campaign success reduces diagnostic pressure: mass/broad falls from .3381
to .0239; specialist/targeted from .3846 to .0152. After 48 idle hours these
recover to .1918 and .2249. Per-audience values, not pressure D, price the
quotes. The same future-slot offer rises 128 -> 1,062 credits for mass/broad
and 31 -> 494 for specialist/targeted with unchanged forecast delivery and
budgets of 17,045 and 17,046. Recovery quotes are not accepted. Positive
recovering deficits cannot replenish an exhausted budget.

Audience and station-profit rankings differ. Changing only broad programme
cost from 150 to zero yields mass profit 2,955, exceeding targeted profit
1,751 without changing allocations, delivery, hype or revenue. Costs and
rates remain provisional. The [advertiser-value experiment](advertiser-hardening.md)
now isolates saturation, repetition and creative quality; repetition-blind
pricing can pay the same for less realized hype gain and preserve future
willingness to pay. This is a documented incentive gap, not evidence of balance
or a proven optimal strategy. The previous schema-2 snapshot at commit
9f10d5f in `contrib/advertising-economy` is historical brand-wide valuation.

Every action preserves 80,000 total credits across available budgets, station
cash, outstanding escrow and the cost sink. Ledger replay reconstructs every
account with nonnegative balances. End-of-run escrow is zero. All forecasts,
priorities, deficits, actual audience delivery, settlements, and transfers are
available in developer JSON schema 3 and the verbose CLI.

## Validation and remaining limits

Local checks on Rust/Cargo 1.98.1 passed 48 integration tests and five doctests,
formatting, Clippy with warnings denied, and Rustdoc with warnings denied.
Both CLI modes and both examples ran successfully. All fuzzy-crate checks also
passed offline after copying that crate outside the workspace. Fuzzy source
and manifest are unchanged; its README only drops obsolete CI status wording.
The [valuation snapshot](../contrib/advertiser-value/README.md) records current
controlled value comparisons. Original schema-1 broadcasting and schema-2
economy artifacts remain explicitly labelled historical.

Checks include formatting, workspace tests/doctests, Clippy and Rustdoc with
warnings denied, both runnable examples, both CLI JSON exports, and extraction
checks for a copied standalone `bts-fuzzy`. CI retains all standalone checks
and runs all three CLI exports, including advertiser-value. Existing conservation/reproducibility tests
remain; new tests cover varied and empty collections, independent ad factors,
conditional interactions, inventory, budgets/escrow, integer refunds,
deadlines, ledger replay, failed-action atomicity, target priorities, demand
feedback/recovery and economic sensitivity. JSON exports reproduce byte for
byte on the same platform. Compiler/Cargo checks backstop stale LSP feedback.

This is a weighted-aggregate target-delivery contract, not separate minimum
quotas per audience. There are no auctions, credit, cancellations, late
makegoods, inventory rescheduling, replenishing budgets, advertiser sales,
ongoing station overhead, tax, inflation or insolvency proceedings. Integer
cash cannot go negative: an unaffordable broadcast goes off air for that slot. Programme
attributes and tastes are fixed; forecasts know proposed schedules and do not
predict intervening changes. Reports disclose true state; fair player evidence,
research costs and information uncertainty remain open. The fixtures establish
causal/accounting properties, not realistic economics, long-run balance or a
complete season game.
