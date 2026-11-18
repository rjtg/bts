# Sway opportunities and timed quests

Status: two finite discussion-production quests implemented, November 2026.
Audience-gated objectives and graded rewards below are design proposals, not
implemented mechanics or a general quest framework.

## Acceptance belongs to the station

Accept an opportunity from the Production desk's quest board, independently of
any project. Accepted quests appear under **Open quests**, with exact due time and
remaining simulated time. They are not promises selected while booking a show.
The two current quests may both be open. Accepting reserves no people, spends no
cash/Sway, and does not guarantee a feasible production or a reward.

Each quest has a provisional **192-tick / 48-hour** time limit from acceptance.
Deadline is `min(accepted_at + time_limit, scenario_end)`. The recorded deadline
never moves when the player changes projects or advances time. Completed and
expired quests remain inspectable and cannot be accepted again in the session.
Reset starts a fresh scenario. No abandonment, extension, renewal or quest generator
is introduced.

| Quest | Required achievement | Fixed current reward |
| --- | --- | ---: |
| Nora with Felix | Finish recording a discussion with Nora and Felix as the two guests | 4 Sway |
| Rafi's everyday-life discussion | Complete a calm everyday-life discussion with Rafi as guest into the library | 3 Sway |

Only recordings starting at or after acceptance can qualify. Work already started
or completed cannot be rewarded retroactively. A project booked before acceptance
may qualify if its recording has not started yet. No project-specific promise is
required. Rust checks matching conditions at the actual achievement boundary, not
at booking or from a UI hint. Deadline is inclusive for that achievement: finishing
at the deadline earns; finishing a tick later does not. At a deadline report, a
quest is completed if earned on that boundary, otherwise expired.

Fulfilment records the production ID/time, adds the reward once and writes the
Sway ledger. Further matching shows/repeats cannot farm rewards. Configuration
locks once a quest is accepted, even at tick zero, so acceptance cannot be erased
through fixture reconfiguration. Ordering is deterministic; if several qualifying
projects finish at the same boundary, the first stable production ID is recorded.

## Production hints are not fulfilment

Rust reports hints on each production for open quests. They distinguish matching
creative/cast requirements from timing eligibility, using the accepted deadline,
recording start and required achievement time. The browser says **could fulfil if
completed as planned**, not **reward guaranteed**. Accepted work in progress can
remain on track; an unstarted draft whose date has passed cannot.

The current binary quests have known production-only conditions. Later objectives
may depend on a broadcast, audience outcome or another event that cannot be known
while producing the work. For example: a prodigy must star in a film seen by at
least 2,000 teenagers. Matching the cast and format would satisfy only prerequisites;
the viewing threshold would remain pending until the qualifying broadcast evidence.
Do not treat a production match, public fame or exact hidden execution as a forecast
of audience success. The viewing unit (unique people versus impressions), permitted
broadcasts, deadline and evidence/measurement basis must be specified before that
objective is implemented. Current expected counts are still provisional reporting.

## Graded rewards: proposal

A future quest may reward how well it was fulfilled rather than always paying a
single amount. Objectives should define meaningful minimum achievement and any
additional quality/reach/ambition criteria, with visible bounded reward terms at
acceptance. Actual outcome determines the reward; a booking setting does not.
Do not turn every quest into a generic hidden-quality multiplier or a promise of
future earnings. Bonus ranges, partial credit, caps, deadlines and evidence sources
remain open. No graded formula or larger quest catalogue is implemented here.

Comebacks, prodigies, excluded guests, fringe reporting and ambitious adaptations
remain future examples. They must not be simulated through placeholder conditions
that claim those systems already exist.

## Protocol, observation and verification

Protocol v5 adds `accept_request { request_id }`; booking no longer accepts
`promise_request`. Quest observations whitelist conditions, fixed rewards, status,
acceptance/deadline, actual fulfilment and production hints. They disclose no hidden
skills, chemistry coefficients, hype or fuzzy traces. Failed commands preserve
quests, cash, Sway, commitments and pending offers. Accepting a quest is a state
change and invalidates old booking/advertising previews conservatively.

Native tests cover acceptance, no retroactive credit, both achievement phases,
exact/late deadlines, no repeat rewards, clipped scenario-end deadlines, configuration
locking, atomic rejection and reproducible tick/batch processing. Browser tests cover
acceptance, matching hints, completion and expiry. See [casting desk](casting-desk.md)
and [current evidence](../contrib/casting-desk/README.md).
