# Bounded pooled Sway

Status: implemented, November 2026. Replaces personal-favour redemption. Sway is
station-wide social capital, following [resource direction](resource-economies-idea.md).
All amounts/effects are provisional. This is two finite discussion-production
requests and two booking uses, not a general quest or relationship framework.

## Requests and fulfilment

The station starts with **0 Sway**. Requests are visible from the Production desk.
Explicitly select a request when booking a matching concept. One request may be
promised per production; each finite request may be promised and rewarded once
per session. Merely booking, paying fees or casting the requested pair without
a promise earns nothing. Promises survive as inspectable records.

| Request | Visible condition | Required achievement | Reward |
| --- | --- | --- | ---: |
| Nora: record with Felix | Nora and Felix must be the two guests | Finish their recording | 4 Sway |
| Rafi: calm everyday-life discussion | Rafi as guest, everyday-life subject, calm tone | Complete the production into the library | 3 Sway |

For recording start `r`, the first request fulfils at `r+4`; the second at `r+12`.
Booking awards zero. At fulfilment Rust rechecks the frozen concept, phase and
achievement boundary, marks the promise fulfilled, adds the reward and appends
one history entry. Later ticks, repeated airings or further matching productions
cannot reward it again. The two requests permit at most **7 earned Sway** in this
scenario. Neither requires commercial success or audience approval.

Normal production semantics remain: one-hour recording, two-hour post-production,
immutable paid bookings, 45-minute standalone output, exact cash reservation
then one recording expense. See [production rules](original-production.md).
Recording requests resolve after recording finishes; library requests after
output insertion, within the same atomic tick advance. A reward earned at an
interval's end can fund the next command, never an earlier booking in that interval.

## Spending at booking

The player chooses either use, both, or neither. Rust validates the whole command
before atomically committing money, participant reservations, the promise and
Sway spending. Each use is bought once for this production; repeated booking is
rejected. A combined purchase requires the full combined balance up front.

| Use | Cost | Concrete effect | Constraints |
| --- | ---: | --- | --- |
| Exceptional participation | 2 | Secure one selected participant outside normal availability | Person must be in the cast, date explicitly negotiable, full fee covered, no commitment conflict |
| Smooth cooperation | 2 | Recover half the tone-coordination shortfall in this production | Does not change talent craft, subject fit, public recognition, fees or runtime |

Nora and Jules are normally available through tick 192 (end of day 2), and
explicitly negotiable for recording starts at/after 192 until scenario end.
Others are normally available throughout. The API supports one exceptional person
per booking; a project needing two cannot be booked in this slice. A recording
straddling the normal-window boundary starts too early for negotiation and is
rejected. Hard unavailability with no negotiable window stays unavailable.
Normally available participation cannot waste Sway through this option.

Any earned Sway can secure any eligible person. Nora's request can fund Jules;
Rafi's reward can smooth a production without Rafi. No creditor, personal credit,
favour identifier or redemption restriction remains. Normal fees are still paid
in cash, and Sway cannot override overlaps or insufficient cash. Spending is final
at booking; booked projects cannot be cancelled in this milestone.

## Intended replacement for cooperation spending

The fixed checkbox/cost below is the current experiment. The revised
[design direction](resource-economies-idea.md#adjustable-production-support)
calls for a freely adjustable Sway allocation with diminishing returns, reducing
cooperation penalties such as bad chemistry. It must leave skill deficiencies
untouched and cannot add quality beyond the recovered cooperation loss. That
slider/dial and a broader deficiency model are not implemented by this milestone.

## Cooperation equation and limits

The existing production model computes weighted tone fit:

`T = Σ w × (1 − |preferred_tone − chosen_tone|)`, with `w=(0.5, 0.25, 0.25)`.

Without smoothing, the execution contribution is `0.10 × T`. With smoothing:

`T_effective = T + 0.5 × (1 − T)`

`contribution = 0.10 × T_effective`

Execution therefore improves by `0.05 × (1 − T)`, bounded between 0 and 0.05.
This models easier coordination across presentation styles. It recovers only a
specific shortfall; weak craft or unsuitable subject skills remain. Final
execution retains existing finite unit bounds. Neither participation nor smoothing
guarantees creative success, audience reach, satisfaction or advertising revenue.
The browser reports the selected and applied cooperation treatment; exact talent
scores and resulting execution remain hidden. No arbitrary review noise is added.

## Balance, history and protocol

The pool belongs to the planner's player station, independent of who proposed a
request. It is private Rust state, separate from the money ledger. Exact integer
Sway has no decay, exchange rate, debt or automatic regeneration in this slice.

`new balance = old balance + reward − participation cost − cooperation cost`

Checked arithmetic and finite reward configuration prevent overflow/negative
balances. History records time, production ID, signed integer change, resulting
balance and reason, plus request ID for earnings or participant ID for negotiation.
Summing its entries reproduces the current balance. Sway does not create or
consume money; existing production escrow and financial conservation remain intact.

Player protocol **v4** replaces v3 (old versions/`call_favour` fields rejected).
`book_production` fields:

```json
{"api_version":4,"command":{"type":"book_production","production_id":2,"promise_request":null,"exceptional_person":5,"smooth_cooperation":true}}
```

`exceptional_person` and `promise_request` are optional IDs; `smooth_cooperation`
is a required boolean. Existing strict DTO validation rejects unknown fields,
malformed numbers/types and obsolete commands. Rejections preserve Sway, cash,
commitments, IDs, history and pending advertising offers. Successful production
commands invalidate pending quotes as before. Reset clears the pool/history and
requests; independent sessions cannot spend one another's Sway. Production/work
IDs retain their existing non-reuse guarantee across reset.

Normal reports whitelist request conditions/rewards, promises, balance, costs,
transactions and own selected/applied spending. No exact craft, preferred tone,
subject-fit coefficients, audience tastes, hype/repetition or fuzzy traces are
exported. Sway values fit u32/i32 and are exactly representable JSON integers;
money remains decimal strings. Client-side filtering is not a security guarantee.

## Controlled evidence and validation

Run `cargo run -p bts-web --example production_experiment`. Its two baseline
programme comparisons retain their original outcomes without Sway. Its separate
chain records Nora's request at 40–44, earns 4, then spends 2 on Jules and 2 on
cooperation for a day-3 production. Rafi's later request earns 3 at ready tick 292.
[Public JSON and actual browser screenshot](../contrib/pooled-sway/README.md)
record the outcome. `--sequence` is the real native reference for Wasm parity.

Tests isolate cooperation at otherwise identical inputs: execution rises within
0.05, recognition and financial state stay identical, and weak topic fit remains.
Other tests cover both fulfilment boundaries, no implicit/duplicate rewards,
finite reward validation, cross-person spending, overspending, repeated bookings,
non-negotiable windows, overlaps, fees, pending-quote preservation, reset/isolation,
serialized hidden-state exclusion and batch-versus-tick reproducibility including
spending. Real browser tests exercise both quests, both uses, cross-person booking,
advertising and completed broadcasts. Existing standalone fuzzy checks remain.

Staged writing, come-back careers, prodigies, ambitious novel adaptations, fringe
reporting and excluded-person narratives are future quest examples. No awards,
Glamour, Credibility, Backing, general content loader, quest parser, reputation
network or broader world simulation is added. Fixed fixtures and these prices do
not establish a balanced social-capital economy.
