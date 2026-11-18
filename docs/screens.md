# Screens and interaction design

Status: broader screen organisation is proposed; the bounded programme-planning
and standalone discussion-production subsets linked below are implemented.
[Visual concept mockups](mockups/README.md) cover station overview, programme
planning and a production editorial profile. These are design references. The [original-production desk](original-production.md)
implements one format with cast/creative choices, timed booking, budget and pooled Sway. The [programme-planning workspace](programme-planning.md) now implements a bounded
subset: seven full days of quarter-hour planning, explicit runtimes, competitor
lane, catalogue, contextual contracts/results and station finances. Other screens
and publication/promotion/ad-share controls below remain proposals.

## Main workspaces

| Screen | Main decisions and information |
| --- | --- |
| Station overview | Current broadcasts, cash, urgent obligations, production progress, scenario goals and developments |
| Programme planning | Full-day/weekly schedule, programme durations, competitor listings, publication, advertising load and block allocation |
| Programme library & market | Owned works, available rights, series/episodes, repeats, availability and broadcast history |
| Production | Create formats/projects, assemble teams, allocate resources, manage editions/episodes and renew/cancel |
| People & organisations | Find talent and guests; inspect known skills, reputation, relationships and affiliations |
| Advertising | Compare client offers, accept commitments, monitor delivery and settlement |
| Audience & research | Read measured ratings, compare researched groups, commission studies, interpret trends |
| Station management | Finances, staff, studios, capacity and upgrades |

This is a navigation proposal, not a requirement to implement eight destinations
at once. Names, grouping and information density must be tested in mockups.

## Shared navigation and detail panels

Keep time, pause/speed/advance controls, cash, urgent notifications and access to
scenario objectives available throughout. A shared world/industry feed surfaces
events, gossip, reviews and announcements. Link items to relevant programmes,
people or organisations; the feed is not a separate newsroom management system.

Programme, person, brand, audience and contract details open in contextual panels
or equivalent linked views, retaining the originating schedule or task.
Prefer a shared production-detail structure with format-specific controls.

## News is produced television

There is no separate Newsroom destination. A news bulletin, current-affairs
magazine, political panel or topical late-night show is created and managed
through Production, then scheduled through Programme planning.

Format, subject matter, tone and editorial leaning are separate dimensions.
News and entertainment can coexist in one programme. Examples:

| Programme | Format | Subjects and style |
| --- | --- | --- |
| Evening bulletin | Presented reports | Current events; concise and restrained |
| Current-affairs magazine | Reports/interviews | Political and social topics; analytical and detailed |
| Political panel | Moderated discussion | Current affairs; serious debate or theatrical confrontation |
| Topical late-night show | Monologue/interviews/comedy | Current events and celebrities; satirical and personality-led |

The player sets host/team, resources, duration, frequency and an editorial
profile. A few controls describe topic priorities, political/issue leaning,
presentation and depth. The simulation selects and frames current events for
each edition. Do not require story-by-story bulletin assembly. Optional
spotlighting of a specific event is a proposal, not a required recurring chore.

Different roles and production demands matter within the shared workflow.
Editorial choices can change audience composition, trust and reputation over
time, and influence attention to public figures and topics.

## Programme planning mockup brief

Start with programme planning, overview and programme details. Produce mockups
before extending the current harness into a full interface.

Planning should show a full-day timeline at 15-minute resolution, with works
spanning their durations. Day/week views, competitors in parallel lanes, a
catalogue and a contextual broadcast panel are proposals to compare visually.

Clearly distinguish internal draft plans, published listings and aired history.
Show which broadcast an offer/action affects; selecting a future broadcast must
not silently book or advance a different one. Ad-block controls show total
duration, campaign shares, remaining capacity and commitments.

The browser is primary, with an 80s television control-room / 8-bit aesthetic.
Readability, clear hierarchy and understandable decisions take precedence over
literal terminal limitations. Use illustrative content unconstrained by the
current fixture counts. Label mockup estimates and unknowns; do not present
hidden simulation truth as normal player information.


