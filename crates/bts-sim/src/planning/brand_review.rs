//! Periodic advertiser reviews based on completed, paid ad-minute delivery.
use super::*;

const WEEK_MINUTES: u32 = 7 * 1440;
const LOW_DELIVERY: f64 = 250.0;
const HIGH_DELIVERY: f64 = 1200.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BidDirection {
    MoreCompetitive,
    MoreCautious,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrandReview {
    pub brand: BrandId,
    pub at: AirMinute,
    pub direction: BidDirection,
}

impl Planner {
    pub fn brand_reviews(&self) -> &[BrandReview] {
        &self.brand_reviews
    }

    pub(super) fn review_brand_bidding(&mut self, at: AirMinute) {
        if at.0 == 0 || !at.0.is_multiple_of(WEEK_MINUTES) {
            return;
        }
        let from = AirMinute(at.0 - WEEK_MINUTES);
        let mut delivered = BTreeMap::<BrandId, (u32, u32, f64)>::new();
        for sale in self.state().economy().minute_sales() {
            let Some(settlement) = &sale.settlement else {
                continue;
            };
            if !(from < settlement.at && settlement.at <= at) || settlement.payment.0 == 0 {
                continue;
            }
            let (deals, minutes, viewer_minutes) = delivered.entry(sale.offer.brand).or_default();
            *deals += 1;
            *minutes += u32::from(settlement.aired_minutes);
            *viewer_minutes += settlement.viewer_minutes;
        }
        for (brand, (deals, minutes, viewer_minutes)) in delivered {
            if deals < 2 || minutes == 0 {
                continue;
            }
            let average = viewer_minutes / f64::from(minutes);
            let previous = self.engine.state.bid_index(brand);
            let next = if average >= HIGH_DELIVERY {
                (previous + 10).min(125)
            } else if average < LOW_DELIVERY {
                (previous - 10).max(75)
            } else {
                previous
            };
            if next != previous {
                self.engine.state.brand_bid_index.insert(brand, next);
                self.brand_reviews.push(BrandReview {
                    brand,
                    at,
                    direction: if next > previous {
                        BidDirection::MoreCompetitive
                    } else {
                        BidDirection::MoreCautious
                    },
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::demo::{MAIN, SNACKS};

    #[test]
    fn equal_purchases_with_opposite_delivery_change_later_quotes_without_moving_cash() {
        let mut world = crate::demo::world();
        for audience in &mut world.audiences {
            audience.availability.fill(0.9);
        }
        let mut planner =
            Planner::new(world, Parameters::default(), MAIN, SimTick(1400), &[]).unwrap();
        let programme = planner.world().programmes[0].id;
        for start in [1080, 2520] {
            let id = planner
                .place_exact(programme, AirMinute(start), 10)
                .unwrap();
            let offer = planner
                .minute_offers(id)
                .unwrap()
                .into_iter()
                .find(|offer| offer.brand == SNACKS)
                .unwrap();
            planner.accept_minutes(&offer).unwrap();
        }
        let total = planner.state().economy().total_money();
        let mut low = planner.clone();
        for audience in &mut low.engine.world.audiences {
            audience.availability.fill(0.0);
        }
        for candidate in [&mut planner, &mut low] {
            candidate.advance_exact(AirMinute(WEEK_MINUTES)).unwrap();
            assert_eq!(candidate.state().economy().total_money(), total);
            assert_eq!(candidate.brand_reviews().len(), 1);
            for sale in candidate.state().economy().minute_sales() {
                assert_eq!(
                    sale.settlement.as_ref().unwrap().payment,
                    sale.offer.payment
                );
            }
        }
        assert_eq!(
            planner.brand_reviews()[0].direction,
            BidDirection::MoreCompetitive
        );
        assert_eq!(low.brand_reviews()[0].direction, BidDirection::MoreCautious);
        assert_eq!(
            low.state().economy().ledger(),
            planner.state().economy().ledger()
        );

        // Equalize the other effects of past viewing before isolating the bid posture.
        low.engine.world = planner.engine.world.clone();
        low.engine.state.hype = planner.engine.state.hype.clone();
        low.engine.state.repetition = planner.engine.state.repetition.clone();
        low.engine.state.familiarity = planner.engine.state.familiarity.clone();
        low.engine.state.loyalty = planner.engine.state.loyalty.clone();
        low.engine.state.progress = planner.engine.state.progress.clone();
        let quote = |mut p: Planner| {
            let id = p
                .place_exact(programme, AirMinute(WEEK_MINUTES + 1080), 10)
                .unwrap();
            p.minute_offers(id)
                .unwrap()
                .into_iter()
                .find(|offer| offer.brand == SNACKS)
                .unwrap()
        };
        let high_quote = quote(planner);
        let low_quote = quote(low);
        assert_eq!(high_quote.forecast_relevant, low_quote.forecast_relevant);
        assert!(high_quote.payment > low_quote.payment);
        println!(
            "Later quote after strong delivery: {} Funds; after weak delivery: {} Funds",
            high_quote.payment.0, low_quote.payment.0
        );
    }
}
