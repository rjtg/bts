//! Integer inventory and frozen prices for one airing's abstract ad block.
use super::*;

pub const MAX_AD_MINUTES: u8 = 30;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct MinuteSaleId(pub u64);

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MinuteOffer {
    pub airing: AiringId,
    pub station: StationId,
    pub brand: BrandId,
    pub minutes: u8,
    pub listed_minutes: u8,
    pub open_minutes: u8,
    pub payment: Money,
    pub forecast_target_viewers: f64,
    pub forecast_relevant: f64,
    pub demand_pressure: f64,
    pub uncapped_inventory_value: f64,
    pub quoted_at: AirMinute,
    revision: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MinuteSale {
    pub id: MinuteSaleId,
    pub offer: MinuteOffer,
    pub accepted_at: AirMinute,
    pub settlement: Option<MinuteSettlement>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MinuteSettlement {
    pub at: AirMinute,
    pub aired_minutes: u8,
    /// Sampled viewer-minute impressions. These are not unique reach.
    pub viewer_minutes: f64,
    pub payment: Money,
    pub refund: Money,
}

impl Planner {
    pub fn sold_minutes(&self, id: AiringId) -> u8 {
        self.state()
            .economy()
            .minute_sales()
            .filter(|s| s.offer.airing == id)
            .map(|s| s.offer.minutes)
            .sum()
    }

    pub fn resize_ad_block(&mut self, id: AiringId, minutes: u8) -> Result<(), PlanError> {
        let airing = self.upcoming(id)?;
        if airing.filler.is_some() && minutes > 0 {
            return Err(error(
                "filler_inventory",
                "Station fillers cannot sell advertising.",
            ));
        }
        if minutes > MAX_AD_MINUTES || minutes < self.sold_minutes(id) {
            return Err(error(
                "ad_inventory",
                "Choose up to 30 minutes and keep every sold minute.",
            ));
        }
        if airing.contract.is_some() || (minutes > 0 && airing.advertiser.is_some()) {
            return Err(error(
                "legacy_advertising",
                "Clear the legacy advertiser before opening an ad block.",
            ));
        }
        if airing.ad_minutes == minutes {
            return Ok(());
        }
        let end = self.validate_placement(
            self.player,
            airing.programme,
            airing.start,
            Some(id),
            minutes,
        )?;
        self.bump()?;
        let airing = self.airings.get_mut(&id).unwrap();
        airing.ad_minutes = minutes;
        airing.end = end;
        Ok(())
    }

    pub fn minute_offers(&self, id: AiringId) -> Result<Vec<MinuteOffer>, PlanError> {
        let airing = self.upcoming(id)?;
        let remaining = airing.ad_minutes - self.sold_minutes(id);
        if remaining == 0 {
            return Ok(vec![]);
        }
        let forecast = self.forecast_engine();
        let engine = forecast.as_ref().unwrap_or(&self.engine);
        let mut offers = Vec::new();
        for brand in &self.world().brands {
            if self
                .state()
                .economy()
                .minute_sales()
                .any(|sale| sale.offer.airing == id && sale.offer.brand == brand.id)
            {
                continue;
            }
            let (proposal, starts) = self.interval_action(airing.start, Some((id, brand.id)));
            let reach = engine
                .quote_cadence(
                    &proposal,
                    self.player,
                    brand.id,
                    airing.end,
                    Some((&starts, airing.start.availability_index())),
                )
                .map_err(engine_error)?;
            let wanted = (1.0 + 5.0 * reach.demand.pressure).ceil().clamp(1.0, 6.0) as u8;
            let ceiling = brand.maximum_offer.0.min(reach.demand.available_budget.0);
            let scarcity = 1.0 / (1.0 + f64::from(airing.ad_minutes) / 15.0);
            let per_minute = reach.uncapped_price * scarcity / 5.0;
            if per_minute <= 0.0 || ceiling == 0 {
                continue;
            }
            let affordable = ((ceiling as f64 / per_minute).floor() as u8).max(1);
            let minutes = wanted.min(remaining).min(affordable);
            let uncapped = per_minute * f64::from(minutes);
            let payment = Money(uncapped.min(ceiling as f64).floor() as u64);
            if payment.0 > 0 {
                offers.push(MinuteOffer {
                    airing: id,
                    station: self.player,
                    brand: brand.id,
                    minutes,
                    listed_minutes: airing.ad_minutes,
                    open_minutes: remaining,
                    payment,
                    forecast_target_viewers: reach
                        .forecast
                        .audiences
                        .iter()
                        .map(|a| a.viewers)
                        .sum(),
                    forecast_relevant: reach.forecast.relevant_delivery,
                    demand_pressure: reach.demand.pressure,
                    uncapped_inventory_value: uncapped,
                    quoted_at: self.now_minute(),
                    revision: self.revision,
                });
            }
        }
        Ok(offers)
    }

    pub fn accept_minutes(&mut self, offer: &MinuteOffer) -> Result<MinuteSaleId, PlanError> {
        if offer.revision != self.revision || !self.minute_offers(offer.airing)?.contains(offer) {
            return Err(error(
                "stale_minute_offer",
                "The market changed. Refresh offers for this airing.",
            ));
        }
        let mut candidate = self.clone();
        candidate.bump()?;
        let now = candidate.now_minute();
        let id = candidate
            .engine
            .state
            .economy
            .reserve_minutes(now, offer.clone())
            .map_err(engine_error)?;
        *self = candidate;
        Ok(id)
    }

    // Sold minutes are consecutive, in sale order, after programme content.
    // Any unsold part of the block follows the purchased minutes.
    pub(super) fn spots(&self) -> Vec<(MinuteSaleId, StationId, BrandId, AirMinute, AirMinute)> {
        let mut offsets = BTreeMap::<AiringId, u32>::new();
        self.state()
            .economy()
            .minute_sales()
            .filter_map(|sale| {
                let airing = &self.airings[&sale.offer.airing];
                if sale.settlement.is_some() || airing.cancelled() {
                    return None;
                }
                let offset = offsets
                    .entry(airing.id)
                    .or_insert_with(|| airing.end.0 - u32::from(airing.ad_minutes));
                let start = AirMinute(*offset);
                *offset += u32::from(sale.offer.minutes);
                Some((
                    sale.id,
                    sale.offer.station,
                    sale.offer.brand,
                    start,
                    AirMinute(*offset),
                ))
            })
            .collect()
    }

    pub(super) fn next_airtime_change(&self, from: AirMinute) -> AirMinute {
        self.airings
            .values()
            .flat_map(|a| {
                [
                    a.start,
                    a.end,
                    if a.contract.is_some() {
                        AirMinute(a.start.0 + 1)
                    } else {
                        a.start
                    },
                ]
            })
            .chain(
                self.spots()
                    .into_iter()
                    .flat_map(|(_, _, _, start, end)| [start, end]),
            )
            .filter(|time| *time > from)
            .min()
            .unwrap_or_else(|| AirMinute::from_tick(self.end))
    }

    pub(super) fn minute_campaigns(
        &self,
        start: AirMinute,
        end: AirMinute,
    ) -> BTreeMap<(StationId, BrandId), u8> {
        self.spots()
            .into_iter()
            .filter(|(_, _, _, begin, finish)| *begin <= start && end <= *finish)
            .map(|(_, station, brand, _, _)| ((station, brand), (end.0 - start.0) as u8))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::demo::{self, MAIN, SNACKS};

    #[test]
    fn a_funded_airing_with_zero_audience_still_pays_and_creates_no_hype() {
        let mut world = demo::world();
        for group in &mut world.audiences {
            group.availability = [0.8; 24];
        }
        let mut p = Planner::new(world, Parameters::default(), MAIN, SimTick(672), &[]).unwrap();
        let id = p.place(ProgrammeId(1), SimTick(72), None).unwrap();
        p.resize_ad_block(id, 15).unwrap();
        let q = p
            .minute_offers(id)
            .unwrap()
            .into_iter()
            .find(|o| o.brand == SNACKS)
            .unwrap();
        p.accept_minutes(&q).unwrap();
        // Controlled change after acceptance, standing in for an audience shock.
        for group in &mut p.engine.world.audiences {
            group.availability = [0.0; 24];
        }
        let events = p.advance_exact(p.airing(id).unwrap().end).unwrap();
        assert!(events
            .iter()
            .flat_map(|e| &e.hype)
            .all(|h| h.campaign_generated == 0.0));
        let sale = p.state().economy().minute_sales().next().unwrap();
        let outcome = sale.settlement.as_ref().unwrap();
        assert_eq!(outcome.aired_minutes, q.minutes);
        assert_eq!(outcome.viewer_minutes, 0.0);
        assert_eq!(outcome.payment, q.payment);
        assert_eq!(outcome.refund, Money(0));
    }
}
