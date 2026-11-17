# First headless broadcasting experiment

Status: implementation specification for a bounded experiment. All equations,
content, units, and coefficients below are provisional, not approved balance or
a resolution of the wider simulation proposals. No randomness is used.

## Boundary and content

`bts-sim` owns TV rules, configuration, explicit simulated hours, ordered actions,
state, and immutable developer events. `bts-cli` runs fixed cases and renders
summaries/JSON. `bts-fuzzy` remains generic and independently extractable.
Developer report construction is separate from state mutation. Developer reports
expose actual tastes and model inputs; they are unsuitable for players. Public
beliefs and a restricted observation layer are not implemented.

Three groups: young viewers (6,000), households (11,000), specialists (2,500).
Each has genre affinities, a preferred tone, two slot availabilities, purchasing
power, and two product interests. Two stations compete at 18:00 and 21:00 each
day. Four programmes represent broad comedy, specialist science, promoted but
poorly executed comedy, and a reliable competing drama. Stable typed identifiers
explicitly connect programmes to series, broadcasts to stations/programmes/ads,
loyalty to audience/series, and hype/repetition to brand/audience.

The three cases change only the first station's programme. The competitor,
ads (specialist equipment at 18:00, everyday snacks at 21:00), times, populations,
and all initial state are identical. Six days of broadcasts are followed by an
empty slot 48 hours later, exposing decay without campaign activity. These are
viewer-slot impressions, not cumulative unique reach. Slots represent snapshots
of an hour-long broadcast, resolved at the named hour; there is no within-slot
channel switching, concurrent slot overlap, or individual viewer identity.

## Inputs and fuzzy rules

All feature values are finite in [0,1]. High(x) is the rising ramp from 0 to 1;
Low(x) is its complement. Each weighted low/high pair contributes constants
0/1, so the complementary pair's total effective weight is its configured
weight. The complete evaluator returns a weighted average, never activation
multiplied by the consequent. Effective-weight sums are neither probabilities
nor confidence. A focused regression contrasts a lone high rule at x=.25
(output 1) with a complementary pair (output .25).

Attraction = weighted fuzzy average of genre affinity (.35), tone fit (.10),
awareness (.30), and cast appeal (.25). Tone fit = 1 - absolute difference
between programme tone and audience preference. Execution is deliberately
absent: promotion and stars may attract viewers to disappointing work.

Satisfaction = weighted fuzzy average of genre fit (.15), tone fit (.15), and
execution (.70). Awareness is deliberately absent. These linear initial models
are intentionally simple fuzzy rule experiments, not calibrated psychology.

