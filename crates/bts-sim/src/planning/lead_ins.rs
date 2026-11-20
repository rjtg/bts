//! Previously tuned cohorts at the opening minute of a successor.
use super::*;

const HANDOFF_MINUTES: u32 = 15;

impl Planner {
    pub fn potential_lead_in(&self, airing: &Airing) -> Option<&Airing> {
        if airing.filler.is_some() || airing.cancelled() {
            return None;
        }
        self.airings()
            .filter(|previous| {
                previous.station == airing.station
                    && previous.end <= airing.start
                    && previous.filler.is_none()
                    && !previous.cancelled()
            })
            .max_by_key(|previous| previous.end)
            .filter(|previous| Self::handoff_gap(previous, airing) < HANDOFF_MINUTES)
    }

    fn handoff_gap(previous: &Airing, next: &Airing) -> u32 {
        next.start.0 - previous.end.0 + u32::from(previous.ad_minutes)
    }

    pub(super) fn lead_in_cohorts(
        &self,
        start: AirMinute,
    ) -> BTreeMap<(StationId, AudienceId), f64> {
        let mut cohorts = BTreeMap::new();
        for airing in self.airings().filter(|a| a.start == start) {
            let Some(previous) = self.potential_lead_in(airing) else {
                continue;
            };
            let Some(result) = &previous.result else {
                continue;
            };
            let retained =
                1.0 - f64::from(Self::handoff_gap(previous, airing)) / f64::from(HANDOFF_MINUTES);
            for allocation in &result.allocations {
                if let Some(viewing) = allocation
                    .viewing
                    .iter()
                    .find(|v| v.station == previous.station)
                {
                    cohorts.insert(
                        (airing.station, allocation.audience),
                        viewing.viewers * retained,
                    );
                }
            }
        }
        cohorts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::demo::{MAIN, YOUNG};

    #[test]
    fn lead_in_is_bounded_and_a_gap_or_weaker_successor_reduces_retention() {
        let mut world = crate::demo::world();
        for audience in &mut world.audiences {
            audience.availability.fill(0.9);
        }
        let mut unfamiliar = world.programmes[0].clone();
        unfamiliar.id = ProgrammeId(99);
        unfamiliar.kind = ProgrammeKind::Standalone;
        unfamiliar.profile = FeatureProfile::fiction([0.05, 0.05, 0.95, 0.05, 0.05, 0.05, 0.05]);
        world.programmes.push(unfamiliar);
        let mut base = Planner::new(world, Parameters::default(), MAIN, SimTick(192), &[]).unwrap();
        let first = base
            .place_exact(ProgrammeId(1), AirMinute(1080), 0)
            .unwrap();
        let boundary = base.airing(first).unwrap().end;
        base.advance_exact(boundary).unwrap();
        let result = base.airing(first).unwrap().result.as_ref().unwrap();
        let previous = result
            .allocations
            .iter()
            .find(|a| a.audience == YOUNG)
            .unwrap()
            .viewing[0]
            .viewers;
        let follow = |programme, extra| {
            let mut planner = base.clone();
            let start = AirMinute(boundary.0 + extra);
            let second = planner.place_exact(programme, start, 0).unwrap();
            let known = planner
                .potential_lead_in(planner.airing(second).unwrap())
                .is_some();
            let events = planner.advance_exact(AirMinute(start.0 + 1)).unwrap();
            let allocation = events
                .last()
                .unwrap()
                .allocations
                .iter()
                .find(|a| a.audience == YOUNG)
                .unwrap();
            let view = &allocation.viewing[0];
            assert!(
                (view.viewers + allocation.not_watching - f64::from(allocation.population)).abs()
                    < 1e-8
            );
            assert!(view.carried_viewers <= previous.min(allocation.available));
            assert!(view.carried_viewers <= view.viewers);
            (known, view.carried_viewers)
        };
        let (known, immediate) = follow(ProgrammeId(1), 0);
        let (_, weaker) = follow(ProgrammeId(99), 0);
        let (gap_known, gap) = follow(ProgrammeId(1), 15);
        assert!(known);
        assert!(!gap_known);
        assert!(immediate > weaker && weaker > gap);
        assert_eq!(gap, 0.0);
        println!("Estimated returning viewers: {immediate:.1} without a gap, {weaker:.1} for a weaker successor, {gap:.1} after a 15-minute gap");
    }
}
