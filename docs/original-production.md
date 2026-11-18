# Bounded original production

Status: implemented experiment, November 2026. All numerical rules below are
provisional. One format: a standalone 45-minute interview/discussion, one host,
two distinct guests, one subject, one tone, total budget and recording date.
This is a playable addition to the [seven-day planner](programme-planning.md),
not a full production, careers or relationship system.

## Browser workflow

1. Open **Production**, choose the title, host, two guests, subject, tone, budget
   and recording time. **Review concept** creates a free draft. Edit or discard
   drafts without charge; the server revalidates dates when booking.
2. **Book production #ID** commits the cast and full budget. Optionally promise
   Nora's listed co-guest request, or select an earned personal favour for a
   later booking. Errors preserve the entire session and pending quote.
3. Advance normally or **Run through production #ID**. This advances the whole
   world, including existing broadcasts, decay and deadlines; it is not a private
   production clock. States are draft → booked → recording → post-production → completed.
4. **Show in programme library** selects the completed original. Place it in a
   future gap, choose an advertiser, request/accept the airing-specific offer,
   run through the airing and inspect its result and exact ledger.

Booked concepts cannot be edited or cancelled. Unfinished programmes cannot be
scheduled, even when the proposed airing would be after their ready date: finish
first, then plan. Drafts do not reserve participants. Titles are visible text of
1–60 characters; roles, IDs, duplicate participants, dates and budgets are checked
in Rust. No imported content or free-text interpretation is involved.

## Time, commitments and accounting

One tick = 15 simulated minutes; seven days = 672 ticks. If recording starts at
`r`, participants are reserved for the half-open interval `[r, r+4)` (one hour).
The programme is ready at `r+12` after a further eight ticks/two hours of
post-production. Half-open intervals permit adjacent recordings. All cast must
be available for the full recording interval; sharing any participant with an
overlapping booked project is rejected. Post-production reserves no talent.
Ready time must fit within the scenario. These fixed times include preparation;
there is no studio-capacity or crew model.

Money is integer credits, within the existing per-amount bound of 10^12. Budget
must be positive and cover the sum of the three published fees. At booking:

`station cash → production escrow(project ID): entire budget`

At recording start, once:

`production escrow(project ID) → production expense sink: entire budget`

The budget includes fees and all other resources; fees are not charged again.
The sink is an accounting destination, not simulated talent wallets. Reservation
reduces spendable cash immediately. Spending escrow does not reduce cash again.
Each subsequent airing separately costs 50 credits, once at its start. No loans,
repeated per-tick production charges, cancellation refunds or resale are added.
Ledger transfers preserve total money including expense accounts. Reserving
production funds can leave a planned broadcast unaffordable; existing off-air
and contract-refund rules still apply.

Within each atomic advance, Rust processes every interval `[t,t+1)`:

1. Start recordings at `t` and spend their reserved budgets once.
2. Run existing broadcast cadence: actual affordable starts, audience allocation,
   one ad opportunity per new airing, decay, delivery and deadline settlement.
3. Apply completed-airing reception/history once at `t+1`.
4. Finish recordings ending at `t+1`, fulfil explicit promises once, then add
   productions ready at `t+1` to the library once. Initialise audience familiarity
   with each new standalone work to zero.

At exactly `r`, the report may still show booked; recording begins when advancing
into its first interval. At exactly `r+4` it is post-production; at `r+12` it is
completed and can be placed at that boundary. Batched and individual tick
advancement yield identical state. Rejected advances commit nothing.

## Talent and creative choices

Six fictional people form the fixed browser roster:

| Person | Role | Fee | Public direction |
| --- | --- | ---: | --- |
| Elian North | Host | 220 | Calm science explanations |
| Mina Sol | Host | 120 | Familiar, lively entertainment |
| Nora Vale | Guest | 220 | Inventor; ordinarily available days 1–2 |
| Felix Reed | Guest | 180 | Detailed technical discussion |
| Jules Hart | Guest | 140 | Well-known performer, theatrical delivery |
| Rafi Linden | Guest | 160 | Everyday life and popular culture |

Nora's normal window ends at tick 192; recording must end by then. A personal
favour permits a later recording starting at/after 192. Others are normally
available throughout the scenario. Fees and conflicting commitments still apply.
The UI offers budgets 600/1,000/1,500; Rust accepts any valid affordable integer
budget. Selected public descriptions and costs inform choices without exposing
precise talent coefficients or generated execution.

Internal, bounded scores use weights `w=(0.5, 0.25, 0.25)` for host and guests:

- `C = Σ w × craft`
- `F = Σ w × subject_fit[chosen_subject]`
- `T = Σ w × (1 − |preferred_tone − chosen_tone|)`
- `R = sqrt(clamp((budget − fees) / 1000, 0, 1))`
- `execution = clamp(0.15 + 0.45C + 0.25F + 0.10T + 0.05R, 0, 1)`
- `cast_appeal = Σ w × recognition`; initial awareness = 0.55.

Calm/conversational/lively tone scores are 0.2/0.5/0.8. Subjects science,
everyday life and entertainment map respectively to existing Science, Drama and
Comedy affinity channels. This is a provisional audience-affinity proxy; an
interview is not literally made into fiction by selecting everyday life.
No additional attribute taxonomy or inference family is introduced. All talent
scores are validated finite unit values; generated works pass world validation.

