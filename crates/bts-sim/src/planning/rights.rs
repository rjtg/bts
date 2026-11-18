//! Fixed-price, exclusive rights and delayed assessments for finished titles.
use super::*;

pub const ASSESSMENT_TICKS: u32 = 12;
pub const RESALE_TICKS: u32 = 96;

#[derive(Clone, Debug)]
pub struct TitleSeed {
    pub programme: Programme,
    pub evidence: &'static str,
    pub art: &'static str,
    pub public_profile: FeatureProfile,
    pub public_execution: f64,
    pub release: SimTick,
    pub deadline: SimTick,
    pub base_price: Money,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TitleStatus {
    Unreleased,
    Listed,
    Expired,
    Owned,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RightsOffer {
    pub token: u64,
    pub price: Money,
    pub deadline: SimTick,
}

#[derive(Clone, Debug)]
pub struct Title {
    pub seed: TitleSeed,
    pub status: TitleStatus,
    pub owner: Option<StationId>,
    pub acquisition: Option<RightsOffer>,
    pub resale: Option<RightsOffer>,
    pub assessment_due: Option<SimTick>,
    pub assessment_at: Option<SimTick>,
    studied: u32,
    pending: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RightsNews {
    pub programme: ProgrammeId,
    pub at: SimTick,
    pub kind: &'static str,
    pub station: Option<StationId>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AssessmentSpend {
    pub programme: ProgrammeId,
    pub at: SimTick,
    pub amount: u32,
    pub balance: u32,
}

#[derive(Clone, Debug)]
pub(super) struct Market {
    rival: StationId,
    next_rival_airing: u32,
    titles: BTreeMap<ProgrammeId, Title>,
    news: Vec<RightsNews>,
    spending: Vec<AssessmentSpend>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TitleForecast {
    pub qualities: FeatureProfile,
    pub expected_viewers: f64,
    pub relative_uncertainty: f64,
    pub groups: Vec<(AudienceId, f64)>,
}

impl Planner {
    pub fn configure_rights(
        &mut self,
        rival: StationId,
        seeds: Vec<TitleSeed>,
    ) -> Result<(), PlanError> {
        if self.now() != SimTick(0)
            || self.rights.is_some()
            || self.production.projects().next().is_some()
            || !self.films.is_empty()
            || self.airings().any(|a| a.station == self.player)
            || rival == self.player
            || self.state().economy().cash(rival).is_none()
        {
            return Err(error(
                "rights_configuration",
                "Configure the rights market before play.",
            ));
        }
        let mut world = self.world().clone();
        let mut titles = BTreeMap::new();
        for seed in seeds {
            if seed.release >= seed.deadline
                || seed.deadline > self.end
                || !seed.public_profile.is_valid()
                || seed.public_profile.production_type != seed.programme.profile.production_type
                || !(0.0..=1.0).contains(&seed.public_execution)
                || seed.base_price.0 == 0
                || seed.base_price.validate().is_err()
                || seed.programme.series().is_some()
            {
                return Err(error(
                    "rights_title",
                    "Invalid market title, evidence, price or date.",
                ));
            }
            world.programmes.push(seed.programme.clone());
            titles.insert(
                seed.programme.id,
                Title {
                    seed,
                    status: TitleStatus::Unreleased,
                    owner: None,
                    acquisition: None,
                    resale: None,
                    assessment_due: None,
                    assessment_at: None,
                    studied: 0,
                    pending: 0,
                },
            );
        }
        world.validate().map_err(engine_error)?;
        let mut candidate = self.clone();
        candidate.rights = Some(Market {
            rival,
            next_rival_airing: u32::MAX,
            titles,
            news: vec![],
            spending: vec![],
        });
        candidate.advance_rights(SimTick(0))?;
        *self = candidate;
        Ok(())
    }

    pub fn titles(&self) -> impl Iterator<Item = &Title> {
        self.rights.iter().flat_map(|market| market.titles.values())
    }

    pub fn rights_news(&self) -> &[RightsNews] {
        self.rights.as_ref().map_or(&[], |m| &m.news)
    }

    pub fn rights_spending(&self) -> &[AssessmentSpend] {
        self.rights.as_ref().map_or(&[], |m| &m.spending)
    }

    fn market_title(&self, id: ProgrammeId) -> Result<&Title, PlanError> {
        self.rights
            .as_ref()
            .and_then(|m| m.titles.get(&id))
            .ok_or_else(|| error("unknown_title", "Choose a title from the rights market."))
    }

    pub fn title_forecast(&self, id: ProgrammeId) -> Result<TitleForecast, PlanError> {
        let title = self.market_title(id)?;
        if title.status == TitleStatus::Unreleased {
            return Err(error(
                "title_unreleased",
                "That title is not yet announced.",
            ));
        }
        Ok(self.estimate_title(title, title.studied))
    }

    fn estimate_title(&self, title: &Title, insight: u32) -> TitleForecast {
        let seed = &title.seed;
        let weight = 0.80 * (1.0 - 0.83_f64.powf(f64::from(insight)));
        let mut qualities = seed.public_profile;
        for (estimate, actual) in qualities
            .values
            .iter_mut()
            .zip(seed.programme.profile.values)
        {
            if let (Some(estimate), Some(actual)) = (estimate, actual) {
                *estimate += weight * (actual - *estimate);
            }
        }
        let execution =
            seed.public_execution + weight * (seed.programme.execution - seed.public_execution);
        let forecast = self.forecast_engine();
        let world = forecast
            .as_ref()
            .map_or(self.world(), |engine| engine.world());
        let groups: Vec<_> = world
            .audiences
            .iter()
            .map(|group| {
                let fit = qualities.fit(&group.feature_preferences);
                let draw = 0.65 * fit.powi(2) + 0.35 * execution;
                (
                    group.id,
                    f64::from(group.population) * group.availability[18] * draw * 0.65,
                )
            })
            .collect();
        TitleForecast {
            qualities,
            expected_viewers: groups.iter().map(|(_, reach)| reach).sum(),
            relative_uncertainty: 0.12 + 0.25 * (1.0 - weight / 0.80),
            groups,
        }
    }

    fn rival_start(&self, title: &Title, after: SimTick) -> Option<SimTick> {
        let rival = self.rights.as_ref()?.rival;
        let duration = u32::from(title.seed.programme.duration_minutes) / 15;
        (after.0 + 1..self.end.0)
            .filter(|tick| matches!(tick % 96, 56 | 60 | 72 | 84))
            .find(|&tick| {
                tick + duration <= self.end.0
                    && !self.airings.values().any(|airing| {
                        airing.station == rival
                            && tick < airing.end.0
                            && tick + duration > airing.start.0
                    })
            })
            .map(SimTick)
    }

    fn rival_can_use(&self, title: &Title, at: SimTick, price: Money) -> bool {
        let rival = self.rights.as_ref().unwrap().rival;
        let cash = self.state().economy().cash(rival).unwrap();
        let minimum = price
            .0
            .saturating_add(title.seed.programme.broadcast_cost.0);
        cash.0 >= minimum
            && self.rival_start(title, at).is_some()
            && self.estimate_title(title, 0).expected_viewers >= 2000.0
    }

    pub fn buy_rights(&mut self, id: ProgrammeId, token: u64) -> Result<(), PlanError> {
        let title = self.market_title(id)?;
        let offer = title
            .acquisition
            .as_ref()
            .filter(|offer| {
                title.status == TitleStatus::Listed
                    && offer.token == token
                    && self.now() < offer.deadline
            })
            .ok_or_else(|| {
                error(
                    "stale_rights_offer",
                    "The acquisition offer is no longer open.",
                )
            })?;
        let mut candidate = self.clone();
        candidate.transfer_title(id, self.player, None, offer.price, "purchased")?;
        *self = candidate;
        Ok(())
    }

    pub fn commission_title_assessment(
        &mut self,
        id: ProgrammeId,
        amount: u32,
    ) -> Result<(), PlanError> {
        let title = self.market_title(id)?;
        let now = self.now();
        let due = SimTick(now.0 + ASSESSMENT_TICKS);
        if !matches!(title.status, TitleStatus::Listed | TitleStatus::Owned)
            || title.assessment_due.is_some()
            || due > self.end
        {
            return Err(error(
                "assessment_unavailable",
                "An assessment cannot be commissioned for this title now.",
            ));
        }
        let available = self
            .research
            .as_ref()
            .map_or(0, |research| research.balance);
        if amount == 0 || amount > available || title.studied.checked_add(amount).is_none() {
            return Err(error(
                "invalid_insight",
                "Spend a positive amount within your Insight balance.",
            ));
        }
        self.bump()?;
        self.research.as_mut().unwrap().balance -= amount;
        let market = self.rights.as_mut().unwrap();
        let title = market.titles.get_mut(&id).unwrap();
        title.assessment_due = Some(due);
        title.pending = amount;
        market.spending.push(AssessmentSpend {
            programme: id,
            at: now,
            amount,
            balance: available - amount,
        });
        Ok(())
    }

    pub fn resale_blocker(&self, id: ProgrammeId) -> Option<AiringId> {
        self.airings()
            .find(|airing| {
                airing.programme == id && airing.station == self.player && airing.end > self.now()
            })
            .map(|a| a.id)
    }

    pub fn list_for_resale(&mut self, id: ProgrammeId) -> Result<(), PlanError> {
        let title = self.market_title(id)?;
        if title.owner != Some(self.player)
            || title.resale.is_some()
            || self.now().0 + RESALE_TICKS > self.end.0
        {
            return Err(error(
                "resale_unavailable",
                "This title cannot be offered for resale now.",
            ));
        }
        let now = self.now();
        let price = Money(((title.acquisition.as_ref().unwrap().price.0 * 70) / 100).max(1));
        if !self.rival_can_use(title, now, price) {
            return Err(error(
                "no_resale_buyer",
                "The rival has no affordable use for this title right now.",
            ));
        }
        self.bump()?;
        let offer = RightsOffer {
            token: self.revision,
            price,
            deadline: SimTick(now.0 + RESALE_TICKS),
        };
        self.rights
            .as_mut()
            .unwrap()
            .titles
            .get_mut(&id)
            .unwrap()
            .resale = Some(offer);
        self.rights_event(id, "resale_bid", Some(self.rights.as_ref().unwrap().rival));
        Ok(())
    }

    pub fn accept_resale(&mut self, id: ProgrammeId, token: u64) -> Result<(), PlanError> {
        let title = self.market_title(id)?;
        let offer = title
            .resale
            .as_ref()
            .filter(|q| {
                title.owner == Some(self.player) && q.token == token && self.now() < q.deadline
            })
            .ok_or_else(|| error("stale_rights_offer", "The resale offer is no longer open."))?;
        if self.resale_blocker(id).is_some() {
            return Err(error(
                "rights_airing_conflict",
                "Resolve the scheduled airing shown on this title before selling.",
            ));
        }
        if !self.rival_can_use(title, self.now(), offer.price) {
            return Err(error(
                "no_resale_buyer",
                "The rival can no longer take this offer.",
            ));
        }
        let rival = self.rights.as_ref().unwrap().rival;
        let start = self.rival_start(title, self.now()).unwrap();
        let mut candidate = self.clone();
        candidate.transfer_title(id, rival, Some(self.player), offer.price, "resold")?;
        candidate.schedule_rival_right(id, start)?;
        *self = candidate;
        Ok(())
    }

    fn schedule_rival_right(
        &mut self,
        programme: ProgrammeId,
        start: SimTick,
    ) -> Result<(), PlanError> {
        let market = self.rights.as_ref().unwrap();
        let station = market.rival;
        let high_id = market.next_rival_airing;
        if high_id <= self.next_id {
            return Err(error("capacity", "Airing identifiers exhausted."));
        }
        let end = self.validate_placement(station, programme, start, None, 0)?;
        self.bump()?;
        self.rights.as_mut().unwrap().next_rival_airing -= 1;
        let id = AiringId(high_id);
        self.airings.insert(
            id,
            Airing {
                id,
                station,
                programme,
                start,
                end,
                advertiser: None,
                ad_minutes: 0,
                weekly: None,
                contract: None,
                result: None,
                snapshot: None,
            },
        );
        Ok(())
    }

    fn rights_event(
        &mut self,
        programme: ProgrammeId,
        kind: &'static str,
        station: Option<StationId>,
    ) {
        let at = self.now();
        self.rights.as_mut().unwrap().news.push(RightsNews {
            programme,
            at,
            kind,
            station,
        });
    }

    fn transfer_title(
        &mut self,
        id: ProgrammeId,
        buyer: StationId,
        seller: Option<StationId>,
        price: Money,
        kind: &'static str,
    ) -> Result<(), PlanError> {
        let now = self.now();
        if self.state().economy().cash(buyer).unwrap() < price {
            return Err(error(
                "insufficient_rights_funds",
                "The buyer cannot afford this title.",
            ));
        }
        self.engine
            .state
            .economy
            .transfer_rights(now, id, buyer, seller, price)
            .map_err(engine_error)?;
        if let Some(seller) = seller {
            self.libraries.get_mut(&seller).unwrap().remove(&id);
        } else {
            let work = self.market_title(id)?.seed.programme.clone();
            self.engine.world.programmes.push(work);
            self.engine.world.validate().map_err(engine_error)?;
            for audience in &self.engine.world.audiences {
                self.engine.state.familiarity.insert((audience.id, id), 0.0);
            }
        }
        self.libraries.entry(buyer).or_default().insert(id);
        let title = self.rights.as_mut().unwrap().titles.get_mut(&id).unwrap();
        title.status = TitleStatus::Owned;
        title.owner = Some(buyer);
        title.resale = None;
        self.bump()?;
        self.rights_event(id, kind, Some(buyer));
        Ok(())
    }

    pub(super) fn advance_rights(&mut self, at: SimTick) -> Result<(), PlanError> {
        let Some(market) = &self.rights else {
            return Ok(());
        };
        let rival = market.rival;
        let ids: Vec<_> = market.titles.keys().copied().collect();
        for id in ids {
            let title = self.market_title(id)?;
            if title.status == TitleStatus::Unreleased && title.seed.release == at {
                let reach = self.estimate_title(title, 0).expected_viewers;
                let base = title
                    .seed
                    .base_price
                    .add(Money((reach / 5.0).floor() as u64))
                    .map_err(engine_error)?;
                let premium = base.add(Money(base.0 / 5)).map_err(engine_error)?;
                let price = if self.rival_can_use(title, title.seed.deadline, premium) {
                    premium
                } else {
                    base
                };
                let offer = RightsOffer {
                    token: u64::from(id.0),
                    price,
                    deadline: title.seed.deadline,
                };
                let title = self.rights.as_mut().unwrap().titles.get_mut(&id).unwrap();
                title.acquisition = Some(offer);
                title.status = TitleStatus::Listed;
                self.rights_event(id, "released", None);
            }
            let title = self.market_title(id)?;
            if title.status == TitleStatus::Listed && title.seed.deadline == at {
                let price = title.acquisition.as_ref().unwrap().price;
                if self.rival_can_use(title, at, price) {
                    let start = self.rival_start(title, at).unwrap();
                    self.transfer_title(id, rival, None, price, "rival_purchased")?;
                    self.schedule_rival_right(id, start)?;
                } else {
                    self.rights
                        .as_mut()
                        .unwrap()
                        .titles
                        .get_mut(&id)
                        .unwrap()
                        .status = TitleStatus::Expired;
                    self.rights_event(id, "expired", None);
                }
            }
            let title = self.rights.as_mut().unwrap().titles.get_mut(&id).unwrap();
            if title
                .resale
                .as_ref()
                .is_some_and(|offer| offer.deadline == at)
            {
                title.resale = None;
            }
            if title.assessment_due == Some(at) {
                title.studied += title.pending;
                title.pending = 0;
                title.assessment_due = None;
                title.assessment_at = Some(at);
                self.rights_event(id, "assessment_ready", Some(self.player));
            }
        }
        Ok(())
    }
}

pub(super) fn original_programme_number(
    programmes: &[Programme],
    rights: Option<&Market>,
    floor: u16,
) -> Result<u16, PlanError> {
    let reserved = |id| rights.is_some_and(|market| market.titles.contains_key(&ProgrammeId(id)));
    let overflow = || error("capacity", "Programme identifiers exhausted.");
    let mut id = programmes
        .iter()
        .map(|p| p.id.0)
        .filter(|id| !reserved(*id))
        .max()
        .unwrap_or(0)
        .checked_add(1)
        .ok_or_else(overflow)?
        .max(floor);
    while reserved(id) {
        id = id.checked_add(1).ok_or_else(overflow)?;
    }
    Ok(id)
}
