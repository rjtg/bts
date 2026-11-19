# Station office and Planning Room

The approved [perspective desk mockup](../web/public/mockups/perspective-desk.png)
shows a staffed television station. This is the design reference for the
browser office. Use perspective on the furniture and keep live text and
controls close to face-on. The shelf joins the table. The table holds a
cassette, open paperwork, a telephone, a small stack of files, and actionable
sticky notes. Behind the glass, quiet studio movement can add atmosphere.
Reduced motion uses a still scene.

The office is for current decisions. Its CRT shows On Air, Up Next, and the
next opening. Open the CRT to enter the Planning Room with the same schedule
position and selection. The room has a minute-accurate Day surface and more
compact Week and Month summaries. The summaries include first remaining gap,
occupied share, unsold advertising, and the last scheduled episode of each
recurring run. Show where playable dates end. Deliberate paging holds the
period. Today resumes following the clock. A summary month does not imply
monthly recurrence or a longer scenario.

The shelf filters by runtime and production type. Fiction includes films and
series. Nonfiction includes news and magazines, with discussion programmes
under magazines. Where the catalogue does not identify a more specific format,
show the known parent type. The desk prioritizes the dossier. Use a uniform
paper size with internal scrolling or additional leaves. Show a pencil-style
radar chart of visible programme quality bands, and list unknown or inapplicable
dimensions separately. Known cast, directors and writers open public person
files, with a route back to the programme. Comparison charts and hiring
directly from a person file remain optional future interactions.

Money is called **Funds** in the player interface. Its fictional glyph is
a capital F with a horizontal cross-stroke below the middle arm, crossing the
stem and leaving its lower half recognizably open. Use the glyph in the HUD
and the ledger with an accessible text equivalent. Manual pause covers the
room with a still 1980s OFF AIR test card. Continue restores the same desk
and focus. A paused player clock and a station's scheduled off-air interval
are separate states.

## Gameplay changes to explore separately

- Add a small transmission charge per minute, alongside title-specific costs.
  Decide how to account for idents, unsold ad time and genuine off-air periods.
  Preserve accepted commitments and exact integer accounting.
- Tune faster wall-clock playback independently of the simulation cadence.
  Set production lead times by format. Films can spend longer in writing and
  recording, while news needs reporting, production and airing aligned on
  the day of transmission.
- Manage a regular newsroom run rather than booking each edition. For example,
  weekday News at Seven can reserve reporters from 14:00, a studio at 18:00,
  and the slot at 19:00. The team prepares and produces eligible editions
  automatically. Preview future staffing and studio changes with their dates,
  costs and conflicts. A new anchor, an extra reporter or a different studio
  changes future editions while past credits and results remain intact.

## Browser implementation

The office is the starting screen. Its CRT opens Day in the Planning Room.
Both views share the selected tape, airing and date in the session. The Day
timeline supports pointer placement, touch and the existing keyboard
placement controls. Week and Month use seven-day and 30-day blocks from
scenario start. Each date shows the first *remaining* gap, percentage of the
day scheduled or aired, and unsold planned ad minutes. Recurring entries show
the last airing in each run within the displayed period.

Dossier charts use the same public quality bands as their text labels.
Their radial positions are illustrative band markers. Unknown qualities
leave an empty axis; a partial profile shows points without a filled shape.
Catalogue credits are shown when present in the player report. The office
uses a composited room plate with live shelf, cassette and paper elements.
The reflection behind the glass is decorative and static under reduced
motion. See the [capture notes](../contrib/station-office/README.md) for
fixture steps and comparison with the approved composition.

### Dossier stationery

The office file uses layered folder artwork, separate paper leaves, side
markers, typewriter text and handwritten annotations. The Brief pairs the
published quality profile with snapshot-based next-gap and suggested-slot
estimates. The Reception pages put a completed airing's provisional reach
shares beside a survey interest index and offer a date and time to try.
The survey index is a screening aid derived from public audience snapshots.
It omits competition and repeat viewing and is separate from the Rust ad
forecast. Members links to public credits. Bottom line uses attributed
Funds, Insight and Sway entries. A single overall quality rating is deferred.

## Agreed direction for the next office pass

