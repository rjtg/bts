//! A three-episode scripted series using the existing film production stages.
use super::film::FilmPhase;
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
    pub sequel_of: Option<SeriesId>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScriptedEpisode {
    pub series: SeriesId,
    pub sequence: u32,
    pub strain: f64,
}

fn episode_strain(prior: f64, previous: f64, standing: f64, target: f64) -> f64 {
    let departure = ((target - standing).abs() - 0.15).max(0.0);
    let jump = ((target - previous).abs() - 0.20).max(0.0);
    (0.55 * prior + 0.35 * departure + 0.45 * jump).min(0.30)
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
            strain = episode_strain(strain, previous, standing[TENSION], target);
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
                sequel_of: None,
            },
        );
        *self = candidate;
        Ok(series)
    }

    pub fn amend_episode_tension(
        &mut self,
        id: ProductionId,
        tension: f64,
    ) -> Result<(), PlanError> {
        let film = self.film(id)?;
        let Some(episode) = film.episode else {
            return Err(error(
                "not_serial",
                "Choose an episode from a scripted run.",
            ));
        };
        if film.phase != FilmPhase::Pitch || !(0.0..=1.0).contains(&tension) {
            return Err(error(
                "brief_locked",
                "Choose a valid target before writing starts.",
            ));
        }
        let run = &self.scripted_runs[&episode.series];
        let order = run.episodes;
        let standing = run.brief.values[TENSION].unwrap();
        self.bump()?;
        self.films.get_mut(&id).unwrap().ambition.values[TENSION] = Some(tension);
        let mut previous = standing;
        let mut prior = 0.0;
        let mut reached = false;
        for project in order {
            reached |= project == id;
            let film = self.films.get_mut(&project).unwrap();
            let target = film.ambition.values[TENSION].unwrap();
            let episode = film.episode.as_mut().unwrap();
            if reached && film.phase == FilmPhase::Pitch {
                episode.strain = episode_strain(prior, previous, standing, target);
            }
            previous = target;
            prior = episode.strain;
        }
        Ok(())
    }

    pub fn pitch_sequel(&mut self, parent: SeriesId, title: String) -> Result<SeriesId, PlanError> {
        let original = self
            .scripted_runs
            .get(&parent)
            .ok_or_else(|| error("unknown_series", "Choose a scripted show."))?;
        if !original
            .episodes
            .iter()
            .all(|id| self.films[id].phase == FilmPhase::Complete)
        {
            return Err(error(
                "series_unfinished",
                "Finish the current series before making a sequel.",
            ));
        }
        if self
            .scripted_runs()
            .any(|run| run.sequel_of == Some(parent))
        {
            return Err(error(
                "series_sequel_exists",
                "The show already has a sequel.",
            ));
        }
        let subject = self.films[&original.episodes[0]].subject;
        let standing = std::array::from_fn(|i| original.brief.values[i].unwrap());
        let tension = original
            .episodes
            .map(|id| self.films[&id].ambition.values[TENSION].unwrap());
        let cast = original.cast;
        let series = self.pitch_scripted_run(title, subject, standing, tension)?;
        let sequel = self.scripted_runs.get_mut(&series).unwrap();
        sequel.cast = cast;
        sequel.sequel_of = Some(parent);
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
