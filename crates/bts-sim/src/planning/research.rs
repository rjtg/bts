//! Station estimates, refreshed only at paid polling boundaries.
use super::*;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Default, Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Polling {
    #[default]
    Off,
    Daily,
    EveryThreeDays,
}

impl Polling {
    fn terms(self) -> Option<(u32, Money)> {
        match self {
            Self::Off => None,
            Self::Daily => Some((96, Money(60))),
            Self::EveryThreeDays => Some((288, Money(90))),
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct AudienceEstimate {
    #[serde(skip)]
    pub(super) audience: Audience,
    pub population: u32,
    pub typical_age: u8,
    #[serde(serialize_with = "serialize_tastes")]
    pub tastes: [f64; 10],
    pub tick: u32,
    pub familiarity_level: &'static str,
    pub uncertainty: f64,
    pub watch_time_note: Option<String>,
    #[serde(skip)]
    watch_pattern: Option<super::weekly::WatchPattern>,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct InsightEntry {
    pub tick: u32,
    pub audience: AudienceId,
    pub spent: u32,
    pub remaining: u32,
}

#[derive(Clone, Debug)]
struct GroupKnowledge {
    age: u8,
    familiarity: f64,
    estimate: AudienceEstimate,
}

#[derive(Clone, Debug)]
pub struct Research {
    polling: Polling,
    next: Option<SimTick>,
    missed: bool,
    seed: u64,
    pub(super) balance: u32,
    initial: u32,
    ledger: Vec<InsightEntry>,
    groups: BTreeMap<AudienceId, GroupKnowledge>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GroupView {
    pub id: AudienceId,
    pub name: String,
    pub estimate: AudienceEstimate,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ResearchView {
    pub polling: Polling,
    pub next_tick: Option<u32>,
    pub missed_poll: bool,
    pub insight: u32,
    pub opening_insight: u32,
    pub spending: Vec<InsightEntry>,
    pub groups: Vec<GroupView>,
}

fn noise(mut n: u64) -> f64 {
    n = (n ^ (n >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    n = (n ^ (n >> 27)).wrapping_mul(0x94d049bb133111eb);
    ((n ^ (n >> 31)) >> 32) as f64 / f64::from(u32::MAX) * 2.0 - 1.0
}

fn estimate(
    seed: u64,
    truth: &Audience,
    age: u8,
    familiarity: f64,
    tick: SimTick,
) -> AudienceEstimate {
    let uncertainty = 0.04 + 0.26 * (1.0 - familiarity);
    let mut attribute = 0_u64;
    let mut error = || {
        attribute += 1;
        let key = noise_key(seed, truth.id, attribute);
        let mut bias = noise(key);
        for day in 1..=tick.0 / 96 {
            bias = 0.98 * bias + 0.02 * noise(key.wrapping_add(u64::from(day)));
        }
        uncertainty * bias
    };
    let mut audience = truth.clone();
    audience.population = (f64::from(truth.population) * (1.0 + error()))
        .round()
        .clamp(1.0, f64::from(u32::MAX)) as u32;
    let typical_age = (f64::from(age) + 60.0 * error()).round().clamp(0.0, 120.0) as u8;
    for value in audience
        .feature_preferences
        .iter_mut()
        .chain(&mut audience.product_interests)
        .chain([&mut audience.purchasing_power])
    {
        *value = (*value + error()).clamp(0.0, 1.0);
    }
    for value in &mut audience.availability {
        *value = (*value * (1.0 + error())).clamp(0.0, 1.0);
    }
    AudienceEstimate {
        population: audience.population,
        typical_age,
        tastes: audience.feature_preferences,
        tick: tick.0,
        familiarity_level: if familiarity >= 0.65 {
            "High"
        } else if familiarity >= 0.3 {
            "Moderate"
        } else {
            "Low"
        },
        uncertainty,
        watch_time_note: None,
        watch_pattern: None,
        audience,
    }
}

impl AudienceEstimate {
    fn with_watch_pattern(
        mut self,
        seed: u64,
        actual: Option<&super::weekly::WatchPattern>,
    ) -> Self {
        let Some(actual) = actual else {
            return self;
        };
        let mut pattern = *actual;
        let mut highest = (0, 0, -1.0_f64);
        let mut lowest = 1.0_f64;
        for (day, bands) in pattern.iter_mut().enumerate() {
            for (band, value) in bands.iter_mut().enumerate() {
                let key = noise_key(seed, self.audience.id, 101 + (day * 3 + band) as u64);
                let mut bias = noise(key);
                for day in 1..=self.tick / 96 {
                    bias = 0.98 * bias + 0.02 * noise(key.wrapping_add(u64::from(day)));
                }
                *value = (*value + self.uncertainty * bias).clamp(0.0, 1.0);
                if *value > highest.2 {
                    highest = (day, band, *value);
                }
                lowest = lowest.min(*value);
            }
        }
        self.watch_time_note = Some(if highest.2 - lowest < 0.15 {
            "No strong watch-time preference in this survey.".into()
        } else {
            format!(
                "{} {} look promising.",
                ["Weekday", "Weekend"][highest.0],
                ["mornings", "afternoons", "evenings"][highest.1]
            )
        });
        self.watch_pattern = Some(pattern);
        self
    }
}

fn noise_key(seed: u64, id: AudienceId, attribute: u64) -> u64 {
    seed ^ (u64::from(id.0) << 40) ^ (attribute << 16)
}

impl Planner {
    pub fn configure_research(
        &mut self,
        seed: u64,
        insight: u32,
        ages: BTreeMap<AudienceId, u8>,
    ) -> Result<(), PlanError> {
        if self.now().0 != 0
            || self.airings.values().any(|a| a.station == self.player)
            || self.research.is_some()
        {
            return Err(error("research_locked", "Configure research before play."));
        }
        if ages.len() != self.world().audiences.len()
            || self
                .world()
                .audiences
                .iter()
                .any(|a| ages.get(&a.id).is_none_or(|age| *age > 120))
        {
            return Err(error(
                "invalid_ages",
                "Provide a typical age for every audience.",
            ));
        }
        let groups = self
            .world()
            .audiences
            .iter()
            .map(|a| {
                let age = ages[&a.id];
                (
                    a.id,
                    GroupKnowledge {
                        age,
                        familiarity: 0.25,
                        estimate: estimate(seed, a, age, 0.25, SimTick(0))
                            .with_watch_pattern(seed, self.engine.calendar.preferences.get(&a.id)),
                    },
                )
            })
            .collect();
        self.research = Some(Research {
            polling: Polling::Off,
            next: None,
            missed: false,
            seed,
            balance: insight,
            initial: insight,
            ledger: vec![],
            groups,
        });
        Ok(())
    }

    pub fn research_view(&self) -> Option<ResearchView> {
        let r = self.research.as_ref()?;
        Some(ResearchView {
            polling: r.polling,
            next_tick: r.next.map(|t| t.0),
            missed_poll: r.missed,
            insight: r.balance,
            opening_insight: r.initial,
            spending: r.ledger.clone(),
            groups: r
                .groups
                .iter()
                .map(|(&id, g)| GroupView {
                    id,
                    name: g.estimate.audience.name.clone(),
                    estimate: g.estimate.clone(),
                })
                .collect(),
        })
    }

    fn research_group(&self, id: AudienceId) -> Result<&GroupKnowledge, PlanError> {
        self.research
            .as_ref()
            .and_then(|r| r.groups.get(&id))
            .ok_or_else(|| error("unknown_research_group", "Choose a known audience group."))
    }

    pub fn set_polling(&mut self, polling: Polling) -> Result<(), PlanError> {
        let research = self
            .research
            .as_ref()
            .ok_or_else(|| error("research_unavailable", "Audience research is unavailable."))?;
        if research.polling == polling {
            return Ok(());
        }
        let next = polling
            .terms()
            .map(|(period, _)| SimTick((self.now().0 / period + 1) * period));
        self.bump()?;
        let research = self.research.as_mut().unwrap();
        research.polling = polling;
        research.next = next;
        research.missed = false;
        Ok(())
    }

    pub fn spend_insight(&mut self, id: AudienceId, amount: u32) -> Result<(), PlanError> {
        self.research_group(id)?;
        let balance = self.research.as_ref().unwrap().balance;
        if amount == 0 || amount > balance {
            return Err(error(
                "invalid_insight",
                "Spend a positive amount within your Insight balance.",
            ));
        }
        let tick = self.now().0;
        self.bump()?;
        let r = self.research.as_mut().unwrap();
        let group = r.groups.get_mut(&id).unwrap();
        group.familiarity += (1.0 - group.familiarity) * (1.0 - 0.85_f64.powf(f64::from(amount)));
        r.balance -= amount;
        r.ledger.push(InsightEntry {
            tick,
            audience: id,
            spent: amount,
            remaining: r.balance,
        });
        Ok(())
    }

    pub(super) fn poll_boundary(&mut self, at: SimTick) -> Result<(), PlanError> {
        let Some(r) = self.research.as_mut() else {
            return Ok(());
        };
        for group in r.groups.values_mut() {
            group.familiarity *= 0.9995;
        }
        if r.next != Some(at) {
            return Ok(());
        }
        let (period, price) = r.polling.terms().unwrap();
        r.next = Some(SimTick(at.0 + period));
        r.missed = self.engine.state.economy.cash(self.player).unwrap() < price;
        if r.missed {
            return Ok(());
        }
        self.engine
            .state
            .economy
            .charge_poll(AirMinute::from_tick(at), self.player, price)
            .map_err(engine_error)?;
        for (&id, group) in &mut r.groups {
            let truth = self
                .engine
                .world
                .audiences
                .iter()
                .find(|a| a.id == id)
                .unwrap();
            group.estimate = estimate(r.seed, truth, group.age, group.familiarity, at)
                .with_watch_pattern(r.seed, self.engine.calendar.preferences.get(&id));
        }
        Ok(())
    }

    pub(super) fn forecast_engine(&self) -> Option<Engine<AirMinute>> {
        let r = self.research.as_ref()?;
        let mut engine = self.engine.clone();
        for audience in &mut engine.world.audiences {
            let published = &r.groups[&audience.id].estimate;
            if let Some(pattern) = published.watch_pattern {
                engine.calendar.preferences.insert(audience.id, pattern);
            }
            *audience = published.audience.clone();
        }
        Some(engine)
    }
}

fn serialize_tastes<S: serde::Serializer>(
    values: &[f64; 10],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    use serde::ser::SerializeMap;
    let mut map = serializer.serialize_map(Some(values.len()))?;
    for (name, value) in FEATURE_NAMES.into_iter().zip(values) {
        let half_stars = (value * 10.0).round() as u8;
        map.serialize_entry(name, &half_stars)?;
    }
    map.end()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poll_watch_patterns_are_used_for_quotes_without_replacing_actual_preferences() {
        use crate::planning::weekly::Weekday;
        let world = crate::demo::world();
        let ages = world.audiences.iter().map(|a| (a.id, 42)).collect();
        let patterns: BTreeMap<_, _> = world
            .audiences
            .iter()
            .map(|a| (a.id, [[0.35; 3], [0.65; 3]]))
            .collect();
        let mut planner = Planner::new(
            world,
            Parameters::default(),
            crate::demo::MAIN,
            SimTick(300),
            &[],
        )
        .unwrap();
        planner
            .configure_viewing_calendar(Weekday::SATURDAY, patterns.clone())
            .unwrap();
        planner.configure_research(719, 8, ages).unwrap();
        let group = crate::demo::YOUNG;
        let opening = planner.forecast_engine().unwrap().calendar.preferences[&group];
        assert_ne!(opening, patterns[&group]);
        planner.spend_insight(group, 8).unwrap();
        assert_eq!(
            opening,
            planner.forecast_engine().unwrap().calendar.preferences[&group]
        );
        planner.set_polling(Polling::Daily).unwrap();
        planner.advance_to(SimTick(96)).unwrap();
        let forecast = planner.forecast_engine().unwrap();
        assert_ne!(forecast.calendar.preferences[&group], opening);
        assert_eq!(planner.engine.calendar.preferences, patterns);
        assert!(planner.research_view().unwrap().groups[0]
            .estimate
            .watch_time_note
            .is_some());
    }

    #[test]
    fn forecast_uses_published_feature_preferences_without_replacing_actual_tastes() {
        let world = crate::demo::world();
        let ages = world.audiences.iter().map(|a| (a.id, 42)).collect();
        let truth = world.audiences[0].feature_preferences;
        let mut planner = Planner::new(
            world,
            Parameters::default(),
            crate::demo::MAIN,
            SimTick(300),
            &[],
        )
        .unwrap();
        planner.configure_research(719, 8, ages).unwrap();
        let id = AudienceId(1);
        let opening = planner.research_group(id).unwrap().estimate.tastes;
        assert_ne!(opening, truth);
        planner.spend_insight(id, 8).unwrap();
        assert_eq!(
            planner.forecast_engine().unwrap().world.audiences[0].feature_preferences,
            opening
        );
        planner.set_polling(Polling::Daily).unwrap();
        planner.advance_to(SimTick(96)).unwrap();
        let published = planner.research_group(id).unwrap().estimate.tastes;
        assert_ne!(published, opening);
        assert_eq!(
            planner.forecast_engine().unwrap().world.audiences[0].feature_preferences,
            published
        );
        assert_eq!(planner.world().audiences[0].feature_preferences, truth);
    }

    #[test]
    fn insight_has_diminishing_gains_and_unaffordable_polls_are_skipped() {
        let mut world = crate::demo::world();
        world
            .stations
            .iter_mut()
            .find(|s| s.id == crate::demo::MAIN)
            .unwrap()
            .initial_cash = Money(60);
        let ages = world.audiences.iter().map(|a| (a.id, 42)).collect();
        let mut planner = Planner::new(
            world,
            Parameters::default(),
            crate::demo::MAIN,
            SimTick(300),
            &[],
        )
        .unwrap();
        planner.configure_research(719, 4, ages).unwrap();
        let id = AudienceId(1);
        let initial = planner.research_group(id).unwrap().familiarity;
        planner.spend_insight(id, 1).unwrap();
        let first = planner.research_group(id).unwrap().familiarity;
        planner.spend_insight(id, 1).unwrap();
        let second = planner.research_group(id).unwrap().familiarity;
        assert!(first > initial && second > first && second - first < first - initial);
        planner.set_polling(Polling::Daily).unwrap();
        planner.advance_to(SimTick(193)).unwrap();
        let view = planner.research_view().unwrap();
        assert_eq!(view.insight, 2);
        assert_eq!(view.spending.len(), 2);
        assert!(view.missed_poll);
        assert_eq!(view.groups[0].estimate.tick, 96);
        assert_eq!(view.next_tick, Some(288));
        assert_eq!(
            planner.state().economy().cash(crate::demo::MAIN),
            Some(Money(0))
        );
    }

    #[test]
    fn successive_biases_stay_close_and_probabilities_stay_bounded() {
        let world = crate::demo::world();
        for seed in [1, 719, u64::MAX] {
            for group in &world.audiences {
                let first = estimate(seed, group, 42, 0.25, SimTick(0));
                let second = estimate(seed, group, 42, 0.25, SimTick(96));
                assert!(second.population > 0);
                for ((before, after), actual) in first
                    .tastes
                    .into_iter()
                    .zip(second.tastes)
                    .zip(group.feature_preferences)
                {
                    assert!((0.0..=1.0).contains(&after));
                    assert!((before - after).abs() <= 0.04 * second.uncertainty);
                    if (before - actual).abs() > 0.04 * second.uncertainty {
                        assert!((before - actual) * (after - actual) > 0.0);
                    }
                }
                assert!(estimate(seed, group, 42, 1.0, SimTick(0)).uncertainty > 0.0);
            }
        }
    }
}
