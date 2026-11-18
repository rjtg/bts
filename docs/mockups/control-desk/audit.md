# Player journey audit

Observed in the browser game on commit aa0fa7b. Replayed the existing Chromium
journeys for planning (including mobile and overlap), ad minutes, rights, film,
polling, and results. All 19 passed. The linked evidence is from the current
game. The redesigned boards use a separate illustrative broadcast day.

| Journey | Decision and evidence | Current action and feedback | Friction and redesign response |
| --- | --- | --- | --- |
| Plan the evening | Choose a show that fits around the next booking. Need runtime, free time and rival context. | Pick a title, set day/time, place, select it, set new day/time, submit. Overlaps produce an error and leave the old block intact. Run Next and Run Through Selected target different airings. | Too many controls represent the same decision. Move cards on the grid, or choose Move and a slot. Put the overlap warning beside the conflict. Keep Next separate from Selection. |
| Sell minutes | Choose the size of the block and which brands fit. Need exact runtime, reserved time, price and remaining inventory. | Set minutes, find offers, expand Review Offer, accept, receive refreshed offers. Trim leftovers or leave the space idle. | Keep one inventory strip and clearly mark held versus open time. Put all terms on the offer card so the player can commit once from there. Show changed-offer feedback in context. |
| Buy rights | Decide whether an archive title or new release is worth its price and a study can arrive in time. Need evidence, price and deadline. | Commission an assessment, advance, inspect the revision, buy and open the library. Industry wire records releases and rival purchases. Resale names the blocking airing. | Keep evidence and deadline together on the illustrated card. Turn the finished study into a link to what changed. Offer Schedule after purchase. |
| Make a film | Decide whether to revise the script. Weigh cast and studio against price and delivery. | Pitch, commission, advance, review, choose a revision or accept. Cast two roles and a director, select room and date, review, confirm, advance, open library. | Cast lists repeat per seat and script notes compete with booking information. Use a stage rail and one portrait tray for the selected seat. Keep the paid booking review distinct from draft choices. |
| Investigate an audience | Decide if shared polling and one group's study are worth using. Need survey fee, next poll, and shared Insight. | Set the shared cadence, turn the group dial, spend and wait for the next survey. Dossiers have portraits and half-star estimates. | Keep portraits, stars and dial. Set cadence once at the desk. Show the poll as an evidence footer and the delayed effect beside Spend Insight. |
| Review an airing | Decide whether to repeat, change airtime or change the ad load. Need group reach, receipts, operating margin and investment. | Read results table, select airing, read counts and separate contract and ledger sections. Original film investment can appear in the result. | Build a recap from the schedule. Lead with reach and contribution, then planning actions. Put auditing detail behind a receipt link. Keep the original investment distinct from airing contribution. |

The current left rail scrolls one long page. The new shell should carry
place and object context across desks. Keep exact prices and deadlines visible
while putting protocol IDs and delivery micro-units in detail. Selection is
inspection context. On Air and Up Next come from Rust. Forecasts use published
evidence. Past expected viewers are still provisional, and payment requires
settlement. An aired block can be paid even if turnout disappoints.

Browser evidence.
[Planning](../../../contrib/programme-planning/README.md) ·
[Ad minutes](../../../contrib/ad-minute-market/README.md) ·
[Rights](../../../contrib/rights-market/README.md) ·
[Film](../../../contrib/original-film/README.md) ·
[Audience research](../../audience-research.md)
