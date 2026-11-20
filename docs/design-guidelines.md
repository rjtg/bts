# Design guidelines

## Start with the player's decision

- Give each interaction a purpose in running a television station. Identify
  the decision, the evidence available at that moment, what committing changes,
  and what the player can do after seeing the result.
- Let the world create different viable bets. Broad reach and specialist reach,
  reputation and solvency, or the money spent and the Sway earned can point to
  different choices. Keep the accounting obvious and let the player decide
  whether the outcome served their goals. Avoid a universal success score.
- Make time matter through deadlines, competing bookings and work that takes
  simulated time. Keep reading, navigation and ordinary input responsive.
  Prototype shared or desk-specific action capacity before adding delays.

## Make information useful and imperfect

- Keep actual state, public evidence, and station estimates distinct. Show the
  source and age of a survey or projection where it affects the decision.
  Preserve a dated estimate when later information arrives. Use broad findings
  or half-star bands where precise decimals would imply knowledge the player
  lacks. Show fees, Funds, dates, minutes and signed obligations exactly.
- Treat low, unknown and inapplicable as different states. Use capabilities
  for people, preferences for audiences, ambition for the intended work and
  achieved qualities for the actual work. Genre helps someone browse a library.
- Let research refine an estimate without revealing exact hidden state.
  Persistent bias should drift gradually. More polling refreshes evidence.
  Spending Insight raises familiarity for subsequent evidence and has
  diminishing returns. Use automatic audience surveys and optional
  decision-triggered research for other entities when that design is built.
- Give results a home in their context. Compare the dated pre-air estimate,
  provisional reception, signed terms, and per-airing costs in the Airing
  file. Put lifetime investment and returns in the Tape file. Keep causal
  diagnostics available to developers through separate reports.

## Build a station people can inhabit

- Lead with the Station Office and its objects. Use the calendar to program,
  the shelf to find a tape and the script file to develop work. Keep the room
  visible around the active job. A bigger task may fill most of the screen
  while still looking like a tool brought forward in the same office.
- Give the set a warm late-1980s broadcast identity. Use perspective and contact
  shadows on furniture, and keep text and interactive surfaces nearly face-on.
  Keep artwork layered and controls live. Use sprites for physical pieces,
  typewriter text for working papers, and handwriting for short human notes.
- Reuse one vocabulary of artefacts. Browse tapes and named project folders
  in collections. Use a canister wheel for a short set of branches, a spread
  for reading and decisions, portrait or studio cards for assignments, a
  flip clock and calendar for time, and a ledger for transactions. Keep a
  central context icon in the wheel, a branch sticker on its side and Back in
  the hub. End lengthy branches in a collection with readable names.
- Give a spread a consistent footprint and two visibly distinct leaves.
  Use labelled edge tabs and folded page corners. Prefer turning a page
  to scrolling a long form. Put each control where its result belongs.
  Reuse sibling implementations of the same interaction and avoid restating
  information already visible on a tape, label, date stamp or receipt. Write in the
  language of the station. Use a short mood-setting line where it adds
  character, and leave out control narration when the paper explains itself.
- Distinguish prop reactions, job transitions and room ambience. A selectable
  prop reacts and names its destination. Decorative people and equipment
  provide atmosphere. Keep motion brief, subtle and independent of gameplay.
  Make reduced motion and keyboard/touch paths first-class. Preserve focus,
  the active object, drafts and calendar position through close or cancel.
  Use actual readiness events for notices and link them to the next action.

## Match controls and commitments to their consequences

- Use stars for a quality ambition or a visible estimate, and provide text
  equivalents. Use cards when the evidence is about a person, room or title.
  Use a dial with whole-number entry for an adjustable spend. Keep exact
  runtime, ad load and the resulting schedule interval together before placement.
- Preview costs, conflicts, availability and affected entities in context.
  Give a clear final action and meaningful feedback such as a signed offer,
  dated call sheet, ledger entry or new tape. Add a separate commitment
  only when the player has another real decision. A recommendation opens an
  editable proposal.
- Preserve distinctions that affect play. A selected airing differs from
  Up Next. A quote differs from a signed sale. A script can be accepted
  before its cast and studio are booked. A talent agreement and assignment
  each carry obligations. Tie actions to stable identities, including
  separate IDs for every airing in a recurring run.
- Spend resources for a specific reason. Funds pay fees and expenses. Sway
  can secure explicitly negotiable participation or reduce authored
  cooperation friction, with bounded recovery and diminishing returns.
  Insight improves knowledge. Keep their ledgers and consequences legible.

## Verify a slice from the office to the outcome

- Keep authoritative time, validation, purchases and accounting in Rust.
  UI drafts and previews preserve the current state until confirmation.
  Failed actions preserve cash, inventory and signed obligations. Keep
  fuzzy inference generic in its own crate and use it where rules explain
  a meaningful interaction.
- Use controlled comparisons to isolate causes. State the starting conditions,
  units, ordering, numerical assumptions and limits. Treat a successful
  fixture or appealing mockup as specific evidence and reserve balance claims
  for broader testing.
- Design from actual play. Capture repeatable Chromium views at desktop and
  narrow widths and compare with the approved reference. Walk the normal
  journey including an invalid action, then test keyboard, touch and reduced
  motion. Preserve working player routes while replacing inherited experiment
  interfaces. Keep screenshots, replay steps and checks with the milestone.
