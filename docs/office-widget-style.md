# Station Office widget guide

This guide records the visual direction for current and future player actions.
The [Station Office](station-office.md#blotter-layout-and-prop-transitions)
describes the implemented room. This file is the component and interaction
reference. See [design guidelines](design-guidelines.md) for the broader rationale.

## The office is the home screen

- Keep the green blotter clear for the current job. At rest, give it the
  closed notebook and a pencil. Cluster other props naturally around the
  edges, with distinct silhouettes, warm contact shadows and a shared
  perspective. Keep the shelf stocked and the room visible during work.
- Let an object suggest what it opens. On hover or keyboard focus, give it
  a small physical reaction and a short label. On activation, bring the prop
  to the blotter and open its workspace. The desk calendar opens Programming.
  The wall monitors and the people and equipment behind the glass provide
  atmosphere.
- Use desk-sized spreads for a single decision and near-full-screen overlays
  for timelines, casting, and other work that needs room. Only the focused
  workspace takes input. Closing it returns focus to the entry prop and
  preserves the draft, selection and scroll position. The desk directory
  provides the same routes by keyboard and touch.
- Keep the slim Today strip and physical transport buttons in reach. The
  strip is an at-a-glance channel readout with tape drops and airing files.
  Use the calendar workspace for moving blocks, comparing channels and
  planning across dates. Manual pause brings up the OFF AIR card with a
  sharp Continue control.

## Interaction model: prop, choice, work surface

The office should connect game actions through a consistent sequence:

1. **Choose a prop or an object already on the desk.** The prop establishes
   the context: the shelf means owned works, the calendar means airings and
   time, and the script file means preparation.
2. **Choose a short branch of actions or categories.** A radial menu may
   appear near the activated prop or selected object. It is a navigation and
   action control, not the visual style of the whole interface.
3. **Bring the relevant work surface forward.** Use a tape collection,
   dossier, calendar, project board or ledger according to the task. Do not
   force detailed content into radial sectors.
4. **Preview and commit.** Show the affected title, dates, runtime, price and
   conflicts before a consequential action. Confirmation leaves a visible
   trace such as a scheduled block, signed offer, dated paper or result sheet.

The interaction pattern is intended to remain separate from presentation.
For example, the same category choice could be presented as a radial control
while drawn as a CRT selector, rotary index or physical filing device. Keep
the interface legible and game-like without requiring every control to look
like a literal wheel.

### Where radial menus fit

Use a radial for a short set of context-specific choices, usually no more
than four to six. It should stay near its prop, remain within the viewport,
show a clear Back path in its hub, close on Escape or an outside click or tap,
and support pointer, keyboard and touch. The lid holds the useful choices. A
radial should not contain tiny title labels or replace a timeline, roster,
slider, dossier tabs or other controls better suited to detailed choices.

When a branch contains many works or people, transition to a readable sorted
collection in the same work area. Selecting an item should return to the
originating context with the item selected; it must not silently commit an
action. The context (such as a selected calendar gap) remains visible or is
clearly summarized during this handoff.

### Owned works and scheduling journeys

The shelf and calendar support two equivalent routes. The player may begin
with a work or with a time:

```text
Shelf → Owned works → Fiction / Nonfiction → format → title or series
     → episode or series binder → Tape dossier
     → Schedule → One-shot / Regular (where applicable)
         ├─ One-shot → Next gap / Recommended slot / Choose on planner
         └─ Regular → Recommended run / Choose days and time
             → review preview → confirm

Calendar → choose date and gap → Choose programme
     → Tape Shelf selector → choose work and episode
     → return to the same gap with placement preview
     → adjust ad load or time → confirm
```

“Selected tape” means a work the player has explicitly chosen and whose
identity is carried forward during the current scheduling task. The calendar
must not assume one is selected. In the time-first route, choosing a gap opens
the owned-work selector; after selection, the player returns to that exact gap
with a provisional preview. Nothing is placed until the player confirms.
Likewise, moving an airing means choosing a destination, reviewing the
resulting time and conflicts, then confirming. A rejected placement preserves
the original airing.

The Tape dossier keeps its information tabs. Its Schedule action offers:

- **One-shot:** Next gap, Recommended slot, or Choose on planner.
- **Regular:** Recommended run or Choose days and time; show this only for
  recurring works or formats that support a regular run.

Recommendations are proposals, not automatic bookings. Show the selected
date or pattern, runtime, ad load and occupied interval before confirmation.
For regular runs, preview each occurrence and resolve date-specific conflicts
before the atomic confirmation.

### Other prop and dossier flows

```text
Incoming Portfolio → available-title collection → select listing
    → Market dossier → Investigate / Buy → acquired work enters Tape Shelf

Calendar airing → Airing dossier → ad offer → Accept / Decline
    → signed term or crossed-out offer → after airing, reception and settlement

Preparation prop → project collection → project dossier
    → Brief / Commission script / Review delivery / Revise or accept
    → Production dossier when ready

Preparation prop → Production → active-project board → project dossier
    → choose role or seat → eligible-person roster
    → choose studio and support → review obligations → Book

Research Binder → Audience dossiers
Notebook → station notes and active requests
Ledger → finances and links to related projects or airings
```

The Incoming Portfolio follows the same browse-and-select pattern as the Tape
Shelf, but its selected title opens a Market dossier. It becomes an owned tape
only after purchase succeeds. Research Binder, Notebook and Ledger each open
their single clear workspace directly; they do not need radial menus.

Dossier tabs remain the way to browse information. Contextual actions belong
on the relevant page or selected item: for example, an offer slip in an Airing
dossier offers Accept and Decline, while selecting a cast seat opens an
eligible-person roster. Keep controls matched to the decision: a roster for
casting, a calendar for dates, and an exact-value slider for Sway. Radials may
choose a short task branch, but should not compress those detailed decisions.

The browser shares one film-can component across the Tape Shelf, Tape
scheduling, gap selection and the Ideas, Scripts and Production branches.
It opens beside the trigger and clamps to the viewport. Arrow keys move
between choices and Enter activates them. Browse all tapes and the planner’s
direct shelf controls provide shorter routes.

The placement slip supports editing the start minute and ad minutes. A
recommended weekly run prefills the weekday and time from the survey-based
suggested slot. Its dated occurrences still require the regular Rust preview
and confirmation. Market titles open a selected file from the portfolio
index. Research, Notebook and Ledger open directly.

Open design questions include how the wheel scales to more formats, whether
to add a separate series binder entry from the shelf, and how to make
programme recommendations broader than a particular date and time. The
[route audit and browser replay](../contrib/office-interactions/README.md)
describe the current treatment.

## Shared visual language

| Element | Treatment |
| --- | --- |
| Furniture and props | Warm late-1980s pixel art. Register separate sprite layers to the room and use perspective on furniture and covers. |
| Working surfaces | Near face-on paper, tape labels, CRTs and device controls. Keep text, charts and interactions live in HTML or SVG. |
| Typography | Typewriter-style briefs and typed labels. Reserve handwriting for short recommendations, crossed-off requests and marginal notes. Use a clear numeral treatment for times, dates and Funds. |
| Dossiers | Two separate leaves with a consistent footprint, labelled edge tabs and folded page corners. Prefer turning the page. Keep a leaf’s scroll local when there is no suitable break. Tape taupe, Airing teal, Preparation pink, Production blue, Character yellow. Name each type on the folder. |
| Controls | Use portrait seats for casting, tabs and tiles for choices, a dial for bounded spending, and buttons labelled with the action. Show exact amounts beside dials and give them a keyboard-operable exact entry. |
| Evidence | Pencil-style radar charts suit a profile. Use timelines for obligations, bars for airtime and ad minutes, and aligned figures in the ledger. Keep observed results, dated estimates and unknowns visibly distinct. |
| Motion and feedback | Stagger tiny ambient gestures independently of game time. Use a stamp, signed line, moved tape or dated notice to confirm consequential changes. Give blocked actions a concrete reason beside the control. Under reduced motion, keep the labels and final poses and skip travel and idle animation. |

## Which widget fits each action

| Player task | Entry and widget | What the player can decide or see |
| --- | --- | --- |
| Find and place a programme | **Shelf → tape tray and taupe file.** Box episodes together and give cassettes readable spine labels. | Filter by runtime or production type. Pull one tape and browse Brief, Reception, Members and Bottom Line. Use Next Gap or the planning controls. Show title and runtime on the tape itself. |
| Arrange a day or weekly run | **Desk calendar → Programming calendar.** Use a CRT for Day and calendar paper for Week and Month. Give regular bookings their own file. | Move or place at exact minutes, compare an available rival schedule, and preview recurring dates and clashes before confirming. Keep the selected airing and Up Next separate. |
| Sell ads or inspect a broadcast | **Airing block or notebook reminder → teal Airing file.** Use an inventory strip and offer slips. | Set ad load and see occupied airtime, sign an offer, or leave space unsold. After air, show dated outlook, provisional turnout, settlement and airing cost in the same file. |
| Develop an idea | **Script file under the lamp → Ideas → Idea dossier.** Keep the ambition stars on one leaf and the title, description and pace on the other. | Set the project's creative targets and choose how long to develop it. When ready, pass the brief to the writing desk. |
| Commission or revise | **Script file under the lamp → pink Preparation spread.** Put the brief and script notes side by side. | Compare writer cards, fees and delivery dates. On return, read what the script achieved, what it demands, and what a paid revision would address. |
| Cast and book | **Preparation → Production → blue Production file.** Place portraits in the role seats and use the studio and time fields on the booking paper. | Compare known strengths and availability, select a studio, set cooperation support where applicable, and review obligations before confirming on the booking paper. |
| Inspect a person | **Known credit or portrait → yellow Character file.** Keep the portrait, public work and known commitments together. | Return to the same project or tape on close. Future auditions and contracts can open from this file with explicit costs and obligations. |
| Buy or sell rights | **Incoming portfolio → illustrated title cards and assessment sheet.** | Compare asking prices, deadlines and public evidence. Request an assessment, buy a work or review a resale offer. Link acquisitions to the tape shelf. |
| Research an audience | **Research binder → illustrated group dossiers.** Use half-star preference marks, a dated survey and an Investigate dial. | Adjust the shared polling schedule or spend Insight on one group. Keep the published snapshot in place until the next poll. |
| Check priorities and requests | **Notebook on the blotter → open handwritten spread.** | See the next gap, unsold ad reminders, and Sway requests with deadlines and crossed-off completions. Open the relevant airing or job directly. Keep station goals here. |
| Understand the money | **Ledger and review sheets → station books and broadcast recap.** | Scan cash and commitments or open a specific airing. Distinguish each airing's costs and receipts from programme investment and lifetime totals. |

### Assigning people

Writers, actors, directors, discussion hosts and guests, and newsroom staff use
the same photo spot. Selecting a spot opens a scrollable stack of clipped
index cards alongside the file, or along the bottom edge on touch layouts.
Picking a card fills the spot and returns focus to it. New film roles start
empty. Established series casts and the discussion and newsroom templates
display their existing assignments. The film booking and writing commands
remain separate from the draft selection.

Pencilled stars represent authored public clippings for particular qualities.
Unrated qualities stay unknown, and people without portrait artwork show a
photo placeholder. The Rust workflow still validates timing and commitments.

Studio choices use the same card stack with distinct photos, prices and
known reservations. Film booking updates the cost on the call sheet and uses
one confirmation on the booking leaf. Date and time fields open a small calendar and a split-flap
clock. Repeating slots show weekdays, and time-only fields show the clock.
Airing choices step by the minute. Film and discussion recording choices
step by the quarter hour. Applying a choice fills the field for review.
See the [booking desk mockups](mockups/booking-desk/README.md) for the proposed
physical calendar, clock, canister and photo treatment.

## Homes for the season mechanics

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
- **Save/load.** The station directory’s Station files holds local download
  and restore.

A widget is ready for review when the player can identify the current job,
the evidence available, the next action, and what committing it will change.
Check the resting room, focused workspace and a consequential result at
desktop and narrow widths, including keyboard, touch and reduced motion.

## Radial mockup review

The radial design package compares sectors and a rotary dial over the same
room. It includes a clickable scheduling walkthrough, icon states and a
narrow touch layout. [Open the radial review](mockups/office-radials/README.md).

The playable radial menus share the small enamel film can. Its lid shows
the contextual icon and the current choices. A paper sticker on the front
wall names the branch. Tape browsing, choosing a programme for a calendar
gap, scheduling a tape, and choosing a production stage all use the can.
Named production projects move to a wooden file box with one folder per
show. See the [Production dossier](production-dossier.md) for the project
journey and the remaining design decisions.
