# Episode progression and audience commitments (schema 4)

Developer-only snapshots from the current provisional model, on Rust/Cargo
1.98.1. These are inspection reports and do not define a save format.

- `episodes.json`: full causal evidence for eight controlled movie/episode runs.
- `summary.txt`: readable episode interest, familiarity, fresh viewing and frontier results.
- `contracts.json`: compact current economy snapshot including complete ledgers,
  frozen audience terms, actual audience delivery and settlement/refund rows.
- `economy-summary.txt`: current four-schedule advertiser comparison.

Regenerate full exports and summaries from the repository root:

```sh
cargo run -p bts-cli -- episodes --json contrib/episodes-and-commitments/episodes.json > contrib/episodes-and-commitments/summary.txt
cargo run -p bts-cli -- economy --json /tmp/bts-economy-v4.json > contrib/episodes-and-commitments/economy-summary.txt
```

`contracts.json` selects each economy report's case, station audience summary,
finances/ledger and contract ID, unit scale, booked hour/deadline, accepted
payment, commitments, actual delivery and status. Monetary amounts remain
integer credits; delivery commitments use 1,000,000 micro-impressions per
relevant impression. The regression in `tests/commitments.rs` additionally
books underserved viewers and substitutes an equally large satisfied audience;
it must not earn full payment.

All equations and interpretation are in
[commitments and episodes](../../docs/episodes-and-commitments.md). Controls
freeze attachment learning and continuity for the repeat/fresh comparison,
change only opening execution for the reception pair, and change only
continuity for the two out-of-order runs. New numerical parameters are
provisional; expected familiarity is not measured individual reach.
Repetition-blind advertiser pricing remains unresolved. Earlier schema-1/2/3
snapshots are historical results from their documented commits.
