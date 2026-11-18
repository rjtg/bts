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
of Sway, including zero, with diminishing marginal returns. The current checkbox
and fixed 2-Sway treatment are an implemented prototype, not this intended interaction.
Exact spending granularity and the response curve remain to be chosen.

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
Only the bounded Sway subset is implemented. Glamour, Credibility, Backing and
the wider editorial resource loop remain unimplemented.
