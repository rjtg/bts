# Resource economies — idea draft

Status: discussion draft, November 2026. Not an implementation specification or a change to the current milestone. See [game design](design.md) and [first playable](first-playable.md).

## Intent

Give the player resources beyond money, each arising from a different activity and operating on a different rhythm. Avoid making every resource a second cash balance earned from successful broadcasts and spent on better broadcasts. No generic conversion rate, shared regeneration rule or automatic decay is agreed.

| Resource | Source system | Rhythm | Possible uses |
| --- | --- | --- | --- |
| Cash | Sell advertising and place it effectively to fulfil commitments | Broadcasts and contract deadlines | Fees, rights, facilities and production budgets |
| Sway | Grant favours requested by people | Negotiations and projects | Call in cooperation, negotiate exceptions, help a team work together |
| Glamour | Win awards with station productions | Award and festival cycles | Attract prestigious talent and collaborations; commit glamour to productions |
| Credibility | Maintain an editorial record with low bias or balanced coverage | Repeated editorial choices over time | Sensitive access, sources and editorial backing; exact spending mechanism remains open |
| Bias / backing (working names) | Support particular interests through unbalanced coverage | Ongoing editorial and organisational relationships | Access, guests, exclusives or assistance from those interests; remains exploratory |

## Sway: a favour economy

People bring concrete requests connected to television work. Examples:

- A talk-show guest asks to appear alongside a particular other guest.
- A director wants a particular actor cast or a screenplay realised.
- A news anchor asks for a change in editorial tone.

Granting a favour earns sway. These requests create tradeoffs: fulfilling one may cost money, constrain a production, upset another collaborator or change editorial direction. Merely completing ordinary paid work should not automatically generate sway.

Sway can then be committed to securing cooperation or improving the conditions for chemistry within a production. Its effect should involve people and their willingness to cooperate, rather than provide an unexplained quality bonus.

Open: is sway owed by specific people, shared across an industry network, or partly pooled? When is a favour fulfilled, how are promises recorded, and what happens if they are broken? How do requests stay meaningful rather than becoming repeatable farming tasks?

## Glamour: an awards economy

Productions compete for awards. Winning generates spendable glamour, giving this resource a much slower and more episodic rhythm than advertising revenue or favours. High ratings alone do not automatically grant glamour.

Award eligibility, categories, judging and competitors should make different kinds of productions viable. Spending glamour could help secure stars or prestigious collaborations; exact uses and amounts are proposals, not settled mechanics.

Open: nominations versus wins, award prestige, entry requirements, and what spending represents. Prevent a permanent winner-takes-all advantage while preserving the value of an award.

## Credibility and editorial alignment

Explore earning credibility through low bias or balanced reporting. Biased coverage could instead generate a different resource associated with the interests being supported. This creates an editorial choice with different opportunities, rather than one universally best reporting style.

“Backing” is a suggested name for support received from aligned organisations; “bias” describes the editorial stance itself. Naming and mechanics are unresolved. Backing may need to be specific to a faction or organisation rather than a universal pool.

Accuracy, balance and independence are distinct dimensions. Whether and how each contributes to credibility still needs design; balanced coverage must not simply mean giving every claim equal weight. These are exploratory questions, not an agreed credibility formula.

Open: how credibility is generated and spent, who recognises it, how corrections or scandals affect it, and whether credibility and partisan backing can coexist. Calling on credibility should have a concrete consequence rather than arbitrarily making viewers trust the station less.

## Interactions to explore

An anchor's requested tone change could earn sway while changing credibility or partisan backing. A director could request an expensive passion project that later competes for an award. The systems meet through production decisions while retaining distinct sources and timing.

Next design step: work through one favour chain, one award cycle and one editorial tradeoff before choosing formulas, caps or UI counters. Do not implement all resources as interchangeable currencies by default.

Implementation note: the [original-production experiment](original-production.md)
records one named guest’s co-guest promise and one later personal favour call.
It has no pooled Sway, fee discount or quality bonus and does not settle the broader
resource design proposed here.
