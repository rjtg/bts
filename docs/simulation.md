# Simulation model

Status: causal direction is agreed; algorithms, parameter values, update ordering, and several mechanisms below are proposals to validate. This document is not a final mathematical specification.

The [headless experiment](headless-experiment.md) now implements a bounded
subset: deterministic expected viewing shares, separate attraction and
satisfaction, series willingness, and exposure-driven hype with decay. The
[bounded economy](advertising-economy.md) connects that hype to finite-budget
offers, target delivery contracts and station cash.
Its precise equations are provisional experiments; the broader mechanisms
below remain proposals. Developer reports expose actual state and are not
player evidence. The separate [browser adapter](browser-foundation.md) now
constructs a restricted player observation, with explicitly provisional exact
expected-count delivery reports and quoted contractual forecasts.

## Purpose

Create a world that is changing, partially observable, and learnable. Observation and adaptation should help the player outperform blind repetition without guaranteeing success.

## State and knowledge

Maintain three separate layers:

1. Actual world state: skills, relationships, audience preferences, campaign effects.
2. Public perception: recognition, reputation, rumours, beliefs.
3. Player evidence: ratings, polls, focus groups, staff assessments, reports, trade press.

The simulation can inspect causes exactly in developer tools. Player reports must
respect source knowledge, delay, and uncertainty. A misleading rumour changes
beliefs, not the underlying fact. Player estimates use familiarity and
temporally-correlated bias: attribute-specific bias changes gradually, while
familiarity controls its uncertainty scale. The same model applies to audiences,
people, programmes, brands and relationships, not just audience polls. A public
disclosure can update current knowledge without rewriting dated evidence.

## Audience choice and satisfaction

Purpose: make programme selection, timeslots, promotion, and production choices matter.

Proposed sequence per broadcast slot:

1. Determine available viewers and access constraints, including household restrictions.
2. Evaluate options using known programme appeal, awareness, talent, loyalty, slot fit, and competition.
3. Allocate viewers among stations and not watching; a group can split across choices.
4. Resolve enjoyment from actual execution and audience fit.
5. Update loyalty, reputation, word of mouth, and fatigue.

The prototype now distinguishes standalone works and distinct sequenced
episodes. Expected work familiarity, predecessor reception, shared series
attachment and continuity affect later selection, using a common pre-slot
snapshot. Repeats reach fewer expected fresh viewers and do not advance an
episode sequence. These provisional equations are documented in the
[episode experiment](episodes-and-commitments.md).

Initial attraction and experienced satisfaction are distinct. A star can drive a premiere that loses viewers later. Promotion cannot directly manufacture enjoyment.

Account for the same viewer only once within a slot. Advertising exposures count viewers of the associated broadcast; cumulative reach and repeated exposures are different measures.

Evidence: audience composition, ratings, return rates, satisfaction studies, and reports with appropriate uncertainty.

### Outcomes, expectations and what counts as failure

The simulation does not store a universal show-failure score. It records outcomes
and updates the state each observer cares about: audience experience affects future
interest and return behaviour; advertiser delivery affects their future demand and
offers; public events affect knowledge and reputation. “Failure” is a judgement
made against a person's goals, expectations or obligations, not a hidden scalar.

Every airing should have a legible resource account: cash and other resource costs
spent, and cash/resources earned or obligations avoided. Show both the individual
airing result and cumulative show investment/returns. An airing may lose cash yet
be strategically worthwhile if it produces needed Sway, Insight, Credibility,
Glamour or Backing. Those resources remain distinct and should not be collapsed
into one universal value score. Exact resource causality can appear in developer
reports; the player report uses legitimate evidence.

Keep airing economics separate from advertiser satisfaction. An ad slot's price
and saleability depend on what the advertiser expects when buying it. Once sold,
payment and any refund follow the explicit contract. A disappointing result does
not retroactively change settlement unless the contract says so. It may reduce
future advertiser interest or offers. Thus a risky unknown show can be sold in
advance and still be paid, while its poor reach changes later demand. Contract
refund rules remain explicit and may be expanded in future market designs.

Expectations are estimates, not verdicts or promises. Record the estimate relevant
to a purchase/sale decision; airing reports can compare the result with that prior
expectation and show its uncertainty and evidenced causes. Results then update
future expectations for the show. Do not label an airing a failure solely because
it missed an uncertain estimate; distinguish underdelivery against a contract,
audience disappointment and station-level resource return.

