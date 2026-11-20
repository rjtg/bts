# Next playable season

Integration branch: `season/office-integration`. Starting point:
`7c37392`, the approved larger office mockups. Main remains separate.

## Checkpoint

Verified slice: tape rack, dated Reception advice and the calendar handoff
(#6 and #18). The season branch includes the implementation commit and
[Chromium capture](tape-fit.webp). Active next: Rights, Research, Ledger
and Airing workspaces (#5 and #8), followed by production filing (#7).
Office controls (#1–#4), simulation pilots (#12–#17) and atmosphere
(#9–#11) are pending. Code health (#19) continues at each boundary.

## Launch

From the root, run `npm --prefix web ci` on a fresh checkout, then
`npm --prefix web run build` and
`npm --prefix web run preview -- --host 0.0.0.0`.
Preview defaults to port 4173. See the [browser setup](../../docs/browser-foundation.md)
for the Rust Wasm target, wasm-bindgen and Chromium prerequisites.
The local toolchain can use `WASM_BINDGEN=/tmp/bts-web-tools/bin/wasm-bindgen`
and `PLAYWRIGHT_BROWSERS_PATH=/tmp/bts-playwright` when those caches exist.

## Baseline

The [approved review package](../../docs/mockups/player-journeys/README.md)
gives the starting interactions and captures. The provisional interaction
defaults are timing advice in Reception, a deliberate Pull tape action after
rights acquisition, stage-filtered editions, and an airing-filtered Ledger
with access to All entries.

## Tape to airtime

From the office shelf, pull Neighbourhood Laughs, Episode 1. Select Reception
and Find airtime. Select 18:00, verify the exact occupied time, and confirm.
After closing the airing, inspect the 18:00 clash and cancel. The first airing
remains booked. The narrow exact-gap path also passed.

The qualitative fit bands compare six published watch-time bands for that
tape. Good is at least 90% of its strongest band, OK is at least 75%, and
Weak is below 75%. Without useful watch-time evidence the fit is Unknown.
Feasibility checks duration plus proposed ad minutes against the opening.
This browser guidance uses existing published survey evidence. Rust still
validates the placement and supplies actual viewing and contractual outcomes.

The Wasm and TypeScript production build and 13 targeted Chromium tests
passed. The TypeScript language server was unavailable locally.
