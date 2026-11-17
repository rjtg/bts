# Audience commitments and programme progression

Status: provisional implementation specification. This closes delivery
substitution and introduces a small inspectable episode model; it does not
establish balance, a viewing schedule planner or individual viewer tracking.

## Frozen audience commitments

Quote-time relevant delivery F_i, value F_i*deficit_i and payment remain
separate. At acceptance record each target's audience ID, forecast F_i,
priority, promised delivery and allocated integer payment. An audience with
zero quote-time value receives zero payment. No other audience can satisfy
its commitment or absorb its payment through overdelivery.

Delivery resolution is 1,000,000 integer micro-impressions per relevant
impression. Promise and actual delivery are each floored at this resolution
per audience, not at whole people. Positive F below one whole impression is
contractible. Below one micro-impression the row is explicitly uncontractible,
contributes no payable price, and receives no payment allocation. A quote with
no contractible value has payment zero. Aggregate whole-impression counts are
retained only as diagnostic summaries and never divide settlement payments.

Allocate capped integer payment by deterministic ordered remaining-budget
apportionment: sort eligible audiences by ID; for each except the last,
allocate floor(remaining credits * its value / remaining eligible value).
The last gets the remaining credits. Clamp each allocation to the remaining
integer budget to protect against floating rounding. Zero-value and
uncontractible rows get zero. This deliberately gives rounding remainders to
later IDs, can give small positive-value targets zero credits, and guarantees
allocations sum exactly to accepted payment. Price/relative value calculations
use finite f64; account balances and transfers remain exact integers.

At the deadline, each row earns floor(allocated_payment *
min(actual_micro_units,promised_micro_units)/promised_micro_units), using u128
intermediates. A zero allocation earns zero without division. Refund the
remainder of each row. Sum row payments/refunds for the existing escrow ledger
transfers and single settlement. Priorities, prices, forecasts and promises
are frozen at acceptance; later hype cannot change them. Actual qualifying
creative/context effectiveness still measures the broadcast actually aired,
using the same model definition; it does not use current hype deficits.

## Identity, familiarity and reception

A programme is either a standalone work or an episode carrying a series ID
and a positive sequence number. Series definitions contain stable identity,
name and continuity strength c in [0,1]. Work attributes and costs live on
standalone programmes/episodes. Validate references and unique (series,sequence)
pairs. Gaps are permitted; missing catalogue predecessors are unfamiliar.

Store actual aired broadcast history independently of audience/series
attachment. Per audience/work familiarity f is an expected fraction [0,1]
that has seen the work. Before a slot read a common snapshot. For v allocated
viewers, expected fresh viewers = v*(1-f). Aggregate simultaneous broadcasts
of the same work; update f_new=f+(1-f)*sum(v)/population once. This assumes
independent uniform viewing draws inside a group, not identified people.
It cannot estimate true unique reach or correlations between episode audiences.

Series attachment L starts at .5 and updates only from expected fresh episode
viewing: L_new=L+learning*sum(fresh_viewers/population*(satisfaction-L)).
Standalone works have no series attachment entry or update. Repeats can reach
expected newcomers but do not become new episode IDs or advance sequence.
Store per-work experienced reception after positive viewing, and an audience/
series exposure frontier (highest sequence with positive expected viewing).
The frontier is diagnostic; it does not mean the whole group has caught up.

## Selection effects (all provisional)

Keep the existing fuzzy base attraction A and experienced satisfaction S.
Utility = A² * attachment_factor * reception_factor * continuity_factor *
repeat_factor. Outside utility and expected-share allocation remain unchanged.

- Standalone attachment factor is 1. Episode attachment factor is .2+1.6L.
- Repeat factor = 1-.35*f_current, bounded [.65,1].
- For sequence n>1, prior familiarity = sum(familiarity of earlier episodes)/
  (n-1), with absent episodes contributing zero; for n=1 it is 1.
  Continuity factor = 1-.6*c*(1-prior familiarity), bounded [.4,1].
- Immediate-predecessor reception factor =
  1+.6*f_previous*(S_previous-.5), bounded [.7,1.3]. With no observed
  predecessor reception it is 1. Sequence 1 and standalone works use 1.

Factors and utility are finite and bounded; base fuzzy membership is not
probability/confidence. Reports expose the factors separately from base
attraction and satisfaction. New viewers can join because continuity never
hard-gates entry; cast/awareness/taste still attract them, and watching earlier
episodes/repeats gradually increases readiness. Only actual aired work changes
history, familiarity, reception or attachment. Passive hype decay, off-air
handling, costs and per-audience contract settlement retain their ordering.

Slot order: validate whole plan; resolve funding; evaluate all viewing from a
common pre-slot snapshot; allocate; update familiarity/reception/attachment and
frontiers once per audience/work or series; append actual aired history;
update ad hype; charge aired costs; record/settle contracts; atomically commit.
No listings, promotion campaigns, habits, variable ad breaks, measurement,
production, UI or politics are introduced. Repetition-blind advertiser pricing
remains a documented limitation and is not redesigned here.

