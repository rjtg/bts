//! A bounded rental pilot for film bookings.
use super::*;
use bts_fuzzy::{Condition, Degree, Membership, Rule, RuleSet};

fn overlap_share(start: SimTick, end: SimTick, other: (SimTick, SimTick)) -> f64 {
    let ticks = end
        .0
        .min(other.1 .0)
        .saturating_sub(start.0.max(other.0 .0));
    f64::from(ticks) / f64::from(end.0 - start.0)
}

pub(super) fn rental_price(base: Money, pressure: f64) -> Money {
    let busy = Condition::is(0, "booked capacity", Membership::rising(0.0, 3.0).unwrap());
    let weight = Degree::new(1.0).unwrap();
    let rules = RuleSet::new(
        vec![
            Rule::new("quiet", busy.clone().negate(), weight, 1.0).unwrap(),
            Rule::new("busy", busy, weight, 2.5).unwrap(),
        ],
        1.0,
    )
    .unwrap();
    let multiplier = rules.evaluate(&[(0, pressure)]).unwrap().output;
    Money((base.0 as f64 * multiplier).round() as u64)
}

impl Planner {
    /// Public fixture reservations in the shared rental market, in 15-minute ticks.
    pub fn configure_rental_demand(
        &mut self,
        bookings: Vec<(SimTick, SimTick)>,
    ) -> Result<(), PlanError> {
        if bookings
            .iter()
            .any(|(start, end)| start >= end || *end > self.end)
        {
            return Err(error(
                "rental_interval",
                "Studio demand requires a valid recording interval.",
            ));
        }
        self.bump()?;
        self.rental_demand = bookings;
        Ok(())
    }

    pub(super) fn rental_pressure(&self, start: SimTick, end: SimTick) -> f64 {
        self.rental_demand
            .iter()
            .copied()
            .chain(self.films().filter_map(|film| {
                let (booking, terms) = film.booking.zip(film.quote.as_ref())?;
                Some((booking.start, terms.recording_end))
            }))
            .map(|other| overlap_share(start, end, other))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contiguous_reservations_do_not_overlap_and_half_counts_half() {
        let start = SimTick(4);
        let end = SimTick(12);
        assert_eq!(overlap_share(start, end, (SimTick(0), SimTick(4))), 0.0);
        assert_eq!(overlap_share(start, end, (SimTick(8), SimTick(16))), 0.5);
        assert_eq!(overlap_share(start, end, (SimTick(0), SimTick(16))), 1.0);
        for (pressure, expected) in [(0.0, 300), (0.5, 375), (1.0, 450), (3.0, 750), (4.0, 750)] {
            assert_eq!(rental_price(Money(300), pressure), Money(expected));
        }
    }
}
