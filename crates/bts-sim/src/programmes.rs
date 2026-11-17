//! Bounded provisional familiarity, attachment and continuity mechanics.
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct BroadcastRecord<T: Clock = SimHour> {
    pub at: T,
    pub station: StationId,
    pub programme: ProgrammeId,
}
/// Pre-slot evidence and selection factors; none are confidence estimates.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SelectionHistory {
    pub familiarity: f64,
    pub prior_familiarity: f64,
    pub predecessor_familiarity: f64,
    pub predecessor_reception: Option<f64>,
    pub continuity_strength: f64,
    pub attachment_factor: f64,
    pub repeat_factor: f64,
    pub reception_factor: f64,
    pub continuity_factor: f64,
    pub frontier_before: Option<u32>,
    pub frontier_after: Option<u32>,
}
impl<T: Clock> State<T> {
    pub fn familiarity(&self, audience: AudienceId, programme: ProgrammeId) -> Option<f64> {
        self.familiarity.get(&(audience, programme)).copied()
    }
    pub fn reception(&self, audience: AudienceId, programme: ProgrammeId) -> Option<f64> {
        self.reception.get(&(audience, programme)).copied()
    }
    pub fn series_progress(&self, audience: AudienceId, series: SeriesId) -> Option<u32> {
        self.progress.get(&(audience, series)).copied()
    }
    pub fn broadcast_history(&self) -> &[BroadcastRecord<T>] {
        &self.broadcasts
    }
}
impl<T: Clock> Engine<T> {
    pub(crate) fn selection_history(
        &self,
        audience: AudienceId,
        programme: &Programme,
    ) -> Result<SelectionHistory, Error> {
        let familiarity = self.state.familiarity[&(audience, programme.id)];
        let mut history = SelectionHistory {
            familiarity,
            prior_familiarity: 1.0,
            predecessor_familiarity: 0.0,
            predecessor_reception: None,
            continuity_strength: 0.0,
            attachment_factor: 1.0,
            repeat_factor: 1.0 - 0.35 * familiarity,
            reception_factor: 1.0,
            continuity_factor: 1.0,
            frontier_before: None,
            frontier_after: None,
        };
        if let ProgrammeKind::Episode { series, sequence } = programme.kind {
            let definition = self
                .world
                .series
                .iter()
                .find(|s| s.id == series)
                .expect("validated series");
            history.continuity_strength = definition.continuity_strength;
            history.attachment_factor = 0.2 + 1.6 * self.state.loyalty[&(audience, series)];
            history.frontier_before = self.state.series_progress(audience, series);
            history.frontier_after = history.frontier_before;
            if sequence > 1 {
                let earlier=self.world.programmes.iter().filter(|p|matches!(p.kind,ProgrammeKind::Episode {series:s,sequence:n} if s==series&&n<sequence));
                history.prior_familiarity = earlier
                    .clone()
                    .map(|p| self.state.familiarity[&(audience, p.id)])
                    .sum::<f64>()
                    / f64::from(sequence - 1);
                if let Some(previous) = earlier
                    .into_iter()
                    .find(|p| p.sequence() == Some(sequence - 1))
                {
                    history.predecessor_familiarity =
                        self.state.familiarity[&(audience, previous.id)];
                    history.predecessor_reception = self.state.reception(audience, previous.id);
                    if let Some(satisfaction) = history.predecessor_reception {
                        history.reception_factor =
                            1.0 + 0.6 * history.predecessor_familiarity * (satisfaction - 0.5);
                    }
                }
                history.continuity_factor =
                    1.0 - 0.6 * definition.continuity_strength * (1.0 - history.prior_familiarity);
            }
        }
        unit(history.familiarity, "work familiarity")?;
        unit(history.prior_familiarity, "prior familiarity")?;
        Ok(history)
    }
    pub(crate) fn update_programme_history(
        &self,
        next: &mut State<T>,
        allocations: &mut [Allocation],
    ) -> Result<(), Error> {
        for allocation in allocations {
            let audience = allocation.audience;
            let population = f64::from(allocation.population);
            let mut works: BTreeMap<ProgrammeId, (f64, f64)> = BTreeMap::new();
            let mut series_totals: BTreeMap<SeriesId, (f64, f64)> = BTreeMap::new();
            for v in &allocation.viewing {
                let work = works.entry(v.programme).or_default();
                work.0 += v.viewers;
                work.1 += v.viewers * v.satisfaction.trace.output;
                if let Some(series) = v.series {
                    let total = series_totals.entry(series).or_default();
                    total.0 += v.fresh_viewers / population;
                    total.1 += v.fresh_viewers / population * v.satisfaction.trace.output;
                }
            }
            for (programme, (viewers, satisfaction_sum)) in works {
                if viewers == 0.0 {
                    continue;
                }
                let old = self.state.familiarity[&(audience, programme)];
                let fraction = viewers / population;
                unit(fraction, "work audience fraction")?;
                let familiar = old + (1.0 - old) * fraction;
                unit(familiar, "updated familiarity")?;
                let reception = satisfaction_sum / viewers;
                unit(reception, "experienced reception")?;
                next.familiarity.insert((audience, programme), familiar);
                next.reception.insert((audience, programme), reception);
                let work = self
                    .world
                    .programmes
                    .iter()
                    .find(|p| p.id == programme)
                    .unwrap();
                if let ProgrammeKind::Episode { series, sequence } = work.kind {
                    let old = next.progress[&(audience, series)];
                    next.progress.insert((audience, series), old.max(sequence));
                }
            }
            for (series, (fraction, satisfaction)) in series_totals {
                let old = self.state.loyalty[&(audience, series)];
                let updated =
                    old + self.parameters.loyalty_learning * (satisfaction - fraction * old);
                unit(updated, "updated series attachment")?;
                next.loyalty.insert((audience, series), updated);
            }
            for v in &mut allocation.viewing {
                v.familiarity_after = next.familiarity[&(audience, v.programme)];
                v.loyalty_after = v.series.and_then(|s| next.loyalty(audience, s));
                v.history.frontier_after = v.series.and_then(|s| next.series_progress(audience, s));
            }
        }
        Ok(())
    }
}
