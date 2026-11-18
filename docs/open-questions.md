# Open questions and experiments

Status: unresolved unless explicitly stated. Proposed defaults are starting points, not approved permanent mechanics.

## Decisions needed before the first playable

| Question | Proposed starting point | How to decide |
| --- | --- | --- |
| Player control | Combined broadcaster and commissioning studio, with selected creative decisions | Test whether production and scheduling form one manageable loop |
| Tick processing | Duration-aware 15-minute planner implemented with provisional start-snapshot viewing | Validate cadence and exposure incentives; see [planning semantics](programme-planning.md) |
| World setting | Fictional world with an 80s identity | Choose before writing substantial political or historical content |
| Format depth | Shared production; news is subject matter across formats, with editorial-profile controls | Choose the first producible formats; avoid separate newsroom mechanics |
| Advertising payment | Provisional delivery contracts implemented with deadline, escrow, capped offers and proportional underdelivery refunds; hype affects demand | Compare clarity, incentives, costs and exploit risks; see [economy experiment](advertising-economy.md). Permanent contract design remains open |
| Hype versus awareness | Separate fast hype and slower awareness if justified | Test whether one variable makes established brands implausibly vanish |
| Audience choice | Deterministic expected-share allocation with not-watching implemented as a provisional headless experiment; later stochastic choice remains open | Check accounting, smoothness, and diversity of outcomes; see [experiment](headless-experiment.md) |
| Fuzzy semantics | Zero-order Sugeno, min/max/complement, explicit fallback, validation, traces and macros implemented | TV rule calibration remains experimental; see [experiment](headless-experiment.md) |
| Browser UI development | Browser primary; Rust/Wasm adapter and plain TypeScript/HTML/CSS day-planning workspace implemented | Validate schedule/contract/report usability before selecting a larger UI framework or polished retro presentation; see [browser foundation](browser-foundation.md) |
| Rust architecture | Cargo workspace; independent `bts-fuzzy` crate; simulation independent of interface | See [architecture](architecture.md); select remaining crates and persistence when needed |

## Newly agreed directions needing detailed mechanics

- [Screen organisation](screens.md): eight proposed workspaces and contextual
  details; compare mockups before building the full UI.
