# Production dossier browser walkthrough

Start from a fresh station. The Preparation script file offers Ideas,
Scripts and Production. The branch name is on
the front sticker. Use the hub to go back and Escape to close the can.

1. Choose **Preparation → Ideas → Draft new → Fiction → Film**. Enter a title and
   description. Set five stars for Tension, Spectacle and Pace. Develop the
   idea, advance to pitch completion, then open its writing file. Commission a
   script and advance to delivery.
2. Read the script notes. Turn to **Revision** to choose what to improve and
   pay for a revision, or accept the script directly. After accepting, assign
   the cast and director. Turn to **Booking**, review the costs and confirm.
3. Advance to delivery. Open the finished tape in the programme library to
   schedule, sell ad time and air it through the existing planner.
4. To try a series, choose **Ideas → Draft new → Fiction → Series**. Save a
   three-episode brief. The file box has one tab for the show. Open it to select
   an episode and close the dossier to return to that episode's tab.
5. To resume a paper after returning to the office, reopen the can and choose
   **Return to desk papers**. The **Project files** index tab also offers plain
   buttons to draft or browse.

## Captures

- [Tape Shelf can](tape-shelf-can.png)
- [One-shot scheduling can](one-shot-can.png)
- [Canister and branch sticker](film-can-branch.png)
- [Show file box with episode folders](show-file-box.png)
- [Script review](script-review.png)
- [Studio booking and call sheet](studio-booking.png)
- [Narrow, touch and reduced-motion layout](production-narrow.png)

## Reproduce

From `web/`, build with `npm run build` and serve with
`npm run preview -- --host 0.0.0.0`. The screenshot fixtures run with
`npm run test:e2e -- film.spec.ts scripted-run.spec.ts office-interactions.spec.ts` and save their captures
under `web/test-results/`.

Film ideas enter the idle-script files after development. The screenshots of
the canister and show box were captured before the stage menus split. Film
draft deletion and script sales remain proposals.
