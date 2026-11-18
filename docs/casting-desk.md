# Casting desk and adjustable production support

Status: implemented bounded discussion-show milestone, November 2026. All numerical
parameters are provisional. The browser uses the existing six fictional portraits;
this is not a talent market, employment contract system or general relationship model.

## Playable workflow

1. Open **Production**. Click the host or either guest seat to open the portrait
   picker. Eligible cards show public specialties, fees, normal/negotiable windows
   and known recording commitments. A person already in another seat, unavailable
   on that date, or committed to an overlapping recording cannot be selected there.
   Rust still validates all commitments; browser guidance is not authorization.
2. Choose a subject tile, a tone-slider position (calm/conversational/lively), a
   budget preset and recording date. Presets are 600, 1,000 and 1,800 integer credits,
   including all cast fees. **Review concept** creates a free, editable draft.
3. On that draft, choose exceptional
   participation and allocate whole Sway to **Help the team work together**. The
   range and exact-number input are linked, from zero to the available balance
   after participation cost. Reducing the support range through a participation
   choice reduces an excessive proposed allocation to fit; no resource is spent.
4. **Review booking** asks Rust to validate a clone and calculate remaining cash
   and Sway. Inspect cast, creative choices, recording/ready times, budget,
   support and irreversible obligations, then **Confirm booking**. The review
   reserves nothing. Confirmation revalidates against current state.
5. Run through completion, choose **Show in programme library**, schedule the work,
   sell advertising and review the existing broadcast recap, delivery and finances.

The native modal supports keyboard navigation, Escape and return focus to the
originating seat. Selection buttons, radio budgets and sliders work by touch;
exact-number input provides an alternative to precise range manipulation. Changing
booking choices hides an obsolete confirmation. Advancing or editing invalidates
Rust's booking review. This is not a polished full-game interface or a complete
accessibility certification; human playtesting and broader device coverage remain.

## Accepted quests and production hints

Accept quests separately on the opportunity board. Open quests show deadlines and
production cards show whether cast/creative requirements and timing match. A match
is a hint, not fulfilment. Existing quests reward actual recording or library
completion; future audience-gated objectives remain pending until their outcome.
[Quest semantics](quest-opportunities.md) specifies the 48-hour limit, inclusive
deadline, no retroactive credit, expiry and proposed graded rewards. Booking has
no quest-promise selector or promise field.

## Authored cooperation deficiencies

Individual craft, subject fit and tone suitability remain separate talent inputs.
Two fixture notes describe collaboration difficulties, not skill deficits:

| Participants | Recorded public concern | Internal provisional penalty |
| --- | --- | ---: |
| Mina Sol and Jules Hart | Previously disagreed over who leads the conversation | 0.12 |
| Nora Vale and Rafi Linden | Have found it difficult to share the floor | 0.08 |

A concern applies only when both named people are in the production. These are
fixed authored working histories. No portrait, gender, profession or public fame
is used to infer chemistry. There are no new roles, generated relationships or
relationships changing through repeated work. Configured pairs must exist, be
unique ignoring order, and have finite penalties in `(0, 0.25]`. An optional known
note is separately authored; hidden issues are not automatically disclosed.

Browser observations carry only those notes and public participant commitments.
An absence of recorded concerns is explicitly not a forecast of quality or proof
that no difficulty exists. Precise penalties and individual ability remain hidden.

## Equation, bounds and diminishing returns

All execution quantities are finite dimensionless unit scores. Let existing
weighted talent/resource terms be craft `C`, subject fit `F`, tone suitability `T`
and resources `R` (see [production](original-production.md)). The skill/resource
baseline is:

`B = clamp(0.15 + 0.45C + 0.25F + 0.10T + 0.05R, 0, 1)`

For matching authored cooperation issues:

`D = min(sum(issue penalties), 0.25, B)`

For allocated whole-number Sway `s >= 0`:

`remaining loss = D × 2 / (2 + s)`

`recovered loss = D − remaining loss`

`execution = clamp(B − remaining loss, 0, 1)`

The constant 2 is provisional and measured in Sway units. Each extra point recovers
less loss than the previous point. Recovery is bounded by `D`; the result never
exceeds `B`. Capping the initial loss by `B` prevents a zero-floor threshold from
creating increasing marginal returns for extremely weak teams with stacked issues.
With no matching deficiency, `D=0`: any allocation yields exactly the original
baseline, with no bonus. Spending on such a team remains possible and still costs
Sway; support is a player choice, not an automatic promise of benefit.

Support cannot alter craft, subject fit, tone suitability, resources, awareness,
recognition, fees, runtime or public tastes. A weak team remains weak. It mitigates
only a separate cooperation loss; it does not guarantee satisfaction, reach or
commercial success. Even substantial finite support leaves some cooperation loss.
The old checkbox's tone-fit boost has been removed.

## Time and accounting

Exceptional participation still costs **2 Sway**, separate from support `s`, for
one explicitly negotiable person. Total booking spend is `s + participation cost`.
It cannot exceed the station pool. Negative/fractional values are malformed;
unsupported/overflowing or unaffordable amounts fail atomically. Zero creates no
support-debit entry. Booked projects cannot spend again or be revised/cancelled.

