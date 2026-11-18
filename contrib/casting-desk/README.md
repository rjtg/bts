# Casting desk: controlled results and browser evidence

Recorded 18 November 2026. This is a bounded fictional fixture, not evidence of
creative or economic balance. Numerical parameters and expected audience reporting
remain provisional. See [mechanics and equations](../../docs/casting-desk.md),
[timed quests](../../docs/quest-opportunities.md) and
[implemented capacity versus next-slice logistics](../../docs/production-logistics.md).

## Results

[Developer comparisons](developer-comparisons.json) contain nine runs from identical
initial conditions: existing skills with cooperation friction, deliberately weak
skills with the same friction, and a team without matching deficiencies. Within
each family only support changes: 0, 1 or 4 whole Sway. Exact causal calculations
are developer-only and never included in browser player observations.

| Team | Execution at 0 / 1 / 4 Sway | Interpretation |
| --- | --- | --- |
| Working friction | 0.724454 / 0.764454 / 0.804454 | One point recovers 0.04; three further points together recover another 0.04 |
| Weak skills, same friction | 0.196829 / 0.236829 / 0.276829 | Support does not repair craft or subject fit |
| No cooperation deficiency | 0.681697 / 0.681697 / 0.681697 | Spending grants no quality bonus |

Within each family initial viewing counts are unchanged. For the working-friction
team audience 1 satisfaction increases from 0.776503 through 0.807272 to 0.838041;
with weak skills it remains 0.370638 / 0.401407 / 0.432176. Other audiences respond
differently; the JSON preserves all groups. Every case ends with station cash
17,950 and conserved total money 80,000. Skills and the baseline remain unchanged
within each family. All JSON exports reproduced byte-for-byte on a second run.

[Player flow](player-flow.json) records normal protocol-v5 observations from the
native adapter: accept Nora’s quest, record her with Felix, earn 4 Sway at tick 44,
then spend it across people on Jules’s exceptional participation (2) and Mina’s
production cooperation (2). Complete, advertise and broadcast both originals;
accept Rafi’s independent quest and complete a third production to earn 3 more
Sway at tick 292. Final cash is 19,478 and Sway is 3. The same ordered commands run
through actual Wasm in Chromium and are compared against native responses.

## Actual browser captures

These are screenshots from the automated real Chromium flow, not mockups:

- [Portrait picker](talent-picker.png): public specialties, fees, availability and commitments.
- [Booking review](booking-review.png): known friction, adjustable support, separate participation cost, exact cash/Sway and studio obligation.
- [Completed productions](completed-productions.png): library handoff and project review.
- [Live planning HUD](live-hud.png): resource icons, Play/Pause, ×2 and dashed NOW marker.

## Verification

Rust 1.100.0; wasm-bindgen-cli 0.2.130; Playwright 1.61.1 with Chromium 149.

- Workspace formatting, 109 native tests and 5 doctests pass.
- Native all-target and Wasm Clippy pass with warnings denied; Rust documentation passes with warnings denied.
- All three library/adapter crates build for wasm32-unknown-unknown; TypeScript checking and the static Vite bundle pass.
- 13 real browser tests cover casting, keyboard/touch-sized controls, booking/review, Sway bounds, conflicts, quests, completion, advertising, broadcast review, native/Wasm parity, reset, playback and HUD.
- Existing CLI demo, economy, advertiser-value and episode experiments, planning-evening example and Sugeno/ventilation examples pass.
- bts-fuzzy copied outside the workspace passes formatting, native tests/doctests, Clippy, docs, Wasm build and ventilation example. Its code and manifest are unchanged.
- LSP diagnostics were attempted. TypeScript’s server is unavailable; final Rust feedback timed out/stayed stale. Cargo, tsc and browser checks supply verification instead.

CI preserves these checks and now runs the controlled casting export. Browser CI
prebuilds the native reference examples before parity tests.

## Limits and next work

One standalone discussion format, authored finite quests and fixed public working
histories; no general relationship model, talent contracts or rival production AI.
Each browser station has distinct rights and assigned hosts; freelancers can work
for the player subject to commitments. One owned studio is reserved independently
of people. Setup/cleanup savings, rentable facilities, recurring batches and reusable
prop stock are explicitly documented for the next slice, not implemented here.

Playback is browser pacing of explicit deterministic commands, starts paused and
pauses for editing/background tabs. Glamour, Credibility and Backing show unavailable
markers rather than invented balances. Daylight scenery, playful event notifications,
resource-responsive offices, audience-gated/graded quests and broader logistics
remain proposals. No save migration, deployment or publication is introduced.