## Implemented planning subset

The [planning milestone](programme-planning.md) follows the concept's timeline,
parallel lane, library and contextual detail structure. Selection is independent
of the Rust-calculated next airing. Stable airing IDs bind moves/removal,
quotes, protected bookings, results and ledger entries. Empty airtime is explicit;
advancement processes all ticks with a documented provisional start-snapshot cadence.
All plans remain internal drafts; publication, promotion, campaign shares,
research and a separate weekly view are deferred; the bounded production desk is now implemented. The mockup's values
and inconsistent runtimes are not imported as simulation rules.

## Player-facing interaction direction

Agreed direction: the browser should feel like running a television station,
with direct manipulation and readable decisions. Changing colours alone does
not turn a simulation inspector into a game interface. The current harness still
uses forms, selects and diagnostic tables; the interactions below are proposed,
not implemented. Preserve Rust-owned validation, accounting and player observations.

### Planning and time

Make the schedule the main workspace. Programme cards show artwork, title and
runtime. Drag a card onto the timeline to place it; drag an existing broadcast
to move it. Snap to 15-minute boundaries, preview occupied airtime and show
conflicts before commitment. Rust remains authoritative: a preview is not a
successful booking. Protected advertising commitments still constrain edits.

Provide click-to-place and accessible keyboard/touch alternatives; precision
mouse dragging must not be the only way to plan. Invalid moves should leave the
broadcast in its previous position and explain what prevents the change.

Clicking a broadcast opens a compact contextual side panel, retaining the
schedule. Move contracts, production and financial review into focused views
rather than one long page of forms. A persistent **On air / Up next** strip shows
the clock, current programme and a prominent **Run until…** action. Selection
must remain distinct from the next broadcast that will air. Preview the time
advance destination and relevant obligations; do not accidentally skip a decision.

### Production and people

Use a casting desk: host and guest portraits occupy visible seats; clicking an
empty seat opens a roster of person cards. Display specialties, fees and known
availability directly. For scripted work, use named roles rather than generic
seats, following [talent and contracts](talent-and-contracts.md).

Prefer subject tiles, buttons or chips for discrete choices. Sliders suit gradual
choices such as restrained-to-lively tone. Budget presets such as **Shoestring /
Standard / Lavish** can reduce form filling, while always displaying the exact
price and what it commits. Preset names and effects need testing; they are not
new balance rules. Avoid replacing every dropdown with a slider indiscriminately.

Star-style ratings can represent public reputation or an explicitly labelled
staff assessment. Show unknowns and the source of an assessment. Do not convert
hidden craft/fit into exact stars, or imply that stars guarantee future results.
Keep exact money and contractual terms readable wherever a decision needs them.

Present Sway requests as opportunities: person portrait, short request, visible
conditions/reward and progress such as **Promised → Recorded → Completed** where
applicable. Use each request's actual fulfilment milestone; not every recording
earns a reward. Spending should be a clear choice during booking, with the cost
and consequence visible before confirmation.

### Results and language

Lead a broadcast recap with viewers, earnings and contract outcome. Put detailed
tables behind **Details**, with causal explanations limited to legitimate player
evidence. Current expected counts remain labelled provisional, not measured ratings.
Do not invent research, staff knowledge or precise satisfaction reports to fill a panel.

Use ordinary station language: **Bring Jules in — 2 Sway**, **Help the team work
together**, **Budget committed**. Remove protocol status, internal IDs, delivery
micro-units and repeated model explanations from the main player flow. Keep
stable IDs internally and retain accessible exact contract details where needed.
A separately enabled developer view may expose technical diagnostics; player
usability must not depend on reading them.

### Suggested UI slice

A drag-and-drop evening planner, compact broadcast panel and readable recap would
remove frequent friction while retaining existing Rust mechanics. Test placing,
moving, rejected overlaps, protected bookings, time advancement and review through
both direct manipulation and a non-drag route. Follow with the casting-desk and
talent-contract views as their mechanics become available. This is a proposed
sequence; it does not implement new simulation systems or choose a UI framework.
