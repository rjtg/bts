# Resource economies — design direction

Updated November 2026. The directions below are agreed; formulas, costs and
unresolved mechanisms remain proposals. This is not an implementation specification
or an expansion of the current milestone. See [game design](design.md#station-resources-and-editorial-capital)
and [simulation](simulation.md#resource-generation-and-editorial-outcomes).

## Distinct economies

| Resource | Generated through | Holding it | Spending or developing it |
| --- | --- | --- | --- |
| Cash | Selling and effectively placing advertising | Funds obligations | Fees, rights and production budgets |
| Sway | Fulfilling people's requests | Available cooperation capacity | Smooth negotiations, collaboration and production chemistry |
| Glamour | Winning awards with productions | Better conditions with glamour-seeking talent; naturally decays | Significant boosts to specific airings |
| Credibility | Consistently truthful, neutral reporting to the best of available knowledge | Strengthens news effectiveness; low reserves weaken it | Amplify opinion pieces to gain backing |
| Backing | Impactful opinion that wins support for an editorial position | Support distinct from habitual viewing | Slowly develop audience loyalty |

There is no agreed universal conversion rate, common regeneration system or
shared decay rule. Glamour's natural decay is explicitly intended.

## Sway

Sway is pooled, abstract social capital. Grant requests such as pairing particular
talk-show guests, casting an actor, realising a screenplay or accommodating an
anchor's requested tone. Fulfilling these earns Sway that can be spent elsewhere,
including smoothing cooperation within productions. No individual creditor
restricts redemption. Requests should create meaningful costs or constraints;
ordinary paid work is not automatically a favour.

Open: amounts, fulfilment conditions, effective uses and repeat-reward safeguards.
The [implemented production experiment](original-production.md) currently awards
one named guest's personal favour. That behaviour remains documented as an
implementation limit; pooled Sway is the intended direction.

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
These resource systems remain unimplemented.
