//! Optional ordinary minute-market purchases for recurring airings.
use super::ad_market::MinuteSaleId;
use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum RunKey {
    Weekly(AiringId),
    Discussion(SeriesId),
    Newsroom(SeriesId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct AutosellPolicy {
    pub minimum_per_minute: Money,
    pub max_minutes: u8,
}

impl Planner {
    pub(super) fn recurring_key(&self, airing: &Airing) -> Option<RunKey> {
        if let Some(weekly) = airing.weekly {
            return Some(RunKey::Weekly(weekly));
        }
        if let Some(news) = self
            .newsroom()
            .filter(|news| news.editions.iter().any(|e| e.airing == Some(airing.id)))
        {
            return Some(RunKey::Newsroom(news.series));
        }
        self.discussion_runs()
            .find(|run| run.editions.iter().any(|e| e.airing == Some(airing.id)))
            .map(|run| RunKey::Discussion(run.series))
    }

    pub fn autosell_policy(&self, airing: &Airing) -> Option<AutosellPolicy> {
        self.autosell.get(&self.recurring_key(airing)?).copied()
    }

    pub fn autosell_eligible(&self, airing: &Airing) -> bool {
        airing.station == self.player && self.recurring_key(airing).is_some()
    }

    pub fn is_automatic_sale(&self, id: MinuteSaleId) -> bool {
        self.automatic_sales.contains(&id)
    }

    pub fn set_autosell(
        &mut self,
        id: AiringId,
        policy: Option<AutosellPolicy>,
    ) -> Result<(), PlanError> {
        let airing = self.airing(id)?;
        let key = self
            .recurring_key(airing)
            .filter(|_| airing.station == self.player)
            .ok_or_else(|| {
                error(
                    "autosell_run_required",
                    "Choose a recurring airing to set the run’s ad policy.",
                )
            })?;
        if policy.is_some_and(|p| {
            p.minimum_per_minute.validate().is_err()
                || p.minimum_per_minute.0 == 0
                || !(1..=ad_market::MAX_AD_MINUTES).contains(&p.max_minutes)
        }) {
            return Err(error(
                "invalid_autosell_policy",
                "Choose a positive whole-Funds floor and a limit of 1–30 minutes.",
            ));
        }
        self.bump()?;
        if let Some(policy) = policy {
            self.autosell.insert(key, policy);
        } else {
            self.autosell.remove(&key);
        }
        Ok(())
    }

    pub(super) fn run_autosell(&mut self, start: AirMinute) -> Result<(), PlanError> {
        if self.autosell.is_empty() {
            return Ok(());
        }
        let mut due: Vec<_> = self
            .airings
            .values()
            .filter_map(|airing| {
                let policy = self.autosell_policy(airing)?;
                (airing.station == self.player
                    && airing.ad_minutes > 0
                    && airing.contract.is_none()
                    && !airing.started()
                    && airing.start >= start
                    && airing.start.0 - start.0 <= 60
                    && !self.autosell_checked.contains(&airing.id))
                .then_some((airing.start, airing.id, policy))
            })
            .collect();
        due.sort_by_key(|(at, id, _)| (*at, *id));
        for (_, id, policy) in due {
            self.autosell_checked.insert(id);
            loop {
                let remaining = policy.max_minutes.saturating_sub(self.sold_minutes(id));
                if remaining == 0 {
                    break;
                }
                let offer = self
                    .minute_offers(id)?
                    .into_iter()
                    .filter(|offer| {
                        offer.minutes <= remaining
                            && offer.payment.0 / u64::from(offer.minutes)
                                >= policy.minimum_per_minute.0
                    })
                    .max_by(|a, b| {
                        let per_minute = |o: &ad_market::MinuteOffer| o.payment.0 as u128;
                        (per_minute(a) * u128::from(b.minutes))
                            .cmp(&(per_minute(b) * u128::from(a.minutes)))
                            .then_with(|| b.brand.cmp(&a.brand))
                    });
                let Some(offer) = offer else { break };
                let sale = self.accept_minutes(&offer)?;
                self.automatic_sales.insert(sale);
            }
        }
        Ok(())
    }
}
