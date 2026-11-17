# Headless experiment results

Developer-only output from the first bounded broadcasting experiment, using
identical initial conditions for the broad, targeted, and disappointing cases.
All parameters are provisional. These results do not establish economic value,
individual retention, or game balance.

- `results.json`: full configuration, fixed schedules, fuzzy traces, allocations,
  satisfaction, return willingness, and hype changes (report schema version 1).
- `summary.txt`: readable comparison of the same runs.

Regenerate from the repository root:

```sh
cargo run -p bts-cli -- demo --json contrib/headless-experiment/results.json > contrib/headless-experiment/summary.txt
```

Generated with Rust/Cargo 1.98.1. Expected viewer-slot impressions are not
cumulative unique reach. See the [experiment specification](../../docs/headless-experiment.md)
for equations, units, update order, validation, and limitations.
