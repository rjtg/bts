# Audience research experiment

The browser Audience panel shows each known group's latest size, typical age and
programme feature preference estimates. The panel shows when the estimate was made and
describes familiarity and uncertainty at that poll. Station familiarity is
separate from the audience's familiarity with a programme.

The browser opens with one free baseline snapshot per group, familiarity 0.25,
all paid polling switched off, and 12 Insight. The age input is one authored
typical age per group in this fixture. Full age distributions and changing
audience traits remain future work.

| Polling tier | Schedule | Cash per survey (all groups) |
| --- | --- | ---: |
| Off | None | 0 |
| Daily | Every 96 quarter-hour ticks | 60 |
| Every three days | Every 288 ticks | 90 |

Changing tiers schedules the next strictly future aligned boundary. Each tick
first processes actual broadcasts, payments and production. Familiarity then
decays by a factor of 0.9995 regardless of polling. One station-wide survey
refreshes every group together and transfers a single flat fee to the research
cost account. If the station cannot pay, all snapshots stay as they were until
the next successful survey. Long advances visit each boundary in order.

Spending a positive integer amount `n` sets familiarity to
`F + (1 - F) × (1 - 0.85^n)`. This immediately debits Insight and records the
group, amount, time and balance. The published snapshot updates at the next
successful poll. No paid poll improves familiarity by itself. This fixture has
a finite opening Insight balance.

A poll's uncertainty scale is `u = 0.04 + 0.26 × (1 - F)`. For each estimated
attribute, a deterministic hash of the seed, group ID and attribute index gives a
bias in [-1, 1]. On each simulated day the bias moves two percent toward a new
deterministically hashed value for that day. Poll frequency never changes this
sequence. Thus the normalized bias changes by at most 0.04 per day, and spending
Insight reduces its size in the estimate without rerolling it. The scenario's
initial seed is 719.

Population uses the relative error `u × bias`, rounded to a positive u32.
Typical age uses an additive error of `60 × u × bias` years, rounded and
bounded to 0–120. Unit attributes use additive error clipped to [0, 1],
except hourly availability which uses relative error to preserve closed hours.
Preferences are independent desired feature levels and do not sum to one. The public report rounds each published preference to the nearest half star on a five-star scale. These are
provisional error scales, not calibrated confidence intervals. Only the dated
estimates and qualitative familiarity enter the public report.

Browser offer-time previews substitute the latest snapshot inputs for audience
population, feature preferences, hourly availability, purchasing power
and product interest. Actual allocation and delivery use the world in Rust.
Estimated delivery freezes into the existing contract on acceptance, and the
existing per-audience settlement compares actual eligible delivery with those
terms. Budget, actual brand demand and the existing airing-history context remain
inputs to the offer model. Snapshot age and uncertainty are available through
the research view. There is no direct risk discount for stale estimates.

The headless experiments and planners without research configured retain their
existing forecasts. See the [paired forecast experiment](../contrib/audience-research/README.md).
