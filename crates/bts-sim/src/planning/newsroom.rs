//! A finite daily newsroom run with clock-driven production.
use super::production::{PersonId, ProductionId, Role, StudioId, Talent};
use super::*;

const DAYS: u32 = 5;
const REPORTING: u32 = 36;
const STUDIO_START: u32 = 52;
const STUDIO_END: u32 = 54;
const DELIVERY: u32 = 56;
pub const NEWS_MINUTES: u16 = 30;
const STUDIO_FEE: Money = Money(100);

#[derive(Clone, Debug)]
pub struct Newsroom {
    pub series: SeriesId,
    pub title: String,
    pub editions: Vec<NewsEdition>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NewsPhase {
    Planned,
    Reporting,
    Recording,
    Editing,
    Delivered,
    Missed,
}

#[derive(Clone, Debug)]
pub struct NewsEdition {
    pub id: ProductionId,
    pub day: u32,
    pub reporter: PersonId,
    pub anchor: PersonId,
    pub studio: StudioId,
    pub air_at: AirMinute,
    pub ad_minutes: u8,
    pub cost: Money,
    pub phase: NewsPhase,
    pub programme: Option<ProgrammeId>,
    pub airing: Option<AiringId>,
    pub exception: Option<&'static str>,
}

#[derive(Clone, Copy, Debug)]
pub struct NewsStaff {
    pub reporter: PersonId,
    pub anchor: PersonId,
    pub studio: StudioId,
}

impl NewsEdition {
    pub fn reporting_at(&self) -> SimTick {
        SimTick(self.day * 96 + REPORTING)
    }
    pub fn recording_at(&self) -> SimTick {
        SimTick(self.day * 96 + STUDIO_START)
    }
    pub fn recording_end(&self) -> SimTick {
        SimTick(self.day * 96 + STUDIO_END)
    }
    pub fn delivery_at(&self) -> SimTick {
        SimTick(self.day * 96 + DELIVERY)
    }
    fn committed(&self) -> bool {
        matches!(
            self.phase,
            NewsPhase::Reporting | NewsPhase::Recording | NewsPhase::Editing
        )
    }
    pub fn reserved_end(&self) -> Option<AirMinute> {
        self.committed().then(|| self.airing_end())
    }
    fn airing_end(&self) -> AirMinute {
        AirMinute(self.air_at.0 + u32::from(NEWS_MINUTES) + u32::from(self.ad_minutes))
    }
}

impl Planner {
    pub fn newsroom(&self) -> Option<&Newsroom> {
        self.newsroom.as_ref()
    }

    fn news_talent(&self, id: PersonId, role: Role) -> Result<&Talent, PlanError> {
        self.production()
            .roster()
            .find(|person| {
                person.id == id
                    && person.role == role
                    && self
                        .staff_owner(id)
                        .is_none_or(|station| station == self.player)
            })
            .ok_or_else(|| error("news_staff", "Choose an eligible reporter and an anchor."))
    }

    fn news_staff_cost(&self, staff: NewsStaff) -> Result<Money, PlanError> {
        if staff.studio.0 == 0 || staff.studio.0 > self.studio_count(self.player) {
            return Err(error(
                "studio_unavailable",
                "Choose one of your owned studios.",
            ));
        }
        self.news_talent(staff.reporter, Role::Reporter)?
            .fee
            .add(self.news_talent(staff.anchor, Role::Anchor)?.fee)
            .and_then(|fees| fees.add(STUDIO_FEE))
            .map_err(engine_error)
    }

