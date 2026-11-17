# Simulation model

Status: causal direction is agreed; algorithms, parameter values, update ordering, and several mechanisms below are proposals to validate. This document is not a final mathematical specification.

The [headless experiment](headless-experiment.md) now implements a bounded
subset: deterministic expected viewing shares, separate attraction and
satisfaction, series willingness, and exposure-driven hype with decay. The
[bounded economy](advertising-economy.md) connects that hype to finite-budget
offers, target delivery contracts and station cash.
Its precise equations are provisional experiments; the broader mechanisms
below remain proposals. Developer reports expose actual state and are not
player evidence.

## Purpose

Create a world that is changing, partially observable, and learnable. Observation and adaptation should help the player outperform blind repetition without guaranteeing success.

## State and knowledge

Maintain three separate layers:

1. Actual world state: skills, relationships, audience preferences, campaign effects.
2. Public perception: recognition, reputation, rumours, beliefs.
3. Player evidence: ratings, polls, focus groups, staff assessments, reports, trade press.

The simulation can inspect causes exactly in developer tools. Player reports must respect source knowledge, delay, and uncertainty. A misleading rumour changes beliefs, not the underlying fact.

## Audience choice and satisfaction

Purpose: make programme selection, timeslots, promotion, and production choices matter.

Proposed sequence per broadcast slot:

1. Determine available viewers and access constraints, including household restrictions.
2. Evaluate options using known programme appeal, awareness, talent, loyalty, slot fit, and competition.
3. Allocate viewers among stations and not watching; a group can split across choices.
4. Resolve enjoyment from actual execution and audience fit.
5. Update loyalty, reputation, word of mouth, and fatigue.

Initial attraction and experienced satisfaction are distinct. A star can drive a premiere that loses viewers later. Promotion cannot directly manufacture enjoyment.

Account for the same viewer only once within a slot. Advertising exposures count viewers of the associated broadcast; cumulative reach and repeated exposures are different measures.

Evidence: audience composition, ratings, return rates, satisfaction studies, and reports with appropriate uncertainty.

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

Concept → development → optional pilot → production → broadcast → renewal, revision, or cancellation.

A project specifies intended format, themes, tone, ambition, people, budget allocation, time, and production demands. Role-specific craft, task fit, relationships, and resources affect different execution dimensions.

Spending has diminishing returns. Not every low-budget show is weak: demands matter. Collaboration improves familiarity but can also produce conflict or predictability.

Actual abilities, hiring assessments, and public fame remain distinct. Production time creates market risk because tastes and competition can change before release.

Evidence: estimates, staff reports, milestones, pilots, and later audience response.

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

A brand's target-hype gap drives urgency. Expected useful exposure, budget, competition, and confidence constrain its offer. Closing the gap can reduce immediate spending; decay and new launches create future demand.

Purchasing power is one targeting dimension. Affordable mass products and niche luxury products need different strategies. Purchase influence can cross household links: children may create demand fulfilled by parents.

Implemented provisional contract experiment: a fixed maximum payment for weighted relevant target delivery in a booked station/hour, with an inclusive deadline and proportional underdelivery refund. Money is integer credits; offers reserve finite budget in escrow. Per-audience hype deficits value each audience's forecast delivery; brand-wide pressure is diagnostic. Accepted prices are not retrospectively changed by hype. Unfunded programmes go off air independently before allocation, and missed delivery refunds at the deadline. See [economy equations](advertising-economy.md). More detailed contracts, bonuses, renewal negotiations and campaign studies remain open; this experiment is not a permanent commercial design.

The station observes estimated outcomes. Attribution is uncertain when campaigns, competitors, news, and social transmission overlap. Brand health should use a coarse commercial model rather than equating hype directly with revenue.

## Events, publicity, and politics

Purpose: supply topical programmes, guests, risks, and future planning opportunities.

Events carry time, participants, subjects, visibility, causes, and consequences. Where relevant, they have anticipation, occurrence, and aftermath. Combine external events, scheduled developments, and endogenous events.

A relationship event may become a story depending on visibility, fame, surprise, and audience interest. Publication changes awareness and beliefs. Attention, approval, station trust, and advertiser suitability can move differently.

World events affect groups differently. International tensions may increase news demand for one group and escapist viewing for another.

Later elections: calculate eligible population, turnout, and divided support within each group. Support depends on issue priorities, identification, perceived competence, candidate appeal, performance, and social/media experience. Apply electoral rules after votes are resolved. Media exposure is not an automatic positive vote modifier.

## Time and population

Proposed scales:

| Scale | Typical updates |
| --- | --- |
| Slot | Viewing, satisfaction, ad exposure |
| Day | Production, research delivery, discussion |
| Week | Reputation, fatigue, commissioning, cultural diffusion |
| Month / season | Career change, sustained trends, group migration |
| Year | Ageing, demographic and economic changes |

The precise calendar and player controls remain open. Long scenarios must allow cohort evolution to become visible without excessive routine scheduling.

Group splits conserve population and inherit relevant memories, preferences, and relationships. Merges need explicit aggregation rules. New cohorts and population exits are demographic processes; new cultural identities often emerge from existing people.

## Research and learning

Research trades money and time for evidence: broad polls, targeted studies, concept tests, pilots, campaign studies. Results can be noisy, biased, delayed, or outdated, but should still have measurable decision value.

Do not show exact future ratings. Do not conceal every useful cause behind noise. Player experiments should improve understanding over repeated decisions.

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
