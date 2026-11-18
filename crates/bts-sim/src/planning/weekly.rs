//! Finite weekly placement and independent watch-time preferences.
use super::*;
use serde::Deserialize;

pub const MAX_DAYS: u32 = 21;
const DAY: u32 = 96;
const WEEK: u32 = 7 * DAY;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Weekday(u8);
impl Weekday {
    pub const SATURDAY: Self = Self(5);
    pub fn new(monday_zero: u8) -> Option<Self> {
        (monday_zero < 7).then_some(Self(monday_zero))
    }
    pub fn number(self) -> u8 {
        self.0
    }
    fn offset(self, day: u32) -> Self {
        Self(((u32::from(self.0) + day) % 7) as u8)
    }
}

pub type WatchPattern = [[f64; 3]; 2];

#[derive(Clone, Debug, Default)]
pub(crate) struct ViewingCalendar {
    pub first: Option<Weekday>,
    pub preferences: BTreeMap<AudienceId, WatchPattern>,
}

impl ViewingCalendar {
    pub fn preference<T: Clock>(
        &self,
        viewer: AudienceId,
        at: T,
        hour: usize,
        interval: bool,
    ) -> f64 {
        let (Some(first), Some(pattern)) = (self.first, self.preferences.get(&viewer)) else {
            return 0.5;
        };
        // Planner actions timestamp the end of a quarter hour. Its audience samples
        // belong to that interval's start, including 23:45 at a day boundary.
        let hours = at.elapsed_hours(T::zero()) - if interval { 0.25 } else { 0.0 };
        let day = first.offset((hours.max(0.0) / 24.0).floor() as u32);
        let band = if hour < 12 {
            0
        } else if hour < 18 {
            1
        } else {
            2
        };
        pattern[usize::from(day.0 >= 5)][band]
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Exception {
    pub occurrence: u8,
    pub start_tick: Option<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WeeklyPlan {
    pub programme: ProgrammeId,
    pub weekday: Weekday,
    pub time: u8,
    pub count: u8,
    pub ad_minutes: u8,
    pub exceptions: Vec<Exception>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Placement {
    pub occurrence: u8,
    pub airing: Option<AiringId>,
    pub regular_tick: u32,
    pub start_tick: Option<u32>,
    pub end_tick: Option<u32>,
    pub runtime_minutes: u16,
    pub conflicts: Vec<AiringId>,
    pub issue: Option<&'static str>,
    pub protected: bool,
}

impl Planner {
    pub fn first_weekday(&self) -> Weekday {
        self.engine.calendar.first.unwrap_or(Weekday::SATURDAY)
    }

    pub fn configure_viewing_calendar(
        &mut self,
        first: Weekday,
        patterns: BTreeMap<AudienceId, WatchPattern>,
    ) -> Result<(), PlanError> {
        if self.now().0 != 0
            || self.research.is_some()
            || self.airings().any(|airing| airing.station == self.player)
            || patterns.len() != self.world().audiences.len()
            || self.world().audiences.iter().any(|group| {
                !patterns.get(&group.id).is_some_and(|pattern| {
                    pattern.iter().flatten().all(|v| (0.0..=1.0).contains(v))
                })
            })
        {
            return Err(error(
                "calendar_configuration",
                "Set valid viewing patterns for all groups before play and research.",
            ));
        }
        self.engine.calendar = ViewingCalendar {
            first: Some(first),
            preferences: patterns,
        };
        Ok(())
    }

    pub fn preview_weekly(&self, plan: &WeeklyPlan) -> Result<Vec<Placement>, PlanError> {
        if plan.count == 0
            || plan.count > 3
            || plan.time >= 96
            || plan.ad_minutes > ad_market::MAX_AD_MINUTES
        {
            return Err(error(
                "weekly_plan",
                "Choose one to three weeks, a grid time, and up to 30 ad minutes.",
            ));
        }
        let exceptions: BTreeMap<_, _> = plan
            .exceptions
            .iter()
            .map(|e| (e.occurrence, e.start_tick))
            .collect();
        if exceptions.len() != plan.exceptions.len()
            || exceptions.keys().any(|&index| index >= plan.count)
        {
            return Err(error(
                "weekly_exception",
                "Use at most one exception for each occurrence.",
            ));
        }
        let first = self.first_weekday();
        let current_day = self.now().0 / DAY;
        let offset = (u32::from(plan.weekday.0) + 7 - u32::from(first.offset(current_day).0)) % 7;
        let start = (current_day + offset) * DAY + u32::from(plan.time);
        let first_start = if start < self.now().0 {
            start + WEEK
        } else {
            start
        };
        let mut rows = Vec::new();
        for occurrence in 0..plan.count {
            let regular = first_start + u32::from(occurrence) * WEEK;
            let actual = exceptions
                .get(&occurrence)
                .copied()
                .unwrap_or(Some(regular));
            rows.push(self.placement_row(
                plan.programme,
                plan.ad_minutes,
                occurrence,
                regular,
                actual,
                None,
            )?);
        }
        mark_internal_clashes(&mut rows);
        Ok(rows)
    }

    fn placement_row(
        &self,
        programme: ProgrammeId,
        ads: u8,
        occurrence: u8,
        regular: u32,
        start: Option<u32>,
        airing: Option<AiringId>,
    ) -> Result<Placement, PlanError> {
        let work = self
            .world()
            .programmes
            .iter()
            .find(|p| p.id == programme && self.owns_programme(self.player, programme))
            .ok_or_else(|| {
                error(
                    "programme_not_owned",
                    "Choose a programme from your library.",
                )
            })?;
        let runtime = work.duration_minutes + u16::from(ads);
        let end = start.and_then(|s| s.checked_add(u32::from(runtime).div_ceil(15)));
        let conflicts = match (start, end) {
            (Some(s), Some(e)) => self
                .airings()
                .filter(|a| {
                    a.station == self.player && Some(a.id) != airing && s < a.end.0 && e > a.start.0
                })
                .map(|a| a.id)
                .collect(),
            _ => vec![],
        };
        let issue = start.and_then(|s| {
            self.validate_placement(self.player, programme, SimTick(s), airing, ads)
                .err()
                .map(|failure| failure.code)
        });
        Ok(Placement {
            occurrence,
            airing,
            regular_tick: regular,
            start_tick: start,
            end_tick: end,
            runtime_minutes: runtime,
            conflicts,
            issue,
            protected: false,
        })
    }

    pub fn place_weekly(&mut self, plan: &WeeklyPlan) -> Result<Vec<AiringId>, PlanError> {
        let rows = self.preview_weekly(plan)?;
        if rows.iter().any(|row| row.issue.is_some()) {
            return Err(error(
                "weekly_conflict",
                "Review each occurrence and fix or skip the marked dates.",
            ));
        }
        let mut candidate = self.clone();
        let mut ids = Vec::new();
        for start in rows.iter().filter_map(|row| row.start_tick) {
            let id = candidate.insert(self.player, plan.programme, SimTick(start), None)?;
            candidate.resize_ad_block(id, plan.ad_minutes)?;
            let group = ids.first().copied().unwrap_or(id);
            candidate.airings.get_mut(&id).unwrap().weekly = Some(group);
            ids.push(id);
        }
        if ids.is_empty() {
            return Err(error("weekly_empty", "Keep at least one occurrence."));
        }
        *self = candidate;
        Ok(ids)
    }

    pub fn preview_weekly_edit(&self, id: AiringId, time: u8) -> Result<Vec<Placement>, PlanError> {
        let a = self.airing(id)?;
        let group = a
            .weekly
            .filter(|_| a.station == self.player && time < 96)
            .ok_or_else(|| error("weekly_edit", "Choose a recurring airing and time."))?;
        let targets: Vec<_> = self
            .airings()
            .filter(|row| row.weekly == Some(group) && row.start >= a.start)
            .collect();
        let mut preview = self.clone();
        for target in &targets {
            if self.editable(target.id).is_ok() {
                preview.airings.remove(&target.id);
            }
        }
        let mut rows = Vec::new();
        for (i, target) in targets.into_iter().enumerate() {
            let protected = self.editable(target.id).is_err();
            let start = if protected {
                target.start.0
            } else {
                (target.start.0 / DAY) * DAY + u32::from(time)
            };
            let mut row = preview.placement_row(
                target.programme,
                target.ad_minutes,
                i as u8,
                target.start.0,
                Some(start),
                Some(target.id),
            )?;
            row.protected = protected;
            if protected {
                row.issue = None;
                row.conflicts.clear();
            }
            rows.push(row);
        }
        mark_internal_clashes(&mut rows);
        Ok(rows)
    }

    pub fn edit_weekly(&mut self, id: AiringId, time: u8) -> Result<(), PlanError> {
        let rows = self.preview_weekly_edit(id, time)?;
        if rows.iter().any(|row| row.issue.is_some()) {
            return Err(error(
                "weekly_conflict",
                "One or more proposed recurring moves conflict. Review the dates.",
            ));
        }
        let mut candidate = self.clone();
        candidate.bump()?;
        for row in rows.into_iter().filter(|row| !row.protected) {
            let changed = candidate.airings.get_mut(&row.airing.unwrap()).unwrap();
            changed.start = SimTick(row.start_tick.unwrap());
            changed.end = SimTick(row.end_tick.unwrap());
        }
        *self = candidate;
        Ok(())
    }
}

fn mark_internal_clashes(rows: &mut [Placement]) {
    for index in 0..rows.len() {
        let Some((start, end)) = rows[index].start_tick.zip(rows[index].end_tick) else {
            continue;
        };
        for next in index + 1..rows.len() {
            if rows[next]
                .start_tick
                .zip(rows[next].end_tick)
                .is_some_and(|(s, e)| start < e && s < end)
            {
                for i in [index, next] {
                    if !rows[i].protected {
                        rows[i].issue = Some("occurrence_overlap");
                    }
                }
            }
        }
    }
}
