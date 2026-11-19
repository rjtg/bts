# Station Office browser comparison

Open [the comparison](index.html) through the network preview at
`/mockups/station-office/`. The approved reference crops retain the source
aspect ratio and are displayed at the same width as the Chromium captures.

## Replay the scene

1. Open the game as a fresh session. The Office is the first screen.
2. Open the Planning Room using the CRT. Choose 18:45 on Day 1 and place the
   starting tape. Choose 18:00 to preview the remaining 45-minute gap.
3. Return to the Office. Open the Neighbourhood Laughs case and select
   Episode 1. The Brief paper and gap note should be visible together.
4. Start Play and immediately press Pause. Continue dismisses the test card
   and returns to the open papers. Open the CRT again for Day, Week and Month.
5. For narrow reproduction use a 390 × 844 viewport and reduced motion.
   Dismiss the tape picker before leaving Planning. Scroll to the papers.

To refresh browser captures from a built bundle, run from `web/`:

```sh
BTS_OFFICE_CAPTURES=../contrib/station-office/after npm run test:e2e -- station-office
```

The old baseline used the same placement and selected the same tape from
the earlier combined planner. Its narrow capture opened the station-file
dialog. The rebuilt narrow office shows the papers inline.

## Comparison notes

| Pass | Browser observation |
| --- | --- |
| Baseline | The timeline and shelf read as separate panels. The cassette and dossier were level with each other in a card grid. Manual pause left the pixelated scene in view. |
| Room and furniture | The angled shelf reaches the table edge, the desk has a phone and files, and the live papers sit on the wood. The first capture exposed a programme tab that accidentally included every quality label. The studio and open dossier competed for attention. |
| Paper and pause | Side filing markers, typewriter text, two facing sheets and a graphite chart make the programme readable. The full-screen still test card puts Continue under OFF AIR. The CRT and notes now sit in the same room as the tape. |
| Remaining differences | The reference uses a shallower office crop, a more strongly angled shelf, a larger binder, and a CRT with a timeline. The playable desk leaves more space for the staffed studio and keeps the CRT a status display. The approved pause panel has two additional calibration circles. The narrow layout stacks controls and papers vertically. |

The initial catalogue episode has no public creative credits. Created films
and discussion programmes use public credits from their corresponding
projects. The chart uses qualitative published bands. Exact hidden programme
quality and audience state are outside the dossier.
