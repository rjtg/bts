# Historical advertiser-value controls (schema 3)

Captured at commit `4bb43a3`, before programme familiarity and per-audience
settlement. Current commands emit schema 4. Initial one-campaign effects remain
comparable, but next-quote reach now includes work familiarity. Reproduce exact
artifacts at that commit. See `contrib/episodes-and-commitments` for current results.

Developer-only matched controls for hype saturation, repetition and creative
quality. These reports use audience-specific deficit valuation and retain
separate delivery forecasts, valuation rows, actual delivery, campaign hype
gain and transfer ledgers. Parameters are provisional; no balance is claimed.

`results.json` contains full causal reports for all six counterfactuals;
`summary.txt` contains readable metrics and the remaining pricing caveat.
Initial stock interventions are explicit experiment controls, not a public
state-loading API or a claim of reachable campaign histories.

Regenerate from the repository root:

```sh
cargo run -p bts-cli -- advertiser-value --json contrib/advertiser-value/results.json > contrib/advertiser-value/summary.txt
```

Generated with Rust/Cargo 1.98.1. See [equations and interpretation](../../docs/advertiser-hardening.md).
Earlier schema-2 economy artifacts remain historical at commit `9f10d5f`;
current economy commands emit schema 3 with audience-specific values.
