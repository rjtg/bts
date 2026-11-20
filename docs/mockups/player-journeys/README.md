# Station Office journey review

**Design for feedback · #18.** Open the [linked review boards](index.html) from
a local checkout, or use the screenshot links below in GitHub. The boards are
authored states connected by links. They reuse game content and art. They send
no game commands and are outside the playable bundle.

Read [journeys and current findings](journeys.md) first, then
[widget specifications](widgets.md). The table pairs each proposed work
surface with a live Chromium capture. Baseline is the published `29f33cc`
game. Sample fit bands, the blocked slot, research example and booking
examples illustrate proposals. The 18:00 showing and its 511 Funds of ad
receipts, 150 Funds of airing cost, and 361 Funds margin come from the replay.

| Journey | Current Chromium capture | Proposed work surface |
| --- | --- | --- |
| Start from the office | [Resting office](baseline/desktop-resting.webp) | [Physical tape rack](mockups/rack.webp) |
| Find a showing | [Tape file](baseline/desktop-tape.webp), [Programming](baseline/desktop-planner.webp), [placement slip](baseline/desktop-placement.webp) | [Reception](mockups/reception.webp), [calendar fit](mockups/calendar.webp), [blocked slot](mockups/blocked.webp) |
| Decide on rights | [The Corner Shop offer](baseline/desktop-portfolio.webp) | [Asking price and signature](mockups/market.webp), [acquired tape handoff](mockups/acquired.webp) |
| Decide what to learn | [Research binder](baseline/research-binder.webp) | [Group file](mockups/research.webp), [investigation receipt](mockups/investigated.webp) |
| Move a show into production | [Harbour Lights file box](baseline/show-file-box.webp), [Discussion work order](baseline/studio-work-orders.webp), [Newsroom](baseline/newsroom-board.webp) | [Show folder](mockups/production.webp), [Call sheet and room](mockups/booking.webp) |
| Review an airing | [Signed going into air](baseline/desktop-airing-planned.webp), [after settlement](baseline/desktop-airing-completed.webp) | [Forecast facing receipt](mockups/airing.webp) |
| Follow the money | [Station finances](baseline/desktop-ledger.webp) | [Ledger filtered to the showing](mockups/ledger.webp) |
| Narrow comparison | [Time-first placement](baseline/narrow-placement.webp), [airing review](baseline/narrow-airing-completed.webp) | [Calendar and slip](mockups/narrow-calendar.webp), [stacked airing papers](mockups/narrow-airing.webp) |

## Review in order

1. From **Tape to airtime**, open Episode 1, read Reception and find airtime.
   Inspect the clash, return, place the 18:00 example and sign the offer. View
   the transmission receipt and follow it into the Ledger and back. Try the
   alternate calendar-gap route at the **Choose a different tape** link.
2. Open **Rights** and follow an assessment through acquisition to the tape.
   Open **Research** and compare the investigation receipt with Survey orders.
3. Open **Production**, select Harbour Lights Episode 1, accept its script,
   inspect an example clash, then follow the booked state through delivery.

The boards use static tab labels where the content is outside these paths.
Use the in-scene Back links or Escape where available. The route links at
the top are a review aid. In the game, the desk props remain the entry points.

## Deferred design decisions

Use most of the viewport for the active workspace. Keep a margin of office
art around it for orientation. The questions below can wait for their
respective implementation slices.

- Should **Best aired** and **Also works** always be the first tape evidence,
  or stay inside Reception? Is the proposed fit strip clear enough to choose
  a time while keeping weak fit and a clash distinct?
- When a rights purchase succeeds, should the player stay with the receipt
  and choose **Pull tape**, or go straight to the new tape?
- Should the show folder show every episode and its state together, or
  show only episodes in the active stage until the player asks for the rest?
- Does the forecast-facing-receipt layout help with the next decision?
  Should Ledger open filtered to one airing or keep all nearby entries visible?
- On a narrow screen, compare stacked leaves with sequential pages before
  committing to a single pattern for reports that need side-by-side reading.

## Replay

Install the project prerequisites and build the browser as documented in
[Browser foundation](../../browser-foundation.md). From the root, run
`npm --prefix web run preview` in one terminal. With Chromium installed and
ImageMagick available, run this from the root in another terminal. Set
`PLAYWRIGHT_BROWSERS_PATH` if using a custom cache.

```sh
node docs/mockups/player-journeys/capture.cjs http://127.0.0.1:4173/
```

This captures the current ledger for the replayed airing, checks its return
focus, follows linked mockup scenes, exercises keyboard and narrow touch, and
regenerates the proposed screenshots. Omit the URL for the mockup-only run.

For the other current-game screens, run the existing Playwright specs from
`web/`. Capture directories are optional. Reuse this folder’s filenames
after converting capture PNGs to WebP at quality 85. For the mockup room
backdrop, crop the resting screenshot to `1280x732+0+168`.

```sh
BTS_OFFICE_INTERACTION_CAPTURES=/tmp/bts-journey-baseline \
BTS_OFFICE_PAPERS_CAPTURES=/tmp/bts-journey-baseline \
BTS_HUB_CAPTURES=/tmp/bts-journey-baseline \
npm run test:e2e -- --workers=2 office-interactions office-papers station-hub scripted-run rights audience newsroom
```

`show-file-box.png` and `newsroom-board.png` are attachments under
`web/test-results/`. If port 4186 is occupied, use a copy of the Playwright
config with an unused port in both the base URL and the preview command.

To replay manually, use the shelf to pull Neighbourhood Laughs Episode 1,
open the calendar and place on Day 1 at 18:00 with 16 ad minutes. From the
Airing file find offers, sign for Everyday Snacks, then run the selected
airing. Keep a pre-air outlook by leaving the file open before transmission.
Close the airing, return to the office and open Ledger. The baseline suite
also checks the time-first path with a conflicting placement, successful
rights acquisition and blocked resale, Research draft state, and both
scripted and newsroom production.
