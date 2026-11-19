# Station time and action capacity

Design experiment for later playtesting. The browser currently runs its
ordinary Rust commands immediately. A manual pause freezes the clock and
locks the workspace until Continue. The experiment below concerns how work
could take time while the station is running.

## One decision to prototype

Preparing the ad listing for Saturday's 18:00 airing takes two 15-minute
ticks. At 17:15 the Ad Sales desk could read:

> **Prepare listing · 30 min**
>
> Ready at 17:45. Sales close at 18:00.
>
> Ad Sales busy until 17:45. Schedule and Broadcast Review stay open.
>
> **Start preparation**

Use the same listing and deadlines in each playtest.

| Model | At 17:15 | Trade-off to test |
| --- | --- | --- |
| Immediate | Offers are available now. | Baseline for speed and ease of use. |
| Shared capacity | One consequential station job can run at a time. A conflicting booking must wait until 17:45. | Work competes for the player's attention and the station's time. A quiet period can turn into idle waiting. |
| Desk capacity | One Ad Sales job can run at a time. Another listing must wait, but a separate booking can proceed. | Deadlines still compete within Ad Sales. The clock leaves the player somewhere useful to go. |

For the first playtest, share an identical 17:15 save position and also try an
empty afternoon. Observe whether players change priorities and whether they
spend most of the delay waiting. Reading, comparing, drafting and navigating
stay immediate. Show the due time and the precise capacity held before Start,
then show pending work in the affected desk and notify once when it finishes.

## Boundary cases to decide

- **Expiry.** Preparation creates a listing draft with no financial hold.
  Offers come from current information when it opens. An older offer expiring
  during the delay gives the player no right to accept it afterwards. A
  separate experiment would be needed for time-consuming acceptance and its
  quote reservations.
- **Cancellation.** Before completion, cancel and free the desk on the current
  tick. Elapsed time stays elapsed. Retain an unsent draft. No escrow or sales
  were created by preparation. Try this when the player changes the airing
  and when a higher-priority listing arrives.
- **Due at airtime.** Treat the selling window as half-open. Work completed
  at 18:00 cannot open sales for an airing starting at 18:00. Show this risk
  in advance, mark the listing missed, and allow the show to run with
  previously accepted bookings. Recheck the airing identity and terms at the
  due tick.
- **Pause and speed.** Progress uses explicit simulated time. Pausing holds
  the job, 2× changes pacing, and jumps must process the same due work in the
  same order. Multi-job same-tick ordering would need a documented tie-break.

The 30-minute duration and selected work item are provisional controls for
comparison. No queue or cooldown is implemented here.
