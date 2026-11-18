# Bounded pooled Sway

Status: implemented, November 2026. Sway is station-wide social capital; personal
favours and production-specific quest promises have been removed. Amounts, deadlines
and effects are provisional. This is two finite quests and two production uses,
not a general quest, career or relationship framework.

## Earning and quests

Start with 0 Sway. Accept a quest from the station's opportunity board, not while
booking a production. Open quests have a 48-hour limit from acceptance, clipped to
scenario end. Nora's co-guest recording earns 4; completing Rafi's calm everyday-life
discussion earns 3. Booking or an apparent production match alone earns nothing.
Only recordings starting at/after acceptance and achieving the condition by the
deadline qualify. Each quest rewards once; completed/expired quests cannot be
reaccepted. See [quest semantics](quest-opportunities.md) for deadline boundaries,
production hints, no retroactive credit and future outcome-gated/graded proposals.

The fixture permits at most 7 earned Sway. No decay, automatic regeneration,
exchange rate or debt is added. The pool belongs to the planner's player station,
independent of the quest proposer. Nora's reward can fund Jules or a different team.

## Spending and exact history

At production booking, choose:

| Use | Cost | Effect |
| --- | ---: | --- |
| Exceptional participation | 2 Sway | Secure one explicitly negotiable participant outside normal availability |
| Team support | Any whole amount from 0 to the available balance after participation cost | Reduce a separate cooperation penalty with diminishing returns |

Nora and Jules normally record through tick 192, and are explicitly negotiable
for starts at/after 192. Hard unavailability remains unavailable. One exceptional
person per production is supported; Sway does not pay fees, bypass conflicts,
remove preparation/recording time or guarantee creative success.

The old cooperation checkbox and tone-fit bonus have been replaced by adjustable
support. [The casting specification](casting-desk.md) documents authored pair
issues, skill-independent loss mitigation and the exact equation. In brief, if `B`
is the unaltered skill/resource baseline and `D=min(sum of cooperation penalties,
0.25,B)`, support `s` leaves loss `D×2/(2+s)`. Execution never exceeds `B`. No matching
deficiency means no quality bonus. Fees, skill inputs, subject fit, tone suitability,
recognition and runtime are unchanged by support.

`new balance = old balance + earned rewards − participation cost − team support`

Rust validates total spend and the whole booking atomically. Failed commands consume
nothing. Successful booking debits once; completion, time ticks and repeat airings
do not charge again. Zero support emits no debit. Separate history records time,
production ID, signed change, resulting balance and reason; earnings identify a
quest and negotiation identifies a person. Summing history reproduces the balance.
Sway is separate from the financial ledger and does not create/consume money.

## Browser protocol and information

Player protocol v5 replaces v4. `accept_request` accepts an opportunity separately.
`review_production` validates a clone and calculates remaining cash/Sway;
`book_production` revalidates and commits. Both booking commands take production ID,
optional `exceptional_person` and required integer `cooperation_sway`. Old versions,
`promise_request` and `smooth_cooperation` are rejected. A preview is not a reservation.

Normal observations show public roster/commitments, known working notes, open quests,
production hints, chosen support, exact balance/history and financial terms. They
exclude exact skills, chemistry penalties, recovered losses, execution, audience
tastes, hype/repetition and fuzzy traces. Money remains decimal strings; bounded Sway
integers are exactly representable in JSON. No developer inspector is exported;
client-side observation filtering is not a security guarantee.

[Current controlled experiments and browser evidence](../contrib/casting-desk/README.md)
replace the [historical protocol-v4 evidence](../contrib/pooled-sway/README.md).
The original production example now uses v5 and includes quest acceptance and booking
reviews; `casting_comparison` separately exports developer-only loss/skill controls.
Standalone fuzzy, native/Wasm, accounting, reproducibility and privacy checks remain.

Staged writing, recurring editions, talent contracts, drag-and-drop scheduling,
awards, Glamour, Credibility and broader relationship systems remain deferred.
Fixed fixtures and provisional prices do not establish economic balance.
