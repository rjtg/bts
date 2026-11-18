# Game design

Status: agreed direction unless explicitly marked proposal or open. This is a design baseline, not an implementation specification. See [simulation](simulation.md), [first playable](first-playable.md), and [open questions](open-questions.md).

## Premise and focus

Behind the Screens is a television station management game about interpreting a changing audience and turning that understanding into successful television.

The player acquires or produces programmes, hires and books people, schedules broadcasts, sells advertising, commissions research, and responds to results. Original production is a major source of strategic depth.

Core loop: observe → acquire or produce → schedule and sell advertising → broadcast → interpret results.

The world supplies viewers, talent, guests, subjects, competitors, and advertisers. Simulate external society only far enough to create or resolve meaningful television decisions. The world continues without the player; the player's station can influence it without being its only cause.

Rust is the development direction, with the browser as the intended primary player interface through WebAssembly. The CLI remains a developer tool. A minimal TypeScript/HTML/CSS harness is implemented; a polished 80s retro / 8-bit presentation remains later work. No game engine is selected. See [browser foundation](browser-foundation.md).

## Goals and scenarios

Running a station is the constant activity; victory conditions vary by scenario. Sandbox play has no mandatory victory condition. There is no universal requirement to maximise total ratings.

A scenario specifies starting conditions, objectives, deadlines, constraints, failure conditions, world developments, and progress feedback. Objectives use normal simulation outcomes rather than secretly forcing them.

Later scenario examples:

| Ambition | Supporting systems |
| --- | --- |
| Save a failing station | Cash flow, commitments, advertising, scheduling |
| Win major television awards in consecutive years | Production, eligible entries, competing programmes, award assessment |
| Help a party win an election four years away | Public figures, issue preferences, turnout, campaigning, media effects |
| Discover and establish new stars | Talent development, original productions, reputation |
| Build loyalty across a generation | Cohort history, ageing, station and series attachment |

Award counts, time limits, board reviews, and particular award names remain proposals. The initial milestone is a smaller survival-and-production scenario.

## Entity vocabulary

These are domain concepts, not a commitment to an inheritance hierarchy or database schema.

| Entity | Meaning | Important state |
| --- | --- | --- |
| Audience group | A population sharing circumstances and some cultural history | Population, age distribution, disposable income, interests, preferences, availability, loyalties |
| Person | Talent, hosts, journalists, politicians, and other public figures | Skills, styles, roles, affiliations, reputation, appeal, commitments |
| Organisation | Brands, parties, unions, communities, companies, broadcasters | Goals, resources, standing, affiliations, audience relationships |
| Programme | A broadcastable work or instalment | Format, topics, participants, creative attributes, execution, rights, history |
| Project | Work toward programmes or an advertising campaign | Concept, resources, people, schedule, progress, demands |

Relationships connect entities. Events record things that happen and their consequences. Contracts and broadcast schedules explicitly track obligations; they need not become generic world entities.

Brands are organisations with brand-specific state and behaviour. An advertisement is a creative asset used by a campaign, not an ordinary programme that audiences independently choose to watch. A bounded headless prototype now gives standalone works and sequenced episodes explicit programme identities linked to series definitions. Series attachment, work familiarity and actual broadcast history are distinct; continuity mechanics are provisional. Broader format, rights and asset storage remains open. See [episode model](episodes-and-commitments.md).

### Programme library, licensing and resale

