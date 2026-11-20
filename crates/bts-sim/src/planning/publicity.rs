//! One exact showing can be listed in advance to build bounded anticipation.
use super::*;

const FULL_NOTICE_MINUTES: u32 = 360;
const MAX_OPENING_GAIN: f64 = 0.08;

impl Planner {
    pub fn publish_airing(&mut self, id: AiringId) -> Result<(), PlanError> {
        let airing = self.upcoming(id)?;
        let now = self.now_minute();
        if airing.filler.is_some() || airing.start <= now {
            return Err(error(
                "listing_time",
                "List a future programme before its airtime.",
            ));
        }
        if airing.published_at.is_some() {
            return Err(error(
                "already_listed",
                "This showing is already in the listings.",
            ));
        }
        self.bump()?;
        self.airings.get_mut(&id).unwrap().published_at = Some(now);
        Ok(())
    }

    pub(super) fn viewer_modifiers(&self, at: AirMinute) -> BTreeMap<(StationId, AudienceId), f64> {
        let mut modifiers = self.slot_habit_modifiers(at);
        for airing in self.airings().filter(|a| a.start == at) {
            let Some(published) = airing.published_at else {
                continue;
            };
            let notice = at.0.saturating_sub(published.0).min(FULL_NOTICE_MINUTES);
            let multiplier =
                1.0 + MAX_OPENING_GAIN * f64::from(notice) / f64::from(FULL_NOTICE_MINUTES);
            for audience in &self.world().audiences {
                *modifiers
                    .entry((airing.station, audience.id))
                    .or_insert(1.0) *= multiplier;
            }
        }
        modifiers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::demo::MAIN;

    #[test]
    fn publication_boosts_only_one_exact_opening_and_moves_remove_the_listing() {
        let mut world = crate::demo::world();
        for group in &mut world.audiences {
            group.availability.fill(0.9);
        }
        let mut baseline =
            Planner::new(world, Parameters::default(), MAIN, SimTick(192), &[]).unwrap();
        let id = baseline
            .place_exact(ProgrammeId(1), AirMinute(1080), 0)
            .unwrap();
        let mut advertised = baseline.clone();
        advertised.publish_airing(id).unwrap();
        assert!(advertised.publish_airing(id).is_err());
        assert!(advertised
            .viewer_modifiers(AirMinute(1080))
            .values()
            .all(|&m| m > 1.0 && m <= 1.08));
        assert!(advertised.viewer_modifiers(AirMinute(1081)).is_empty());
        let mut moved = advertised.clone();
        moved.move_exact(id, AirMinute(1095)).unwrap();
        assert!(moved.airing(id).unwrap().published_at.is_none());
        moved.move_exact(id, AirMinute(1080)).unwrap();
        let end = baseline.airing(id).unwrap().end;
        for planner in [&mut baseline, &mut advertised, &mut moved] {
            planner.advance_exact(end).unwrap();
        }
        let viewers = |planner: &Planner| {
            planner
                .airing(id)
                .unwrap()
                .result
                .as_ref()
                .unwrap()
                .allocations
                .iter()
                .flat_map(|allocation| &allocation.viewing)
                .filter(|view| view.station == MAIN)
                .map(|view| view.viewers)
                .sum::<f64>()
        };
        println!(
            "Opening viewers: internal {:.2}, published {:.2}, withdrawn {:.2}",
            viewers(&baseline),
            viewers(&advertised),
            viewers(&moved),
        );
        assert!(viewers(&advertised) > viewers(&baseline));
        assert_eq!(viewers(&moved), viewers(&baseline));
    }
}
