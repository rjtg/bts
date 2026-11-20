# Season roadmap review

## Starting point

The goal started at `f179eb3` with an empty index and working-tree changes
for exact-minute airtime, the Station Office and its design/capture evidence.
These are prerequisites for the new work. Their baseline is tracked separately
from the nine milestone commits. Baseline: `e178404` (workspace Rust tests/doctests, production Wasm/browser build, 40 Chromium tests).

## Review

From the repository root, enter `web/` before running these commands.
These paths describe the local tool installation used for the captures.

```sh
RUSTUP_HOME=/tmp/bts-rust-1100 RUSTUP_TOOLCHAIN=1.100.0 WASM_BINDGEN=/tmp/bts-web-tools/bin/wasm-bindgen npm run build
RUSTUP_HOME=/tmp/bts-rust-1100 RUSTUP_TOOLCHAIN=1.100.0 PLAYWRIGHT_BROWSERS_PATH=/tmp/bts-playwright npm run test:e2e
npm run preview -- --host 0.0.0.0
```

Open port 4173 on the preview host. The existing network address is
`http://100.71.132.116:4173/`. For historical review, use
`git worktree add --detach <directory> <commit>` and build there.

## Progress checkpoint

- Active slice: persistence and season wrap-up.
- Verified milestones: `7313d17`, `c6b68e4`, `bf42a8f`, `ed2f906`, `656746d`, `2dffc4e` `41a2095`, and Produce a short scripted series with recurring roles.
- Next: implement versioned save journals and verify a full season.
- Runtime token refresh schedule: unavailable.

## 1. Recurring discussion editions

Find this slice with `git log --oneline --grep='Add playable weekly discussion runs'`.
The [implemented rules](../../docs/recurring-discussion.md) describe the timing,
booking commitments and missed editions.

1. Open Production and expand **Make this a weekly show**. Name the show
   Studio Roundtable and choose 18:01. Open the three-edition binder.
2. Book edition one. Finish it, open its Airing file, run it and review its
   completed result.
3. Return to Production. Edit edition two and replace Nora Vale with Rafi
   Linden. Choose Entertainment & culture and Lively. Edition one's
   displayed brief remains unchanged.
4. Book edition two. Replace Nora with Rafi on edition three as well, since
   Nora's ordinary availability covers only the start of the scenario.
   Book, finish and air both episodes.

The native fixture checks three distinct episodes and airings, protection of
the reserved interval, no charge for missed editions, 3,000 Funds in total
production costs, money conservation and exact replay of the later editions.
The browser walkthrough checks that the edited second brief leaves the first
intact. It does not establish that the new brief improves reception.

[Chromium capture of the binder](discussion-binder.png). Workspace tests and
doctests, strict Clippy and rustdoc, formatting, isolated fuzzy tests,
Wasm/browser build and all 41 Chromium tests passed.

## 2. Six-week station goals and scoped autosell