At successful booking Rust freezes the concept, support, exact production outcome
and participant reservations, reserves the whole cash budget and debits Sway once.
A failed command consumes nothing, including cash, Sway, IDs, commitments and pending
ad offers. Recording spends the cash escrow once at its start. Support is not
reapplied per tick, at completion or on repeats. Output insertion, familiarity,
quest rewards, advertising delivery and settlement retain their existing cadence.
Sway and money remain separate ledgers; support creates no financial transfer.

## Protocol and diagnostics

Player protocol **v5** replaces v4. `book_production` replaces the boolean
`smooth_cooperation` with required `cooperation_sway: u32`. The new read-only
`review_production` takes the same booking fields. Example:

```json
{"api_version":5,"command":{"type":"review_production","production_id":2,"exceptional_person":5,"cooperation_sway":2}}
```

The report's optional `booking_review` gives the Rust-calculated budget, remaining
cash, total Sway and remaining Sway. It is a preview, not a contract or token;
the browser confirms its exact choices through ordinary `book_production`, which
always revalidates. Native callers may book directly. No save schema is introduced.
Old protocol versions and cooperation-checkbox fields are rejected.

Person reports include own station production commitments. Production observations
include selected support and authored public concerns. Exact outcomes, skill
coefficients, base execution, penalties, recovered/remaining losses and fuzzy
traces are excluded from player DTOs. Money remains decimal strings; Sway is bounded
exact JSON integers. Client-side filtering is not a secrecy guarantee.

`developer::ProductionReport::from_production` separately constructs exact skill,
resource and loss contributions from frozen evidence. Simulation state is not
serialized wholesale. The developer-only `casting_comparison` example exports
these reports alongside expected audience outcomes; it is not browser protocol.

## Controlled comparisons

Run `cargo run -p bts-web --example casting_comparison`. Each case starts from the
same fixture, earns the same 4 Sway by recording Nora with Felix, then produces
and airs at the same time against the same rival. Within each family, only support
changes. No advertising is sold, so money and initial viewing attraction are held
constant. Values below are rounded, not balance commitments:

| Family | Support | Baseline | Initial cooperation loss | Recovery | Final execution |
| --- | ---: | ---: | ---: | ---: | ---: |
| Mina/Jules/Rafi, existing skills | 0 | 0.844454 | 0.120000 | 0 | 0.724454 |
| Same | 1 | 0.844454 | 0.120000 | 0.040000 | 0.764454 |
| Same | 4 | 0.844454 | 0.120000 | 0.080000 | 0.804454 |
| Same cast, craft and subject fit fixed to 0.05 | 0 | 0.316829 | 0.120000 | 0 | 0.196829 |
| Same weak skills | 1 | 0.316829 | 0.120000 | 0.040000 | 0.236829 |
| Same weak skills | 4 | 0.316829 | 0.120000 | 0.080000 | 0.276829 |
| No matching issue: Elian/Nora/Felix | 0 / 1 / 4 | 0.681697 | 0 | 0 | 0.681697 |

One point recovers 0.04; three further points together recover only another 0.04.
Weak skills remain limiting. A team without cooperation deficiencies gets no
bonus. All nine cases end with cash 17,950 and conserved total money 80,000.
Different families are controls, not an assertion that casting or subjects are
otherwise equivalent. [JSON, actual browser evidence and checks](../contrib/casting-desk/README.md)
record full results, including experienced satisfaction. No universal economic
or creative balance is claimed from fixed fixtures.

## Remaining limits

No talent-market contracts, recurring editions, staged writing, drag-and-drop
scheduling, awards, research or wider relationship system. Tone has three explicit
positions; it is not an arbitrary continuous editorial profile. Availability and
cooperation notes are fixed fixture content. Views still include the existing
planner and developer-oriented financial detail. This milestone improves production
interaction without claiming the full proposed game interface is complete.

## Browser pacing, resource HUD and reusable components

The browser starts paused. **Play/Pause** requests one explicit 15-minute Rust
advance every two real seconds; **×2** uses one second. These are provisional UI
pacing values. The timer schedules only after the previous synchronous command
finishes, never catches up for elapsed background time, and pauses on hidden tabs,
errors, reset, scenario end and planning/form interactions. The simulation still
has no wall clock: the same ordered advances reproduce the same outcome. During
play, the timeline follows the current day and keeps the dashed vertical **NOW**
marker in view; browsing another day pauses playback. The marker is absent when
that viewed day is not current. Manual tick/day/airing advances remain available.

A shared resource HUD groups icon/value pairs for Cash, Sway, Glamour, Credibility
and Backing. Cash and Sway use current Rust reports. The other three display **—**
and keyboard/hover tooltips saying they are not available; no fake resource stocks
or gameplay effects are introduced. Planned daylight, notifications and richer
office appearance remain future presentation work.

Plain TypeScript components under `web/src/ui/` share safe DOM helpers, portraits,
modal selection/focus restoration, choice buttons, linked amount inputs, playback
and the declarative resource HUD. Page code supplies public data and Rust commands;
components do not duplicate simulation, quality, rights or accounting logic.
The casting desk also shows [station teams/studios](production-logistics.md): the
owned library is distinct from the rival's schedule and production rooms are
reserved independently of participant commitments.
