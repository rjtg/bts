# Next playable season

Integration branch: `season/office-integration`. Starting point:
`7c37392`, the approved larger office mockups. Main remains separate.

## Checkpoint

Verified slice: tape rack, dated Reception advice and the calendar handoff
(#6 and #18). The season branch includes the implementation commit and
[Chromium capture](tape-fit.webp). Rights, Research, Ledger and Airing workspaces (#5 and #8) have a first
verified pass. Production filing (#7) is verified through the pilot formats.
The station strip, Directory, outside dismissal and desk props (#1–#4)
are verified. Action timing (#12) is documented with a tested experiment.
The public listing pilot (#13), private preparation and auditions (#14), and
editable scripted runs and sequels (#15) are verified. Next are studio logistics
(#16), advertiser evolution (#17) and atmosphere (#9–#11). Code health (#19)
continues at each boundary.

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

## Rights, Research and airing receipts

Acquire The Corner Shop in Rights and return to the office through **Pull
tape**. Open Reception to compare times. In Research, a posted Insight
receipt identifies the group, dated investment and when a fresh survey can
show the effect. The current estimate stays as published.

After transmission, the dated outlook or **No outlook filed** faces the
actual provisional count and signed settlement. **Tape bottom line** opens
the lifetime airing ledger. **Open Ledger** filters exact postings to that
showing. **All entries** opens the general book while retaining a route back
to the same airing. See the [settled airing](airing-receipt.webp) and
[filtered Ledger](airing-ledger.webp).

The production build and 15 targeted Chromium tests passed, including the
rights handoff, rights native/Wasm parity, and the Ledger return and focus
check.

## Production filing

Open the script prop, select a stage and choose the Harbour Lights folder.
Episode entries show the action for their stage and a date when one exists.
The review, casting and booking papers keep the same episode. Discussion
work orders show edition numbers, next actions and airtime. Newsroom leaves
show when to check tape delivery or open the linked airing. These additions
reuse the existing portrait, studio, and booking controls.
[Show folder capture](show-folder.webp).

The production build, all 55 Chromium tests including native/Wasm parity,
Rust workspace tests and doctests, and formatting passed. Chromium used a
temporary configuration with port 4191 because 4186 was occupied.

## Station strip

At desktop widths, simulated time and the station identity sit above one
line of mechanical controls, the Today tuner, essential resources and
Directory. On narrow displays the tuner wraps below the keys. The Directory
provides the old destinations, station files, room motion and reset. It
opens with the keyboard or by tapping and closes after navigation.
[Station strip capture](station-strip.webp).

## Office dismissal and props

Click or tap beside the tape rack, Tape file or office-framed workspace to
close it and return focus. The closing pointer event cannot activate an
underlying prop. The station strip remains usable, and confirmation
dialogs retain their own explicit controls.

The desk telephone is a newly drawn static sprite. Reviews has a printed
label and the desk calendar retains its accessible name without a hover
tooltip. [Resting office capture](new-phone.webp) and
[asset generation record](telephone-art.md).

The strip passed the full 56-test Chromium suite. Dismissal and the prop
pass passed the final set of 18 targeted Chromium tests.

Station notes now open from Today. They keep season goals, a link to an
available gap, an unsold-inventory reminder when applicable, and access to
Sway requests. The blotter no longer has a notebook prop. The full suite
passed all 58 Chromium tests at this boundary, including native/Wasm parity.
[Resting desk with the new Notes control](station-notes.webp).

## Action timing comparison

The reproducible [timing output](action-timing.txt) compares immediate,
station-wide and desk-local capacity on identical fixtures. Only desk-local
capacity both constrained the second listing and let the independent studio
booking proceed. In a quiet spell both timed variants imposed the same
wait. Commands stay immediate pending evidence from playtesting.
Two focused Rust tests passed. See the [decision](../../docs/station-time-experiment.md).

## Public listings

Select **Publish showing** in a future airing's file. The programme is now
a public listing for that start time. A move withdraws it. A rival's
unannounced schedule remains internal beyond the next 24 hours.

With otherwise identical schedules, the matched [fixture](publicity.txt)
drew approximately 12,991 opening viewers for an internal plan and 13,238
with a full-notice listing. Withdrawing the listing returned the outcome
to baseline. See the provisional [rules](../../docs/weekly-scheduling.md#public-listing-pilot).

## Private preparation and auditions

Open a delivered script and commission **Private preparation**. The study
adds one Insight after an hour of simulated time, with no transmission
required. In the casting file, select a cast and director. You can spend one
to three Insight on dated role-fit or cast-chemistry evidence, then book with
the same choices or revisit the roster. Casting without an audition remains
available. The evidence is qualitative and rehearsal does not alter the
team's capabilities. See the [research rules](../../docs/audience-research.md).
The [casting file capture](audition-evidence.webp) shows the evidence. The
native regression covers delivery timing, costs, duplicate requests,
observation privacy, and save replay.

## Editable series and sequels

Open the [series binder](series-binder.webp), select an unwritten episode and
file a new tension target. Written episodes stay locked. After completing all
three episodes, the binder offers **Open sequel brief**. The new run carries
the recurring cast and links back to its predecessor. The original tapes and
their identities remain available. See [lifecycle rules](../../docs/scripted-series-pilot.md).

The matched finale reached 0.594 script tension with gradual changes, compared
with 0.490 after abrupt shifts. The same native tests verify the cast lock,
published episode identities, invalid amendments and sequel restrictions.
The production build, all 58 Chromium tests including native/Wasm parity,
and both focused scripted-run browser tests passed.
