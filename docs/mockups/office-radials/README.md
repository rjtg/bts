# Station Office radial controls

Start with the [interactive walkthrough](review.html). Open it directly from
the checkout in a browser. The review page offers a comparison, the interactive
scenes and the icon sheet. The artwork is drawn from the existing game records
and assets. The scenes use fixture data and local preview state.

## Review order

| Screen | What to look for |
| --- | --- |
| [Treatment comparison](treatment-comparison.png) | Radial sectors and a physical rotary knob at the same size and shelf anchor. The control is pushed inward at the left edge. |
| [Tape scheduling](tape-scheduling.png) | The tape and dossier remain visible when the center switches to a calendar. |
| [Narrow choices](narrow-choices.png) | The same branch reads as two wide tiles. The selected time stays visible and an unavailable choice includes its reason. |
| [Exact time handoff](exact-time-handoff.png) | Episode 1 arrives back at Saturday, Day 1, 18:07. Changing the ad minutes updates the occupied interval before confirmation. |
| [Production roster](production-roster.png) | A short clapperboard menu hands off to named portrait cards with rates. |
| [Icon and state sheet](icon-and-state-sheet.png) | Context and action icons, including normal, focused, pressed and unavailable treatments. |

## Direction

Use **radial sectors** with a chamfered brass rim, dark green enamel and
cream printed legends. Keep the control nearly face-on over the existing
perspective room. The brass tick marks borrow the useful tactile detail
from the alternate rotary control.

| Decision | Radial sectors | Station rotary |
| --- | --- | --- |
| Scan the choices | Separate wedges give the direction, icon and label one place. | The player sees a knob and discrete keys. It can suggest that they have to turn the knob before choosing. |
| Return to the previous level | Breadcrumb above, consistent Back and Cancel below. | The same return controls are available, with extra visual emphasis on the knob. |
| Keep the room in view | A 435 px diameter on a 1280 px scene. No surrounding modal card covers the furniture. | The same diameter and position. The knob uses more of the center for hardware. |

The sectors fit both direct selection and keyboard movement. The larger
effective hit areas shown by the wedge boundaries are a design target.
The walkthrough prototype uses labelled buttons within the wedges.

## Interaction guide

- **Context.** Use a cassette for Owned works, a calendar for Schedule and
  One-shot, a recurring arrow for Regular, script pages for New project and
  a clapperboard for Production. Changes in category have their own center
  label and icon. Use words with the symbols. Keep the current branch in
  the small strip above the wheel.
- **Placement.** Open by the trigger. If the rim would cross a viewport
  edge, shift the full unit inward and keep its proportions. The shelf
  comparison demonstrates the left edge. Do not move the wheel between
  branches unless needed to keep it in view.
- **Choices.** Fit up to four to six short choices. Long collections use a
  tape index or a portrait roster, with names and actual asset artwork.
  Use a solid dark surface at rest, an amber outline in focus, and a
  recessed bright surface during activation. A disabled choice gets a
  dashed outline and a short reason.
- **Keyboard.** Opening the wheel focuses the first available item. Right
  or Down moves forward and Left or Up moves backward, wrapping and skipping
  unavailable items. Enter activates the item. Tab reaches Back, Browse all
  tapes where available and Cancel. Back goes up one level. Cancel and Escape
  return focus to the original prop or button. Escape exits the entire
  wheel, even from an inner branch.
- **Touch.** On a narrow viewport, keep the branch and center icon in a
  header above 64 px high tiles. Back and Cancel use at least 44 px high
  controls. The selected minute remains in the header and on the booking
  slip. No hover is needed.
- **Preview.** Scheduling actions open an editable slip or the calendar.
  Keep the programme duration, date, minute, offered ad time and end time
  together. A regular booking lists each proposed occurrence. In the
  actual game, the player confirms the Rust review before any place is
  committed.

## Follow the scenes

1. **Work first.** Open shelf → Fiction → Series → Neighbourhood Laughs →
   Episode 1. The cassette and file are visible together. Choose
   Schedule this tape → One-shot. Try Next gap, Recommended slot, and
   Choose on planner. The booking preview accepts an edited minute and ad
   load.
2. **Regular.** Return to Tape scheduling → Schedule this tape → Regular.
   Compare Recommended run with Choose days and time. Change the day and
   use Refresh dates to see the proposed occurrences. This is a fixture
   preview of the weekly editor.
3. **Time first.** Open calendar → select the 18:07 gap → Choose programme → Fiction →
   Series → Neighbourhood Laughs → Episode 1. The preview still starts at
   18:07. Cancel returns to the selected time and focuses Choose programme.
4. **Project.** Open Preparation → New project → Discussion show →
   Open Production board → Cast or book. Choose host leads to the eligible
   portrait roster. Studio booking opens a sheet for the same project.

## Current build and proposals

The game already provides short wheels on the shelf, Tape scheduling and
New project, a list for titles, editable booking and weekly reviews, and
labelled narrow controls. The screenshots in the comparison use a captured
office as the backdrop. The proposals are the sector surfaces and finishes,
the return key treatment, and a shorter path to the active material.

The clapperboard wheel and fixture filtering hint **No fit** are proposed.
The game uses the Studio book and existing casting controls. The mockup
shows a representative game title and roster. Its preview dates and
availability are illustrative. The walkthrough changes its own scene on
confirmation.

## Verification

A Chromium walkthrough starts at the visible props, navigates the shelf by
keyboard, selects all placement branches, edits a one-shot and weekly preview,
follows the production roster and studio paths, and uses touch for the 18:07
handoff. It checks that Cancel and Escape restore focus and that the narrow
scene fits the viewport. This was an automated walkthrough, with screenshots
inspected separately. It has yet to be tried by a first-time player.

From the repository root, with Playwright Chromium installed for the web
package:

```sh
node docs/mockups/office-radials/verify.cjs
```

The script regenerates the six review PNGs alongside the source.

## Film can follow-up

[View three compact film can studies](film-cans/index.html) from the next
office review. The [Production dossier proposal](../../production-dossier.md)
covers the taller spread, page turns and the Ideas, Scripts and Production
branches shown on the can.