## Controlled runs and observed results

`cargo run -p bts-cli -- episodes --json /tmp/bts-episodes.json` exports all
eight developer-only runs. The fixture has one group of 10,000 viewers with
.8 availability, identical genre/tone fit, .8 awareness/cast appeal, .9
execution except the disappointing opening (.05), 10-credit airing costs,
and 100,000 initial station cash. Episode attributes remain separate; episode
two and three are identical across opening-quality controls. Normal learning
is .9 and outside utility .18. The series' continuity is .7 in the opening
comparison and 0/1 in the out-of-order comparison. No ads or brand budgets
confound these runs.

| Controlled observation | Expected viewers | Expected fresh viewers |
| --- | ---: | ---: |
| Standalone movie, first showing | 6,468.7 | 6,468.7 |
| Episode two after strong opening | 6,830.3 | 6,830.3 |
| Episode two after disappointing opening | 5,760.0 | 5,760.0 |
| Earlier episode repeat, attachment frozen and continuity 0 | 6,125.6 | 2,163.1 |
| Fresh movie after the same prefix and frozen attachment | 6,468.7 | 6,468.7 |
| Starting at episode three, continuity 0 | 6,468.7 | 6,468.7 |
| Starting at episode three, continuity 1 | 5,025.7 | 5,025.7 |

The first opening attracts exactly the same viewers in both quality cases;
only experienced reception differs. Episode two's base fuzzy attraction stays
.872, but history-adjusted utility is 1.051 versus .463. Good later episodes
can recover some interest after the poor opening. An unviewed opening does not
leak execution quality into predecessor reception or subsequent selection.

The repeat/fresh pair freezes attachment learning at zero and continuity at
zero, so its third-slot base attraction, attachment, reception and continuity
factors match. Only current-work familiarity changes the selection factor.
The repeat raises familiarity .647 -> .863 and retains frontier 2 -> 2;
the subsequent episode three moves it to 3. In out-of-order controls, frontier
3 after the first showing still has prior familiarity 0. New viewers can join
at continuity 1 because the factor bottoms out at .4, while seeing earlier
episodes gradually improves readiness. Standalone works retain null attachment.

The delivery-substitution regression forecasts underserved audience two but
actually broadcasts to the equally large satisfied audience one. Aggregate
relevant delivery is unchanged, but it earns less than one quarter of the
accepted payment: audience one's zero allocation cannot cover audience two's
shortfall. Successful delivery still earns the full allocation per audience;
partial delivery and off-air cases preserve exact refunds and single settlement.
A capped 10-credit equal-value example allocates 3,3,4,0 in audience-ID order,
with the zero-value fourth audience receiving zero. Sub-whole but above-quantum
forecasts settle successfully; below-quantum rows are explicitly uncontractible.

## Limitations and verification

These are deterministic expected-group mechanics, not individual retention,
measured unique reach or a comprehension model. Familiarity assumes independent
uniform viewing draws within each group, does not decay, and does not model
correlated audiences across episodes. Continuity currently changes selection,
not intrinsic satisfaction. The predecessor signal affects only groups with
positive expected predecessor familiarity. Attachment is a group-level stock,
not a personal subscription. History storage grows with aired slots; this is
for small bounded experiments and not a long-running save protocol.

Delivery is quantized at 10^-6 relevant-impression resolution using f64
inputs; ordinary floating rounding applies at numerical boundaries. Extremely small rows below that resolution receive no allocated
payment. Ordered apportionment biases rounding toward later IDs; it is an
explicit reproducible rounding choice, not an optimal economic allocation.
Offer value and balances remain distinct; repetition-blind advertiser pricing
is still unresolved, as demonstrated by `advertiser-value`. Programme repeats
can change reach through familiarity without directly fixing ad repetition
valuation. Fixed fixtures establish causal/accounting properties, not balance.

Schema 4 adds programme kinds, series definitions, observed history, expected
fresh viewing, selection factors, per-audience frozen commitments and audience
settlement rows. Internal State is not serialized; developer DTO construction
borrows immutable evidence. This episode milestone added no player observation layer, listings, promotion,
habits, ad-break changes, measurement, production, UI or politics. The subsequent
[browser foundation](browser-foundation.md) now adds a separate observation
whitelist and minimal harness without changing these numerical mechanics.

Local verification on Rust/Cargo 1.98.1 passed 63 integration tests and five
doctests, formatting, Clippy with warnings denied, and Rustdoc with warnings
denied. All four CLI exports and both examples ran successfully. A fresh
outside-workspace copy of unchanged `bts-fuzzy` passed formatting, tests,
doctests, Clippy, docs and its example offline. Some LSP revisions were stale;
current compiler/Cargo checks supplied the fallback semantic verification.
The [schema-4 snapshots](../contrib/episodes-and-commitments/README.md) include
full episode traces and current per-audience contract/ledger evidence.
Publication settings and licensing remain unchanged.
