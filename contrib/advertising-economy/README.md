# Advertiser-funded experiment snapshot

Current schema-2 comparisons on Rust/Cargo 1.98.1. All costs, rates and models
are provisional. Reports expose actual developer-only information.

- `summary.txt`: readable output of all four economy cases.
- `results.json`: compact machine-readable comparison, including demand,
  available budgets, station finances, complete transfer ledgers, accepted
  forecast/actual relevant delivery and fixed-forecast decay comparisons.

Generate the full causal export and readable summary from the repository root:

```sh
cargo run -p bts-cli -- economy --json /tmp/bts-economy.json > contrib/advertising-economy/summary.txt
```

The full export includes per-slot fuzzy traces, actual allocations, offers,
contracts and settlement detail. The compact snapshot selects each report's
`broadcasts.case`, `brand`, `broadcasts.summary.stations`, `finances`, first
offer demand, demand after campaigns, recovery demand and before/after fixed
future-slot quotes. Accepted forecast and actual relevant delivery are sums
across `contracts`; all money remains integer credits. It is a report, not a
save file. See [equations and interpretation](../../docs/advertising-economy.md).

Within each advertiser pair, starting configuration and state are identical
and only the first station's programme changes. The fixed-forecast recovery
comparison keeps the future proposal, delivery forecast and available budget
identical while 48 idle hours allow hype to decay. Cost-only sensitivity is
covered by simulation tests. No universal winner, realistic economics,
unique reach or complete season viability is established.
