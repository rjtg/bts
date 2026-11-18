# Talent market, contracts and recurring roles

Status: design direction, November 2026. A dedicated place to find and sign people,
with meaningful obligations, is intended. Contract structures, negotiation rules,
prices and consequences below are proposals requiring bounded experiments.
This document does not claim implementation or expand the current milestone.

The existing discussion experiment books one host and two guests for a single
recording, reserves a fixed budget and supports exceptional participation through
[pooled Sway](pooled-sway.md). It does not implement employment, multi-edition
contracts, character casting, script rewrites or a dynamic talent market.

## Three distinct layers

| Layer | Meaning | Example |
| --- | --- | --- |
| Talent market | People seeking or open to work, their known qualities, interests, availability and preferred terms | A presenter interested in a regular discussion programme |
| Contract | Explicit promises between station and person: pay, dates, guaranteed work and any negotiated restrictions | Six guaranteed editions, preparation time and agreed fees |
| Assignment | The particular production job or dramatic role performed under a contract | Hosting edition four, or playing a detective in specified episodes |

Signing someone is not the same as assigning them to every production, and a
signed contract does not imply that all their time is available. Show existing
commitments and remaining obligations before the player confirms an assignment.
Production assignments must satisfy the relevant contract and time constraints.

## Different people seek different arrangements

Proposed examples:

- A talk-show host may demand a guaranteed run of several editions, preparation
  time and adequate notice of recording dates.
- A news anchor may prefer a season contract with a regular workload.
- A film actor may sign for one role, with rehearsal and recording commitments.
- A procedural-series actor may sign for specified episodes, with separately
  negotiated options for additional appearances or a later run.

These are preferences and negotiated terms, not rigid profession rules. An actor
can want a long commitment; a host can accept a one-off assignment. Known career
interests and public reputation should support distinct hiring decisions without
exposing exact hidden craft or fit coefficients.

Possible contract terms include fees, payment timing, minimum guaranteed work,
recording/preparation windows, notice, options, exclusivity and release conditions.
Do not implement every term at once. An option is not free guaranteed availability:
its reservation, exercise deadline and additional cost need explicit rules.

The player must understand what is guaranteed, what is merely preferred, what
cash is committed and which dates are reserved. Contracts should create decisions,
not require routine negotiation of every minor clause.

## Roles, actors and episode continuity

A scripted character/role has an identity separate from the actor portraying it.
A recurring detective belongs to the series; an actor is cast and contracted to
portray that role. Each script specifies which roles are required for which
episodes. A recurring role normally requires consistent casting across those
appearances; not every recurring character must appear in every episode.

Keep series identity, episode identity, script requirements, casting and talent
commitments distinct. A multi-episode actor contract must cover the promised
appearances and their work windows. Avoid representing continuity as a programme
name string or treating every episode as an unrelated casting task.

Scripts may be rewritten to reduce appearances, remove a character, alter scenes
or accommodate an unavailable actor. Rewriting is paid creative work with elapsed
time and possible compromises, linked to [staged production](production-ambition-draft.md).
It must not instantly erase defects, role requirements or contractual obligations.

Removing a role from a script does not cancel the actor's agreement. Guaranteed
pay/work may remain due; changes could require consent, a negotiated release or
another remedy. Recasting should be an explicit decision with documented terms
and continuity consequences, not a consequence-free portrait swap. Exact remedies,
audience responses and rights remain open; no penalty formula is chosen here.

## Player workflow

A **Talent & Contracts** workspace should feel like finding collaborators and
negotiating commitments, rather than selecting names from a technical dropdown.
It can sit within a broader people destination; final navigation is open.

- Browse person cards with portraits, public reputation, specialties, interests,
  fees or expectations, and known availability.
- Inspect a concise negotiation sheet showing the offer and resulting obligations.
- View a calendar of existing commitments and conflicts.
- On a production casting board, choose **Use contracted talent** or **Find someone**
  from an empty seat or role. Link back to the person's contract and calendar.
- Keep ongoing contracts, proposed negotiations and completed obligations distinct.

Stars may represent known public reputation or an explicitly labelled assessment;
they must not reveal hidden skill, promise creative success or imply guaranteed
availability. Exact payable amounts and dates remain inspectable in ordinary play.

Pooled Sway can open negotiations or obtain a specific concession from someone
other than the original request proposer. It does not pay fees, override existing
commitments or create a general quality guarantee. Broader contract concessions
remain proposals; current Sway effects stay as documented in the implemented model.

## Suggested bounded experiment and open questions

A possible first slice is one host contract covering a short run of discussion
editions, with explicit minimum work, fees, preparation/recording reservations and
clear fulfilment. This is a suggested sequence, not a commitment to implement
recurring editions now. Compare a flexible one-off booking with a longer guarantee
and verify cash, time, conflicts, fulfilment and atomic rejection in the browser.
Leave the existing one-off path available where appropriate.

Test scripted recurring roles and rewriting in a later slice once scripts and
episodes have meaningful production requirements. Do not build a universal labour
market, legal system or career simulation merely to support the first contract.

Open: minimum-work interpretation; reservation versus confirmed dates; payment
and cancellation timing; breach/release remedies; option pricing; exclusivity;
renewal; reassignment; recasting consequences; player evidence about fit; and the
scope of an initial negotiation UI. Define these visibly before implementing them.
