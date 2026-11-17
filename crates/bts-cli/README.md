# bts-cli

Run and inspect the fixed headless broadcasting experiments:

```sh
cargo run -p bts-cli -- demo
cargo run -p bts-cli -- demo --case targeted --json /tmp/targeted.json
cargo run -p bts-cli -- demo --case all --trace --json /tmp/all-cases.json
```

Cases: `broad`, `targeted`, `disappointing`; default `all`. `--help` describes
options; invalid options or output failures return nonzero exit status.
Each case creates a fresh identical initial world and fixes the rival schedule,
ads, timing and parameters. Only the first station's programme differs.

The readable summary compares viewer-slot impressions, experienced
satisfaction, group-level return willingness and brand/audience hype.
`--trace` prints inputs and rule contributions, allocations and hype changes.
`--json PATH` writes a JSON array (also for a single selected case) containing
schema version 1 developer reports with full causal traces and fixed actions.
The output is a report, not a save file or a stable interchange API.

Reports intentionally reveal actual tastes and other hidden inputs. They
must not be reused directly as player-facing observations. Hype is not
revenue, expected impressions are not unique reach, and the cases do not
establish economic balance or a universal winning schedule. See the
[provisional equations and results](../../docs/headless-experiment.md).
