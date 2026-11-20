# Open questions and experiments

This file tracks decisions that still need a design choice or experiment.
See the [playable season](first-playable.md) for current coverage and
[design guidelines](design-guidelines.md) for settled principles.

## Near-term decisions

| Area | Current baseline | Question or experiment |
| --- | --- | --- |
| Office tools | The office, tape and film spreads, shared canister and calendar are playable. Other workspaces have uneven physical treatment. | Review the entire journey for rights, research, review, and the recurring project sheets. Improve one decision at a time using the [widget guide](office-widget-style.md). |
| Programme discovery | A gap and a tape can each begin a booking. Slot suggestions are provisional. | Test a physical tape rack, broad Best aired and Also works findings, and estimated fit colours in Programming with a second accessible cue. Keep low estimated fit distinct from a clash. |
| Action capacity | Work and deadlines advance on simulated time. Most station interface actions apply immediately. | Test no delay, a shared queue, and local desk capacity for one consequential action. See [action timing](#station-time-and-action-capacity). |
| Public schedule and promotion | Exact internal schedules, one rival comparison, fillers and authored teasers are playable. | Decide what rivals publish, when they commit, and how anticipation, promotion, and moved listings change later outcomes. |
| Advertising | Minute sales and scoped autosell are playable. Weekly brand review can change willingness to bid. | Test long-run incentives around low hype and repeated campaigns. Decide whether brands should revise budgets or priorities, and how future measured guarantees would work. |
| Research | Shared audience survey cadence, group-specific Insight and title assessments are playable. | Design Insight generation from private preparation and optional auditions. Decide how collaboration and public disclosures affect familiarity and later estimates. |
| Production logistics | Staff and studio reservations are enforced. Film room choice exposes price and staging trade-offs. | Explore always-available rentals priced by overlap-weighted demand and quality. Low quality should remain bookable. Test batch setup savings and reusable prop stock separately. |
| Show lifecycle | Film, recurring discussion, scripted-series and newsroom pilots exercise parts of the workflow. | Define a general standing brief, finite and open-ended runs, sequels, format-specific shelf life, and editable episode ambition curves. |
| Expectations and receipts | Airing files can show a dated pre-air outlook beside provisional turnout, receipts and costs. | Decide how to retain forecasts over time and show earned or spent resources beyond Funds. Keep advertiser expectations, audience reception and station outcomes distinct. |
| Visual time and feedback | Visual working hours, notices and ambient poses are in the office. | Compare independently timed pose changes, changing office dressing and a day transition. Keep these independent of simulation. See [playtest direction](office-playtest-direction.md). |

Monthly recurrence, broad rival strategy, commercial-delivery measurement,
self-promotion within an ad block and outcome-graded Sway rewards all remain
proposals. Their bounded predecessors are linked in the [design library](README.md).

## Later design questions

### Audience evolution

How many groups remain understandable? What triggers a split or merge? Which history and relationships transfer? How do births, ageing, and exits work? How long must scenarios run for cohorts to matter?

Experiment: run multi-year headless worlds and inspect population conservation, group count, cultural diversity, and inherited attachments.

### Influence and households

How much household detail is necessary for parental control, co-viewing, and purchasing? How do aspiration and rejection coexist without oscillation?

Experiment: model one parent–child connection and one teen–younger-viewer connection before adding a larger network.

### Talent contracts and recurring roles

See [talent and contracts](talent-and-contracts.md). Which minimum-work guarantee,
payment schedule and reservation rules make a first multi-edition host agreement
meaningful? How should options, exclusivity, cancellation and release work? How
can scripts change role requirements while preserving existing obligations?
What are the consequences of recasting, and what can the player legitimately know?

Use the [guaranteed host dates pilot](guaranteed-host-dates.md) as the baseline
for distinguishing contractual commitments from individual assignments.

### Talent and relationships

Which role-specific capabilities let people deliver distinct programme features, and how do those combine with general craft? How should format experience affect fit? How can partnerships improve without becoming permanent optimal combinations?

Experiment: compare familiar, mismatched, inexpensive, and fashionable teams across several project types.

### Brands and commercial health

How are target hype and budgets set? How much product competition and purchase response is necessary? When do brands enter, exit, or recover? Can the player profit from letting client hype decline?

Experiment: run repeated campaigns and look for perpetual demand, runaway spending, and perverse incentives.

### News and public perception

Which stories become public, through which sources, and with what reliability? News formats use the shared production system and automatic event selection from a few editorial settings. How should optional event spotlighting work? What distinguishes recognition from approval?

Experiment: trace a private event through publication, audience beliefs, programme interest, and advertiser response.

### Politics and elections

Which issues and electoral rules provide useful broadcasting decisions? How does media influence coexist with stable identities and lived circumstances? How can an election scenario remain influenceable without making television omnipotent?

A simple proportional system is a proposal only. No political simulation is required for the first playable.

### Awards

What makes productions eligible? Who judges them, and how do taste, execution, prestige, campaigning, and competition interact? Can an awards objective reward a different production strategy from ratings?

Award names, categories, counts, and consecutive-year goals are scenario examples, not settled content.

### Optional AI

Can a small model reliably interpret player pitches into useful attributes? What are the hardware, latency, distribution, and consistency requirements? How can players review the interpretation before committing resources?

Experiment only after structured concepts produce interesting play. Preserve a non-AI route.

## Process

Resolve a question by recording the decision and evidence in the relevant design section, then remove or update its entry here. Keep exact balance values with future configuration and experiment results rather than treating early guesses as design commitments.

## Station time and action capacity

The clock should create choices about what the station has time to do before
airtime. Explore work that stays pending until a visible simulated due time,
for example preparing an ad listing or arranging a booking. Compare one shared
station workload with capacity local to the relevant desk. A global lock can
make high-pressure decisions matter, but can also leave the player waiting
without anything useful to do. Start the design experiment with one action and
compare a no-delay baseline, a shared queue and a desk-specific queue.

Reading, navigation, drafting and accessibility interactions should stay
responsive. Consider charging time for a consequential commitment rather than
for individual input gestures. Show the duration, which work would be blocked,
and the deadline before committing. Decide when terms and funds are reserved,
what happens if an offer expires while work is pending, whether cancellation
is possible, and how due work is ordered against broadcasts on the same tick.
Use explicit simulated ticks so speed and control method do not change outcomes.
Test a deadline conflict and a quiet stretch to see whether the delay creates
a decision or idle waiting. Timing, scope and duration remain proposals.

See the [ad listing playtest](station-time-experiment.md) for a concrete
deadline, capacity comparison, and due-tick cases.

## Exact airtime and quarter-hour simulation

Exact-minute placement and weighted exposure across quarter-hour inputs are
playable. The remaining questions concern refined viewing and reporting:
calibrate start-of-airing counts against sustained attention, and decide what
ratings evidence the station and advertiser can know. See [minute airtime](minute-airtime.md).

### One-minute fillers and teasers

The planner already supports authored idents and teasers in short gaps, using
their own airtime. A moved or cancelled teaser target falls back to an ident.
The open question is whether actual exposure should build anticipation for
the exact teased airing, how repetition should saturate, and what happens
when a spot already aired before its target moved. Custom trailers, automatic
gap filling and using unsold ad inventory for self-promotion are separate
proposals. Preserve signed ad inventory and allocate viewers once per
overlapping airtime segment.