The office is the main way into the station's work. Keep a compact desk
directory alongside the scene so every activity is reachable without
discovering a hotspot. Use live HTML controls over the artwork. A warm
outline and a short label identify interactive objects on hover or keyboard
focus. On touch, make the labels available without hover. Return to the
office without losing the selected tape, project, airing, or calendar position.

| Office object | Opens |
| --- | --- |
| CRT | Planning. Bring the screen forward into the interactive timeline for placing and moving airings. Keep the same date, scroll position, and selection when returning. |
| Tape shelf | Tape dossier beside the selected cassette. |
| Staff tables | Preparation. Pitch a project, commission writing or reporting where supported, and review what comes back. |
| Studio behind the glass | Production. Cast prepared work, book the studio, and check recording and completion. |
| Incoming files | Rights market and arrivals, assessments, and deadlines. |
| Research folder | Audience dossiers, polling, and Insight. |
| Ledger | Station finances and recent broadcast reviews. |

Preparation and Production are different views onto the same project.
A delivered script can make the staff table actionable, and a finished work
can appear as a cassette ready for the library. The telephone can remain
part of the room. Advertising actions live with the scheduled airing.

### Opening and closing files

In the resting office the dossier is closed and the CRT shows On Air, Up
Next, and the next gap. Selecting a tape opens its file across the desk,
using the CRT's space to give the papers more room. Close File and Escape
restore the office and leave the tape selected. Give dossier views consistent
two-leaf proportions, including Members when credits are sparse. Keep the
default Reception spread short enough to read without scrolling on desktop.
Put the previous airing on the left and the next-gap outlook on the right,
with the suggested time as an annotation. **Try another time** reveals the
date and time controls.

Selecting an airing block in the focused CRT opens that exact airing's
dossier. Closing it returns to the same point on the timeline. Set or review
the ad-block length during placement, including when a default sets it.
Manage offered minutes, compare offers, and sign terms in the airing's file.
An unsold-ad sticky note opens the same file directly. After broadcast, the
file gains available turnout and settlement. If turnout is compared with a
forecast, identify the dated forecast used before airtime and label observed
or provisional results separately. Accepted commitments keep their existing
protections.

### Folder types

Use the colour on the folder stock and side tabs, with warm paper inside.
Print the dossier type on the folder so it can also be identified by text.

| Folder | Colour | Contents |
| --- | --- | --- |
| Tape | Taupe | The work itself. Qualities, public credits, repeat history, and lifetime resource costs and receipts. |
| Airing | Teal | One showing. Airtime, expected audience, ad inventory, offers and signed terms, then turnout and settlement when available. |
| Preparation | Light pink | The creative brief, script or reporting assignment, delivery status, and review or revision decisions. |
| Production | Light blue | Cast, studio, commitments, recording and expected completion. |
| Character | Yellow | Public strengths, known availability and commitments, with a return to the file that opened it. |

This section records the agreed design. The current coverage is described below.
Player files continue to use legitimately available evidence.

The [Station Office widget guide](office-widget-style.md) defines the shared visual language and maps player tasks to their desk props.

## Playable office hub

The room now starts with the tape file closed. The CRT opens the existing
minute planner. Shelf selections open a closable Tape file, and the room
objects link to Preparation, Film Production, Rights, Audience, Finances,
and Broadcast Review. Discussion production is reachable from the production
desk and directory. The keyboard directory is also available on touch.
The film's project ID persists from the staff tables through script
acceptance, casting, and studio booking.

A per-placement ad-minute field starts at the station default and previews
the total occupied duration. The setting applies to the selected tape's
placement, including dragging the selected tape, and clears on successful
placement. The teal Airing file includes the existing sale and settlement
controls. The planner gives that file its own work surface below the
timeline and outside the narrow tape picker. Both placement and the agenda
can open it. Closing the file preserves the selected airing and timeline
position.

An inspected future airing can show a dated audience outlook based on the
published survey. The browser keeps the last such outlook for that start
time in the current session and can display it with the completed turnout.
The outlook remains a provisional screening estimate and is separate from
the Rust advertiser quotes. Opening a completed airing for the first time
does not invent a pre-air estimate.

The shared dossier uses HTML controls and layered paper. Tape files carry
the main set of labelled tabs. Airing, film project, and character files
use one spread each. The production views reuse the same project and public
credits. See the [browser evidence and replay steps](../contrib/station-hub/README.md).

