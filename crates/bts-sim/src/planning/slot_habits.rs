//! A learned appointment at a recurring run's station and weekly slot.
use super::autosell::RunKey;
use super::*;

const WEEK: u32 = 7 * 1440;
const SLOT_MINUTES: u32 = 15;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct HabitSlot {
    run: RunKey,
    station: StationId,
    quarter: u32,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct Habit {
    week: u32,
    strength: f64,
}

impl Habit {
    fn strength_at(self, week: u32) -> f64 {
        let missed = week.saturating_sub(self.week).saturating_sub(1);
        self.strength * 0.75_f64.powi(missed as i32)
    }
}

impl Planner {
    fn habit_slot(&self, airing: &Airing) -> Option<HabitSlot> {
        Some(HabitSlot {
            run: self.recurring_key(airing)?,
            station: airing.station,
            quarter: (airing.start.0 % WEEK) / SLOT_MINUTES,
        })
    }

    pub fn has_prior_slot_airing(&self, airing: &Airing) -> bool {
        let Some(slot) = self.habit_slot(airing) else {
            return false;
        };
        self.airings().any(|earlier| {
            earlier.start.0 / WEEK < airing.start.0 / WEEK
                && earlier.result.is_some()
                && !earlier.cancelled()
                && self.habit_slot(earlier) == Some(slot)
        })
    }

    pub(super) fn slot_habit_modifiers(
        &self,
        at: AirMinute,
    ) -> BTreeMap<(StationId, AudienceId), f64> {
        if self.slot_habits.is_empty() {
            return BTreeMap::new();
        }
        let mut modifiers = BTreeMap::new();
        for airing in self
            .airings()
            .filter(|airing| airing.start <= at && at < airing.end && !airing.cancelled())
        {
            let Some(slot) = self.habit_slot(airing) else {
                continue;
            };
            let week = airing.start.0 / WEEK;
            for group in &self.world().audiences {
                if let Some(habit) = self
                    .slot_habits
                    .get(&(slot, group.id))
                    .filter(|habit| habit.week < week)
                {
                    modifiers.insert(
                        (airing.station, group.id),
                        1.0 + 0.30 * habit.strength_at(week),
                    );
                }
            }
        }
        modifiers
    }

    pub(super) fn learn_slot_habit(&mut self, airing: &Airing, result: Option<&AiringResult>) {
        let Some(slot) = self.habit_slot(airing) else {
            return;
        };
        let Some(result) = result.filter(|r| !r.off_air) else {
            return;
        };
        let week = airing.start.0 / WEEK;
        for allocation in &result.allocations {
            let Some(view) = allocation
                .viewing
                .iter()
                .find(|v| v.station == airing.station)
            else {
                continue;
            };
            let exposure = if allocation.population > 0 {
                view.viewers / f64::from(allocation.population)
            } else {
                0.0
            };
            let experienced = exposure * view.satisfaction.trace.output;
            let key = (slot, allocation.audience);
            let prior = self
                .slot_habits
                .get(&key)
                .map_or(0.0, |habit| habit.strength_at(week));
            self.slot_habits.insert(
                key,
                Habit {
                    week,
                    strength: 0.55 * prior + 0.45 * experienced,
                },
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::demo::{MAIN, YOUNG};
    use crate::planning::weekly::{Weekday, WeeklyPlan};

    #[test]
    fn stable_slot_keeps_its_appointment_when_the_same_title_moved_loses_it() {
        let mut world = crate::demo::world();
        for group in &mut world.audiences {
            group.availability.fill(0.8);
        }
        let mut base =
            Planner::new(world, Parameters::default(), MAIN, SimTick(2016), &[]).unwrap();
        let ids = base
            .place_weekly(&WeeklyPlan {
                programme: ProgrammeId(1),
                weekday: Weekday::SATURDAY,
                time: 1080,
                count: 3,
                ad_minutes: 0,
                exceptions: vec![],
            })
            .unwrap();
        base.advance_exact(base.airing(ids[0]).unwrap().end)
            .unwrap();
        let mut moved = base.clone();
        let second = base.airing(ids[1]).unwrap().start;
        let late = AirMinute(second.0 + 15);
        moved.move_exact(ids[1], late).unwrap();
        assert!(base.has_prior_slot_airing(base.airing(ids[1]).unwrap()));
        assert!(!moved.has_prior_slot_airing(moved.airing(ids[1]).unwrap()));
        assert!(base.slot_habit_modifiers(second)[&(MAIN, YOUNG)] > 1.0);
        assert!(moved.slot_habit_modifiers(late).is_empty());
        let third = moved.airing(ids[2]).unwrap().start;
        assert!(
            moved.slot_habit_modifiers(third)[&(MAIN, YOUNG)]
                < base.slot_habit_modifiers(second)[&(MAIN, YOUNG)]
        );
        let viewers = |planner: &mut Planner| {
            planner
                .advance_exact(planner.airing(ids[1]).unwrap().end)
                .unwrap();
            let result = planner.airing(ids[1]).unwrap().result.as_ref().unwrap();
            let group = result
                .allocations
                .iter()
                .find(|a| a.audience == YOUNG)
                .unwrap();
            let watched = group.viewing.iter().map(|view| view.viewers).sum::<f64>();
            assert!((watched + group.not_watching - f64::from(group.population)).abs() < 1e-8);
            watched
        };
        let stable = viewers(&mut base);
        let moved = viewers(&mut moved);
        assert!(stable > moved);
        println!("Young viewers: {stable:.1} in the stable slot vs {moved:.1} one quarter later");
    }
}
