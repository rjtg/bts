//! Opt-in research for a script and one specific casting question.
use super::film::{audition_fit, FilmPhase};
use super::production::{PersonId, ProductionId};
use super::*;
use serde::Serialize;

const STUDY_TICKS: u32 = 4;
const STUDY_REWARD: u32 = 1;
const MAX_AUDITION_INSIGHT: u32 = 3;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PreparationStudy {
    pub film: ProductionId,
    pub ready: SimTick,
    pub collected: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AuditionEvidence {
    pub film: ProductionId,
    pub person: PersonId,
    pub role: usize,
    pub spent: u32,
    pub observed: SimTick,
    pub fit: &'static str,
    pub uncertainty: &'static str,
}

#[derive(Clone, Debug, Default)]
pub(super) struct TalentResearch {
    studies: BTreeMap<ProductionId, PreparationStudy>,
    auditions: BTreeMap<(ProductionId, PersonId, usize), AuditionEvidence>,
}

impl Planner {
    pub fn study_script(&mut self, id: ProductionId) -> Result<(), PlanError> {
        if self.research.is_none() {
            return Err(error("research_unavailable", "There is no research desk."));
        }
        let film = self
            .films
            .get(&id)
            .ok_or_else(|| error("unknown_film", "Choose a project."))?;
        if !matches!(film.phase, FilmPhase::ScriptReview | FilmPhase::Casting) {
            return Err(error(
                "script_study_phase",
                "Study a delivered script before booking the shoot.",
            ));
        }
        if self.talent_research.studies.contains_key(&id) {
            return Err(error(
                "script_study_exists",
                "There is already a study for this script.",
            ));
        }
        let ready = SimTick(self.now().0 + STUDY_TICKS);
        if ready > self.end {
            return Err(error(
                "outside_scenario",
                "The study would finish after the season.",
            ));
        }
        self.bump()?;
        self.talent_research.studies.insert(
            id,
            PreparationStudy {
                film: id,
                ready,
                collected: false,
            },
        );
        Ok(())
    }

    pub fn audition(
        &mut self,
        id: ProductionId,
        person: PersonId,
        role: usize,
        spent: u32,
    ) -> Result<(), PlanError> {
        let research = self
            .research
            .as_ref()
            .ok_or_else(|| error("research_unavailable", "There is no research desk."))?;
        if role >= 2 || spent == 0 || spent > MAX_AUDITION_INSIGHT || spent > research.balance {
            return Err(error(
                "audition_terms",
                "Choose a role and spend one to three available Insight.",
            ));
        }
        let film = self
            .films
            .get(&id)
            .ok_or_else(|| error("unknown_film", "Choose a project."))?;
        if film.phase != FilmPhase::Casting {
            return Err(error(
                "audition_phase",
                "Accept the script before auditioning.",
            ));
        }
        let key = (id, person, role);
        if self.talent_research.auditions.contains_key(&key) {
            return Err(error(
                "audition_exists",
                "An audition already covers this actor and role.",
            ));
        }

        // The role emphasizes a different subset of the delivered script.
        let fit = audition_fit(film.script.as_ref().unwrap(), person, role)?;
        let mut key_bits = u64::from(id.0) * 71 + u64::from(person.0) * 911 + role as u64 * 13;
        key_bits ^= key_bits >> 7;
        key_bits = key_bits.wrapping_mul(0x9e3779b97f4a7c15);
        let bias = f64::from((key_bits >> 48) as u16) / f64::from(u16::MAX) * 2.0 - 1.0;
        let uncertainty = 0.08 + 0.32 * 0.65_f64.powi(spent as i32);
        let estimate = fit + uncertainty * bias;
        let evidence = AuditionEvidence {
            film: id,
            person,
            role,
            spent,
            observed: self.now(),
            fit: if estimate >= 0.75 {
                "Promising"
            } else if estimate >= 0.52 {
                "Mixed"
            } else {
                "Tentative"
            },
            uncertainty: if spent >= 3 { "Moderate" } else { "High" },
        };
        self.bump()?;
        self.research.as_mut().unwrap().balance -= spent;
        self.talent_research.auditions.insert(key, evidence);
        Ok(())
    }

    pub fn script_studies(&self) -> impl Iterator<Item = &PreparationStudy> {
        self.talent_research.studies.values()
    }

    pub fn audition_evidence(&self) -> impl Iterator<Item = &AuditionEvidence> {
        self.talent_research.auditions.values()
    }

    pub(super) fn finish_script_studies(&mut self, at: SimTick) -> Result<(), PlanError> {
        let Some(research) = &mut self.research else {
            return Ok(());
        };
        for study in self
            .talent_research
            .studies
            .values_mut()
            .filter(|study| !study.collected && study.ready <= at)
        {
            research.balance = research
                .balance
                .checked_add(STUDY_REWARD)
                .ok_or_else(|| error("insight_overflow", "The Insight account is full."))?;
            study.collected = true;
        }
        Ok(())
    }
}
