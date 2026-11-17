# Screens and interaction design

Status: proposed screen organisation for mockups, based on agreed game direction.
[Visual concept mockups](mockups/README.md) cover station overview, programme
planning and a production editorial profile. These are design references; no
production UI is implemented by this document. The existing
[browser harness](browser-foundation.md) is an integration test page; its two
daily slots and forms are not constraints on the game design.

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
