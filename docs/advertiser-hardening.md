# Advertiser valuation and off-air hardening

Status: provisional model specification. No claims of economic balance.

## Audience-specific value

For each target i, delivery F_i = expected ad viewers_i * priority_i *
fuzzy effectiveness_i. Current unmet demand d_i=max(0,desired_i-current_hype_i)/100.
Value-weighted delivery V_i=F_i*d_i, and the uncapped offer value is
rate_per_thousand/1000 * sum(V_i). Apply the existing available-budget and
maximum-offer caps, then floor once into integer credits. All terms remain
finite and inspectable. Brand-wide pressure remains a diagnostic summary and
has no role multiplying delivery. A satisfied reached audience contributes
zero value even when other, unreached audiences have large deficits.

Delivery F, valuation V, and settlement are distinct. Contract promises still
floor aggregate relevant delivery, and actual relevant delivery settles the
fixed accepted payment under the existing integer pro-rata/refund rules.
Deficits do not retrospectively reprice contracts. Aggregate guarantees still
allow substitution between target audiences; separate audience quotas remain
unimplemented. Desired hype and deficit use the quote-time actual state,
without pricing hypothetical future decay.

## Valid plans that cannot be funded

Validate the entire requested slot first, including unknown IDs, duplicate
stations, time ordering, and conflicting reserved advertisers. Malformed plans
fail atomically even if the affected station cannot afford to air them.

Then each station independently checks its cash against its proposed programme
cost, before any same-slot settlement. A programme it cannot fund goes off air
for that slot. It pays no cost and generates no programme evaluation, viewing,
satisfaction/loyalty learning, ad exposure or campaign hype. Other stations
continue. Recompute allocation over exactly the aired broadcasts plus outside
viewing. If all stations are off air, everybody is not watching. Global time,
passive hype/repetition decay, and due settlements still advance.

Reports preserve the original plan, the actual aired slot, and off-air records
with station, programme/ad, available cash and required cost. Forecasts use the
same funding resolution from current cash; they do not assume future receipts
or loans. A station already unable to air has zero forecast delivery and no
payable quote. A funded accepted booking can subsequently miss delivery if
other broadcasts exhaust its cash: the deadline refunds underdelivery exactly
once. There is no bankruptcy status, credit or automatic substitute programme.
Zero-cost programmes remain affordable at zero cash. Cash equal to cost is
sufficient. Time remains explicit and all transitions deterministic.

## Controlled advertiser-value experiment

`cargo run -p bts-cli -- advertiser-value --json /tmp/bts-advertiser-value.json`
compares low/high initial hype, fresh/repeated exposure stock, and strong/weak
ad creative quality. The fixture uses one audience/station/programme/brand,
fixed viewing availability, frozen loyalty and nonbinding budget/offer caps.
Initial stock interventions are confined to this developer experiment; there
is no state loader or public mutation API. All cases explicitly report their
initial hype, repetition, configuration and parameters. One simulated hour
elapses before airing; passive decay is reported separately from campaign gain.

Matched hype/creative cases vary only repetition. The current price formula
does not directly discount repetition or predict saturated marginal hype gain;
it values relevant impression delivery against audience-specific deficits.
The experiment reports predicted/actual delivery, payment, campaign gain,
credits per hype point, remaining demand and the next unaccepted quote. High
hype changes both deficit and saturation headroom; a repetition-only comparison
holds deficit and delivery fixed to isolate the remaining pricing blind spot.
Weak creative can preserve demand while also reducing payable relevant delivery.
These controls examine incentives; they do not establish an optimal campaign
or justify a market expansion.

## Observed matched controls

The fixture uses population 10,000; one station with 100,000 credits; one brand
with 100,000 credits, maximum offer 100,000 and rate 1,000 credits per thousand
relevant impressions. Target priority is 1, desired hype 90. Programme traits,
genre/tone fit, affordability and product interest are all 1. Availability is 1
at every hour, loyalty learning is zero, and each actual broadcast costs 10
credits. Half-lives are 48 hours for hype and 24 for repetition; campaign gain
is .8. Caps do not bind in these controls.

