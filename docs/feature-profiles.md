# Programme feature profiles

The first implementation covers the existing catalogue and standalone discussion
production. The [feature vocabulary](design.md#programme-formats-and-subject-matter)
applies across formats. Fiction permits the seven creative features. Nonfiction
also permits information depth, topical urgency and editorial rigour. Applicable
values are finite in [0, 1]. A missing value is skipped, while a present zero
can agree or disagree with a group’s preferred level. Each authored work has at
least one achieved feature.

## Viewing and knowledge

For applicable features, match is `1 − |achieved − preferred|`. Profile fit is
the square of the mean match and weakest fit is the minimum match. The existing
fuzzy model uses these two inputs in place of genre and tone. Attraction retains
awareness and cast appeal. Satisfaction retains general execution. These
weights and the square are provisional. A feature-free advertised profile has
zero mean fit and no weakest-feature penalty.

Selectable library genre labels derive from achieved features at a threshold of
0.65, with humour taking priority over tension for fiction. Nonfiction uses
Late-night for high humour and topical urgency, News for high topical urgency,
Explainer for high depth, and Magazine otherwise. The label does not enter
the inference rules.

Catalogue descriptors give bands for visible qualities. For editorial rigour,
the current experiment models a reporting quality that can affect experienced
satisfaction. Attraction and the browser catalogue omit this hidden value.
The simulation makes no claim about verified truth and has no claim-level
evidence, audience belief updates or credibility stock. Genre labels use no
hidden rigour. The remaining achieved features stand in for advertised
qualities at selection time.

Audience research estimates feature preferences with the existing persistent
bias and familiarity scale. Published snapshots govern ad forecasts, while
airings use actual preferences and existing contracts settle as before. The
browser receives estimated preferences rounded to half-star steps on a five-star
scale. More stars on the playfulness axis mean a preference for a more playful
register. Catalogue and ambition descriptions still use qualitative bands.

## Discussion delivery

The existing three-position creative brief maps to playfulness 0.2, 0.5 or
0.8. It also sets humour to 0.9 times playfulness and pace to 0.35 plus half
playfulness. Warmth targets 0.7. Subject sets depth to 0.97 for Science, 0.58
for Everyday Life or 0.25 for Entertainment. Other dimensions are absent
in this prototype.

Each person's capabilities describe skill in an assigned role and use optional
values in [0, 1]. The roster can represent actors, directors, writers, anchors,
reporters, hosts and guests. Only hosts and guests are booked by the current
workflow. For each targeted feature, discussion delivery skill combines the
host at 50% and each guest at 25%. An absent capability supplies zero
contribution. The delivered feature interpolates from a baseline toward the
target by `0.30 + 0.40 × delivery skill + 0.20 × craft + 0.10 × subject fit`.
Baseline is 0.5 for the playfulness–seriousness axis and zero elsewhere.
The average delivery skill across targeted features also contributes to
overall execution. General craft, subject fit, resources and recognition
remain separate. Cooperation affects execution through the existing penalty
and Sway recovery. It does not change capability or ambition.

The browser booking review shows intended qualities, and the completed show's
library entry shows achieved qualities. Protocol v6 introduces these reports.
Exact delivered values and causal calculations stay in developer reports.

[Controlled results](../contrib/feature-profiles/README.md) demonstrate
specific scenario differences. Role pipelines, preferred watch times and
editorial credibility remain future work.
