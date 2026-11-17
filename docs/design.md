# Game design

Status: agreed direction unless explicitly marked proposal or open. This is a design baseline, not an implementation specification. See [simulation](simulation.md), [first playable](first-playable.md), and [open questions](open-questions.md).

## Premise and focus

Behind the Screens is a television station management game about interpreting a changing audience and turning that understanding into successful television.

The player acquires or produces programmes, hires and books people, schedules broadcasts, sells advertising, commissions research, and responds to results. Original production is a major source of strategic depth.

Core loop: observe → acquire or produce → schedule and sell advertising → broadcast → interpret results.

The world supplies viewers, talent, guests, subjects, competitors, and advertisers. Simulate external society only far enough to create or resolve meaningful television decisions. The world continues without the player; the player's station can influence it without being its only cause.

Rust is the development direction. Presentation is secondary: an 80s retro / 8-bit aesthetic, potentially a TUI. No particular engine or terminal framework is selected.

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

## Audience groups

Groups are evolving cohorts, not fixed demographic slots. Multiple teenage groups can coexist and grow into different adult groups while retaining some tastes, memories, and associations.

Track population and age distribution; income and price sensitivity; interests and spending relevance; viewing availability; genre, theme, tone, and novelty preferences; quality expectations; advertising tolerance; social cohesion; and station, series, talent, or organisation attachments.

Preferences vary within a group. Cohesion describes social transmission, not complete agreement. Age influences circumstances but does not fully determine taste.

Life stage, cohort history, and current cultural climate contribute separately. Groups can split, merge, and emerge through population processes. Splitting conserves population and transfers relevant history and relationships. Births, deaths, and migration, if implemented, are explicit demographic changes rather than unexplained growth.

## People and production talent

People can have multiple roles: acting, directing, screenwriting, producing, composing, presenting, journalism, politics, and others as gameplay requires. Public figure is a status, not a separate kind of person.

Separate craft skill, task specialisation, genre experience, creative style, working traits, relationships, career state, and public perception. Ability, project fit, recognition, fashionability, and reputation are distinct.

Skills usually change slowly; public attention can change quickly. Shared work develops familiarity and relationships. New entrants, role changes, discovery, rising fees, breaks, and retirement support a changing talent market.

## Programme formats

| Format | Distinctive production choices |
| --- | --- |
| Scripted series / film | Concept, writing, casting, direction, production |
| Game show | Rules, host, contestants, prizes |
| Interview / talk | Guests, subjects, preparation, host, editorial approach |
| News / current affairs | Stories, sources, journalists, coverage, credibility |
| Music / variety | Performers, host, running order, presentation |
| Live coverage | Event access or rights, commentary, production capability |

All share scheduling, promotion, audience choice, satisfaction, reputation, and advertising. Each uses an appropriate production recipe rather than being treated as a reskinned scripted series.

A production project consumes time and resources. Its result can be broadcast, repeated, licensed, or continued. Development, pilots, revision, renewal, and cancellation offer staged commitments; not every format needs every stage.

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

Awareness as a slower-moving state alongside hype is a proposal. Contract structure and performance attribution remain open; see [simulation](simulation.md).

## Public world

World events include elections, visits by public figures, international tensions, economic changes, cultural happenings, and consequences of simulated activity.

People belong to parties, unions, and communities; audiences have differentiated agreement, trust, identification, and interest. Elections, when introduced, emerge from eligible populations, turnout, and divided support, then apply explicit electoral rules.

News and interviews connect this world to television. Exposure can inform, persuade, alienate, or merely increase recognition. Detailed government, household, or personal daily-life simulation is outside scope unless it supports a concrete TV decision.
