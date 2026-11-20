# Resource economies — design direction

The player sees **Funds** with a fictional barred F. See [the office](station-office.md)
for presentation and [the ad market](ad-minute-market.md) for money and settlement.

Funds, Insight spending and Sway are playable in bounded form. The broader
earning loops and the Glamour, Credibility and Backing economies are proposals. See [game design](design.md#station-resources-and-editorial-capital)
and [simulation](simulation.md#resource-generation-and-editorial-outcomes).

## Distinct economies

| Resource | Generated through | Holding it | Spending or developing it |
| --- | --- | --- | --- |
| Funds | Advertising receipts and rights resale | Covers obligations | Fees, rights and production costs |
| Insight | Polling, research staff and completed studies | Research capacity available to the station | Improve estimates about audiences, programmes, markets and people |
| Sway | Fulfilling people's requests | Available cooperation capacity | Smooth negotiations, collaboration and production chemistry |
| Glamour | Winning awards with productions | Better conditions with glamour-seeking talent; naturally decays | Significant boosts to specific airings |
| Credibility | Consistently truthful, neutral reporting to the best of available knowledge | Strengthens news effectiveness; low reserves weaken it | Amplify opinion pieces to gain backing |
| Backing | Impactful opinion that wins support for an editorial position | Support distinct from habitual viewing | Slowly develop audience loyalty |

There is no agreed universal conversion rate, common regeneration system or
shared decay rule. Glamour's natural decay is explicitly intended.

## Insight and market research

Insight is a station-wide research resource, distinct from Funds and
Credibility. Proposed sources include preparation that teaches the station something:
polls, studies, researcher work, and investigative preparation for magazines or
talk-show interviews. Preparing an interview can reveal more about a guest than
the material ultimately aired. That knowledge belongs to the station even if the
programme is delayed, changed or never broadcast. Airing the programme changes
what becomes public; it does not generate this private preparation Insight.
The station can spend accumulated Insight on a particular question to improve its
estimate; it does not directly change the underlying audience, market or person.

Possible questions include expected audience response to a programme or licensed
title, likely competitor interest in an acquisition, an actor's fit for a role,
and whether two prospective collaborators are likely to work well together. Role
fit and interpersonal chemistry are separate questions: either may be encouraging
while the other remains uncertain. This can inform negotiation and casting without
guaranteeing success or exposing exact hidden skill values.

Begin with public evidence. A film's cinema reception, box-office performance and
general reputation are widely available; an established market title is therefore
easier to assess than an unmade original. Research adds a better but still
imperfect estimate. Private role fit or chemistry starts with less evidence. If a
relationship later becomes public through news, that disclosure updates current
player knowledge; it does not rewrite the older report as if the research team
already knew.

Preparation can add subject knowledge directly or generate Insight that is then
allocated to a question; which route applies should be clear in the activity. Each
report should identify its subject and question, evidence available at the time,
date, estimate/range and confidence or uncertainty. Additional Insight gives
diminishing improvements and never reveals exact world truth. Reports persist as
dated evidence and can become stale when audiences, competition, casting or public
relationships change. New research or news may update the player's current view,
while retaining the old report for context.

Insight should be a spendable research capacity rather than a universal currency
for unrelated actions. Researchers, polls and investigative preparation cost
cash/time and generate it; targeted analysis spends it to narrow a defined estimate.
Programme reach is not a prerequisite for earning preparation Insight. Whether it replenishes passively,
how reports expire or degrade, which sources can be public, and how confidence maps
to numeric ranges remain open. The browser scenario seeds a spendable balance for the [audience research experiment](audience-research.md). Insight generation remains proposed.

### Audience polling cadence and familiarity

Audience polling is automatic on one station-wide schedule selected by
the player. The browser pilot offers off, daily or every three days.
A successful paid survey publishes a new dated snapshot for every group.
Insight is targeted at an individual group. Frequency controls how often
fresh evidence appears; familiarity still decays between surveys.
[Audience research](audience-research.md) specifies the implemented fees,
bias drift, clipping and due-time processing.

The station separately tracks familiarity with each audience group. Familiarity
decays continuously/at a regular simulated-time update even when the group is
polled. It represents accumulated understanding and controls the noise range of
future estimates. Spending Insight on a group raises its internal familiarity
immediately, but the player sees no revised estimate until that group's next
scheduled poll. The poll then mixes the group's actual current values with
familiarity-scaled noise and publishes a dated snapshot. Noise is temporally
correlated, not independently redrawn each poll: each hidden attribute has a
persistent normalized bias value whose direction changes only gradually. A simple
conceptual form is `estimate = actual + uncertainty_scale(familiarity) × bias`,
with bounds appropriate to the attribute. If the station underestimates a group's
interest today, tomorrow's estimate will usually remain low; increased uncertainty
widens the effect of that same bias rather than randomly flipping it. Bias can drift
slowly, preferably with mean reversion, so it can change over time without erratic
jumps. Familiarity never removes all uncertainty. Frequent polls expose the trend
of the estimate but do not by themselves eliminate a persistent bias.

Pay Funds to observe all groups more often, or spend Insight on a selected
group to improve the quality of its subsequent evidence. The Audience file
shows the latest published snapshot. An Insight spend raises familiarity
immediately and becomes visible in the estimates with the next successful
survey. Targeting individual attributes is still an open design question.

This knowledge model is not audience-only. Familiarity and persistent estimation
bias apply to researched entities and attributes generally: actors, directors,
writers, programmes, brands and relationships can all be better or worse known.
Audience groups use scheduled polling, but people and pair relationships are
researched when the player has a concrete decision. Before booking an audition,
the player may allocate Insight to examine a candidate's fit for a particular
role, cast chemistry with named collaborators, or both as distinct questions.
The audition is an active research action, not a background poll. The player may
also cast without an audition, relying on current public information and existing
familiarity. Working with a person can raise familiarity; not working with them
lets it decay. A later news disclosure changes current public evidence and can
correct the station's current view, but it does not rewrite old auditions or
research reports.

Published audience estimates feed offer-time expected reach. The current
market uses the published estimates while retaining internal advertiser demand.
Direct pricing adjustments for snapshot age and uncertainty remain open. Better Insight can improve the station's view of likely
value, while more frequent polling can reveal changes over time. Keep this forecast
role distinct from actual world delivery. The minute market settles the aired inventory. Measurement-based audience
guarantees remain a separate design decision.

Use the audience pilot for provisional bias and familiarity equations.
Attribute correlations and audition timing and costs remain open.
Repeated polls are evidence for the player to interpret; the game does not
automatically average away the persistent bias unless an explicit analysis action
does so.

## Sway

Sway is pooled, abstract social capital. Grant requests such as pairing particular
talk-show guests, casting an actor, realising a screenplay or accommodating an
anchor's requested tone. Fulfilling these earns Sway that can be spent elsewhere,
including smoothing cooperation within productions. No individual creditor
restricts redemption. Accept timed opportunities independently of production
bookings; track open quests and show production matches without promising success.
See [quests](quest-opportunities.md) for implemented deadlines and the proposed
outcome-gated/graded reward direction. Requests should create meaningful costs or constraints;
ordinary paid work earns no Sway automatically. Future requests can support
comebacks, opportunities for prodigies, excluded guests, neglected topics and
ambitious adaptations of supposedly unfilmable novels. Those require their own
concrete achievement conditions and are outside the implemented discussion slice.

Implemented bounded subset: [pooled Sway](pooled-sway.md) replaces the personal
favour prototype with two finite discussion-production requests, once-only
achievement rewards, a station balance/history and two paid uses: exceptional
participation and cooperation smoothing. Amounts and effects are provisional.
Broader request types, rates and uses remain open.

### Adjustable production support

Updated direction: production cooperation spending should use a free slider or
dial, not a fixed yes/no purchase. The player can allocate any available amount
of Sway, including zero, with diminishing marginal returns. The [casting milestone](casting-desk.md) now implements whole-point allocation
with a bounded diminishing-return curve. Coefficients remain provisional; the
previous checkbox/tone-fit boost has been removed.

Its production function is mitigation of identifiable cooperation deficiencies:
bad cast chemistry, friction between an actor and director, or similar difficulties
working together. Sway must not compensate for missing craft, weak acting/writing/
directing ability or other skill deficiencies. Do not apply a general quality
bonus or silently increase underlying skill values.

Keep these causes separate in the model and developer evidence. More support
can reduce an existing cooperation penalty within bounds, with smaller gains as
spending rises; it cannot create a surplus quality bonus once that penalty is
removed. With no relevant cooperation deficiency there is nothing to recover.
Some conflicts may remain despite investment. Fees, time commitments and explicit
participation negotiations remain separate constraints. Staff/player explanations
must use legitimate evidence rather than expose exact hidden chemistry or skills.
This correction does not implement a general relationship simulation.

## Glamour

Award wins generate Glamour on an award-cycle timescale. Retained Glamour
improves conditions with talent interested in prestige. It naturally decays,
but can also be spent on significant performance boosts to individual airings.
Keeping prestige versus committing it to a particular broadcast is the decision.

Open: award eligibility/judging, decay rates, talent conditions, spending costs,
and whether an airing boost affects attraction, reach or another dimension.
Do not silently equate it with improved truthfulness, craft or enjoyment.

## Credibility → opinion → backing → loyalty

Credibility is difficult to earn: news must remain consistently truthful and
neutral to the best of available knowledge. This requires competent staff,
verification, good sources, preparation and editorial judgement. Story selection
and framing matter. A slider cannot guarantee neutrality or credibility.
More broadly, all production settings express intentions whose achievement
depends on people, resources and execution.

Stored credibility strengthens news effectiveness. Spend it to lend authority
to an opinion piece, increasing its impact and potential backing. With little
credibility left in stock, subsequent news has less effect. Rebuilding authority
requires sustained reporting rather than a quick setting change.

Backing represents support for the position advanced. The preferred use is
slowly transforming it into audience loyalty through sustained service and
coverage. Supporters do not instantly become habitual viewers. Converting
Backing into Sway was considered; loyalty is the preferred direction.

Credibility and Backing require consistency and can pull against one another:
serving supporters may compromise neutrality, while neutral reporting may
disappoint those supporters. They are not necessarily mutually exclusive.
Labelled opinion is not automatically dishonest; factual failures and concealed
advocacy may have additional consequences.

Open: how backing is attached to audiences or causes; how it becomes loyalty;
measurement of neutrality and reasonable knowledge; evaluation windows and
generation rates; rebuilding from zero credibility; consequences of corrections,
misinformation and changes in editorial direction. Balance does not mean
assigning equal weight to unsupported and well-evidenced claims.

## Implementation boundaries

No generic money exchange or interchangeable resource bonuses. Work through
a favour/Sway example, an award/Glamour cycle and a reporting/opinion/loyalty
sequence before choosing formulas. Preserve the distinction between actual
world state, editorial knowledge, public perception and player evidence.
Bounded Sway and audience research are implemented. Glamour, Credibility, Backing and the wider editorial resource loop remain proposals.