## Rules and fuzzy logic

Use exact accounting for cash, time, rights, and contractual commitments. Use continuous attributes and selected fuzzy rules for fit and creative interactions. Use probabilistic choice and bounded randomness for uncertain behaviour.

Illustrative rules, not fixed formulas:

| Condition | Consequence |
| --- | --- |
| Strong actor appeal and genre fit | Higher initial interest |
| High exposure and disappointing recent work | Greater fatigue |
| High production demands and insufficient resources | Weaker execution |
| Strong writing and cast chemistry | Better satisfaction |
| Strong familiarity and genre saturation | Less excitement |

Membership curves should create smooth effects. Fuzzy membership is not probability or uncertainty: partial fashionability differs from an uncertain estimate of fashionability.

Begin with a small inspectable rule set. Avoid a separate rule for every combination of attributes. Log important contributions and check that combined effects remain bounded.

## Production

Purpose: let the player make informed creative and resource commitments.

Proposed broader workflow: concept → development → optional pilot → production → broadcast → renewal, revision, or cancellation.

Implemented subset: [one standalone discussion format](original-production.md),
with free concept drafts, fixed recording/post-production time, paid talent
commitments, completed library works and finite requests and pooled Sway.
Recurring editions, pilots, careers and general relationship effects remain proposals.

A project specifies intended format, themes, tone, ambition, people, budget allocation, time, and production demands. Role-specific craft, task fit, relationships, and resources affect different execution dimensions.

Spending has diminishing returns. Not every low-budget show is weak: demands matter. Collaboration improves familiarity but can also produce conflict or predictability.

Actual abilities, hiring assessments, and public fame remain distinct. Production time creates market risk because tastes and competition can change before release.

News and topical entertainment use this same production system. Their format
and editorial profile determine how current events become editions; no separate
newsroom or routine manual bulletin assembly is required. See [design](design.md)
and [screens](screens.md). Editorial profile covers topic priorities, leaning,
presentation and depth; it can change interest and trust over repeated editions.

Evidence: estimates, staff reports, milestones, pilots, and later audience response.

## Resource generation and editorial outcomes

