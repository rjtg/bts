# Screens and interaction design

The proposed [Control Desk redesign](mockups/control-desk/README.md) links the browser journey audit, directions, mockups and component guide.

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

The future library/market should combine a persistent station catalogue with a
dated acquisition feed. It starts with older setting-appropriate titles; new
cinema releases enter after theatrical runs. Show public reception and box office,
an imperfect airing-success estimate, asking price and acquisition deadline. Listed
owned titles remain in the library while offered for resale; incoming competitor
offers are reviewable obligations/actions, not auction bids. See [programme rights
and acquisition](design.md#programme-library-licensing-and-resale).

The browser now includes an Audience panel for dated estimates, a shared survey schedule and
Insight spending. See [audience research](audience-research.md).

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

Production type and format establish the applicable feature vocabulary; subject
and the achieved profile shape the actual programme. Nonfiction can combine
reporting rigour with entertainment features, so news and comedy can coexist in
one programme. Examples:

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
not silently book or advance a different one. The proposed ad-market panel lets
the player offer a chosen number of minutes for a selected airing/context, review
non-exclusive brand offers (minutes and total price), accept a mix up to capacity,
or leave time unsold/shorten the break/use station teasers. It must show that ad
minutes extend airtime. Break placement and refunds/thresholds remain deferred.

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
All plans remain internal drafts. See the [ad-minute market](ad-minute-market.md) for selling time from the selected airing. Publication, promotion and a separate weekly view are deferred; the bounded production desk is now implemented. The mockup's values
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

For recurring placement, preview dates, occupied time, exceptions and conflicts.
Let players edit the current occurrence or future occurrences explicitly. Show
autosell policy and its sales on the selected airing. Audience dossiers can
summarize estimated weekday and weekend viewing patterns. See the
[weekly scheduling proposal](weekly-scheduling.md).

The Control Desk planner now implements drag and drop for draft airings and
library titles, a non-drag slot picker, a narrow agenda and weekly review. Its
ghost and conflict hint are provisional. A drop or form submission still runs
through Rust, and the selected airing remains distinct from Up Next.
See [weekly scheduling](weekly-scheduling.md).

### Production and people
The [Film desk](original-film.md) provides project cards, script delivery links,
portrait casting and a booking review for standalone fiction.


Use a casting desk: host and guest portraits occupy visible seats; clicking an
empty seat opens a roster of person cards. Display specialties, fees and known
availability directly. For scripted work, use named roles rather than generic
seats, following [talent and contracts](talent-and-contracts.md).

Prefer subject tiles, buttons or chips for discrete choices. Sliders suit gradual
choices such as the current discussion prototype's calm-to-lively tone. Future
ambition controls should use the concrete feature vocabulary in
[game design](design.md#programme-formats-and-subject-matter), not treat that
prototype tone value as a universal content axis. Budget presets such as **Shoestring /
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
and consequence visible before confirmation. Production cooperation support
should use a slider/dial allocating any available amount of Sway, including zero,
rather than a fixed checkbox. Show the allocated amount and remaining balance.
Diminishing returns mitigate cooperation deficiencies, not skill shortages; do
not imply guaranteed creative success or reveal hidden chemistry coefficients.
The fixed-cost checkbox has now been replaced by the bounded casting desk; see [resource direction](resource-economies-idea.md#adjustable-production-support).

### Results and language

Lead a broadcast recap with viewers, earnings and contract outcome. Show the
airing's cash and other resource costs/returns, then link to cumulative investment
and returns for its show. A cash loss may still be worthwhile when the show earns
needed Sway, Insight or another distinct resource. Do not assign a universal
failure score: audiences, advertisers and the station have different expectations
and consequences. Show the expectation recorded for the decision, outcome and
uncertainty; contract refunds still follow their explicit terms. Put detailed
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


## Implemented casting subset

The [casting desk](casting-desk.md) now implements three portrait seats, a modal
eligible-person picker with commitment guidance, subject tiles, a three-position
tone slider, exact budget presets, adjustable Sway range/number inputs and a
Rust-calculated booking review before confirmation. Accepted timed quests have
an open list and production hints; there is no promise selector on a production.
Other screen directions above, including drag-and-drop scheduling and talent-market
contracts, remain proposals. This does not claim the full game interface is complete.

## Playful feedback and visible passage of time

Presentation direction: make station life feel active through meaningful
notifications and ambient changes. The dashed current-time marker is implemented;
notifications and ambient scenery remain future work.

- Celebrate completed productions with a characterful notification and a direct
  action such as **View programme** or **Schedule it**. Later systems can announce
  a finished script or a new prodigy arriving on the talent market. Those systems
  are not implemented yet; do not generate fictional progress notifications for them.
- Distinguish celebrations, useful opportunities and urgent obligations. Keep a
  readable notification history and avoid one popup per simulation tick. Events
  should refer to the relevant project/person/contract and announce each achievement
  once, using actual Rust events or state transitions rather than UI timers.
- Mark current simulated time on the schedule with a dashed vertical line crossing
  the station and competitor lanes. Include a readable time label and an accessible
  equivalent. When viewing another day, make that distinction clear rather than
  drawing today's clock time as if it were current on that day.
- Let background lighting reflect the simulated day: sunrise, daylight, sunset and
  night can give a playful sense of elapsed time. Follow the game's explicit clock,
  not the user's wall clock; pausing must not silently advance world time. Prefer
  restrained changes that retain text contrast, with reduced-motion support.
- Over time, the station office could look more luxurious or better connected as
  cash, Sway or Glamour grow. This is an ambient presentation proposal, not a new
  conversion rate, common resource bonus or guarantee of success. Different resources
  may need different visual cues; their gameplay meanings remain distinct.

Office changes must use legitimate player-known information. Decide whether cash
presentation follows spendable cash, longer-term prosperity or another explicit
measure; commitments must remain understandable. Glamour-driven changes wait for
an implemented Glamour system. Decoration must not conceal exact financial or
contractual information. Consider stable visual tiers so one small transaction
does not repeatedly rebuild the room, but thresholds and art direction remain open.

Open: notification placement and dismissal/history controls, whether important
milestones interrupt a long advance, animation/audio preferences, dawn/dusk art,
resource-to-office cues and the timing of visual upgrades. Avoid a general effects
framework or a separate decoration economy solely for these presentation ideas.

Implemented follow-up: browser Play/Pause/×2 now drive explicit ticks, a dashed
NOW marker crosses the schedule, and a shared icon/tooltip HUD groups station
resources. Future stocks remain visibly unavailable. Daylight, playful milestone
notifications and richer office appearance above are still proposals. See
[casting/pacing](casting-desk.md#browser-pacing-resource-hud-and-reusable-components).
