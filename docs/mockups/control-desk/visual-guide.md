# Control desk visual guide

| Role | Token | Use |
| --- | --- | --- |
| Cabinet | #121e24 / #0b151b | Outer shell and inset timing rail |
| Panel | #203137 | Schedule blocks and drawers |
| Paper | #f3e7cb / #203137 ink | Rights, script notes and recap |
| Amber | #f4c36e | Current selection and primary action |
| Sea green | #a8d3b7 | Booked or paid receipt with text |
| Rose | #f3a49a | On-air lamp and blocked actions, each with text |
| Blue grey | #a4b7b8 | Secondary labels on dark |

Use condensed sans headings, a readable sans for decisions, and tabular mono
figures for time and credits. The boards use system fonts without network
dependencies. Give portraits a consistent crop and thin frame. Use pixel art
for programme identity and people, with clean interface text over solid surfaces.

- **Station strip**. Keep the explicit game clock, transport, cash, and bookings
  visible. Now, Next and Selection are distinct. On another day, label the date
  viewed and omit the Now line. On narrow screens retain time, lamp and
  transport; let the remaining resources wrap into a labelled row.
- **Time grid**. Name channel, title, start, end and status for assistive tech.
  Amber outline means selected. Dashed line means Now. Hatch a pending drop or
  conflict and label it in text. Give tiny programme blocks another focus target.
- **Decision drawer**. Lead with the title and time, then the decision. Separate
  draft minutes, current offer, held terms and paid result. Opening an offer
  creates no commitment. If it expires, offer Refresh without hiding old terms.
- **Stage card and portrait seat**. Give each project a next action. Use one
  picker, keyed to the active seat, with fee and availability. Return focus to
  that seat. Show studio shortages and script notes with words.
- **Industry wire and status tickets**. Celebrate readiness and flag decisions
  with distinct labels and one destination per event. Announce actual changes
  once via a polite live region. Reserve assertive feedback for rejected
  actions. Keep event history accessible and honour reduced motion.
- **Receipt**. Show paid or held credits, exact ad time, and occupied ticks.
  Keep one-airing contribution distinct from acquisition or production cost.
  Provide disclosure for original terms and ledger entries.
- **Research dial**. Preserve the preference stars. Show amount, balance after
  spend and next relevant publication. Support a stepped range and keys.

The [icon specimen](index.html#icons) labels the supplied 16×16 pixel sprite,
ten columns in one row. View at 1× or 2× with nearest-neighbour scaling.

| Column | Shape | Meaning |
| ---: | --- | --- |
| 0 | Banknote | Cash |
| 1 | Magnifier | Insight or investigate |
| 2 | Joined hands | Sway |
| 3 | Triangle | Resume |
| 4 | Twin bars | Pause |
| 5 | Double chevron | Speed, paired with 2× |
| 6 | Transmitter | On air |
| 7 | Dated card | Booking, paired with date |
| 8 | Alert triangle | Attention, alongside reason |
| 9 | Padlock | Protected commitment, alongside available action |

Use a text label for unfamiliar symbols and an accessible name for every
icon-only button. Preserve keyboard focus across ticks and return focus when
drawers close. Aim for 44×44 CSS px touch targets. Colour, audio, and motion are
supplementary signals.
