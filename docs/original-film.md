# Original film experiment

The browser Film desk supports multiple standalone pitches. Each follows
pitch → script commission → script review → optional focused revision →
acceptance → cast and room booking → recording → edit → station library.
The same planner and advertising contracts serve films and discussion shows.

The seven-day scenario starts with 20,000 credits. The film roster has two
writers, two directors and three actors. Scripts use two named role slots,
The Lead and The Confidant. Freelancers are distinct from the discussion roster.
Film rooms are two fixed rentals, separate from the station's discussion room.
Each rental is exclusive during a booked interval. Talent and rooms can be
reused at the end boundary. The film roster is authored, with fixed availability
and no employment contracts. Rates, roles and durations are provisional.

## Time and credits

One tick is 15 minutes. Writing starts when commissioned and takes 48 ticks.
One revision can target a noted weakness and takes 32 ticks. The fee is half
the chosen writer's regular fee, rounded up. Writing payments are settled
immediately. Filming takes eight ticks and editing takes 16. The film booking
reserves the full talent and room price, then spends the reserve at the
recording boundary. Shared production accounts and project IDs preserve exact
money and prevent simultaneous reservations on one escrow. Rejected commands
preserve the planner and all balances. A completed film runs for 90 minutes
and costs 120 credits each time it airs. Airing revenue is settled by the
existing audience delivery contract.

## Creative model

Each fiction feature has an intended level and a baseline (0.5 for
playfulness, 0 elsewhere). Complexity is the intended level, or twice the
distance from neutral for playfulness. Writing capacity is 30% craft, 60%
feature capability and 10% subject fit. The achieved deviation from baseline
is the intended deviation multiplied by `1 − 0.8 × max(0, complexity − capacity)`.
A gap of at least 0.075 is flagged in script review. Revision closes
`0.25 + 0.40 × writer capability` of the selected feature gap.
Equipment demand is `0.2 + 0.75 × scripted spectacle`.

Direction, lead and supporting actor contribute to filming craft and
capability with weights 50/30/20. One authored director and actor
combination reduces cooperation to 0.8. It otherwise equals 1. Equipment fit
is room equipment divided by demand, capped at 1. The spectacle interaction
uses complementary Sugeno rules with the fuzzy AND of direction capability and
equipment fit. The output is their minimum because both inputs use linear
memberships and the rule pair has equal weights.

The finished feature inherits the script's deviation from baseline, scaled
by `(0.40 + 0.45 × capability + 0.15 × craft) × cooperation`. Spectacle uses
the fuzzy result as its capability. Overall execution is
`(0.20 + 0.65 × craft + 0.15 × equipment fit) × cooperation × (1 − mean script gap)`.
Awareness starts at 0.52. Cast appeal combines lead and supporting recognition
at 60/40. These are comparison parameters, not balance commitments.

Player reports show script qualities in bands and note gaps and room
requirements. Exact ability, production causes and execution stay in developer
evidence. The airing report shows provisional expected counts and coarse modeled
reception where a group has at least one expected viewer. This reception is
an experimental observation without independent measurement. Film investment,
each airing's cost and settled ad revenue are visible separately.

See the [playthrough and comparisons](../contrib/original-film/README.md).
