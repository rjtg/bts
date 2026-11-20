# Station saves and season review

Use **Station files** in the office directory to download a JSON save or
restore one from the device. A successful restore opens the office in
standby. The clock, schedule, sold minutes, research, project stages,
contracts, cash, and public reports return to the saved point. The
currently focused widget, unsubmitted form values, desk-pinned outlooks and local animation
state are browser preferences outside the file.

The file records the version, the six-week scenario identifier, and the
ordered, accepted commands. Rust reconstructs the station by replaying
the journal into a fresh session. Deals in the journal reference the same
airing or title and are replayed against the corresponding valid offer.
The restore creates new session-scoped quote tokens, so pending offers
remain usable from the restored report. Old tokens do not authorize sales
against the restored session. Rejected commands are not recorded. A bad
file leaves the current station intact.

Version 1 supports the present six-week scenario and a limit of 8 MiB or
20,000 commands. The replay depends on the authored scenario and rule
semantics. Changes that alter past outcomes require a new version or an
explicit migration. The file is for local progress, without a cloud
account or cross-version compatibility promise.

## Finish the season

The notebook tracks three goals through the closing boundary. The season
ends after 42 days and the notebook shows how many goals were completed.
For a reproducible walkthrough, the sample checkpoint in the
[season review index](../contrib/season-roadmap/README.md) starts week two
with the remaining airings and ad policy in place. Continue the calendar
from there or start fresh and use the preparation and production desks
to make your own schedule.
