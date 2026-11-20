//! A three-episode scripted series using the existing film production stages.
use super::production::{PersonId, ProductionId, Subject};
use super::*;

const TENSION: usize = 2;
const EPISODES: usize = 3;

#[derive(Clone, Debug)]
pub struct ScriptedRun {
    pub series: SeriesId,
    pub title: String,
    pub brief: FeatureProfile,
    pub episodes: [ProductionId; EPISODES],
    pub cast: Option<[PersonId; 2]>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScriptedEpisode {
    pub series: SeriesId,
    pub sequence: u32,
    pub strain: f64,
}

impl Planner {
    pub fn scripted_runs(&self) -> impl Iterator<Item = &ScriptedRun> {
        self.scripted_runs.values()
    }

    pub fn pitch_scripted_run(
        &mut self,
        title: String,
        subject: Subject,
        standing: [f64; 7],
        tension: [f64; EPISODES],
    ) -> Result<SeriesId, PlanError> {
        let title = title.trim().to_owned();
        let brief = FeatureProfile::fiction(standing);
        if title.is_empty()
            || title.chars().count() > 45
            || title.chars().any(char::is_control)
            || !brief.is_valid()
            || tension.iter().any(|v| !(0.0..=1.0).contains(v))
        {
            return Err(error(
                "series_pitch",
                "Choose a title and valid standing and episode ambitions.",
            ));
        }
        let mut candidate = self.clone();
        let series = candidate.register_series(title.clone(), 0.65)?;
        let mut episodes = [ProductionId(0); EPISODES];
        let mut previous = standing[TENSION];
        let mut strain: f64 = 0.0;
        for (index, (project, target)) in episodes.iter_mut().zip(tension).enumerate() {
            let departure = ((target - standing[TENSION]).abs() - 0.15).max(0.0);
            let jump = ((target - previous).abs() - 0.20).max(0.0);
            strain = (0.55 * strain + 0.35 * departure + 0.45 * jump).min(0.30);
            let mut ambition = standing;
            ambition[TENSION] = target;
            let sequence = index as u32 + 1;
            *project =
                candidate.pitch_film(format!("{title} · Episode {sequence}"), subject, ambition)?;
            candidate.films.get_mut(project).unwrap().episode = Some(ScriptedEpisode {
                series,
                sequence,
                strain,
            });
            previous = target;
        }
        candidate.scripted_runs.insert(
            series,
            ScriptedRun {
                series,
                title,
                brief,
                episodes,
                cast: None,
            },
        );
        *self = candidate;
        Ok(series)
    }

    pub(super) fn validate_series_cast(
        &self,
        episode: ScriptedEpisode,
        cast: [PersonId; 2],
    ) -> Result<(), PlanError> {
        let run = &self.scripted_runs[&episode.series];
        if let Some(standing) = run.cast {
            if cast != standing {
                return Err(error(
                    "continuity_cast",
                    "Keep the established actors in their recurring roles.",
                ));
            }
        } else if episode.sequence != 1 {
            return Err(error(
                "continuity_cast",
                "Book the first episode to establish the recurring cast.",
            ));
        }
        Ok(())
    }
}
