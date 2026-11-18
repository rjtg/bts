# Control Desk weekly playthrough

1. Start the browser scenario on Saturday. Select **Workshop Science** and slot
   it on Day 8 at 18:00.
2. Under **Give a show a regular slot**, choose **Neighbourhood Laughs**,
   Saturday at 18:00, three weeks and ten ad minutes. Preview the dates.
3. The second Saturday clashes. Choose **Move this occurrence**, change its
   time to 20:00, update the preview and confirm. The other Saturdays keep 18:00.
4. Select the first airing in the planner. Find offers in its ad drawer and
   accept Everyday Snacks and Precision Equipment. Run through the selected
   airing and inspect its provisional audience and paid ad receipts.
5. At a narrow viewport, the agenda links to the same drawer and parallel grid.
   Use the day/time slot picker or click a gap to place a programme.

[Clash review](clash.png) · [Ad booking](booking.png) · [Receipt](receipt.png) ·
[Narrow agenda](agenda.png)

To reproduce the screenshots, build the app and run
`BTS_WEEKLY_SCREENSHOTS=1 npm run test:e2e -- tests/weekly.spec.ts` from `web/`.

## Controlled watch-time comparison

The same programme and rival aired at 18:00 in two fresh runs. The population
of young viewers was 6,000 and 4,800 could watch in each case. Saturday's
inclination to watch was 0.85 and Monday's was 0.15.

| Day | Your channel | Rival | Not watching |
| --- | ---: | ---: | ---: |
| Saturday | 2,963 | 1,336 | 1,701 |
| Monday | 2,664 | 1,201 | 2,135 |

Counts are provisional expected viewers rounded to whole numbers. Saturday
increased viewing for both stations without altering their relative appeal or
creating extra availability. This comparison isolates the preference parameter.
It does not establish balance or show that a slot habit has formed.
Regenerate the exact values with `cargo run -q -p bts-web --example weekly_planner`.