- [Player-facing interactions](screens.md#player-facing-interaction-direction): direct
  schedule manipulation, focused panels, casting cards, selective sliders/stars
  and readable recaps; preserve accessible alternatives and hidden information.
- [Playful feedback](screens.md#playful-feedback-and-visible-passage-of-time): milestone
  notifications, a dashed current-time line, simulated daylight and later
  resource-sensitive office visuals; thresholds and interruption policies remain open.
- Advance publication, airing-specific anticipation and recurring-slot habits:
  choose buildup, delays, rescheduling consequences and readable evidence.
- [Ad-minute market](simulation.md#proposed-ad-minute-market): tune how limited
  station-offered minutes and non-exclusive brand offers set per-minute price;
  decide how advertisers' exposure needs and unsold/self-promotional time work.
  Delivery thresholds, measurement and refunds are explicitly deferred.
- Measurement: if delivery-conditioned offers are added later, specify sampling,
  bias, delay, forecasts and the measurement basis; current prototype settlement
  still uses internal expected delivery.
- Keep timing simple: due ticks plus selected state updates; no general effects
  framework is required.
- [Production logistics](production-logistics.md): prototype always-available studio
  rental with overlap-weighted demand pricing, script requirements for a few studio
  quality dimensions, and a bounded quality star selector. Test price monotonicity,
  integer settlement, affordability and whether the choice is legible before adding
  ownership markets, recurring batches or prop inventories.
- [Show identity and staged production](production-ambition-draft.md): separate a
  show's ambition/run shape from preparation and each produced instalment. Decide
  how one-shot, finite and open-ended runs link to sequels, and how prepared work's
  freshness varies between news, discussion editions and scripted works.
- Procedural episode ambition: model deviation from the show's promise, local
  episode-to-episode swings, cumulative strain and recovery separately. Test an
  isolated dark/action episode, repeated extremes, a return to baseline and a
  gradual planned arc resolved by a finale before choosing adaptation/recovery
  rates.
- [Insight and research](resource-economies-idea.md#insight-and-market-research):
  decide how paid studies and investigative preparation generate private station
  knowledge independently of airing, how spending targets one estimate, how
  reports age, and how diminishing returns preserve uncertainty across audience
  forecasts, acquisitions, role fit and cast chemistry.
- Research cadence: audience groups use paid automatic polls up to daily; talent
  and pair relationships use optional decision-triggered auditions, with Insight
  allocated immediately beforehand. Blind casting remains possible. Define shared
  familiarity/noise and persistent-bias drift curves, how working together changes
  familiarity, and how public disclosures update current knowledge without
  rewriting old evidence.
- Ad-market information: use each audience group's latest published poll and
  uncertainty for offer-time reach/value estimates; the browser experiment uses published point estimates. How advertisers value
  age and uncertainty directly remains open.
- [Programme library and acquisition](design.md#programme-library-licensing-and-resale):
  determine release cadence, price formation from expected performance and solvent
  competitor demand, acquisition deadlines, and resale offers under perpetual
  ownership rights.
- Expectations and airing outcomes: keep prior estimates, contract delivery,
  audience response and station resource returns distinct. Define how estimates
  are recorded and updated, and how reports show per-airing versus cumulative show
  costs/returns without a universal failure score.

## Later design questions

### Audience evolution

How many groups remain understandable? What triggers a split or merge? Which history and relationships transfer? How do births, ageing, and exits work? How long must scenarios run for cohorts to matter?

Experiment: run multi-year headless worlds and inspect population conservation, group count, cultural diversity, and inherited attachments.

### Influence and households

How much household detail is necessary for parental control, co-viewing, and purchasing? How do aspiration and rejection coexist without oscillation?

Experiment: model one parent–child connection and one teen–younger-viewer connection before adding a larger network.

### Talent contracts and recurring roles

See [talent and contracts](talent-and-contracts.md). Which minimum-work guarantee,
payment schedule and reservation rules make a first multi-edition host agreement
meaningful? How should options, exclusivity, cancellation and release work? How
can scripts change role requirements while preserving existing obligations?
What are the consequences of recasting, and what can the player legitimately know?

Suggested experiment: one short host agreement before a full series-casting or
labour-market system. This is not an implementation commitment.

### Talent and relationships

Which craft dimensions create distinct hiring decisions? How much genre experience is explicit versus derived? How can partnerships improve without becoming permanent optimal combinations?

Experiment: compare familiar, mismatched, inexpensive, and fashionable teams across several project types.

### Brands and commercial health

How are target hype and budgets set? How much product competition and purchase response is necessary? When do brands enter, exit, or recover? Can the player profit from letting client hype decline?

Experiment: run repeated campaigns and look for perpetual demand, runaway spending, and perverse incentives.

### News and public perception

Which stories become public, through which sources, and with what reliability? News formats use the shared production system and automatic event selection from a few editorial settings. How should optional event spotlighting work? What distinguishes recognition from approval?

Experiment: trace a private event through publication, audience beliefs, programme interest, and advertiser response.

### Politics and elections

Which issues and electoral rules provide useful broadcasting decisions? How does media influence coexist with stable identities and lived circumstances? How can an election scenario remain influenceable without making television omnipotent?

A simple proportional system is a proposal only. No political simulation is required for the first playable.

### Awards

What makes productions eligible? Who judges them, and how do taste, execution, prestige, campaigning, and competition interact? Can an awards objective reward a different production strategy from ratings?

Award names, categories, counts, and consecutive-year goals are scenario examples, not settled content.

### Optional AI

Can a small model reliably interpret player pitches into useful attributes? What are the hardware, latency, distribution, and consistency requirements? How can players review the interpretation before committing resources?

Experiment only after structured concepts produce interesting play. Preserve a non-AI route.

## Process

Resolve a question by recording the decision and evidence in the relevant design section, then remove or update its entry here. Keep exact balance values with future configuration and experiment results rather than treating early guesses as design commitments.
