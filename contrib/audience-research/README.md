# Audience research comparison

Run `cargo run -p bts-web --example audience_research` to regenerate
[comparison.json](comparison.json). Three runs schedule the same comedy and
advertiser on day two. All quote at tick 96. Only the young-viewer group's
research differs.

| Case | Snapshot tick | Forecast relevant impressions | Offer ceiling |
| --- | ---: | ---: | ---: |
| Opening estimate retained | 0 | 4,901 | 1,429 credits |
| Daily poll | 96 | 4,856 | 1,416 credits |
| Daily poll with 8 Insight | 96 | 6,478 | 1,889 credits |

All three runs deliver 6,728.49 relevant impressions before contract crediting.
The fresh poll alone is slightly farther from the actual result in this fixture.
Insight narrows the poll's imprecision and brings this forecast closer. The
fixture has static audience traits, so it demonstrates persistent bias and the
effect of changing evidence rather than modelling demographic change. All
branches conserve 80,000 credits.
