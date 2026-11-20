//! Prepaid date-specific host agreements for discussion productions.
use super::production::{PersonId, Phase, ProductionId, Role};
use super::*;

#[derive(Clone, Debug)]
pub struct HostAgreement {
    pub id: ProductionId,
    pub host: PersonId,
    pub total: Money,
    pub engagements: Vec<HostEngagement>,
}

#[derive(Clone, Copy, Debug)]
pub struct HostEngagement {
    pub production: ProductionId,
    pub start: SimTick,
    pub end: SimTick,
    pub fee: Money,
}

impl Planner {
    pub fn host_agreements(&self) -> impl Iterator<Item = &HostAgreement> {
        self.host_agreements.values()
    }

    pub fn host_engagement(&self, id: ProductionId) -> Option<HostEngagement> {
        self.host_agreements()
            .flat_map(|hire| &hire.engagements)
            .find(|date| date.production == id)
            .copied()
    }

    pub fn production_balance_due(&self, id: ProductionId) -> Result<Money, PlanError> {
        let production = self.production().project(id)?;
        let prepaid = self
            .host_engagement(id)
            .map_or(0, |engagement| engagement.fee.0);
        Ok(Money(production.concept.budget.0 - prepaid))
    }

    pub(super) fn host_reserved_elsewhere(
        &self,
        host: PersonId,
        project: ProductionId,
        start: SimTick,
        end: SimTick,
    ) -> bool {
        self.host_agreements()
            .filter(|hire| hire.host == host)
            .flat_map(|hire| &hire.engagements)
            .any(|date| date.production != project && start < date.end && end > date.start)
    }

    pub fn sign_host(
        &mut self,
        host: PersonId,
        ids: &[ProductionId],
    ) -> Result<ProductionId, PlanError> {
        if !(1..=3).contains(&ids.len())
            || ids.iter().copied().collect::<BTreeSet<_>>().len() != ids.len()
        {
            return Err(error(
                "host_dates",
                "Choose one to three distinct recording dates.",
            ));
        }
        let person = self
            .production()
            .roster()
            .find(|p| {
                p.id == host
                    && p.role == Role::Host
                    && self
                        .staff_owner(host)
                        .is_none_or(|owner| owner == self.player)
            })
            .ok_or_else(|| error("host_unavailable", "Choose an eligible station host."))?;
        let mut engagements = Vec::<HostEngagement>::new();
        let mut total = Money(0);
        let run = self.discussion_edition(ids[0]).map(|(series, _, _)| series);
        if ids.len() > 1 && run.is_none() {
            return Err(error(
                "host_show_required",
                "Multi-edition agreements cover one recurring discussion run.",
            ));
        }
        for &id in ids {
            let project = self.production().project(id)?;
            let start = project.concept.record_at;
            let end = project.recording_end;
            if project.phase != Phase::Draft
                || project.concept.host != host
                || self.host_engagement(id).is_some()
                || AirMinute::from_tick(start) < self.now_minute()
                || end > person.normal_until
                || (ids.len() > 1
                    && self.discussion_edition(id).map(|(series, _, _)| series) != run)
            {
                return Err(error(
                    "host_date_unavailable",
                    "Choose unbooked future recording dates for this host and show.",
                ));
            }
            if self.host_reserved_elsewhere(host, id, start, end)
                || engagements
                    .iter()
                    .any(|date| start < date.end && end > date.start)
                || self.production().projects().any(|other| {
                    other.phase != Phase::Draft
                        && other.concept.host == host
                        && start < other.recording_end
                        && end > other.concept.record_at
                })
            {
                return Err(error(
                    "talent_conflict",
                    "This host has an existing booking during one of these dates.",
                ));
            }
            total = total.add(person.fee).map_err(engine_error)?;
            engagements.push(HostEngagement {
                production: id,
                start,
                end,
                fee: person.fee,
            });
        }
        if self.state().economy().cash(self.player).unwrap() < total {
            return Err(error(
                "insufficient_production_funds",
                "Station cash cannot cover the guaranteed host fees.",
            ));
        }
        engagements.sort_by_key(|date| (date.start, date.production));
        let id = ProductionId(self.production.next_project);
        let next =
            id.0.checked_add(1)
                .ok_or_else(|| error("capacity", "Production identifiers exhausted."))?;
        let mut candidate = self.clone();
        let now = self.now_minute();
        let economy = &mut candidate.engine.state.economy;
        if total.0 > 0 {
            economy
                .reserve_production(now, self.player, id, total)
                .map_err(engine_error)?;
            economy.spend_production(now, id).map_err(engine_error)?;
        }
        candidate.bump()?;
        candidate.production.next_project = next;
        candidate.host_agreements.insert(
            id,
            HostAgreement {
                id,
                host,
                total,
                engagements,
            },
        );
        *self = candidate;
        Ok(id)
    }
}
