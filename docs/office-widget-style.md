# Station Office widget guide

This guide records the visual direction for current and future player actions.
The [Station Office](station-office.md#blotter-layout-and-prop-transitions)
describes the implemented room. The widget treatments below are design targets.
Future entries follow their own gameplay milestones.

## The office is the home screen

- Keep the green blotter clear for the current job. At rest, give it the
  closed notebook and a pencil. Cluster other props naturally around the
  edges, with distinct silhouettes, warm contact shadows and a shared
  perspective. Keep the shelf stocked and the room visible during work.
- Let an object suggest what it opens. On hover or keyboard focus, give it
  a small physical reaction and a short label. On activation, bring the prop
  to the blotter and open its workspace. The monitors are the entry to
  Programming. The people and equipment behind the glass provide atmosphere.
- Use desk-sized spreads for a single decision and near-full-screen overlays
  for timelines, casting, and other work that needs room. Only the focused
  workspace takes input. Closing it returns focus to the entry prop and
  preserves the draft, selection and scroll position. The desk directory
  provides the same routes by keyboard and touch.
- Keep the slim Today strip and physical transport buttons in reach. The
  strip is an at-a-glance channel readout with tape drops and airing files.
  Use the monitor workspace for moving blocks, comparing channels and
  planning across dates. Manual pause brings up the OFF AIR card with a
  sharp Continue control.

## Shared visual language

| Element | Treatment |
| --- | --- |
| Furniture and props | Warm late-1980s pixel art. Register separate sprite layers to the room and use perspective on furniture and covers. |
| Working surfaces | Near face-on paper, tape labels, CRTs and device controls. Keep text, charts and interactions live in HTML or SVG. |
| Typography | Typewriter-style briefs and typed labels. Reserve handwriting for short recommendations, crossed-off requests and marginal notes. Use a clear numeral treatment for times, dates and Funds. |
| Dossiers | Two separate, fixed-size leaves with labelled edge tabs. Overflow goes to another sheet or scrolls within a leaf. Tape taupe, Airing teal, Preparation pink, Production blue, Character yellow. Name each type on the folder. |
| Controls | Use portrait seats for casting, tabs and tiles for choices, a dial for bounded spending, and buttons labelled with the action. Show exact amounts beside dials and give them a keyboard-operable exact entry. |
| Evidence | Pencil-style radar charts suit a profile. Use timelines for obligations, bars for airtime and ad minutes, and aligned figures in the ledger. Keep observed results, dated estimates and unknowns visibly distinct. |
| Motion and feedback | Stagger tiny ambient gestures independently of game time. Use a stamp, signed line, moved tape or dated notice to confirm consequential changes. Give blocked actions a concrete reason beside the control. Under reduced motion, keep the labels and final poses and skip travel and idle animation. |

## Which widget fits each action

| Player task | Entry and widget | What the player can decide or see |
| --- | --- | --- |
| Find and place a programme | **Shelf → tape tray and taupe file.** Box episodes together and give cassettes readable spine labels. | Filter by runtime or production type. Pull one tape and browse Brief, Reception, Members and Bottom Line. Use Next Gap or the planning controls. Show title and runtime on the tape itself. |
| Arrange a day or weekly run | **Wall monitors → Programming overlay.** Give the timeline the full width, with Day, Week, Month and Today. | Move or place at exact minutes, compare an available rival schedule, and preview recurring dates and clashes before confirming. Keep the selected airing and Up Next separate. |
| Sell ads or inspect a broadcast | **Airing block or notebook reminder → teal Airing file.** Use an inventory strip and offer slips. | Set ad load and see occupied airtime, sign an offer, or leave space unsold. After air, show dated outlook, provisional turnout, settlement and airing cost in the same file. |
| Pitch, commission or revise | **Script file under the lamp → pink Preparation spread.** Put the brief and script notes side by side. | Compare writer cards, fees and delivery dates. On return, read what the script achieved, what it demands, and what a paid revision would address. |
| Cast and book | **Clapper clipboard → blue Production board.** Pin role cards and place portraits in their seats. | Compare known strengths and availability, select a studio, set cooperation support, and review exact obligations before booking. |
| Inspect a person | **Known credit or portrait → yellow Character file.** Keep the portrait, public work and known commitments together. | Return to the same project or tape on close. Future auditions and contracts can open from this file with explicit costs and obligations. |
| Buy or sell rights | **Incoming portfolio → illustrated title cards and assessment sheet.** | Compare asking prices, deadlines and public evidence. Request an assessment, buy a work or review a resale offer. Link acquisitions to the tape shelf. |
| Research an audience | **Research binder → illustrated group dossiers.** Use half-star preference marks, a dated survey and an Investigate dial. | Adjust the shared polling schedule or spend Insight on one group. Keep the published snapshot in place until the next poll. |
| Check priorities and requests | **Notebook on the blotter → open handwritten spread.** | See the next gap, unsold ad reminders, and Sway requests with deadlines and crossed-off completions. Open the relevant airing or job directly. Add station goals here when available. |
| Understand the money | **Ledger and review sheets → station books and broadcast recap.** | Scan cash and commitments or open a specific airing. Distinguish each airing's costs and receipts from programme investment and lifetime totals. |

## Widget homes for the roadmap

- **Recurring discussion editions and scripted series.** Use a show binder
  with one card per edition. Keep the standing brief and host on the cover.
  Put dates, changes and reception on the edition cards. An edit says which
  future editions it applies to.
- **Autosell.** Keep it in the recurring run and teal Airing file. Give it a
  labelled switch with visible limits and show which bookings it made.
- **Brand evolution.** Keep player-visible developments on advertiser cards
  and the industry wire. Use dated notices for public strategy changes and
  show their consequences in offers. Keep hidden hype and strategy internal.
- **Viewing habits and lead-ins.** Put audience evidence in Reception and
  the Programming overlay. Use a short note or a link between adjacent
  blocks when the station has evidence of carryover.
- **Newsroom and talent contracts.** Open the newsroom from Preparation or
  Production. Use a shared edition board for reporting, studio and airtime,
  with standing assignments and future exceptions. Use signed contract
  sheets for fees, dates and role obligations, linked to the Character file.
- **Save/load.** Place it in the station menu when persistence arrives.

A widget is ready for review when the player can identify the current job,
the evidence available, the next action, and what committing it will change.
Check the resting room, focused workspace and a consequential result at
desktop and narrow widths, including keyboard, touch and reduced motion.