| Initial hype / repetition / creative | Relevant delivery | Value-weighted delivery | Paid credits | Campaign hype gain | Credits / hype point |
| --- | ---: | ---: | ---: | ---: | ---: |
| 5 / 0 / .8 | 6,779.66 | 5,762.71 | 5,762 | 39.8001 | 144.77 |
| 65 / 0 / .8 | 6,779.66 | 1,694.92 | 1,694 | 15.0423 | 112.62 |
| 5 / 4 / .8 | 6,779.66 | 5,762.71 | 5,762 | 9.9886 | 576.86 |
| 65 / 4 / .8 | 6,779.66 | 1,694.92 | 1,694 | 3.7751 | 448.73 |
| 5 / 0 / .2 | 1,694.92 | 1,440.68 | 1,440 | 12.0553 | 119.45 |
| 5 / 4 / .2 | 1,694.92 | 1,440.68 | 1,440 | 2.6020 | 553.41 |

Hype gain is exposure-driven gain after the separately reported one-hour
passive decay. Value uses quote-time hype, so rows with equal initial hype,
creative and reach have equal value. Increasing hype reduces headroom for
actual gain and lowers the offer through its audience's smaller deficit.
Increasing repetition alone reduces gain about fourfold but leaves delivery,
value and payment exactly unchanged. The repeated low-hype case retains a
post-campaign deficit .7508 versus .4527 for the fresh case; its next unaccepted
quote is 5,090 versus 3,069 credits. Both cases conserve 200,000 credits across
cash, budget, escrow and costs.

Thus preserving low hype increases willingness per equal relevant impression.
Repetition-blind pricing can pay equally for inefficient campaigns and leave
more immediate future demand. This is an incentive problem in the surrogate
valuation, even after removing cross-audience subsidy. Weak creative also
preserves deficit (.7302) but reduces qualified delivery and payment to 1,440;
its next quote is 1,237, below the strong fresh creative's 3,069. Inefficiency
is not uniformly profitable across factors. These one-step counterfactuals
do not prove that deliberately preserving hype is an optimal cumulative
station strategy. Finite budgets, costs, caps, timing, and alternative schedules
still matter.

Remaining assumptions are explicit: relevant impressions proxy advertiser
value; the price does not forecast marginal saturated/repeated hype gain or
sales; rates and desired hype are invented; buyers do not optimize campaign
frequency; contracts guarantee aggregate delivery instead of outcomes or
per-audience quotas. Stock interventions isolate sensitivities and are not a
claim that a planner can independently choose hype and repetition through a
real history. Marginal-uplift pricing or frequency policies would require
separate design decisions and experiments; neither is introduced here.

## Verification and reports

Developer schema 3 separates `offer.forecast`, `offer.valuation`, and actual
contract delivery. Slot reports contain `action` (requested plan), `aired`
(effective broadcasts), and `off_air` records. An off-air forecast has zero
viewers/delivery and no fictitious ad evaluation (`effectiveness: null`).
These reports expose actual state and are unsuitable for a player interface.

Two otherwise identical target groups verify that satisfied delivery is worth
zero while unmet-demand delivery is favoured. Reaching both groups increases
the delivery guarantee without adding value for the satisfied group. Accepted
settlement is tested separately: closing a deficit after acceptance does not
reprice a successfully delivered contract.

Funding tests cover either station and both stations off air, redistribution,
zero cancelled exposure/gain, unchanged cancelled-series loyalty, inclusive
cost thresholds, free broadcasts, refunds after cash is exhausted between
booking and deadline, conservation, reproducibility and same-slot receipts.
Unknown IDs, duplicate stations, invalid time and conflicting booked ads remain
atomic errors even on unfunded stations. Compiler/Cargo checks provide fallback
when LSP diagnostics are stale. CI retains the standalone fuzzy-crate copy
checks and adds the new CLI experiment/export.

Local verification on Rust/Cargo 1.98.1 passed 48 integration tests, five
doctests, formatting, Clippy with warnings denied, and Rustdoc with warnings
denied. Both generic examples and all three CLI modes/exports ran. A fresh
outside-workspace copy of unchanged `bts-fuzzy` passed formatting, tests,
doctests, Clippy, documentation and its example offline. The controlled
[JSON and readable snapshot](../contrib/advertiser-value/README.md) records the
new experiment. Source and manifest of `bts-fuzzy`, publication settings and
licensing remain unchanged.
