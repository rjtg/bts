# Next playable season

Integration branch: `season/office-integration`. Starting point:
`7c37392`, the approved larger office mockups. Main remains separate.

## Checkpoint

The connected office journeys, simulation pilots, working-day art, and
six-week browser playthrough are ready for review on the integration branch.
The issue index records the implementation and comparison commits. Each
issue remains open for follow-up.

## Launch

From the repository root:

```sh
git fetch origin
git switch season/office-integration
npm --prefix web ci
npm --prefix web run build
npm --prefix web run preview -- --host 0.0.0.0
```

Preview defaults to port 4173. On the development network, the review build
is served at `http://100.71.132.116:4173/` while the preview process runs.
To review a particular commit, use `git switch --detach <commit>` in a clean
checkout and rebuild. See the [browser setup](../../docs/browser-foundation.md)
for the Rust Wasm target, wasm-bindgen and Chromium prerequisites.

The review machine used these cached tools from `web/`:

```sh
RUSTUP_HOME=/tmp/bts-rust-1100 RUSTUP_TOOLCHAIN=1.100.0 WASM_BINDGEN=/tmp/bts-web-tools/bin/wasm-bindgen npm run build
RUSTUP_HOME=/tmp/bts-rust-1100 RUSTUP_TOOLCHAIN=1.100.0 PLAYWRIGHT_BROWSERS_PATH=/tmp/bts-playwright BTS_E2E_PORT=4231 npm run test:e2e -- --workers=2
```

## Issue index

