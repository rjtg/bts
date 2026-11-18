# Controlled feature profile comparisons

Run `cargo run -q -p bts-web --example feature_profiles` and
`cargo run -q -p bts-web --example audience_research` to regenerate the developer
outputs in this directory.

| Host’s depth capability | Intended depth | Achieved depth | Young viewers | Specialist viewers |
| --- | ---: | ---: | ---: | ---: |
| 0.10 | 0.970 | 0.762 | 1,031 | 624 |
| 0.90 | 0.970 | 0.917 | 967 | 644 |

The first comparison changes only the host’s depth capability. Both productions
use the same production brief, cast, general craft, subject fit, budget and airing.
Neither reaches the depth target. Specialist satisfaction rises from 0.798 to
0.822, while young viewers’ satisfaction falls slightly from 0.581 to 0.575.
These are expected counts from the fixture. They illustrate a trade-off, with
no claim about broader balance.

| Evidence used at offer time | Forecast relevant delivery | Actual relevant delivery |
| --- | ---: | ---: |
| Opening poll | 6,416 | 6,637 |
| Daily poll | 6,432 | 6,637 |
| Daily poll after spending 8 Insight | 6,478 | 6,637 |

The second comparison holds the airing fixed. Research changes the information
used to quote the offer. Actual delivery stays the same across all three cases.
Total money is 80,000 credits in every run of both comparisons.
