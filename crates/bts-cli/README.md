# bts-cli

Run the advertiser-funded comparisons or the original unpaid-ad experiments:

```sh
cargo run -p bts-cli -- economy --json /tmp/bts-economy.json
cargo run -p bts-cli -- economy --case specialist-targeted --trace
cargo run -p bts-cli -- demo --case targeted --json /tmp/targeted.json
```

Economy cases are `mass-broad`, `mass-targeted`, `specialist-broad`, and
`specialist-targeted`. Original demo cases are `broad`, `targeted`, and
`disappointing`. `--case all` is the default; `--help` describes options.
Invalid arguments and output failures return nonzero exit status.

Each case starts fresh from identical configuration, cash/budgets and biological
state. Same-advertiser comparisons change only the first station's programme;
the rival, dates, costs per programme, and advertiser rules stay fixed. Quotes
are accepted independently before each broadcast. A 48-hour idle period then
shows demand recovery and an unaccepted quote with remaining budget.

Summaries show expected viewer-slot impressions, satisfaction, return
willingness, brand/audience hype, forecast versus actual target delivery,
integer revenue/cost/profit, budgets and demand. `--trace` additionally prints
inputs, fuzzy rules, offer targeting, actual delivery, settlements and ledger.
`--json PATH` writes a JSON array even for one case. Developer report schema
version 2 includes explicit forecasts, actual deliveries, contracts and account
transfers. It is not a save file or stable interchange API.

Actual internal hype and preferences are developer-only. No player research
interface is implemented. Hype is not advertiser sales; expected impressions
are not unique reach. The comparisons do not establish balance or a universal
winner. See [economy equations, results and limitations](../../docs/advertising-economy.md).