### Resting desk and Programming overlay

The shelf starts put away. Pulling tapes out reveals the library and cassette
work surface. Browsing filters and tape selection persist when it closes.
The phone is a room prop. Labelled folders open Research, Rights, Production,
Preparation, Finances, and Reviews. The notebook holds today's next evening
gap, an unsold-ad reminder when applicable, and accepted Sway requests.
It crosses off completed requests. It uses the current work and requests
because the scenario has no separate selectable objective.

The slim Today readout follows the current day and the player's channel.
Gaps are visual. A tape drop uses the existing exact placement validation,
and a completed block opens its airing file. The tape tray still provides the
next-gap action for pointer, keyboard, and touch. Background monitors open
the scrollable Programming overlay over the office. Moving airings, rival
comparison, and calendar controls stay together there.

The resting room uses a bound notebook sprite and recoloured folder-stack
sprites with live labels and controls. Decorative cassette spines remain on
the shelf while the interactive catalogue is put away.

Hover and focus animate the resting props. Reduced motion keeps a still reaction and the destination label.

The lower shelf rows are now a labelled cassette layer fitted to the shelf
openings. The room keeps its notebook and folders visible while the tape tray
or a file is out. These background controls are inert until the player puts
that work away. Research is the first desk prop with separate closed and open
art. Preparation uses an alternate glance pose composited over the resting
staff member. The first prototype established both reactions before the remaining props were drawn.

The desk spreads props across the table by type. The script file, clapper
clipboard, research binder, rights portfolio, ledger, and review sheets each
have their own silhouette and a second hover/focus pose. The notebook shows a
curling page and the camera has a turned-head pose. The phone handset lift is
decorative. The three monitors run separate four-frame illustrated previews
on hover or focus. Reduced motion displays one frame per monitor.

### Blotter layout and prop transitions

The clean desk reference establishes an empty green work surface around the
closed notebook. The denser reference guides edge dressing: lamp and script
papers, ledger and calculator, phone and calendar. The browser draws these
on separate layers in the existing room. The notebook moves aside for other
jobs. Dense work continues in the existing accessible workspaces over the
room, and the background controls are inert while a workspace is in focus.

Clicking a prop or opening it by keyboard first moves its artwork toward the
blotter. The browser runs a 280 ms opening movement and a 190 ms return.
Escape can cancel a pending opening, and repeating an activation during
the movement opens only one workspace. Reduced motion skips travel and
ambient loops. Workspace scroll, the selected airing, and unsubmitted
inputs remain with their owning view.

The office has staggered plant, steam, and staff motions. Camera operators and interview participants move as part of the room
ambience. Their movement and prop travel are browser presentation effects. See the [browser comparison and replay](../contrib/blotter-office/README.md).

The Room motion control defaults to Auto and follows the system
reduced-motion setting. On explicitly enables the idle loops, and Off
disables them. The compact office shows the animated plant and mug. The
wide room also shows the staff pose change.

The wide office also varies the lamp's warm spill and changes the pictures
on the wall monitors between idle intervals. The camera occasionally rolls
toward the interview and back, the presenter gestures, and the tally light
brightens softly. The plant's pot stays fixed while the canopy moves.

Staff glances and the guest's nod use small registered pose patches.
The foreground glance changes the face area and leaves the original hand
in the room art. Each studio camera and its operator share a pose layer
and change together on staggered cycles.

The resting office randomly selects one visible subject at a time, with
no immediate repeat when there is another eligible subject. People, cameras
and lighting hold their new poses until selected again. The current actors
have two poses each. The foliage rustle settles after its brief movement.
The mug and wall monitors have independent cycles. Hovering an active prop,
opening a workspace or switching Room motion off stops new idle changes.
The compact office retains the leaf and mug effects. Idle choices use a
browser timer and have no effect on the station clock or simulation. The
next cue waits for the current movement to finish, then waits 40–120 ms. Cues are drawn without replacement until each visible subject has had a turn, including six desk staff, both camera operators, the floor assistant beside the left camera, and both interview participants. The distant figure behind the office window also has a cue.

Preparation and Production open from the desk props. Background people
have no hover targets or actions. The wall monitors remain the background
entry to Programming.
