# Fictional character portraits

Initial art set, November 2026: three male and three female fictional adults,
with varied ages, appearances, hairstyles and presentation. These are reusable
portraits for the talent/guest roster, not photographs of real people or actors.
The existing discussion roster uses them in its portrait casting desk.
They can later support the talent market and scripted casting UI.

| Person | Authored gender | Approximate visual age | Current fixture job | Portrait |
| --- | --- | ---: | --- | --- |
| Elian North | Male | 60 | Host | ![Elian](../web/public/portraits/elian-north.png) |
| Mina Sol | Female | 34 | Host | ![Mina](../web/public/portraits/mina-sol.png) |
| Nora Vale | Female | 48 | Guest | ![Nora](../web/public/portraits/nora-vale.png) |
| Felix Reed | Male | 29 | Guest | ![Felix](../web/public/portraits/felix-reed.png) |
| Jules Hart | Female | 64 | Guest | ![Jules](../web/public/portraits/jules-hart.png) |
| Rafi Linden | Male | 42 | Guest | ![Rafi](../web/public/portraits/rafi-linden.png) |

Gender and visual ages here are authored art/roster notes, not inferred from the
images or new simulation parameters. Performer gender is distinct from a dramatic
character's gender and any explicit casting requirements. No gender-based skill
modifiers or casting restrictions are implemented. See [talent and contracts](talent-and-contracts.md#performer-gender-and-character-requirements).
The current host/guest assignments are production jobs, not fictional character roles.

Style: head-and-shoulders pixel-art portraits, consistent dark teal studio
backdrops and warm light, restrained period clothing, square composition and
readable faces. Art should identify people without implying that an appearance
reveals talent, morality, temperament or hidden numerical attributes.

Generated with the built-in imagegen tool, one image per person. The complete
[prompt set](portrait-prompts.json) records the shared direction and individual
briefs. Original PNGs are copied into `web/public/portraits/`; no image processing
or external image service is required at runtime. Names/IDs are mapped to artwork
only in the bounded browser fixture. An unfamiliar person gets no unrelated
portrait by fallback. Publication remains disabled and licensing is unchanged.

Validation: TypeScript/static bundle build passed. A real Chromium check switched
through both hosts and all four guests, decoded all six square portraits, verified
accessible image labels and recorded no page errors. The TypeScript language
server was unavailable; `tsc --noEmit` and the browser check supplied feedback.
No Rust simulation or protocol changes were needed.

## Audience group portraits

The audience dossiers use three illustrative group portraits, mapped by scenario
audience ID in `web/src/audience.ts`. Add future illustrations to
`web/public/audiences/` and extend that mapping when a group needs one.

Built-in ImageGen prompt set, with the existing Mina Sol portrait as style
reference. Each brief uses this shared direction.

> Landscape group portrait for Behind the Screens audience dossier. Match the
> existing Mina Sol portrait's 1980s pixel-art game illustration, crisp deliberate
> pixel clusters, warm shading, deep teal backdrop, coral and amber accents.
> Distinct faces and readable silhouettes at thumbnail size. Frame all faces
> centrally with breathing room. No text, logos or borders.

- `young-viewers.webp` depicts three diverse late teens and young adults in
  colourful 1980s casual clothes sharing popcorn on a couch.
- `households.webp` depicts a relaxed multigenerational household, including an
  older adult and a child watching together on a sofa.
- `specialists.webp` depicts three adult hobbyists in a community workshop with
  an electronics project and notebook.

## Film freelancers

Built-in ImageGen created [the film roster atlas](../web/public/portraits/film-roster.webp).
The prompt used Mina Sol as a style reference for 1980s pixel art, warm shading
and teal backdrops. It requested a text-free four-column, two-row atlas of
new, centred head-and-shoulders portraits in roster order. The top row is Ada,
Leon, Inez and Dev. The bottom row is Nina, Sam, Rosa and a spare silhouette.

## Rights listing art

The [rights title atlas](../web/public/portraits/rights-titles.webp) was made
with built-in ImageGen. The prompt requested one landscape 3:2 late-1980s
pixel-art contact sheet with a two-by-two grid, no gutters or text, crisp pixel
clusters, deep navy and teal and amber windows. The four tiles are a warm
neighbourhood shop at twilight, a retro observatory, a running silhouette
and vintage car in neon rain, and a spare cinema exterior.
