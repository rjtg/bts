# Station libraries, staff and production logistics

Status: distinct browser station libraries/host rosters and owned-studio capacity
implemented. Rental, setup/cleanup economies, reusable props and multi-edition
batching are next-slice design directions, not implemented mechanics.

The [original film experiment](original-film.md) adds two fixed-price rental
rooms with exclusive bookings and an equipment shortfall model.

## Implemented station resources

The world catalogue defines works; each station's library defines its rights to
air them. The browser scenario starts with disjoint inventories:

- Your station owns the seven existing catalogue works and has Elian North and
  Mina Sol assigned as its hosts.
- Channel Two owns **After Hours**, **Tomorrow's Workshop**, **City Stories** and
  **The Last Tram**, and has Vera Quinn assigned as its host.
- Nora, Felix, Jules and Rafi remain freelance guests, subject to their existing
  availability, fees and recording commitments. A roster assignment is not yet a
  negotiated employment contract or wage obligation.

Each station owns one recording studio in this fixture. Programme rights, studio
counts and staff assignments are configured in Rust, not inferred from browser
cards. Empty inventories/rosters and zero studios are valid configurations; the
corresponding activities may then be unavailable. Content counts are fixture limits,
not engine validation rules. Staff cannot be assigned to two stations in this
bounded ownership model; freelance people may remain unassigned.

Rust rejects scheduling a work outside the station's library, and rejects assigning
another station's staff to your production. A completed original is inserted into
your library only; world-catalogue existence does not grant the rival rights to it.
Draft/uncompleted works remain unschedulable. No rights purchase/trade, staff
poaching, rental contract or rival production AI is added.

Player catalogue DTOs include only owned works. Rival public schedule entries carry
title, genre and runtime so the planner can show competition without offering those
works for placement. Exact rival execution, production costs, staff skills, cash
and studio capacity are not exposed. The fixture publishes rival staff names.
The browser's caster includes own staff and eligible freelancers.

`Planner::configure_station_resources` supplies all stations' rights/rosters/studio
counts before play, validating references and existing fixed broadcasts atomically.
Configuration locks after player planning, quest acceptance, production or time
advancement. Legacy native experiments intentionally retain the shared-catalogue
initialisation when no partition is supplied; actual browser sessions always apply
the explicit disjoint configuration. Shared licensing is not universally forbidden
by the model, but the current browser fixture has no common works or staff.

## Independent person and studio reservations

Confirmed productions reserve all participants and one station-local studio over
the half-open recording interval `[r,r+4)`. Adjacent sessions can reuse the same
people and room at `r+4`. Draft concepts and booking reviews reserve nothing;
confirmation always revalidates. Finishing post-production does not retain the
recording room. Broadcasting an already completed work reserves neither cast nor
recording space.

Rust already rejected overlapping confirmed bookings for a person. The casting
picker now exposes their upcoming commitments and blocks obvious conflicts, while
Rust remains authoritative. Room capacity is checked independently: two shows
with entirely different casts still cannot record simultaneously in a one-studio
station. With two configured studios they may, while a shared host still cannot.
Booking chooses the lowest available local studio number deterministically and
records it in the project/review. Staff/room failures consume no cash, Sway, IDs
or commitments. Sway cannot override either constraint.

The one-hour recording interval currently includes its preparation/cleanup time.
The full production budget is reserved once and spent once; there are no extra
per-tick or hidden room fees. This capacity milestone does not claim to model
setup-cost savings, studio acquisition or physical materials.

## Next slice proposal: elastic studio rental and quality

The player can always rent studio time: rental capacity itself never hard-blocks a
production. Price rises with demand for the same overlapping time. Count all
stations' bookings, including rivals, as a public or scenario-provided demand
signal. A competing booking contributes only for the fraction of the player's
requested interval that overlaps it. For requested interval `P`, booking `i`, and
their overlap duration `o_i`, define slot pressure as
`L = sum_i(o_i / duration(P))`. Thus an equally long booking contributes `1`, a
half-length overlap contributes `0.5`, and non-overlapping sessions contribute
`0`. Do not count distinct but simultaneous rooms as a single booking unless the
fixture models their capacity explicitly. This ratio is a provisional demand
measure, not a capacity limit.

Studio selection replaces the current free-form production-budget choice. The
player chooses a studio package whose quality ratings determine the rental price;
cast fees are then added to that price to produce the full production cost. There
is no separate budget slider that can buy a generic execution bonus. The booking
review shows studio rental, cast fees, total cash commitment and remaining cash.
The existing affordability and atomic booking rules apply to the full amount.

Studios expose a small, authored set of quality dimensions. Initial proposal:

| Dimension | What it represents | Who may require it |
| --- | --- | --- |
| Equipment | Recording, lighting and sound capability | Script or format requirement |
| Size | Stage/room capacity for cast and set | Scripted scene, cast or set requirement |
| Atmosphere | How comfortable and collaborative the room feels | May soften a separately modeled cooperation difficulty |

The player selects a one-to-five-star level for each dimension. Stars are readable
choices, not numeric talent bonuses. A script/format can state desired equipment or
size levels. Choosing below those requirements is allowed: the production proceeds,
and the shortfall becomes a concrete production deficiency that can lower the
relevant achieved quality and audience response. It is the same risk shape as
choosing an ambition beyond the team or budget's capacity; see
[overambition and achieved quality](production-ambition-draft.md#overambition-and-achieved-quality).
The player can accept a cheaper inadequate room instead of being blocked from
making the show. Meeting a requirement avoids that specific deficiency; exceeding
it may add only a bounded, dimension-specific benefit, never a generic quality
bonus.

Equipment shortfall should affect technical presentation (for example, unclear
sound or lighting), while size shortfall should constrain staging or scene scope.
Atmosphere can affect comfort or cooperation outcomes, but cannot improve craft or
subject fit and should not erase an authored cooperation deficiency. Its effect
must remain distinct from adjustable Sway, with no double recovery of the same
penalty. Reports should identify the resulting deficiency and its causal effect;
player-facing booking should show desired versus selected stars and the likely
tradeoff without revealing hidden skill values or guaranteeing reception.

Rental price should combine the quality choices and time pressure through a
small fuzzy model in `bts-sim`, using the independent generic `bts-fuzzy` crate.
The intended shape is monotonic: higher selected quality and greater overlapping
booking pressure increase rental price; an empty/low-demand slot and minimum viable
studio package are cheapest. A low quality choice can be inexpensive while creating
a production shortfall. Cast fees remain separate fixed inputs to total production
cost. Use complementary low/high rules where a gradual response is wanted,
and inspect the rules and trace in developer reports. Sugeno produces a weighted
average output; rule activation is not a probability or a multiplier on price.
Convert to integer credits once at quote/booking using documented rounding, freeze
the quote through confirmation, and reserve/account for it atomically. The player
must see quality requirements, selected stars, interval pressure summary, total
price and remaining cash before booking. Expose no rival private finances or hidden
production attributes merely to explain a quote.

This direction means rented capacity is elastic while price is scarce: an
unavailable studio is not a rejection reason, but high demand can make the desired
room unaffordable. A valid but unaffordable production should follow the existing
atomic rejection/off-air principles as appropriate to whether a commitment has
already been made. Never silently book a better room or spend beyond cash.

Setting up a studio for several consecutive editions/episodes of the same
production should save repeated setup and cleanup costs. Use explicit production/
format and setup identity, not matching title strings. Account for setup once,
record the planned sessions, and clean up once at the end of the compatible batch.
An interruption, incompatible set change or reservation conflict needs explicit
rules for which costs/time recur. Savings must not remove talent booking conflicts
or imply that editing/post-production becomes free.

Recurring editions and batches are not implemented now. Open: what makes setups
compatible, preparation/cleanup duration, cost allocation across episodes, booking
and cancellation terms, when rental is charged (booking, recording or settlement),
how all-station interval demand is observed, quality-to-price calibration, and how a
batch that is partly completed is handled. Choose these in a bounded experiment;
do not build a general facility market.

## Next slice: produce props once and retain them

Props and reusable set pieces should be produced/acquired once, owned by the
station and kept in stock for later compatible productions. Separate the cost and
time of creating an asset from the cost of using it. Do not charge for fabricating
the same retained prop on every episode.

Stock does not imply unlimited simultaneous use: an item or set may be reserved
for a recording or installed in a studio. Distinguish reusable assets from genuinely
consumed materials if that distinction becomes relevant. Stable asset identity,
station ownership and reservation history should support explanation and exact
accounting, without a universal untyped inventory framework.

Open: prop/set requirements, storage limits if useful, wear/repair, rented assets,
transport, disposal and cancellation consequences. No prop bonuses, depreciation,
storage fees or crafting system are implemented by this document.

A possible next experiment is elastic rental for one discussion studio, three
quality dimensions with one explicit script requirement, and a short recording
batch with one reusable set. Compare low/high demand at controlled overlap ratios,
below-requirement, adequate and premium quality, and consecutive versus interrupted sessions.
Verify quote determinism, affordability, one-time asset creation, setup/cleanup
savings, commitments and financial conservation.
That is a proposal for the following production-logistics slice, as requested;
it does not replace the deferred staged-writing or broader talent-contract design.