Find this slice with `git log --oneline --grep='Extend the station season and add scoped autosell'`.
See [weekly scheduling](../../docs/weekly-scheduling.md#optional-autosell)
for the check time and ranking rules.

1. Open the office notebook to see three season goals. Open Programming
   and make a weekly run with a 15-minute ad block.
2. Open the first Airing file and expand **Autosell this run**. Choose a
   minimum of 1 Fund per minute and a maximum of 10 sold minutes.
3. Run the airing and open Broadcast Review. Look for **Autosell** on the
   signed receipt. Minutes beyond the limit stay available for manual sale
   or trimming before air.
4. Use Week or Month in Programming to browse the extended calendar.

[Autosell policy capture](autosell-policy.png). The native comparison starts
with the same manually signed deal in both runs. The 9,000 Funds floor
leaves only that deal, while the 1 Fund floor fills another three minutes.
The ordinary airing remains unsold. Both paths preserve escrow and normal
settlement. The comparison does not establish that either floor is a good
long-run station strategy.

Workspace tests and doctests, strict Clippy and rustdoc, formatting,
Wasm/browser build and all 42 Chromium tests passed, including native/Wasm
parity and the autosell walkthrough.

## 3. Weekly advertiser reviews

Find this slice with `git log --oneline --grep='Review brand bids after weekly campaign results'`.
The [brand review notes](../../docs/brand-reviews.md) explain the threshold,
the bid adjustment, and the observation boundary.

1. Place two copies of Neighbourhood Laughs in separate 18:00 slots during
   the first week. On both Airing files, list ad minutes and sign an offer
   from Everyday Snacks.
2. Run both airings and finish the first week. In Rights Market, open the
   industry wire to read the brand's bidding announcement.
3. Open a future ad offer from that brand. Its card also carries the most
   recent public bidding change.

The native comparison uses the same purchases and alters delivery only.
It records a later quote of 987 Funds versus 808 Funds after equalizing
other quote inputs. Cash, booked prices and the ledger agree across the
cases. The sampled delivery proxy does not establish conversion or brand
profit.

[Browser capture of the industry wire](brand-review-wire.png). Workspace tests
and doctests, strict Clippy and rustdoc, formatting, the Wasm/browser build,
native/Wasm parity and all 43 Chromium tests passed.

## 4a. Weekly viewing habits

Find this slice with `git log --oneline --grep='Learn audience habits for recurring slots'`.
The [habit rules](../../docs/weekly-scheduling.md#slot-habits) describe the
quarter-hour slot key, decay and why the UI presents scheduling evidence
without revealing the learned strength.

Create a three-occurrence run in Programming, move the middle occurrence to
another time and air the first. Inspect the moved and unchanged future
airings. Only the unchanged time shows **Returning to regular slot**. The
Tape file's Reception section also notes a matching planned airing.

The controlled native fixture holds availability and competition fixed.
The unchanged second airing has 3,864.2 young viewers versus 3,812.2 after
moving by 15 minutes. Population remains conserved. The multiplier models
one possible habit, and forecasts still use published research instead.

[Chromium capture of the returning slot](returning-slot.png). Workspace
tests and doctests, strict Clippy and rustdoc, formatting, the Wasm/browser
build, native/Wasm parity and all 43 Chromium tests passed.

## 4b. Lead-in carryover

Find this slice with `git log --oneline --grep='Model bounded lead-ins between programmes'`.
The [lead-in rules](../../docs/weekly-scheduling.md#audience-carryover-between-airings)
explain the handoff, the ad-time discount and the allocation cap.

Place Workshop Science at 18:00 with nine ad minutes and Neighbourhood
Laughs immediately after it at 18:54. Open the second Airing file to see the
link to its possible lead-in. Leave at least 15 minutes between content end
and the successor to remove the note.

In the isolated native fixture the opening group carries forward about
1,039 viewers for an immediate successor with the same appeal, 254 for
a mismatched profile, and zero with a 15-minute gap. The retained viewers
are included in the station's allocation. The result is a simulation
estimate based on the predecessor's opening audience.

[Chromium capture of the potential lead-in](lead-in.png). Workspace tests
and doctests, strict Clippy and rustdoc, formatting, Wasm/browser build,
native/Wasm parity and all 43 Chromium tests passed.

## 5a. Recurring newsroom

Find this slice with `git log --oneline --grep='Run same-day news editions from Production'`.
The [newsroom rules](../../docs/recurring-newsroom.md) give the daily
timeline, financial commitments, and scope of the bulletin fixture.

Open Preparation, choose **Open daily newsroom**, and start five editions
using the default Evening Bulletin brief. Advance through the first tape
delivery. Replace Iris Bell with Tomas Avery for the unstarted editions,
then use the first card to open its airing and run it. The first card and
Tape file retain Iris's credit.

The controlled native replay deliberately blocks edition two with another
airing. Four delivered bulletins cost 1,400 Funds, including studio work.
The second edition is missed and charges nothing. A conflicting studio
booking and an attempt to move a bulletin into the next day both fail.

[Chromium capture of the newsroom board](newsroom-board.png). Workspace
tests and doctests, strict Clippy and rustdoc, formatting, Wasm/browser build
and native/Wasm parity passed. The full run passed 43 existing browser
tests. The new newsroom walkthrough passed on rerun after correcting
the staff selectors' accessible names.

## 5b. Guaranteed host dates

Find this slice with `git log --oneline --grep='Reserve prepaid host dates in a hiring desk'`.
The [agreement rules](../../docs/guaranteed-host-dates.md) describe
availability, exact obligations and the credit against booking.

Draft a weekly discussion binder with Elian North, open Hiring desk in
Production, choose his portrait card, and select all three dates. Sign for
660 Funds and inspect the signed dates. Book the first edition for the
remaining 780 Funds, then run through delivery and see the first date
marked fulfilled. A standalone draft can be signed by selecting just one
date.

The native test compares signed reservation conflicts and the existing
direct booking path. It verifies no duplicate payment, no cancellation
of a signed draft, cash conservation, and a separate one-date agreement.
This desk covers hosting dates rather than a general contract market.

[Capture of the signed agreement](host-agreement.png). Workspace tests and
doctests, strict Clippy and rustdoc, formatting, the Wasm/browser build,
native/Wasm parity and all 45 Chromium tests passed.

## 5c. Scripted series

Find this slice with `git log --oneline --grep='Produce a short scripted series with recurring roles'`.
See [scripted-series rules](../../docs/scripted-series-pilot.md) for the
tension curve and continuity decisions.

In Preparation, choose a standing brief, open **Pitch a three-episode
series**, and set the three tension targets. Commission and accept the
first script, then book its cast. In later episodes, the standing actors
appear in their original role seats. Completed episodes can be placed
from the tape library.

The controlled fixture uses the same final target and writer in both
cases. The script reaches 0.594 after a gradual curve and 0.490 after
abrupt shifts. It also verifies separate episode identities and rejects
a swap of the two recurring roles. The pilot does not model recasting or
audience response to every possible curve.

[Chromium capture of the scripted binder](scripted-series-binder.png).
Workspace tests and doctests, strict Clippy and rustdoc, formatting, the
Wasm/browser build, native/Wasm parity and 45 existing Chromium tests
passed. The new scripted-series browser test passed on its rerun with
normalized slider inputs.
