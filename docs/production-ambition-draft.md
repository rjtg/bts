# Production ambition — draft

Status: discussion draft, November 2026. Captures the intended move toward
ambition and staged delivery; mechanisms and examples below are proposals,
not implemented behaviour or a new implementation commitment.
See [game design](design.md), [simulation](simulation.md) and the
[current production experiment](original-production.md).

## Core idea

The player starts by creating a **show**: a durable creative identity and plan,
not a completed programme. The show has a format, ambition and intended run shape.
It may be a one-shot, a finite run with a chosen episode count, or open-ended
(such as a news or discussion programme). A sequel can explicitly continue an
existing original or series, retaining its identity and continuity while defining
the new run's ambition. Creating the show is a low-cost planning action; preparation
and production create distinct work and commitments later.

Each instalment then follows the stages appropriate to its format:

`show and ambition → prepare material → produce the instalment → schedule/air`

Preparation might mean writing a script, researching a report, or preparing a
talk-show edition. Production uses the prepared material with the required people,
studio and other resources. Format-specific stages can differ; do not force every
show through a writer-room pipeline. Hiring a writer, reporter, director, host or
cast buys an attempt at an ambition, not a guaranteed result from adding attributes.
Every stage has demands and achieved delivery.

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

## Preparation freshness and shelf life

Prepared material is a distinct, versioned deliverable with a completion time,
intended instalment or episode, achieved attributes, defects and downstream
requirements. It may be ready before it is produced, but usefulness can depend on
format and subject:

- A news report may need same-day research and production/airing. A changed event
  can make prior work stale or require paid updating.
- A talk-show edition may remain usable for a week or two, unless its topic is
  unusually time-sensitive.
- A film or scripted series may remain ready much longer; cast, studio and release
  plans can still change independently.

These are illustrative bands, not settled durations. A stale item should create a
clear update/research decision or a quality/credibility risk, not disappear without
explanation. Track freshness on prepared material, not by changing the show's
identity or ambition. Open: whether shelf life is a hard expiry, gradual decay,
event-driven staleness, or a combination; how much research can be reused across
editions; and when work is locked to a particular episode.

## Run shape, episodes and sequels

The show declares one-shot, finite run with planned episode count, or open-ended
run. A finite plan communicates scale and obligations; it does not require booking
all episodes at once. Open-ended shows can still have separately prepared and
produced editions. A second season or revival links to the existing show/series
identity rather than making a name-based copy. It may change ambition or format
within explicit compatibility rules. Episode sequence, prior audience reception,
contracts and prepared scripts remain separately identified.

Whether an open-ended programme needs an explicit renewal point, how the player
ends a run, and how changes between seasons affect continuity are open questions.
News and talk shows need a preparation cadence tied to their subject matter, not a
fictional season limit solely because the engine requires a count.

## Procedural ambition across episodes

A procedural has a stable **show ambition**: its established genre promise, tone,
and typical intensity. Each episode may deliberately vary along those same
dimensions. A lighter procedural can try a darker episode or a more action-heavy
case; variation creates opportunity and risk rather than being automatically
forbidden.

Track episode ambition against both the show baseline and recent episodes. The
farther an episode departs from the established show, the harder it is to realise
and the greater the risk of weakening the show's identity or audience trust. A
single successful experiment may broaden what the audience accepts, but repeating
extreme departures can accumulate strain. Returning toward the established tone
can let that strain recover; a slightly lighter episode may help, but recovery
should be gradual and must not erase earlier consequences instantly.

Episode-to-episode inconsistency is a separate source of difficulty. Large,
frequent swings can feel erratic even if each episode individually fits the broad
show ambition. Compare the current episode with the previous episode as well as
with the show baseline. Penalize repeated zigzagging more than a coherent arc that
changes gradually in one direction and resolves deliberately.

For finite runs, allow the player to set episode ambitions together as a curve.
For example, a procedural might begin with restrained tension, build it over
several episodes, and resolve the season with an unusually action-heavy finale.
The curve communicates intended progression and lets preparation plan toward it;
each episode still has to deliver its own ambition, and abrupt steps remain risky.
The player may also revise future points in the curve, with already produced or
aired episodes retained as history. Open-ended shows can plan a rolling arc over
the next few episodes without committing to a final episode count.

Keep at least three causal records distinct: deviation from the show's established
ambition, change from the immediately previous episode, and accumulated audience
response/identity strain. A coherent planned curve can reduce the inconsistency
cost of gradual movement, but must not cancel the production difficulty of an
extreme episode. Audience response can update the show's accepted range over time;
the exact learning rate, recovery rate, and whether strain is group-specific remain
open. These are proposals, not mechanics in the current episode model.

## Roles, casting and contractual consequences

Scripts should carry required roles and their episode appearances separately from
the actors eventually cast. Recurring characters imply casting continuity across
specified episodes. Rewriting can adapt those requirements, but takes paid work
and does not automatically release actors from, or cancel, existing contracts.
See [talent and contracts](talent-and-contracts.md) for the distinct market,
contract and assignment layers and open continuity/release rules. This remains
future work; the first writing experiment need not implement a full casting pipeline.

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
