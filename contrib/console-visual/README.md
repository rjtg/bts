# Control Desk visual comparison

Reference: [approved tape-shelf mockup](../../web/public/mockups/tape-shelf.png).
The artwork depicts a short illustrative evening and the review fixture uses
the existing 21-day scenario. The fixtures keep the same schedule and station
data in the before and after captures.

## Reproduce

Build the browser bundle, then run from the repository root:

```sh
npm --prefix web run build
cd web
BTS_VISUAL_DIR=../contrib/console-visual/after npm run test:e2e -- console-visual
```

The desktop fixture uses a 1280×900 viewport. Book a 30-minute tape at 18:45
to bound the gap. Select 18:00 on your channel, preview ten ad minutes, open
the Neighbourhood Laughs case, and pick Episode 1. Capture Schedule at the
top of the page, then open the station file to Audience. The narrow fixture
uses 390×844 and reduced motion.

## Artwork

The built-in ImageGen tool produced a six-cell material atlas with a
straight-on charcoal-and-bronze CRT bezel, wood shelf, blank labelled VHS
spine, burgundy series case, two-reel VHS cassette, and an open three-tab
manila folder. The prompt requested crisp late-1980s pixel art, separate
objects, no lettering, and a solid magenta background. Exported pieces are in
`web/public/console-art/`. Only the magenta matte and the space between sprites
were removed. CSS slices the CRT's frame so its corners keep their dimensions
around the scrolling HTML timeline. The tape label, case contents, folder
tabs and estimates are live HTML.

## What changed in each capture

| State | What the equal-width comparison showed | Response |
| --- | --- | --- |
| Before | The left rail and stacked controls occupied the first screen. The shelf was clipped, and the cassette and folder looked like nested panels. | Move navigation to a slim station strip, make the CRT span the workspace, and give the desk and shelf separate silhouettes immediately below. |
| First art pass | The frame and case read as objects, but the tape had collapsed to near-zero width and the shelf still extended below the desktop viewport. | Give the cassette a definite aspect ratio and width, combine time controls around the CRT, and shorten the shelf and the cabinet trim. |
| Refined art pass | The gap and cassette read clearly at a glance. The file still obscured the selected gap, and the cassette overlapped much of the folder button's hit area. | Bring the file up from the lower desk, keep the gap visible above the folder, and limit the folder button to the exposed tab. |
| Current | The CRT, selected gap, shelf, cassette, file tab and placement action fit together in the specified desktop viewport. The desk action sits beside the cassette. | Keep the responsive HTML controls and open the papers full-screen on narrow displays. |

The reference gives a more compressed, almost edge-to-edge scene, with
bespoke meter art, perspective on the desk and individually designed tape
labels. The live desk still uses a straight-on cabinet, a few functional
station rows and a wider scrolling evening window. Its title and schedule
come from scenario data, so both art and typography must accommodate longer
names. The reference's illustrative clock and channels are separate from the
scenario state used for testing.

## Serve the comparison with preview

After building and capturing, copy the comparison directory into the static
preview output from the repository root:

```sh
cp -R contrib/console-visual web/dist/mockups/
```

Open `/mockups/console-visual/index.html` on the preview host. Each row shows
the reference panel and the before/after captures at the same displayed width.
The reference remains in `web/public/mockups/tape-shelf.png` and is included by
the browser build.
