# Ad-minute market playthrough

1. Place **Neighbourhood Laughs** on Day 1 at 18:00.
2. In **Sell ad time**, list 16 minutes. The 30-minute programme and block
   have a 46-minute runtime and reserve 60 minutes on the 15-minute grid.
3. Select **Find offers**, review and accept **Everyday Snacks**, then review
   and accept **Precision Equipment** from the refreshed batch.
4. Select **Remove unsold minutes**. Only the bought minutes remain in the
   block and the reservation now occupies 45 minutes.
5. Run the selected airing. Inspect the audience counts, airing cost and
   separate payment line for each brand. The captured airing paid 615 credits
   against a broadcast cost of 150.

See [offers.png](offers.png) and [settlement.png](settlement.png). To reproduce
the captures, build the browser app and run
`BTS_MARKET_SCREENSHOTS=1 npm run test:e2e -- tests/ad-market.spec.ts` from
`web/`. Regenerate developer JSON with
`cargo run -q -p bts-web --example ad_minute_market`.

## Controlled results

| Case | Forecast viewers | Actual expected viewers | Snacks sold | Equipment sold |
| --- | ---: | ---: | ---: | ---: |
| Broad, 15 minutes listed | 12,104 | 12,104 | 3 min / 751 cr | 3 min / 264 cr |
| Specialist, 15 minutes listed | 6,706 | 6,706 | 3 min / 359 cr | 3 min / 364 cr |
| Broad, 5 minutes listed | 12,104 | 12,104 | 3 min / 1,127 cr | 2 min / 264 cr |
| Broad, prior repeats placed after sale | 12,104 | 11,063 | 3 min / 751 cr | 3 min / 264 cr |
| Broad, very low availability at quote and airing | 289 | 289 | 3 min / 17 cr | 3 min / 8 cr |

The specialist programme draws fewer viewers but gets a higher offer from
Precision Equipment. Cutting the listed minutes raises the unit price in the
fixture. The smaller block makes the order of acceptance matter. In the repeat
case the forecast and frozen prices match the broad baseline, while familiarity
reduces actual viewing. The low-availability case also pays its full agreed
price. Exact money is conserved in all cases. These experiments isolate parts
of the model and do not establish balance or predict actual ratings.
