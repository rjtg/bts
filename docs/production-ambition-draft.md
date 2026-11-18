# Production ambition — draft

Status: discussion draft, November 2026. Captures the intended move toward
ambition and staged delivery; mechanisms and examples below are proposals,
not implemented behaviour or a new implementation commitment.
See [game design](design.md), [simulation](simulation.md) and the
[current production experiment](original-production.md).

## Core idea

Hiring a writer, director and cast buys an attempt at a creative ambition,
not a guaranteed result from adding their attributes together.
Every project or production stage has an ambition, demands and achieved delivery.

| Element | Meaning |
| --- | --- |
| Ambition | What the work attempts, across several dimensions |
| Demands | Skills, time, resources and coordination required to achieve it |
| Delivery | Achieved strengths, shortcomings and compromises |

A writing brief might aim for strong humour, elaborate action, convincing
romance and a little science fiction. These dimensions are not one quality
slider. Writers or writing teams receive time and consequently pay to attempt
the brief. Relevant skill and working conditions determine how fully they
deliver each aspect. Success and failure are gradual; selected fuzzy rules
may suit these judgements, but no formula or stochastic model is chosen.

## Work, review and revision

Proposed loop: set ambition → allocate team and paid time → work →
review progress → revise, extend or accept.

Intermediate work and reports should give the player decisions before final
completion. Extending a stage costs money and may collide with talent bookings
or broadcast deadlines. Reducing ambition can require rework; it must not
instantly erase defects. More time helps within limits, rather than guaranteeing
that any team can eventually achieve anything.

Production settings express intentions. Their achievement depends on execution,
including skills, preparation, material and constraints. This principle applies
to writing, directing, acting, reporting and other relevant production work;
formats need not share an identical sequence of stages.

## Overambition and achieved quality

An overambitious script can fail through confusing plotting, jokes that do not
land, unearned romance or poorly integrated science fiction. A modest script
executed well should be able to outperform it. Ambition is not itself a penalty:
shortfalls produce concrete defects that affect the work and audience response.

Track intended ambition separately from achieved material. Different audiences
may value its strengths and tolerate its defects differently. Avoid collapsing
everything into one opaque final score or making the lowest ambition universally
optimal.

## Downstream demands

A completed script carries achieved creative attributes, defects and production
requirements into later stages. Demanding material can require greater directing,
acting, rehearsal or technical skill to realise. Later talent cannot automatically
erase flaws in the script; adaptation and rescue involve choices and compromises.

Creative ambition and downstream production difficulty should be separable.
A brilliant two-person comedy can be difficult to write but cheap to shoot;
an ordinary action story can be easy to write and difficult to film.
Writing something easier for a later team to realise can itself require more
writing effort or skill. Explicit downstream skill requirements could be part
of the brief, with stricter limits making the writing task harder.

Exactly how to author these constraints, whether difficulty is explicit or
derived, and how demands transfer between stages remain open.

## News and topical programmes

Truthful, neutral reporting is a high ambition, especially for less-skilled
reporters or difficult, poorly documented subjects. Verification, corroboration,
context and fair framing consume skill and time.

Under pressure the player can narrow coverage, allow more preparation, qualify
uncertain claims, delay a piece or risk errors. Choosing a neutral editorial
profile does not guarantee neutral output. Assess reporting against reasonably
available knowledge, keeping actual truth, staff knowledge and player evidence
distinct.

News still uses the shared production workflow and can draw automatically from
current events; this proposal does not require manual story-by-story assembly.
Credibility should follow achieved reporting over time, as described in
[resource economies](resource-economies-idea.md), rather than a selected setting.

## Information and implementation boundaries

Player reviews provide legitimate, potentially imperfect assessments of progress
and shortcomings. Developer traces can expose exact causes; player screens must
not reveal hidden skill coefficients or guarantee future reception.

The current standalone discussion experiment uses fixed durations and a bounded
arithmetic execution model. This draft proposes a richer direction; it does
not claim existing mechanics already implement writing, staged reviews or
ambition shortfalls.

Open: ambition dimensions, time/pay contracts, team coordination, assessment
accuracy, revision costs, cancellation, diminishing returns, downstream adaptation
and uncertainty. If randomness is used, runs must remain reproducible.

## Suggested first experiment

Prototype one writing stage with a small fictional writer roster, a few ambition
dimensions, paid time and an inspectable script result. Add at least one progress
review and a meaningful extension or revision decision.

Compare the same writer tackling a feasible and an excessive brief, plus a
stronger writer tackling the demanding brief. Demonstrate a modest success
outperforming an overambitious failure and a demanding success having worthwhile
upside. Expose downstream demands on the resulting script without implementing
the full director/cast pipeline yet. Label all balance parameters provisional.
