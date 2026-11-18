# Rights market playthrough

1. Open **Rights market**. Commission a six-Insight assessment of
   **The Corner Shop** and advance to its delivery. Review the revised forecast.
2. Buy the rights and open the film in the programme library. Schedule it for
   Day 1 at 18:00. List the title for resale. The resale card links to the airing
   that currently blocks acceptance.
3. Open that airing. List ad minutes, accept one of the offers, then run it.
   Review ad settlement. Return to the Rights market and sell the rights.
   The film leaves your library but remains in your broadcast history.
4. Advance to Day 4. **Night Chase** has been released and **Beyond the
   Observatory** has gone to Channel Two after its original deadline. The
   industry wire records both events. Open a pending rival airing from its card
   before it runs, or view its completed slot on the rival timeline later.

See [market.png](market.png) and [industry-wire.png](industry-wire.png). To
recreate the screenshots, build the app and run
`BTS_RIGHTS_SCREENSHOTS=1 npm run test:e2e -- tests/rights.spec.ts` from `web/`.

## Controlled results

| Title and evidence | Forecast viewers | Specialist viewers | Price with solvent rival | Price with cash-poor rival |
| --- | ---: | ---: | ---: | ---: |
| The Corner Shop, public notes | 8,487 | 422 | 2,456 | 2,047 |
| Beyond the Observatory, public notes | 4,153 | 629 | 1,260 | 1,050 |
| The Corner Shop, after 6 Insight | 7,765 | 381 | 2,456 | 2,047 |

The specialist title has a smaller overall forecast and a larger forecast
among specialists. For the same broad title, the solvent rival raises the
asking price by 409 credits and buys at expiration. A rival starting with
150 credits causes neither a premium nor an acquisition. The assessment lowers
the broad estimate by about 722 viewers without changing the asking price.
The experiments conserve money and show distinct decisions in this fixture.
They make no balance claim.

Regenerate `comparisons.json` with
`cargo run -q -p bts-web --example rights_market`.