The station should begin with a back catalogue of older films and shows that fit
the setting. New cinema releases enter the acquisition market on a regular weekly
or monthly cadence after their theatrical window. Public reception, box office and
general reputation provide baseline information; research can improve estimates
without revealing exact future performance. See [Insight and research](resource-economies-idea.md#insight-and-market-research).

Acquisition is a direct purchase at a price informed by expected airing success,
title fit and solvent competitors' willingness and ability to pay. This is not an
auction: competitors affect the price and may acquire a title if the player passes
before a deadline, but the player does not bid against them. Once acquired, rights
remain with the station indefinitely. Owners can list a title for resale; interested
solvent competitors may make offers, which the owner can accept or decline. Without
an offer, the work stays in the owner's library. Licensing, releases, prices,
competitor decisions and resale are design proposals; the browser fixture currently
has fixed disjoint libraries and no acquisition market.

## Audience groups

Groups are evolving cohorts, not fixed demographic slots. Multiple teenage groups can coexist and grow into different adult groups while retaining some tastes, memories, and associations.

Track population and age distribution; income and price sensitivity; interests and spending relevance; viewing availability; genre, theme, tone, and novelty preferences; quality expectations; advertising tolerance; social cohesion; and station, series, talent, or organisation attachments.

Preferences vary within a group. Cohesion describes social transmission, not complete agreement. Age influences circumstances but does not fully determine taste.

Life stage, cohort history, and current cultural climate contribute separately. Groups can split, merge, and emerge through population processes. Splitting conserves population and transfers relevant history and relationships. Births, deaths, and migration, if implemented, are explicit demographic changes rather than unexplained growth.

## People and production talent

People can have multiple roles: acting, directing, screenwriting, producing, composing, presenting, journalism, politics, and others as gameplay requires. Public figure is a status, not a separate kind of person.

Separate craft skill, task specialisation, genre experience, creative style, working traits, relationships, career state, and public perception. Ability, project fit, recognition, fashionability, and reputation are distinct.

Skills usually change slowly; public attention can change quickly. Shared work develops familiarity and relationships. New entrants, role changes, discovery, rising fees, breaks, and retirement support a changing talent market.

### Finding, signing and assigning people

A talent market and explicit contracts should connect hiring decisions to ongoing
obligations. Keep market availability, the contract and individual production
assignments separate. Hosts/anchors may seek guaranteed runs, while actors may
prefer a film role or specified series episodes; these are negotiated preferences,
not fixed profession rules. See [talent and contracts](talent-and-contracts.md) for
proposed terms, workflows and unresolved mechanics.

Scripted roles have identities separate from their performers. Scripts require
roles in particular episodes; recurring casting should preserve continuity.
Rewriting may change those requirements through paid work, but does not silently
cancel talent contracts. Multi-episode contracts, role continuity, rewriting and
the talent-market UI remain unimplemented.

## Programme formats and subject matter

Agreed direction: programmes combine format, subject matter, tone, editorial
profile and production choices. These are not exclusive genre buckets: a
programme can cover news through comedy, interviews or theatrical debate.

| Format | Distinctive production choices |
| --- | --- |
| Scripted series / film | Concept, writing, casting, direction, production |
| Game show | Rules, host, contestants, prizes |
| Interview / discussion | Guests, topics, host/moderator, preparation, approach |
| Presented reports / magazine | Journalists, host, editorial profile, coverage resources |
| Monologue / topical comedy | Host, writers, subjects, guests, satirical style |
| Music / variety | Performers, host, running order, presentation |
| Live coverage | Event access or rights, commentary, production capability |

All share production resources, scheduling, promotion, audience response,
reputation and advertising, with role/format-specific recipes. Production
projects consume time and resources; outputs may be aired, repeated, licensed
or continued. Not every format requires scripts, pilots or the same stages.

### Ambition and staged delivery

See the [production ambition draft](production-ambition-draft.md) for a proposed
workflow in which each stage attempts explicit creative ambitions using skilled
people and paid time. Results can fall short gradually; reviews allow revision
or extension. Finished material carries strengths, flaws and downstream demands.
Creative ambition and difficulty of later production are separate concerns.
This is a draft direction, not implemented behaviour.

### News and topical entertainment

Produce a news format like any other show: choose host/team, budget, duration,
frequency and editorial settings. A few factors set topic mix, political/issue
leaning, restrained versus sensational presentation, and depth. Editions draw
on current events automatically; story-by-story assembly is not the intended
routine. An optional spotlight on a particular event remains a proposal.

Dry current-affairs magazines, serious panels, showbiz-like political discussion
and satirical late-night programmes can all treat news subjects. Talent fit
depends on how material is presented. Repeated choices affect who watches,
station trust/reputation and audience attention or beliefs.

## Station resources and editorial capital

Agreed direction, not implemented: resources have distinct sources, uses and
timescales. See [resource economies](resource-economies-idea.md) for details
and open mechanics.

- **Sway** is an abstract pooled resource earned by granting people's requests:
  guest pairings, casting, realising a screenplay or changing editorial tone.
  Spend it across negotiations and productions to smooth cooperation and help
  chemistry. Cooperation support should be a freely chosen investment with
  diminishing returns that mitigates cooperation deficiencies, never missing
  skill or craft. It is not a favour redeemable only from the person who earned it.
- **Insight** is proposed research capacity earned by spending cash and time on
  polls, researchers or analysis, then spent to improve estimates about audiences,
  acquisitions, role fit or cast chemistry. It reduces uncertainty with diminishing
  returns, never reveals exact hidden truth, and retains dated evidence when public
  news later changes what the player knows. The browser experiment supports seeded
  Insight spending on audience groups. See [audience research](audience-research.md).
- **Glamour** comes from winning awards and naturally decays. Holding it improves
  conditions with talent who value glamour; spending it grants significant
  performance boosts to specific airings. The affected performance dimensions,
  costs and decay rate remain open.
- **Credibility** is hard to earn through consistently truthful, neutral news
  reporting to the best of available knowledge. Stored credibility strengthens
  news effectiveness; depleted reserves weaken it. Spend credibility on clearly
  labelled opinion pieces to amplify their impact and build backing.
- **Backing** is support for an editorial position. The preferred direction is
  to turn it slowly into audience loyalty through sustained service and coverage.
  It does not instantly create viewers or convert automatically into Sway.
  Alignment with particular audiences or causes and conversion rules remain open.

Intended editorial loop: truthful, neutral reporting → credibility → influential
opinion → backing → gradual audience loyalty. Neutral reporting and serving
committed supporters can pull in different directions; maintaining both stocks
should be difficult, not mechanically impossible.

Truthfulness and neutrality are achieved through reporting work, not a slider:
staff judgement, source quality, verification, time, resources, story selection
and framing matter. This principle applies to all production choices: settings
express intent, while people, constraints and execution determine the outcome.
News editions can still draw automatically from events; this does not require
routine story-by-story assembly.

Labelled opinion spends accumulated authority without automatically constituting
dishonesty. False claims or concealed advocacy may cause separate credibility
losses; exact rules remain proposals. Truth, staff knowledge and player evidence
stay distinct: reporting is assessed against what could reasonably be known,
rather than requiring omniscience.

These systems do not expand the current implementation milestone. The [bounded pooled-Sway experiment](pooled-sway.md) now implements two production
requests and station-wide spending, replacing the personal-favour prototype.
Awards and the other resource stocks/editorial loop remain unimplemented.

## Screens and scheduling direction

See [screens and interaction design](screens.md) for the proposed navigation.
There is no separate Newsroom screen: editorial controls belong to programme
details inside the shared production workflow. The world/industry feed is
shared information, distinct from the programmes made from it.

Agreed direction: a 15-minute simulation tick, four updates per hour, rather
than the current isolated broadcast snapshots. Full schedules occupy real
airtime and advance through intervening ticks. Specific processing rules are
still to design; this is not implemented by the browser harness.

Internal plans, published listings and promotion are distinct. Publishing in
advance and promotion can build awareness/anticipation for an airing; regular
slots can build habit. Schedule changes can lose anticipation or trust.
These systems are agreed direction with uncalibrated mechanics, not implemented
features. Do not force all long-term processes to update every tick.

Advertising load is a station choice. In the proposed market the player sells
minutes attached to a particular airing. Those minutes extend its runtime on
the schedule. Total ad load can influence viewer annoyance. See [ad minutes and
advertising load](simulation.md#ad-minutes-and-advertising-load).

## Relationships and news

Use typed, potentially directional connections:

- Collaboration: trust, familiarity, personal affinity, creative compatibility.
- Personal: friendship, romance, separation, rivalry.
- Cultural influence: reach, attention, credibility, aspiration, subject relevance.
- Household: authority, co-viewing, access limits, purchase influence.
- Institutional: membership, affiliation, employment, representation.

Relationship history affects work and can become news. A private romance, a published rumour, and audience belief are separate states. Publicity can increase attention without increasing approval.

Parents can limit children's access; children can influence household purchases. Teen enthusiasm can create younger viewers' curiosity without guaranteeing enjoyment.

## Brands and advertising

Hype is a persistent, decaying stock of attention and desire attached to a brand and audience group. Campaigns create hype through relevant reach and effective creative work.

Brands have product categories, positioning, target audiences, desired hype, current hype, budgets, and commercial health. Targeting combines attributes such as purchasing power, age, interests, and status preference rather than using one rigid demographic rule.

Hype deficits create demand for advertising; budgets and expected effectiveness constrain willingness to pay. Brands can launch, grow, decline, be acquired, or disappear. Hype alone does not determine survival.

Awareness as a slower-moving state alongside hype is a proposal. Audience-specific
delivery commitments are implemented experimentally. Intended direction is a
station-offered ad-minute market: advertisers make non-exclusive offers for part of
the offered block, the player fills or shortens the break, and unsold time may carry
station teasers. Ad minutes extend programme airtime; exact placement, thresholds,
measurement and refunds remain open. The current fixed delivery contract is a
provisional experiment, not this final market. See [advertising mechanics](simulation.md#proposed-ad-minute-market).

## Public world

World events include elections, visits by public figures, international tensions, economic changes, cultural happenings, and consequences of simulated activity.

People belong to parties, unions, and communities; audiences have differentiated agreement, trust, identification, and interest. Elections, when introduced, emerge from eligible populations, turnout, and divided support, then apply explicit electoral rules.

News and interviews connect this world to television. Exposure can inform, persuade, alienate, or merely increase recognition. Detailed government, household, or personal daily-life simulation is outside scope unless it supports a concrete TV decision.
