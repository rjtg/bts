# Headless broadcasting model

Status: implemented experimental substrate, with provisional content, equations
and coefficients. The [advertising economy](advertising-economy.md) now connects
this model to finite-budget demand, delivery contracts and station finances.
Neither milestone completes the first-playable game or validates balance.

## Boundary and data

`bts-sim` owns typed world data, explicit integer simulated hours, TV fuzzy
rules, state transitions and immutable causal events. `bts-cli` runs fixed
comparisons and renders readable/JSON developer reports. `bts-fuzzy` remains
generic and independently extractable. Actual state is private and not
serializable. Report conversion is separate from mutation; reports deliberately
expose actual tastes/hype and are not a player research interface. Public beliefs
and restricted player observations are unimplemented.

There are no exact engine entity counts. All collections may be empty, subject
to typed references: an empty catalogue allows only idle stations; no brands
means no ads; no audiences means no delivery or paid offers. IDs and per-brand
target audience IDs must be unique and references must exist. Populations are
positive u32. Any strictly increasing u32 simulated hour after origin 0 is valid;
24 hourly availability fractions select by hour modulo 24.

Small fixed fixtures remain in `demo`: young viewers (6,000), households
(11,000), specialists (2,500), two stations, four programmes and two brands.
Groups have genre affinities, tone preference, availability, purchasing power
and product interests. Programmes have genre, tone, awareness, cast appeal and
execution. The experiment broadcasts at 18:00 and 21:00 for six days and then
advances 48 idle hours. Each action is an hour-long slot snapshot. There is no
within-slot switching or individual viewer identity.

`demo` changes only the first station's programme across broad, targeted and
disappointing cases. The rival, unpaid ad placements (equipment at 18:00,
snacks at 21:00), initial state and timing stay fixed. `economy_demo` instead
quotes/accepts one selected advertiser at each broadcast and settles contracts.
These are separate controlled experiments, with the same broadcasting model.

## Fuzzy semantics and interactions

All features are finite [0,1]. High(x) is a rising 0..1 ramp; Low(x)=1-High(x).
AND=min, OR=max, NOT=complement. A complementary low/high pair emitting 0/1
produces a gradual response. Sugeno computes a weighted average, not activation
times output. Weight sums are normalization terms, not probabilities/confidence.
`cargo run -p bts-sim --example sugeno_semantics` demonstrates high-only output
1 at x=.25 versus complementary output .25. No new feature catalogue was added
for the reviewed interactions.

Let g=genre affinity, t=1-|preferred tone-programme tone|, a=awareness,
c=cast appeal, e=execution. Previous linear scores were
A0=.35g+.10t+.30a+.25c and S0=.15g+.15t+.70e.
Current attraction A=(A0+.25*min(g,c))/1.25, with a complementary interaction
pair for cast appeal AND genre fit. Current satisfaction
S=(S0+.30*min(g,e))/1.30, with a complementary pair for execution AND fit.
Fit makes cast/craft useful rather than independently interchangeable points.
Tests distinguish equal old linear scores by at least .09/.10 in these models.
Execution still cannot affect attraction; awareness cannot improve satisfaction.

Ad effectiveness = .75*min(product interest, affordability, creative quality)
+ .25*min(product interest, affordability, creative quality, satisfaction).
Each term is a nested AND complementary pair with the stated weight.
Affordability=min(1,purchasing power/brand requirement). Programme satisfaction
supports an effective ad but cannot replace creative quality. Zero creative,
interest or affordability gives zero effectiveness. Satisfaction=0 retains the
base creative response; independent-factor tests cover all four features.

## Update order, units and equations

1. Validate the whole plan (increasing hour, unique known stations, known
   programmes/brands, booked-ad consistency). Work on tentative state; malformed
   plans commit nothing. Resolve cash availability independently per station:
   an unaffordable programme goes off air. All following viewing and campaign
   calculations use only the broadcasts actually aired. Reports retain the
   original plan, aired slot and each funding failure.
2. Decay brand/audience hype H in [0,100] points and repetition R in expected
   impressions/person from previous time to this action:
   Hdecay=H*2^(-elapsed_hours/48), Rdecay=R*2^(-elapsed_hours/24).
   Initial H=5, R=0. Half-lives are configurable [1,1,000,000] hours.
3. Use the common pre-slot snapshot for every actual airing. Base fuzzy
   attraction A is combined with series attachment (episodes only), observed
   predecessor reception, continuity readiness and current-work familiarity.
   See [exact provisional factors](episodes-and-commitments.md). Standalone
   works have no series attachment. Utility is nonnegative and finite.
   Available viewers = population*hourly availability. Station viewers =
   available*utility/(outside utility+sum station utilities); outside utility
   is .18 by default. Not watching=population-sum station viewers.