| Issue | Outcome | Commit |
| --- | --- | --- |
| [#18](https://github.com/rjtg/bts/issues/18) | Approved larger spreads, linked journeys and browser review | [7c37392](https://github.com/rjtg/bts/commit/7c37392), [8339c7e](https://github.com/rjtg/bts/commit/8339c7e) |
| [#5](https://github.com/rjtg/bts/issues/5) | Rights to Pull tape, dated research, and related Ledger entries | [4800d5c](https://github.com/rjtg/bts/commit/4800d5c) |
| [#6](https://github.com/rjtg/bts/issues/6) | Grouped rack, Reception advice and editable exact-time comparison | [7eb7850](https://github.com/rjtg/bts/commit/7eb7850) |
| [#7](https://github.com/rjtg/bts/issues/7) | Stage collections and show folders with edition decisions and dates | [8339c7e](https://github.com/rjtg/bts/commit/8339c7e) |
| [#8](https://github.com/rjtg/bts/issues/8) | Dated outlook beside turnout, receipts and costs | [4800d5c](https://github.com/rjtg/bts/commit/4800d5c) |
| [#1](https://github.com/rjtg/bts/issues/1) | One transport, Today and resource strip with Directory access | [560365a](https://github.com/rjtg/bts/commit/560365a) |
| [#2](https://github.com/rjtg/bts/issues/2) | Outside dismissal and focus return protect clicks underneath | [2d2566c](https://github.com/rjtg/bts/commit/2d2566c) |
| [#3](https://github.com/rjtg/bts/issues/3) | Registered phone sprite, printed Reviews label and calendar cleanup | [e599993](https://github.com/rjtg/bts/commit/e599993) |
| [#4](https://github.com/rjtg/bts/issues/4) | Station notes and request routes replace the notebook prop | [81307d3](https://github.com/rjtg/bts/commit/81307d3) |
| [#12](https://github.com/rjtg/bts/issues/12) | Timing comparison with an immediate-execution decision | [40d79c2](https://github.com/rjtg/bts/commit/40d79c2) |
| [#13](https://github.com/rjtg/bts/issues/13) | Exact public listings and automatic withdrawal on move | [452b83d](https://github.com/rjtg/bts/commit/452b83d) |
| [#14](https://github.com/rjtg/bts/issues/14) | Private script study and optional dated auditions | [81a3bbc](https://github.com/rjtg/bts/commit/81a3bbc) |
| [#15](https://github.com/rjtg/bts/issues/15) | Unwritten episode amendments and linked sequels | [0119c1d](https://github.com/rjtg/bts/commit/0119c1d) |
| [#16](https://github.com/rjtg/bts/issues/16) | Elastic film hire with separate setup and prop counterfactuals | [15e9433](https://github.com/rjtg/bts/commit/15e9433), [3292a10](https://github.com/rjtg/bts/commit/3292a10) |
| [#17](https://github.com/rjtg/bts/issues/17) | Three-review incentive comparison and policy decision | [6b4b827](https://github.com/rjtg/bts/commit/6b4b827) |
| [#9](https://github.com/rjtg/bts/issues/9) | Independent layered poses for all twelve visible characters | [3a27bdd](https://github.com/rjtg/bts/commit/3a27bdd) |
| [#10](https://github.com/rjtg/bts/issues/10) | Fitted CRTs with a rotating reel pool and matched stills | [c3997e3](https://github.com/rjtg/bts/commit/c3997e3) |
| [#11](https://github.com/rjtg/bts/issues/11) | Staggered shifts, the coffee cue and optional exterior | [c852a14](https://github.com/rjtg/bts/commit/c852a14) |
| [#19](https://github.com/rjtg/bts/issues/19) | Shared paper selection, dismissal and ambient controls, plus compatibility checks | [cbf8fab](https://github.com/rjtg/bts/commit/cbf8fab), [2d2566c](https://github.com/rjtg/bts/commit/2d2566c), [c3997e3](https://github.com/rjtg/bts/commit/c3997e3), [1327eb9](https://github.com/rjtg/bts/commit/1327eb9) |

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

## Film rental desk

On the Studio page choose **Hire · Edit Bay** or **Hire · Grand Floor**.
Set the recording time to see an exact hire rate, the casting fees and the
signed total. The [browser capture](rental-quote.webp) shows an inexpensive
room with a staging warning. In the native fixture the same room costs
240 Funds with an empty calendar and 300 with one half-overlapping recording.
The higher-spec package costs 710 in the same quiet slot. The fixture verifies
payment, staffing conflicts and conservation. The production build and nine
targeted Chromium tests passed, including production native/Wasm parity.

[Separate setup and stock results](studio-logistics.txt) ·
[Booking rules and reproduction commands](../../docs/production-logistics.md#film-rental-pilot)

## Advertiser review over three weeks

The [matched comparison](brand-comparison.txt) follows the same show and
airtimes under regular, weak-creative, weak-turnout, and unsold conditions.
One brand's low-hype histories pay higher later quotes and, for both types
of weak delivery, more cumulative receipts in this diary. Forgoing sales
improves the next quote but earns no receipts until then. The existing
weekly caution reduces the quote after weak turnout, while deficit pricing
still dominates. Financing and signed payments are unchanged. The policy
decision and exact branch conditions are in [weekly reviews](../../docs/brand-reviews.md#three-review-comparison).

## Independent office poses

Open the [room](independent-poses.webp) at desktop width with Room motion set
to Auto or On. Twelve masked office and studio characters now run independent
timers. Minor postures change more often than gesture positions or the painted
alternate pose. The set uses the original pose illustrations and small
registered sprite movements. Its animation clocks use ordinary browser time.
The build and four focused Chromium tests passed. The seeded test visits all
twelve subjects, detects simultaneous changes, checks the relative frequencies
and verifies the station time and reduced-motion controls.

## Wall monitors

The [three fitted CRTs](rotating-crts.webp) rotate through a shared pool of
nineteen programmes and one test card. Three of the reels use the previous
painted frames. Sixteen new programme plates were generated as a contact
sheet and use small stepped motions inside the glass. The animation test
visits every reel on each screen, checks two or more animated screens in
each batch, and verifies matched stills and an unchanged clock under reduced
motion. The production build and all five office-focused tests passed.

## Working day

Use the time controls to see the [empty early office](office-before-open.webp),
[arrivals around 08:45](office-arrivals.webp), and the [afternoon desk](office-afternoon.webp)
with an empty mug. **Exterior at close** in Directory enables a
[short exterior shot](exterior-at-close.webp) at 18:00. The atmosphere uses two
image-generated plates registered to the existing art for the vacant room
and the station exterior. Arrivals and departures use group changes. The
exterior and changes in light are decorative. Exact airings still settle
through the regular clock controls, as checked in Chromium.

## Six-week Chromium replay

1. Open the Rights portfolio and inspect **The Corner Shop**. Spend three
   Insight on the assessment, advance to delivery, buy the rights, then
   choose **Pull tape**. [Acquisition](season-rights.webp)
2. In Tape Reception choose **Find airtime** and book Saturday, Day 1,
   at 18:00 with ten listed ad minutes. In that exact Airing file accept the
   Everyday Snacks offer. The move control is locked after acceptance.
3. Run the showing. The dated outlook was about 9,900 viewers. The simulated
   airing count was about 7,438 across the three groups. The signed deal still
   paid 528 Funds for three minutes. The airing margin was 428 Funds before
   the cost of rights. [Airing review](season-first-receipt.webp)
4. Use the Research binder to request a daily survey and spend eight Insight
   on Young viewers. Finish the day to file the dated Day 2 survey at high
   familiarity. Return to Survey Orders and turn polling off to stop the
   recurring cost. [Survey](season-survey.webp)
5. Return to Office, close the restored Airing file, and pull **Neighbourhood
   Laughs, Episode 1** from the grouped tape rack. Use **Schedule this tape →
   Regular → Choose days and time**. Confirm the next three Saturdays at
   18:00 with ten ad minutes each. Set autosell to at most eight minutes and
   a 35 Funds floor. The test sets the floor to one quarter of the first
   paid per-minute rate, capped at 35, so the received price informs this
   later decision. [Autosell terms](season-policy.webp)
6. Run the first occurrence and inspect its automatic receipt. Finish each
   remaining day. Station notes show all three goals met and 20,565 Funds in
   the register. [Season close](season-close.webp) · [Recorded figures](season-browser.json)

The [replay test](../../web/tests/next-season.spec.ts) uses visible controls
and the normal Rust/Wasm browser entry. To run the replay alone, append
`-- next-season.spec.ts` to `npm run test:e2e` from `web/`.

## Verification and decisions

- Workspace formatting, tests and doctests, Clippy with warnings denied, and
  workspace documentation passed. The three Rust libraries built for
  `wasm32-unknown-unknown` and browser-crate Wasm Clippy passed.
- A separate copy of `bts-fuzzy` built for Wasm. Its ten tests, three
  doctests, Clippy, strict docs and ventilation example passed there.
- The production TypeScript/Vite build and all 61 Chromium tests passed,
  including native/Wasm parity, narrow touch navigation, reduced motion and
  the full season. The prior published station save replays to the same
  balance, goals and signed receipts.
- Timed work stays an experiment. Teaser anticipation, live sets and prop
  inventory, topic-dependent shelf life, measured ratings, and a change to
  the advertiser bid policy remain open. The timing, production and ad
  fixtures are bounded comparisons with provisional settings.
