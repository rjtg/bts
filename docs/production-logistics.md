# Station libraries, staff and production logistics

Status: distinct browser station libraries/host rosters and owned-studio capacity
implemented. Rental, setup/cleanup economies, reusable props and multi-edition
batching are next-slice design directions, not implemented mechanics.

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

## Next slice: facilities and batching

Direction: recording requires suitable studio space, which may later be owned or
rented. Ownership, rental dates and prices should create explicit obligations and
capacity constraints. Do not silently add a studio merely because a project was
accepted. Room requirements and availability must be visible before commitments.

Setting up a studio for several consecutive editions/episodes of the same
production should save repeated setup and cleanup costs. Use explicit production/
format and setup identity, not matching title strings. Account for setup once,
record the planned sessions, and clean up once at the end of the compatible batch.
An interruption, incompatible set change or reservation conflict needs explicit
rules for which costs/time recur. Savings must not remove talent booking conflicts
or imply that editing/post-production becomes free.

Recurring editions and batches are not implemented now. Open: what makes setups
compatible, preparation/cleanup duration, cost allocation across episodes, booking
and cancellation terms, rental settlement, room capabilities and handling a batch
that is partly completed. Choose these in a bounded experiment before implementing
a general facility market.

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

A possible next experiment is one reusable discussion set, one owned/rentable room
and a short explicit recording batch. Compare consecutive and interrupted sessions,
verify one-time asset creation, setup/cleanup savings, commitments and conservation.
That is a proposal for the following production-logistics slice, as requested;
it does not replace the deferred staged-writing or broader talent-contract design.