    pub fn start_newsroom(
        &mut self,
        title: String,
        first_day: u32,
        minute_of_day: u16,
        ad_minutes: u8,
        staff: NewsStaff,
    ) -> Result<(), PlanError> {
        if self.newsroom.is_some() {
            return Err(error(
                "newsroom_exists",
                "This fixture supports one daily bulletin.",
            ));
        }
        let title = title.trim().to_owned();
        if title.is_empty() || title.chars().count() > 45 || title.chars().any(char::is_control) {
            return Err(error(
                "invalid_title",
                "Use a bulletin title of 1–45 visible characters.",
            ));
        }
        let runtime = u32::from(NEWS_MINUTES) + u32::from(ad_minutes);
        if ad_minutes > ad_market::MAX_AD_MINUTES
            || first_day.saturating_add(DAYS) > self.end.0 / 96
            || u32::from(minute_of_day) < DELIVERY * 15
            || u32::from(minute_of_day) + runtime > 1440
            || AirMinute((first_day * 96 + REPORTING) * 15) <= self.now_minute()
        {
            return Err(error("news_freshness", "Plan five future days and allow the whole bulletin to air between 14:00 and midnight."));
        }
        let cost = self.news_staff_cost(staff)?;
        let mut candidate = self.clone();
        let series = candidate.register_series(title.clone(), 0.0)?;
        let mut editions = Vec::new();
        for day in first_day..first_day + DAYS {
            let id = ProductionId(candidate.production.next_project);
            candidate.production.next_project =
                id.0.checked_add(1)
                    .ok_or_else(|| error("capacity", "Production identifiers exhausted."))?;
            editions.push(NewsEdition {
                id,
                day,
                reporter: staff.reporter,
                anchor: staff.anchor,
                studio: staff.studio,
                air_at: AirMinute(day * 1440 + u32::from(minute_of_day)),
                ad_minutes,
                cost,
                phase: NewsPhase::Planned,
                programme: None,
                airing: None,
                exception: None,
            });
        }
        candidate.newsroom = Some(Newsroom {
            series,
            title,
            editions,
        });
        candidate.bump()?;
        *self = candidate;
        Ok(())
    }

    pub fn update_newsroom_staff(&mut self, staff: NewsStaff) -> Result<(), PlanError> {
        let cost = self.news_staff_cost(staff)?;
        if !self
            .newsroom()
            .is_some_and(|news| news.editions.iter().any(|e| e.phase == NewsPhase::Planned))
        {
            return Err(error(
                "newsroom_locked",
                "There are no unstarted editions to change.",
            ));
        }
        self.bump()?;
        for edition in &mut self.newsroom.as_mut().unwrap().editions {
            if edition.phase == NewsPhase::Planned {
                edition.reporter = staff.reporter;
                edition.anchor = staff.anchor;
                edition.studio = staff.studio;
                edition.cost = cost;
            }
        }
        Ok(())
    }

    pub(super) fn newsroom_holds_studio(
        &self,
        studio: StudioId,
        from: SimTick,
        to: SimTick,
    ) -> bool {
        self.newsroom().is_some_and(|news| {
            news.editions.iter().any(|edition| {
                edition.committed()
                    && edition.studio == studio
                    && from < edition.recording_end()
                    && to > edition.recording_at()
            })
        })
    }

    pub(super) fn validate_news_freshness(
        &self,
        programme: ProgrammeId,
        start: AirMinute,
        end: AirMinute,
    ) -> Result<(), PlanError> {
        if self.newsroom().is_some_and(|news| {
            news.editions.iter().any(|edition| {
                edition.programme == Some(programme)
                    && (start.0 / 1440 != edition.day || end.0 > (edition.day + 1) * 1440)
            })
        }) {
            return Err(error(
                "news_expired",
                "This bulletin can only air on its reporting day.",
            ));
        }
        Ok(())
    }

    fn news_exception(&self, edition: &NewsEdition) -> Option<&'static str> {
        let reporter = self.news_talent(edition.reporter, Role::Reporter).ok()?;
        let anchor = self.news_talent(edition.anchor, Role::Anchor).ok()?;
        if reporter.normal_until < edition.recording_at()
            || anchor.normal_until < edition.recording_end()
        {
            return Some("Staff unavailable");
        }
        if self.production().projects().any(|p| {
            p.studio == Some(edition.studio)
                && p.concept.record_at < edition.recording_end()
                && p.recording_end > edition.recording_at()
        }) || self.newsroom_holds_studio(
            edition.studio,
            edition.recording_at(),
            edition.recording_end(),
        ) {
            return Some("Studio already booked");
        }
        if self.overlaps_reserved_edition(self.player, edition.air_at, edition.airing_end())
            || self.airings().any(|airing| {
                airing.station == self.player
                    && edition.air_at < airing.end
                    && edition.airing_end() > airing.start
            })
        {
            return Some("Airtime already occupied");
        }
        if self.state().economy().cash(self.player).unwrap() < edition.cost {
            return Some("Production funds unavailable");
        }
        None
    }

