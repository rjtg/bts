# Recurring discussion editions

The Production clipboard opens the discussion desk. Open a weekly show binder
from its concept form. A binder holds three editions on the same weekday,
seven days apart, with the same host and recording time. Each has a separate
production ID, recording commitment, programme ID and eventual airing ID.

## Booking and time

- Drafting costs nothing. Subject, guest pair, creative direction and budget
  can be changed on an unbooked future edition. Choose an owned studio when
  booking the edition.
- Booking uses the existing talent, studio and budget checks. It reserves
  the budget, participants, studio and airtime atomically. A clash leaves
  all reservations and balances intact. A booked edition retains its terms.
- Recordings use 15-minute simulation boundaries. Each edition records for
  four ticks, then spends eight ticks in post-production. The airtime may
  start on any minute after delivery, including the instant of delivery.
  The edition occupies 45 minutes plus the selected ad minutes.
- A draft whose recording time passes is marked missed. It incurs no cost,
  holds no airtime and yields no programme. A failed booking can be retried
  until that time passes.
- Successful delivery publishes the episode in the station library and
  converts its reservation into an ordinary airing with the existing ad
  market. Ads can be sold from this air date after the tape is ready.
  Standard edit and signed-deal protections apply to the resulting airing.

Edition qualities use the existing discussion model. Editions share the
existing series-attachment state, initialized at 0.5 for each group.
Continuity strength is zero for this episodic format. The weekly time step,
run length and lead times are provisional. Slot habits, automatic booking,
special availability negotiation from the binder and automatic ad sales are
outside this slice. Existing one-off productions retain those existing
booking options they already support.
