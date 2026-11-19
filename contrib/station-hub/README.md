# Station Office hub

## Replay

Open the office, choose Pull out tapes, expand Neighbourhood Laughs, and select Episode 1. Close
the Tape file with Escape or Close file. Open the wall monitors, jump to Evening,
and pick 18:00. Enter 16 ad minutes and place the 30-minute episode. The
teal file shows 18:00–18:46. Find offers, expand Everyday Snacks and sign
its offer. The move button is locked and the ad selector's minimum is now
the sold inventory. Run through the airing, then read its settlement.
Close the file and return to the office.

For film preparation, open the staff tables, pitch a film, commission the
script, and advance to script delivery. Accept the script to move that
same project to Film Production. Return to the office and use the studio
hotspot to reopen it. Review and confirm the booking.

Capture from the built bundle with Chromium:

```sh
cd web
BTS_HUB_CAPTURES=../contrib/station-hub/after npm run test:e2e -- station-hub
```

Desktop uses 1280 × 900. The narrow journey uses a 390 × 844 touch viewport
with reduced motion. The baseline images were taken from the previously
verified office build. Previous office screenshots show an already open
dossier because that build opened it automatically.

## Visual comparison

- The resting desk now leaves the CRT and furniture visible. Hovering over
  the staff or studio shows which work area it opens.
- The Tape file uses the height freed by the CRT and retains distinct leaves.
  Expanding custom airtime is optional.
- The initial narrow test exposed that the old tape dialog could conceal
  the airing file after placement. The first desktop pass also squeezed it
  beside the shelf. Its own planner surface gives both leaves room and
  keeps the file available after the picker closes. Long result and market
  sections scroll on narrow screens.
- The focused CRT retains the flat two-channel timeline for readable editing.
  Work-area transitions use separate views rather than animated camera moves.

The Today strip supports a shorter replay. Pull out the tapes and drag Episode 1 onto the 18:00 position. Run it and select its finished block on the strip to reopen its airing file.