    pub(super) fn advance_newsroom(&mut self, at: SimTick) -> Result<(), PlanError> {
        let Some(index) = self.newsroom().and_then(|news| {
            news.editions
                .iter()
                .position(|edition| edition.day == at.0 / 96)
        }) else {
            return Ok(());
        };
        let edition = self.newsroom().unwrap().editions[index].clone();
        let phase = match edition.phase {
            NewsPhase::Planned if at == edition.reporting_at() => {
                if let Some(reason) = self.news_exception(&edition) {
                    self.newsroom.as_mut().unwrap().editions[index].exception = Some(reason);
                    NewsPhase::Missed
                } else {
                    let book = &mut self.engine.state.economy;
                    book.reserve_production(
                        AirMinute::from_tick(at),
                        self.player,
                        edition.id,
                        edition.cost,
                    )
                    .map_err(engine_error)?;
                    book.spend_production(AirMinute::from_tick(at), edition.id)
                        .map_err(engine_error)?;
                    NewsPhase::Reporting
                }
            }
            NewsPhase::Reporting if at == edition.recording_at() => NewsPhase::Recording,
            NewsPhase::Recording if at == edition.recording_end() => NewsPhase::Editing,
            NewsPhase::Editing if at == edition.delivery_at() => {
                self.publish_news(index, &edition)?;
                NewsPhase::Delivered
            }
            _ => return Ok(()),
        };
        self.newsroom.as_mut().unwrap().editions[index].phase = phase;
        Ok(())
    }

    fn publish_news(&mut self, index: usize, edition: &NewsEdition) -> Result<(), PlanError> {
        let number = super::rights::original_programme_number(
            &self.engine.world.programmes,
            self.rights.as_ref(),
            self.production.next_programme,
        )?;
        let following = number
            .checked_add(1)
            .ok_or_else(|| error("capacity", "Programme identifiers exhausted."))?;
        let id = ProgrammeId(number);
        let news = self.newsroom().unwrap();
        let reporter = self.news_talent(edition.reporter, Role::Reporter)?;
        let anchor = self.news_talent(edition.anchor, Role::Anchor)?;
        let role_strength =
            |person: &Talent, index: usize| person.capabilities.0[index].unwrap_or(0.0);
        let profile = FeatureProfile::nonfiction([
            Some(0.10),
            Some(0.15),
            Some(0.35),
            Some(0.55),
            None,
            None,
            Some(0.60),
            Some(role_strength(reporter, 7)),
            Some(0.90),
            Some(role_strength(reporter, 9)),
        ]);
        let execution = 0.6 * reporter.craft + 0.4 * anchor.craft;
        let work = Programme {
            id,
            kind: ProgrammeKind::Episode {
                series: news.series,
                sequence: index as u32 + 1,
            },
            name: format!("{} · Edition {}", news.title, index + 1),
            profile,
            execution,
            awareness: 0.60,
            cast_appeal: anchor.recognition,
            broadcast_cost: Money(40),
            duration_minutes: NEWS_MINUTES,
        };
        self.engine.world.programmes.push(work);
        self.engine.world.validate().map_err(engine_error)?;
        self.production.next_programme = following;
        self.libraries.entry(self.player).or_default().insert(id);
        for audience in &self.engine.world.audiences {
            self.engine.state.familiarity.insert((audience.id, id), 0.0);
        }
        let card = &mut self.newsroom.as_mut().unwrap().editions[index];
        card.phase = NewsPhase::Delivered;
        card.programme = Some(id);
        let airing = self.place_exact(id, edition.air_at, edition.ad_minutes)?;
        self.newsroom.as_mut().unwrap().editions[index].airing = Some(airing);
        Ok(())
    }
}
