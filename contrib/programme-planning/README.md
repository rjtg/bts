# Programme-planning evidence

Bounded local evidence for the [planning milestone](../../docs/programme-planning.md),
not economic balance or a complete game. No internal tastes, hype, repetition,
execution or fuzzy traces appear in these artifacts.

- [Evening player report](evening-player-report.json): protocol-v2 player DTO
  emitted by `cargo run -p bts-web --example planning_evening`, after day one.
- [Workspace screenshot](workspace.png): actual Chromium rendering during the
  browser workflow test, following the [planning concept](../../docs/mockups/programme-planning.png).
  It shows a film selected/quoted while an earlier comedy remains next to air.

The evening is comedy 18:00–18:30, science 18:30–19:15, an off-air gap, and a film
20:00–21:30. Everyday Snacks is booked on the film before running the day.
The native report has airing IDs 57/58/59. The browser screenshot has 57/59/60
because its test first moves, removes and replaces the science airing; IDs are
not reused. Those edits occur before broadcasting and do not change this
control's final programme schedule or numerical outcomes.

Costs total 300 credits, advertising pays 996 of the quoted 1,046, and the
advertiser receives a 50-credit underdelivery refund. Midnight station cash is
20,696 credits. The ledger has one cost per programme and one advertising
payment at the film's completion, rather than a repeated charge on every tick.
Per-audience commitments and refunds remain distinct; intervening reception
can change actual delivery relative to the provisional forecast.

The screenshot and report are test evidence, not runtime content or save files.
Counts are provisional start snapshots, not measured ratings, duration-integrated
viewing or unique reach. One implicit ad opportunity per airing and the existing
pricing assumptions remain provisional. Publication, promotion, campaign shares,
research and production are not implemented.
