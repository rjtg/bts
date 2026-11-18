# Original film playthrough

1. In **Film desk**, select **Big-city thriller**, then **Save free pitch**.
2. Commission Ada Mercer and advance to script delivery. Under the script notes
   select **Spectacle** for revision. Pay and advance to the revised script.
3. Accept the script. Cast Nina Bell and Sam Ives, select Dev Rowan and
   Soundstage B, and review the booking. Confirm and advance to film delivery.
4. Open the completed film in the library. Schedule it for Day 2 at 18:00.
   Select the first advertiser, request an offer, accept and run the airing.
   The result shows audience counts, reception bands and settled ad revenue
   alongside the 2,920-credit film investment. The captured run settled
   1,125 credits in ad revenue and charged 120 credits to air the film.

See [booking.png](booking.png) and [airing.png](airing.png). To regenerate
screenshots, run the production browser build, then in `web/` run
`BTS_FILM_SCREENSHOTS=1 npm run test:e2e -- tests/film.spec.ts`.

## Controlled comparisons

All cases use the same scenario, cast, director and recording date. Only
the brief, writing option or studio changes as indicated.
Regenerate the developer JSON with
`cargo run -q -p bts-web --example film_comparison`.

| Film | Script spectacle | Screen spectacle | Execution | Credits |
| --- | ---: | ---: | ---: | ---: |
| Modest brief, Ada, Backlot | .120 | .116 | .899 | 2,090 |
| Demanding brief, Ada, Backlot | .596 | .446 | .731 | 2,090 |
| Demanding brief, Ada after revision, Backlot | .733 | .527 | .738 | 2,270 |
| Same revised script, Soundstage | .733 | .706 | .816 | 2,920 |
| Demanding brief, Leon, Backlot | .950 | .650 | .802 | 2,410 |

Ada met the modest spectacle target and fell short of the ambitious one.
A focused revision helped, and the better studio preserved more of the
same revised script. Leon met the ambitious spectacle target on the page
while the Backlot still constrained filming. Modest execution can be stronger
without delivering the same viewing experience. This fixture establishes
specific causal differences, with commercial balance still open.