Broader causal direction (bounded Sway subset implemented): see [station resources](design.md#station-resources-and-editorial-capital)
and [resource economies](resource-economies-idea.md).

Resolve fulfilled requests into pooled Sway once; spending is not restricted to
the requester. Awards generate Glamour on their own cycle; its retained stock
affects glamour-seeking talent terms and decays with simulated time. An explicit
spend targets an airing; the performance effect is still to design.

Credibility generation requires a sustained reporting record. Model intended
editorial choices separately from achieved truthfulness and neutrality. Verification,
sources, staff, preparation and framing must have causal effects; setting a
neutral profile cannot guarantee credible output. Preserve actual facts,
reasonably available editorial knowledge and player observations separately.

Stored credibility affects news effectiveness. Opinion pieces can consume it
to increase influence and generate backing among receptive audiences.
Backing can gradually develop into loyalty, with timing, audience attribution,
conditions and rates still open. Support, viewing, enjoyment and habitual loyalty
are distinct; no immediate viewer transfer or guaranteed persuasion is implied.
Consistent reporting and consistent advocacy create tension without requiring
a hard either/or switch.

Open before implementation: evaluation windows, generation/spending units,
zero-stock bootstrapping, Glamour's specific airing effects, backing-to-loyalty
conversion and safeguards against repeating the same item to farm resources.
Do not introduce a universal cash conversion, shared decay rule or a generic
quality multiplier for every resource. Production controls express intent, not
automatic success.

## Talent obligations and script requirements

Design direction, not implemented: distinguish a person's market availability,
contractual obligations and assignments to specific productions. Contracts can
span editions or episode appearances, with explicit money and time commitments.
A signed person is not automatically available for every task. See
[talent and contracts](talent-and-contracts.md) for proposed terms and open questions.

Script requirements name dramatic roles and episode appearances independently of
actors. Casting and contracts must satisfy those requirements and continuity.
Rewrites consume work/time and can change creative requirements; they must not
retroactively erase guaranteed pay, reserved commitments or other obligations.
Recasting/release consequences need explicit design before implementation.
Preserve exact accounting, conflict checks, atomic rejection and reproducibility.
The existing one-off discussion booking remains the implemented baseline.

## Cultural change and relationships

Purpose: create discoveries, changing opportunities, and lasting consequences.

Existing tastes influence viewing. Viewing experiences affect discussion and reputation. Those spread through directional relationships and affect future attention and production supply. Repeated supply can produce saturation and fatigue.

Distinguish stable preferences, temporary topic salience, awareness, satisfaction, and fashionability. Not all change is fashion.

Influence initially transmits awareness and curiosity more readily than approval. Connections differ by subject and can produce imitation or rejection. Parents exercise access control separately from children's desire to watch.

Proposal: apply influence from a consistent state snapshot per update to avoid entity-order advantages and instantaneous repeated propagation. Bound feedback so one hit does not permanently homogenise the population.

Talent collaboration tracks personal affinity, trust, familiarity, and project-specific creative fit. Relationships change through events and shared work; neither friendship nor conflict guarantees quality.

## Brand hype and campaigns

Purpose: make audience composition economically meaningful and create changing advertiser demand.

Track hype per brand and audience group. Proposal: also track slower-decaying awareness so established but unfashionable brands remain recognisable.

Conceptual update, not a chosen equation:

Next hype = retained hype + effective campaign exposure + relevant event / social effects.

Clamp to meaningful bounds. Effective exposure depends on audience relevance, creative effectiveness, saturation headroom, repetition, and programme context. Repetition may reinforce a campaign before wearing it out. Calibrate decay by simulated elapsed time, not UI frames.

Per-audience delivery commitments now prevent a satisfied audience from
substituting for an underserved audience in settlement. Accepted prices and
priorities are frozen; integer micro-impressions permit fractional delivery
without division by zero. See [commitment and episode semantics](episodes-and-commitments.md).

A brand's target-hype gap drives urgency. Expected useful exposure, budget, competition, and confidence constrain its offer. Closing the gap can reduce immediate spending; decay and new launches create future demand.

Purchasing power is one targeting dimension. Affordable mass products and niche luxury products need different strategies. Purchase influence can cross household links: children may create demand fulfilled by parents.

Implemented provisional contract experiment: a fixed maximum payment for weighted relevant target delivery in a booked station/hour, with an inclusive deadline and proportional underdelivery refund. Money is integer credits; offers reserve finite budget in escrow. Per-audience hype deficits value each audience's forecast delivery; brand-wide pressure is diagnostic. Accepted prices are not retrospectively changed by hype. Unfunded programmes go off air independently before allocation, and missed delivery refunds at the deadline. See [economy equations](advertising-economy.md). More detailed contracts, bonuses, renewal negotiations and campaign studies remain open; this experiment is not a permanent commercial design.

### Proposed ad-minute market

The intended player action is to offer a chosen number of advertising minutes for
a specific airing and surrounding programme context. Advertisers independently
make non-exclusive offers for some number of those minutes at a total price. The
station accepts any combination up to its offered capacity, leaves some inventory
unsold, or shortens the break. Scarcer offered minutes may command a higher price
per minute. Advertisers may offer to several stations at once and buy toward their
own exposure needs; this is not an exclusive auction.

The ad duration is added to the programme's airtime, extending its schedule
occupancy. The player chooses total ad time for the airing; placement within the
show is not modeled. Unsold minutes can remain empty or carry station-made teasers
and trailers, which are self-promotion rather than advertiser revenue. Campaign
delivery thresholds, measurement and refund clauses are deferred for this market
design; they must not be smuggled into offers as implicit promises.

This replaces the current one-advertiser-per-airing fixed delivery-contract model
as the intended future market direction. The existing implementation remains the
provisional experiment described above, including its explicit refunds. Pricing
must eventually account for audience fit, expected performance, finite advertiser
budgets, offered supply and non-exclusive competing demand without treating
expected counts as measured truth.

Agreed information boundary: hype responds to internally simulated exposure;
audience measurement produces an observed estimate, potentially noisy, biased
or delayed. The intended commercial direction is settlement against an agreed
measurement method, not omniscient truth. Forecast, internal delivery, reported
delivery and contract-credited delivery must stay distinguishable.

Current contracts still settle against internal expected relevant delivery,
and browser reports explicitly expose provisional exact expected counts.
Measurement and measurement-based settlement are not implemented. Do not claim
otherwise or add arbitrary noise without specifying its sampling and timing.

The station observes estimated outcomes. Attribution is uncertain when campaigns, competitors, news, and social transmission overlap. Brand health should use a coarse commercial model rather than equating hype directly with revenue.

## Events, publicity, and politics

Purpose: supply topical programmes, guests, risks, and future planning opportunities.

Events carry time, participants, subjects, visibility, causes, and consequences. Where relevant, they have anticipation, occurrence, and aftermath. Combine external events, scheduled developments, and endogenous events.

A relationship event may become a story depending on visibility, fame, surprise, and audience interest. Publication changes awareness and beliefs. Attention, approval, station trust, and advertiser suitability can move differently.

World events affect groups differently. International tensions may increase news demand for one group and escapist viewing for another.

Later elections: calculate eligible population, turnout, and divided support within each group. Support depends on issue priorities, identification, perceived competence, candidate appeal, performance, and social/media experience. Apply electoral rules after votes are resolved. Media exposure is not an automatic positive vote modifier.

## Time, update frequency and delays

Agreed direction: one tick is 15 simulated minutes, four ticks per hour and
96 per day. No continuously running wall clock is required. Advancing a day
processes all intervening ticks and scheduled activity rather than resolving
only its destination. The browser now implements a bounded tick planner; legacy CLI comparisons retain
whole-hour point snapshots. See [programme-planning cadence](programme-planning.md).

A programme occupies its duration across ticks. Tick resolution is not a
requirement to run every rule on every tick:

| Trigger | Relevant updates |
| --- | --- |
| Programme start / competing schedule change | Audience choice and expectations |
| Watched interval / ad block | Exposure and appropriate viewing/advertising response |
| Programme completion | Accumulated reception, familiarity and series consequences |
| Daily boundary | Selected gradual cultural processes and summaries |
| Due tick | Research/report delivery, completion or settlement |

These triggers remain the broader proposed processing semantics. The implemented
planner uses a provisional one-start-snapshot cadence, completes reception once
at the work's end, and advances passive decay/deadlines at every tick. It does
not integrate interval viewing or ad blocks; see [exact implemented order](programme-planning.md). In particular,
splitting a work into four ticks must not charge four full broadcast costs,
treat it as four fresh episodes, or multiply full-viewing satisfaction updates.
The implemented normalization is one sampled delivery opportunity per airing,
with no sum of tick audiences. Stored starting reception is applied once at
completion. Duration-integrated exposure and retention remain future work. Account for interval viewers separately from unique reach.

Use consistent pre-update snapshots for interacting changes and deterministic
ordering for scheduled actions. Exact phase ordering remains open. Quantities
with elapsed-time rates must retain their meaning when frequency changes.

Delays can be simple due-tick records: information may arrive later than the
event it describes; gradual effects can update selected remembered values.
Fuzzy rules may calculate a target or rate; persistent state supplies memory.
Do not build a general effects-entity framework or detailed social propagation
without a concrete gameplay need.

Internal schedule, publication and promotion are distinct. Awareness of a work,
anticipation for a particular airing and recurring-slot habit have distinct
meanings. Early publication creates an opportunity to be discovered, not
automatic unlimited hype. Promotion may build anticipation with diminishing
returns; moving/cancelling announced work can lose viewers or trust.
Precise buildup, decay, cancellation and habit equations remain open.

## Ad minutes and advertising load

The current provisional market and its explicit delivery refunds are described
in [advertising economy](advertising-economy.md); this section records the intended
minute-based direction.

The proposed market has the station choose total commercial minutes for a
particular airing. That time extends the show's schedule occupancy; it does not
consume an interval inside its programme runtime. Detailed break placement and
individual arrival/departure simulation are out of scope. The station offers that
inventory to the market, accepts non-exclusive offers for some of it, and may
leave minutes unsold or use station teasers. See [ad-minute market](#proposed-ad-minute-market).

Commercial load may affect viewer annoyance, departures and longer-term station
expectations, with different audience tolerances. Its effect should depend on total
ad minutes and programme context, not on a simulated position for every spot.
Audience reach and ad duration/dose remain distinct measures; do not multiply down
reach and dose twice. Exact exposure and refund measurement are deferred alongside
market settlement terms.

## Audience evolution

Long scenarios should make cohort evolution visible without excessive routine
scheduling. Ageing, careers and demographics can update monthly/yearly rather
than every tick. Group splits conserve population and inherit relevant memories,
preferences and relationships. New cohorts and exits are demographic processes;
new cultural identities often emerge from existing people.

## Research and learning

Research trades money and time for evidence: broad polls, targeted studies, concept tests, pilots, campaign studies. Results can be noisy, biased, delayed, or outdated, but should still have measurable decision value.

Proposed [Insight](resource-economies-idea.md#insight-and-market-research) is a
station-wide capacity generated through research and investigative preparation,
including magazine and talk-show interview work. Preparation can teach the station
more about a guest than the resulting programme reveals. This Insight is retained
even if the show is not aired; broadcasting affects public knowledge separately.
Insight can be spent to improve a
specific estimate. It can target audience/programme response, licensing-market
demand, actor-role fit, or cast chemistry. These are separate questions and reports
retain their evidence date. Public cinema reception makes existing titles easier
to estimate than unmade work; later public news may update current beliefs without
altering what an earlier report knew. Diminishing returns leave uncertainty in
place. The browser implements a bounded version for audience groups. See
[audience research](audience-research.md).

Audience polling is automatic and group-specific, with paid frequency
tiers up to daily. Polling produces snapshots but does not preserve familiarity or
reduce noise. Familiarity with each group decays regardless of polling. Spending
Insight raises that familiarity immediately; the improved estimate appears only
at the next scheduled poll, which combines current group values with
familiarity-scaled noise. Thus frequency helps the player observe trends, while
Insight improves snapshot quality.

People and pair relationships use decision-triggered research instead of a fixed
poll schedule. Before an audition, the player can spend Insight to assess a
candidate's role fit, chemistry with named collaborators, or both. Auditions
produce dated estimates using current familiarity and persistent bias; casting
without an audition remains allowed. Working together can build familiarity, while
inactivity lets it decay. Audition research and audience polling share the
familiarity/bias model but have different triggers. Auditions remain proposed. The browser Audience panel shows the latest dated poll.

Poll noise is persistent across snapshots. For each estimated attribute, retain a
hidden normalized bias `b` that drifts only slightly (optionally mean-reverting).
Conceptually, `estimate = clamp(actual + uncertainty(familiarity) × b)`; familiarity
changes the scale, while the bias direction generally persists. A low-familiarity
poll can be farther from truth, but should not jump from underestimating to
overestimating solely because a fresh independent random value was drawn. Exact
parameters for the audience experiment are documented in [audience research](audience-research.md).

The latest published audience snapshots also feed player and advertiser estimates
of ad-slot reach and value. Offer-time pricing uses available information and its
uncertainty, not hidden true audience preferences or group sizes. Actual delivery
remains world state; what evidence eventually settles a delivery-conditioned
contract is a separate, unresolved measurement rule.

The eventual research interface should not show exact future ratings. The browser integration milestone temporarily exposes exact model delivery forecasts as contractual terms and exact expected past counts, labelled provisional rather than measured ratings; it adds no noise or research mechanics. Do not conceal every useful cause behind noise. Player experiments should improve understanding over repeated decisions.

## Optional AI concept interpretation

A later model may translate free-text pitches into structured genre, theme, tone, complexity, originality, and production-demand attributes, plus an editorial report.

Core outcomes remain in the simulation. Do not make an arbitrary model quality score authoritative. Store accepted interpretations so saves remain stable. Structured concept entry must work without AI; model choice, hardware, and integration are open.

## Verification and balancing

Before expanding scope, investigate:

- Reproducibility from seed, state, and actions.
- Population, viewer, money, rights, and contract accounting.
- Smooth, bounded effects and diminishing returns.
- Stars attracting premieres without rescuing persistent dissatisfaction.
- Meaningfully different audience reactions and advertiser targeting.
- Research improving decisions across multiple seeds.
- Adaptation outperforming rigid repetition in suitable changing markets.
- No universally dominant genre, demographic, or production recipe.
- Stable long runs without runaway hype, population fragments, or feedback.
- Developer explanations tracing surprising results while player evidence remains limited.

Separate random streams or equivalent isolation are proposed so unrelated new random draws do not unnecessarily disrupt comparisons.

Implemented Sway mechanics are documented in [pooled Sway](pooled-sway.md): two
finite production requests, achievement-timed rewards and atomic booking uses.
The broader resource directions above remain separate from these provisional rules.
