# The Control Desk

A proposed UI for Behind the Screens. These are deliberately separate from
the playable browser harness. Open [the linked boards](index.html) in a
browser, or use the static previews below. Sample times and economics are
illustrative, with current mechanics and terminology carried through.

## Review in play order

| Decision | Linked board | Preview |
| --- | --- | --- |
| Assess and acquire The Corner Shop | [Rights listing](index.html#rights) · [Optional title assessment](index.html#title-study) | [Rights](previews/rights.png) · [Assessment](previews/title-study.png) |
| Place it from the library | [Empty schedule and placement](index.html#empty) | [Placement](previews/empty.png) |
| Check the evening and take a second ad offer | [Planner and ad drawer](index.html#planning) | [Planner](previews/planning.png) |
| See a rejected move and keep the original slot | [Blocked draft variant](index.html#blocked) | [Conflict](previews/blocked.png) |
| Confirm the block and run through the correct airing | [Held inventory](index.html#ready) | [Held inventory](previews/ready.png) · [On air](previews/on-air.png) |
| Decide whether it is worth airing again | [Airing recap](index.html#recap) | [Recap](previews/recap.png) |

In the connected day, first buy the film and place it at 18:00. Separately
place Neighbourhood Laughs at 17:30 so it runs next. Fifteen ad minutes
produce 105 minutes of exact runtime and 105 minutes reserved. After both
three-minute sales, trim to six ad minutes. The resulting 96-minute exact
runtime still occupies 105 minutes. Run through the selected film to 19:45.
Its 615-credit settlement less 100 to air leaves 515 for this airing.
Against the 2,456-credit acquisition, 1,941 remains to recover. This is
a title accounting view, not an audience score.

| Related desk | Linked board | Preview |
| --- | --- | --- |
| Film production | [Script review](index.html#script) · [Cast and studio](index.html#casting) · [Film ready](index.html#film-ready) | [Script](previews/script.png) · [Booking](previews/casting.png) · [Film ready](previews/film-ready.png) |
| Audience research | [Audience dossiers](index.html#audience) | [Audience and Investigate](previews/audience.png) |
| New release and rival acquisition | [Day 3 industry wire](index.html#wire) | [Industry wire](previews/wire.png) |
| Touch planning | [Narrow planner](index.html#narrow) | [Narrow planner](previews/narrow.png) |
| Design references | [Two explorations](index.html#directions) · [Icon specimen](index.html#icons) | [Explorations](previews/directions.png) · [Icons](previews/icons.png) |

## Design and behaviour notes

- [Current journey audit](audit.md) records decisions and friction before the
  redesign. [Visual explorations](directions.md) compare the control room
  and producer's office. [Visual guide](visual-guide.md) specifies the shared
  components, tokens, accessibility and sprite.
- **Current mechanics.** Explicit ticks and duration, Rust-owned booking locks,
  offer acceptance and settlement, assessments, polling and title transfers.
  A prior airing's audience is a provisional start-of-airing expectation.
  The recap shows actual settled money. Programme reception beyond current
  player evidence remains unavailable. Script copy in the boards illustrates
  what can be expressed using current strengths and shortcomings.
- **Proposed interactions.** Direct timeline manipulation, contextual drawers,
  one active casting seat and one picker, destination links for industry notices,
  ambient lighting, and a shorter completion-to-recap route. These boards use
  links to illustrate states. Buttons without a destination are specimens.
- **Mechanic or protocol questions.** A true placement dry run may need a
  read-only planner preview. Do not infer authority from the optimistic preview.
  Warnings about events passed by a time jump require dates already known to the
  player; do not predict unpublished rival events. A persistent title recovery
  figure may need an explicit public projection from the rights ledger and
  linked airing receipts. None of this requires changing contracts or revealing
  hidden audience traits. Do not add publication, new ratings or new refunds.
- **Input equivalence.** Pointer users can drag a library card into the grid or
  move an existing draft. Every user can select the card, use Place or Move,
  choose a day and 15-minute slot, review the end time, and confirm. Touch can
  tap a gap or use the same slot picker. On keyboard, Tab to the selected card,
  Enter to open its action, then use the slot control and Enter to confirm or
  Escape to cancel. On failure, keep the old booking and focus the reason with
  a link to the conflict. A locked airing leaves its start protected and
  directs the player to available actions, including trimming unsold minutes.
- **Build order for a later slice.** First implement the station shell, planner
  draft/selection states and ad drawer. Then ship the recap and rights-to-planner
  link, followed by shared studio and research components. Use the connected
  evening as the acceptance journey on desktop and narrow layouts.

Evidence for **the existing browser** remains in the linked milestone reports.
The final previews are renders of the local HTML and CSS boards. They use
existing game portraits and programme art, plus the new small icon sprite.

Follow-up design: [weekly scheduling and audience flow](../../weekly-scheduling.md)
adds recurring slots, autosell, weekday preferences, viewing habits and lead-in
carryover to the planner direction. The current boards illustrate single airings.