Existing attraction uses topic affinity, tone, public cast appeal and awareness;
experienced satisfaction additionally uses execution. Thus famous casting can
attract viewers without guaranteeing satisfaction. Higher hidden craft/subject
fit improves experienced quality; budget resources have a small saturating effect.
The production model is deterministic arithmetic feeding the existing TV-specific
fuzzy models. It adds no randomness, files, wall clock or browser dependency.

## One explicit personal favour

Nora requests Felix as co-guest. The player must explicitly promise request #1
when booking a project containing both as guests. Booking records project ID and
promise time. Actual recording completion fulfils that promise at `r+4`, records
the fulfilment time and awards one favour owed by Nora. Ordinary casting without
a promise earns nothing. The request is one-shot across the session: repeating
the show, advancing again or promising another production cannot reward it again.

Calling favour #1 at a later booking records destination project/time and spends
it once. It secures only Nora outside her normal window. It cannot bypass a
conflict, pay her fee, secure someone else, boost quality or be spent when she is
normally available. Booking failure preserves the unused credit. There is no
cancellation after a successful call. Promise, fulfilment, creditor, source and
call remain inspectable in the production report.

This person-specific experiment predates the agreed pooled Sway direction in
[resource economies](resource-economies-idea.md). The intended system earns abstract
Sway from granting requests and spends it across people and productions. The
implemented personal credit is a prototype limitation, not that intended design.
There is no numerical relationship score or exchange market.

## Protocol and information boundary

Current player API/schema version is **3**; v1/v2 are rejected. Existing planning
commands retain their meanings. New strict typed commands are:

| Command | Fields |
| --- | --- |
| `create_production` | `concept` |
| `revise_production` | `production_id`, `concept` |
| `discard_production` | `production_id` |
| `book_production` | `production_id`, optional `promise_request`, optional `call_favour` |

A concept contains `title`, `host` ID, `guests` (two IDs), `subject` (`science`,
`everyday_life`, `entertainment`), `tone` (`calm`, `conversational`, `lively`),
`budget` (decimal string) and `record_at` (tick number). E.g.:

```json
{"api_version":3,"command":{"type":"create_production","concept":{"title":"Workshop Conversations","host":1,"guests":[3,4],"subject":"science","tone":"calm","budget":"1000","record_at":40}}}
```

Rust owns all decisions and exact accounts. Player observations whitelist public
roster/fees/windows, own choices, progress, budget/escrow/spend, programme provenance,
and explicit promise/favour records. They exclude precise craft/fit/preferred-tone,
hidden execution, exact audience tastes, hype/repetition and fuzzy evidence.
Money crosses JavaScript as decimal strings; small project/person/favour IDs and
ticks are safe JSON integers. Project/work IDs are not reused after reset; reset
clears commitments/promises/credits. Sessions remain isolated. Client-side filtering
is not a security guarantee. Counts/quotes remain labelled provisional expected
counts, not measured ratings; research and measurement-based settlement are deferred.

## Controlled evidence

Run `cargo run -p bts-web --example production_experiment`. Both comparison cases
start from identical finances, audiences, rival plan and time, record at tick 40,
finish at 52 and air at 72 (day 1, 18:00) with the same snack advertiser:

| Choice | Production budget | Young viewers | Households | Specialists | Ad receipts | Final cash |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Calm science: Elian / Nora / Felix | 1,000 | 664.69 | 1,521.51 | 661.96 | 469 | 19,419 |
| Lively culture: Mina / Jules / Rafi | 600 | 2,672.40 | 4,577.89 | 286.29 | 1,664 | 21,014 |

Both pay a separate 50-credit broadcast cost. Science attracts more specialists;
culture reaches more young viewers/households and earns more from this advertiser.
Several creative choices vary together here; this is a package comparison, not an
isolated estimate of each coefficient or a balance claim. A focused Rust test
changes only the host's subject fit and verifies a >0.04 change in specialist
satisfaction, with the same other inputs.

The separate favour sequence fulfils at tick 44, calls the single credit at 75
for a day-3 recording at 232, finishes at 244, and broadcasts at 264–267. Without
the credit the later booking is rejected atomically. With both ads settled, cash
is 20,315: 20,000 − 2,000 production − 100 broadcast + 2,415 advertising.
[Committed public reports and browser evidence](../contrib/original-production/README.md)
record these outputs and verification. `--sequence` supplies the native command
reference used for actual Wasm parity testing, not a save format.

## Verification and remaining limits

Tests cover full production/ad/broadcast flow, repeat airings without another
production charge, conflicts and adjacent bookings, free edits/discards, invalid
or expired concepts and money, insufficient funds, atomic rejection, promise
mismatch/no implicit reward/one-shot reward/person-specific redemption, tick-batch
reproducibility, ledger conservation, production-induced off-air/refunds and
hidden-field exclusion in serialized reports. Real Chromium tests exercise both
productions, favour earning/calling, conflicts, draft editing and native/Wasm parity.

Recording is reliable once booked; no cancellation, failure, studio/crew capacity,
contracts with talent accounts or dynamic availability. Quality remains unknown
to the player and influences existing satisfaction/ad-context models; no new
review system or quality-driven programme popularity is claimed. No unfinished-work
pre-scheduling. No recurring editions, pilots, awards, Glamour, Credibility,
partisan backing, pooled Sway, general relationships, research, publication/promotion,
campaign shares, save/load or wider world simulation. Existing start-snapshot
viewing, one ad opportunity per airing and repetition-blind advertiser pricing
remain provisional. Fixed fixtures do not establish economic balance.