Advertising effectiveness uses a nested condition:
High(interest) AND (High(affordability) AND
(High(creative quality) OR High(satisfaction))). A complementary NOT rule emits
0; the positive rule emits 1, both weight 1. Thus output varies gradually and
is zero when interest or affordability is zero. Affordability =
min(1, purchasing power / brand's required purchasing power).

## Slot update order and equations

1. Validate the complete action before mutation: increasing legal timestamp,
   unique known stations, known programmes/brands; configuration and numeric
   bounds were validated at construction. A failed action leaves state intact.
2. From previous time to slot time, decay each brand/audience hype H (points,
   [0,100]) and repetition stock R (expected impressions/person):
   Hdecay = H * 2^(-elapsed_hours / 48),
   Rdecay = R * 2^(-elapsed_hours / 24). Initial H=5, R=0.
3. Read a common pre-broadcast loyalty snapshot L (audience/series, [0,1],
   initially .5). Utility per station = attraction² * (.2 + 1.6*L).
   Idle station utility=0. Outside/not-watching utility=.18.
   Available viewers = population * slot availability.
   Station viewers = available * utility / (.18 + sum station utilities).
   Not watching = population - sum station viewers, including unavailable
   people. Counts are fractional deterministic expectations, not sampled or
   rounded integer people. Reports also retain available non-viewers.
4. Compute satisfaction independently. For each audience/series, aggregate
   both stations before updating once:
   Lnew = L + .9 * (series viewers/population) * (viewer-weighted satisfaction-L).
   Unwatched series do not update. This is expected willingness, not tracked
   individual retention. Subsequent utilities reflect this new willingness.
5. One ad per broadcast reaches exactly that broadcast's viewers. For each
   brand/audience, aggregate station exposures before applying any hype pulse:
   E = sum viewers/population; Q = sum (viewers/population * effectiveness).
   Dose = .8 * Q / (1+Rdecay).
   Campaign gain = (100-Hdecay) * (1-exp(-Dose)).
   Hnew = Hdecay + campaign gain; Rnew = Rdecay + E.
   Exposure volume is separate from fuzzy effectiveness. Zero viewers implies
   exactly zero campaign gain. Repetition suppresses marginal response;
   remaining headroom provides saturation. State is bounded and checked for
   finite values. Decay and campaign gains are reported separately.
6. Commit state and explicit time, return a developer event. Report conversion
   does not mutate state. No filesystem, wall clock, randomness, or rendering
   occurs in simulation. Rule, audience, station, and brand order is explicit.

The constants above live in `Parameters` (learning, outside utility, half-lives,
hype gain) and the TV model rules; content is ordinary Rust data in `demo`.
Changing data does not require a parser or content-loading framework.

## Running and interpreting

Run `cargo run -p bts-cli -- demo` to compare cases. Use
`cargo run -p bts-cli -- demo --case targeted --json /tmp/targeted.json` for one
case and a machine-readable export; `--case all` is the default. JSON includes
configuration, initial assumptions, fixed schedules, fuzzy inputs and complete
condition/rule traces, allocation, satisfaction, loyalty, and hype decomposition.
`--trace` prints detailed per-slot diagnostics in addition to the summary.

The smaller audience case trades aggregate reach for specialist exposure.
The disappointing series can attract a premiere without sustaining return
willingness. Advertising effectiveness comparisons at controlled exposure
isolate targeting from reach. These experiments establish causal properties
of this provisional model; they do not prove balance, real behaviour,
profitability, season viability, or a universal best schedule. No money,
contracts, production, save files, demographics, AI, or UI are implemented.

## Observed demo results

With the default data and parameters on Rust 1.98.1:

| Case | First station viewer-slot impressions | Mean satisfaction | Specialist equipment hype in specialist group |
| --- | ---: | ---: | ---: |
| Broad | 91,002 | .798 | 24.07 |
| Targeted | 30,959 | .784 | 37.58 |
| Disappointing | 78,311 | .287 | 20.42 |

Hype in this table is measured after the final broadcast, before the cooldown.
The targeted case delivers higher specialist-group hype while the broad case
produces higher population-weighted equipment hype (9.72 versus 7.40). Thus
focusing on the small group sacrifices total reach and some aggregate brand
response; there is no economic value function deciding which is preferable.

For the disappointing series, young viewers' early-slot audience falls from
3,113 to 2,545 and willingness falls from .500 to .320. Initial attraction is
.990 despite satisfaction of .320. The same programme's attraction features
remain fixed; learning and the competitor's improving loyalty change shares.
After the final empty 48-hour interval, every brand/group's hype is halved
with exactly zero campaign-generated gain. The regression example prints
high-only output 1 at input .25 versus complementary-rule output .25.

These values are deterministic fixtures, not empirical claims. Alternative
schedules, initial loyalty, coefficients, heterogeneous individual behaviour,
unique reach, costs, and long-term competitive balance remain unproven.

## Validation and implementation limits

Local verification on Rust/Cargo 1.98.1 passed 22 integration tests and four
doctests, formatting, Clippy with warnings denied, and documentation with
warnings denied. Both runnable examples and the CLI JSON export succeeded.
A fresh outside-workspace copy of unchanged `bts-fuzzy` passed formatting,
tests, doctests, Clippy, documentation, and its example offline. LSP diagnostics
were intermittently stale/timed out; final compiler/Cargo checks passed.
The extended GitHub Actions workflow has not yet run remotely.

Run `cargo fmt --all -- --check`, `cargo test --workspace --all-targets`,
`cargo test --workspace --doc`, `cargo clippy --workspace --all-targets -- -D warnings`,
and `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`.
CI also runs both examples, the CLI JSON export, and unchanged extraction
checks for a copied `bts-fuzzy`. The JSON CLI test parses full output and
checks byte-identical repeated exports; simulation tests check conservation,
controlled targeting, no-exposure gain, decay/bounds/repetition, common-snapshot
updates, failed-action atomicity, gradual overlap, and reproducibility.

Numeric configuration is bounded: outside utility is positive and at most 1;
learning is [0,1]; half-lives are [1,1,000,000] hours; campaign gain is [0,5].
Required purchasing power is positive normal `f64` and at most 1. Populations
are positive `u32`; tastes and programme/brand traits are finite [0,1]. Time is
an absolute `u32` hour, strictly increasing at the two supported daily slots.
The engine validates first and commits after finite/bound checks; numerical
failure leaves the old state intact. Expected counts use ordinary floating
point and are compared within tolerances, not rounded into whole viewers.

No model here identifies individuals, models beliefs, predicts sales, stores
saves, or proves game balance. Developer JSON schema version 1 is an inspection
format, not a stable public protocol. Publication stays disabled; licensing
is unchanged.