4. Compute satisfaction independently. Expected fresh viewers for each work
   are allocated viewers*(1-pre-slot work familiarity). Aggregate simultaneous
   airings of the same work before familiarity/reception updates. Update
   series attachment only from fresh episode viewing; standalone works never
   gain attachment. Record sequence frontier as a diagnostic, and append only
   actual aired broadcasts to history. Repeating an earlier episode does not
   advance to a new episode. Familiarity assumes uniform independent choices
   within a group, not identified viewers or measured unique reach.
5. An ad reaches exactly its broadcast's allocated viewers. Aggregate by
   brand/audience before updating any hype:
   E=sum(ad viewers/population), Q=sum(ad viewers/population*effectiveness).
   Dose=.8*Q/(1+Rdecay); campaign gain=(100-Hdecay)*(1-exp(-Dose)).
   Hnew=Hdecay+gain, Rnew=Rdecay+E. Saturation uses remaining headroom;
   repetition reduces marginal response. Zero viewers means exactly zero
   campaign gain, even if fuzzy effectiveness is positive. R is finite and
   bounded by the maximum elapsed u32 hour count because each action is at
   least one hour apart and exposure/person per action is at most one.
6. Charge broadcast costs from pre-settlement station cash, record actual
   booked delivery, settle due contracts, then commit time and all state.
   Quote/accept actions reserve budget/inventory before broadcasts. See the
   [economy specification](advertising-economy.md) for exact integer rules.

Other parameter bounds: outside utility positive and <=1; learning [0,1];
campaign gain [0,5]. Brand purchasing-power requirement is positive normal f64
and <=1. Inputs, outputs and state are finite and checked. Counts use ordinary
IEEE-754 rounding, with accounting tests using numerical tolerances. Simulation
uses no wall clock, filesystem, randomness, rendering or inference service.

## Run and inspect

```sh
cargo run -p bts-cli -- demo --json /tmp/bts-demo.json
cargo run -p bts-cli -- demo --case targeted --trace
cargo run -p bts-cli -- economy --json /tmp/bts-economy.json
```

JSON schema version 4 includes configuration, initial assumptions, actions,
full fuzzy inputs/condition/rule traces, allocations, satisfaction/willingness,
hype decomposition, planned/actual airing, off-air reasons and financial events. Economy reports add audience valuation rows, frozen audience commitments, offers, forecast
versus actual delivery, deadlines, settlements, budgets and the complete ledger.
This is a developer inspection format, not a stable public protocol or save.

Current unpaid-ad demo results (rounded):

| Case | First station viewer-slot impressions | Mean satisfaction | Specialist equipment hype in specialist group |
| --- | ---: | ---: | ---: |
| Broad | 82,915 | 0.787 | 17.68 |
| Targeted | 26,413 | 0.690 | 31.18 |
| Disappointing | 75,603 | 0.232 | 15.92 |

Hype is before the cooldown. These legacy schedules repeat the same episode
ID, so familiarity now reduces fresh viewing; they are not successive new
episodes. The [new episode controls](episodes-and-commitments.md) isolate
opening quality, repeats and continuity with distinct work IDs. Advertising
valuation, relevant delivery and audience-specific settlement remain separate.

The original schema-1 baseline in `contrib/headless-experiment` belongs to
commit b74d453, before these review corrections. It is historical evidence,
not the output of current rules. Its linear-model results (91,002 broad
impressions, 30,959 targeted) should not be compared as if generated by the
same model version.

See the [hardening experiments](advertiser-hardening.md) for two-audience
valuation controls, player/rival funding failures, and the saturation/repetition
counterfactuals. Paid value and settlement guarantees are distinct quantities.

## Verification and limits

Run formatting, workspace all-target tests/doctests, Clippy with warnings
denied and Rustdoc with warnings denied. CI also runs both generic examples,
all four CLI modes, and all checks on a copied standalone `bts-fuzzy`.
Tests cover audience conservation, distinct tastes, weak satisfaction despite
promotion, targeting at controlled exposure, zero-exposure gain, decay/bounds,
repetition, overlap, snapshot ordering, money/budget/ledger conservation,
deadlines, inventory, refunds, atomic failures and reproducibility. LSP feedback
can be stale; compiler/Cargo checks are the fallback.

Parameters, tastes and rates are invented experimental inputs. No empirical
validity, unique reach, individual retention, optimal strategy, stable long-run
balance or full-season viability is established. No production, general market,
politics, demographics, AI, saves or TUI is implemented. Publication remains
disabled and licensing unchanged.
