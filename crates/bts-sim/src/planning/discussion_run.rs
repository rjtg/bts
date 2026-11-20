//! Finite weekly discussion runs. Each edition uses the ordinary production path.
use super::production::{Concept, Phase, ProductionId, StudioId};
use super::*;

const EDITIONS: u32 = 3;
const WEEK_TICKS: u32 = 7 * 96;
pub(super) const DISCUSSION_MINUTES: u16 = 45;

#[derive(Clone, Debug)]
pub struct DiscussionRun {
    pub series: SeriesId,
    pub brief: Concept,
    pub editions: Vec<DiscussionEdition>,
}

#[derive(Clone, Debug)]
pub struct DiscussionEdition {
    pub production: ProductionId,
    pub air_at: AirMinute,
    pub ad_minutes: u8,
    pub airing: Option<AiringId>,
    pub(super) reserved: bool,
}

impl DiscussionEdition {
    pub fn reserved_end(&self) -> Option<AirMinute> {
        self.reserved.then(|| {
            AirMinute(self.air_at.0 + u32::from(DISCUSSION_MINUTES) + u32::from(self.ad_minutes))
        })
    }
}

impl Planner {
    pub fn discussion_runs(&self) -> impl Iterator<Item = &DiscussionRun> {
        self.discussion_runs.values()
    }

    pub fn create_discussion_run(
        &mut self,
        brief: Concept,
        first_air: AirMinute,
        ad_minutes: u8,
    ) -> Result<SeriesId, PlanError> {
        if ad_minutes > ad_market::MAX_AD_MINUTES {
            return Err(error("ad_inventory", "Choose up to 30 ad minutes."));
        }
        let mut candidate = self.clone();
        let id = candidate.register_series(brief.title.trim().to_owned(), 0.0)?;
        let mut editions = Vec::new();
        for index in 0..EDITIONS {
            let repeat = index * WEEK_TICKS;
            let mut concept = brief.clone();
            concept.title = format!("{} · {}", brief.title.trim(), index + 1);
            concept.record_at =
                SimTick(brief.record_at.0.checked_add(repeat).ok_or_else(|| {
                    error(
                        "invalid_recording_time",
                        "Recording falls outside the scenario.",
                    )
                })?);
            let air_at = AirMinute(
                first_air
                    .0
                    .checked_add(repeat * 15)
                    .ok_or_else(|| error("invalid_time", "Airing falls outside the scenario."))?,
            );
            let production = candidate.create_production(concept)?;
            let edition = DiscussionEdition {
                production,
                air_at,
                ad_minutes,
                airing: None,
                reserved: false,
            };
            candidate.validate_edition_time(&edition)?;
            editions.push(edition);
        }
        candidate.discussion_runs.insert(
            id,
            DiscussionRun {
                series: id,
                brief,
                editions,
            },
        );
        *self = candidate;
        Ok(id)
    }

    pub(super) fn discussion_edition(
        &self,
        production: ProductionId,
    ) -> Option<(SeriesId, usize, &DiscussionEdition)> {
        self.discussion_runs.values().find_map(|run| {
            run.editions
                .iter()
                .enumerate()
                .find(|(_, edition)| edition.production == production)
                .map(|(index, edition)| (run.series, index, edition))
        })
    }

    pub fn book_discussion_edition(
        &mut self,
        id: ProductionId,
        studio: StudioId,
    ) -> Result<(), PlanError> {
        if self.discussion_edition(id).is_none() {
            return Err(error(
                "unknown_edition",
                "Choose an edition from the show binder.",
            ));
        }
        self.book_discussion(id, None, 0, Some(studio))
    }

    pub(super) fn validate_edition_time(
        &self,
        edition: &DiscussionEdition,
    ) -> Result<(), PlanError> {
        let production = self.production().project(edition.production)?;
        if edition.air_at < AirMinute::from_tick(production.ready_at)
            || self.edition_end(edition)? > AirMinute::from_tick(self.end)
        {
            return Err(error(
                "edition_delivery",
                "Leave three hours from recording to airtime and room for the whole edition.",
            ));
        }
        Ok(())
    }

    fn edition_end(&self, edition: &DiscussionEdition) -> Result<AirMinute, PlanError> {
        edition
            .air_at
            .0
            .checked_add(u32::from(DISCUSSION_MINUTES) + u32::from(edition.ad_minutes))
            .map(AirMinute)
            .ok_or_else(|| error("invalid_time", "Airing falls outside the scenario."))
    }

    pub(super) fn overlaps_reserved_edition(
        &self,
        station: StationId,
        start: AirMinute,
        end: AirMinute,
    ) -> bool {
        station == self.player
            && self
                .discussion_runs()
                .flat_map(|run| &run.editions)
                .any(|edition| {
                    edition.reserved
                        && start < self.edition_end(edition).expect("validated reservation")
                        && end > edition.air_at
                })
    }

    pub(super) fn validate_edition_reservation(
        &self,
        edition: &DiscussionEdition,
    ) -> Result<(), PlanError> {
        self.validate_edition_time(edition)?;
        let end = self.edition_end(edition)?;
        if self.overlaps_reserved_edition(self.player, edition.air_at, end)
            || self.newsroom().is_some_and(|news| {
                news.editions.iter().any(|bulletin| {
                    bulletin.reserved_end().is_some_and(|reserved_end| {
                        edition.air_at < reserved_end && end > bulletin.air_at
                    })
                })
            })
            || self.airings().any(|airing| {
                airing.station == self.player && edition.air_at < airing.end && end > airing.start
            })
        {
            return Err(error(
                "overlap",
                "The edition overlaps an airing or another reserved edition.",
            ));
        }
        Ok(())
    }

    pub(super) fn release_discussion_edition(
        &mut self,
        production: ProductionId,
        programme: ProgrammeId,
    ) -> Result<(), PlanError> {
        if let Some((series, index, edition)) = self.discussion_edition(production) {
            let (air_at, minutes) = (edition.air_at, edition.ad_minutes);
            self.discussion_runs.get_mut(&series).unwrap().editions[index].reserved = false;
            let airing = self.place_exact(programme, air_at, minutes)?;
            self.discussion_runs.get_mut(&series).unwrap().editions[index].airing = Some(airing);
        }
        Ok(())
    }

    pub fn discussion_edition_missed(&self, edition: &DiscussionEdition) -> bool {
        self.production()
            .project(edition.production)
            .is_ok_and(|production| {
                production.phase == Phase::Draft
                    && AirMinute::from_tick(production.concept.record_at) < self.now_minute()
            })
    }
}
