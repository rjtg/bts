# One Saturday from Schedule to Broadcast Review

From the repository root, build the app and start a preview:

```sh
npm --prefix web run build
npm --prefix web run preview -- --host 0.0.0.0
```

Open port 4173 on the host in a browser. The scenario opens in standby with
planning available. Use the shared clock and transport on any desk.

1. On **Schedule**, open the tape shelf and book **Workshop Science** for
   Day 8 at 18:00. Try dragging a title onto the timeline on desktop or
   choosing its day and start time in the shelf on touch or keyboard.
2. Open **Give a show a weekly slot**. In **Weekly Run**, preview
   **Neighbourhood Laughs** for three Saturdays at 18:00 with 10 ad minutes.
   Move the clashing Day 8 occurrence to 20:00 and confirm. Schedule returns
   with the first occurrence selected.
3. Open **Ad Sales** from the first airing. Find offers and accept **Everyday
   Snacks** and **Precision Equipment**. Each agreement identifies this
   airing. Return to Schedule to verify the booked airing is protected.
4. Press **Play**, then **Pause**. The picture freezes and **Continue** is the
   only interactive control. Resume and see the same desk and agreements.
   A hidden tab stops playback until you explicitly press Play again.
5. Run through the selected airing. Choose **Review broadcast** to see
   audience delivery, settled receipts and airing costs. Use **Next showing**
   to look at the moved occurrence on Schedule.

For a shelf-only test, book a title at 18:45 and select the 18:00 gap. Preview
10 ad minutes and turn on **Fits this gap**. A 30-minute title plus 10 minutes
of advertising fits in the 45-minute reservation. A 45-minute title alone
fits but is filtered out when the preview includes advertising. The planner
still uses quarter-hour reservations. Sell the actual minutes in Ad Sales
after placement.

[Schedule](schedule.png) · [Weekly Run](weekly-run.png) ·
[Ad Sales](ad-sales.png) · [Paused on a narrow screen](pause.png) ·
[Broadcast Review](review.png)

Regenerate the captures with `BTS_DESK_SCREENSHOTS=1 npm --prefix web run test:e2e`
after building the bundle and installing Chromium for Playwright.

See the [station-time playtest proposal](../../docs/station-time-experiment.md)
for the queue and deadline comparison. The browser slice does not add action
cooldowns. Minute-accurate placement and partial-tick exposure are a separate
follow-up.

## Verification

Chromium passed all 25 browser tests, including drag and conflict rejection,
keyboard placement, narrow tape-shelf placement, retained planner context,
pause and focus, weekly exceptions, ad settlement, and native/Wasm replays.
Workspace tests and doctests, formatting, Clippy, docs, Wasm and production
browser builds, and an extracted standalone fuzzy-crate check passed.
