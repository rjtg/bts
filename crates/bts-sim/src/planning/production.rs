//! One-off discussion production and bounded pooled station Sway.
//! This state is not a player-report DTO. See docs/original-production.md.
use super::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct ProductionId(pub u32);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct PersonId(pub u16);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct StudioId(pub u16);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct RequestId(pub u16);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Actor,
    Director,
    Writer,
    Anchor,
    Reporter,
    Host,
    Guest,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Subject {
    Science,
    EverydayLife,
    Entertainment,
}
impl Subject {
    pub(super) fn index(self) -> usize {
        match self {
            Self::Science => 0,
            Self::EverydayLife => 1,
            Self::Entertainment => 2,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Science => "Science and invention",
            Self::EverydayLife => "Everyday life",
            Self::Entertainment => "Entertainment and culture",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tone {
    Calm,
    Conversational,
    Lively,
}
impl Tone {
    pub fn score(self) -> f64 {
        match self {
            Self::Calm => 0.2,
            Self::Conversational => 0.5,
            Self::Lively => 0.8,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Calm => "Calm",
            Self::Conversational => "Conversational",
            Self::Lively => "Lively",
        }
    }
}
/// Developer configuration. Exact craft/fit are deliberately not in browser DTOs.
#[derive(Clone, Debug, PartialEq)]
pub struct Talent {
    pub id: PersonId,
    pub name: String,
    pub role: Role,
    pub fee: Money,
    pub description: String,
    pub recognition: f64,
    pub craft: f64,
    /// Science, everyday life, entertainment.
    pub subject_fit: [f64; 3],
    pub capabilities: Capabilities,
    pub normal_until: SimTick,
    pub negotiable_from: Option<SimTick>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Concept {
    pub title: String,
    pub host: PersonId,
    pub guests: [PersonId; 2],
    pub subject: Subject,
    pub tone: Tone,
    pub budget: Money,
    pub record_at: SimTick,
}
impl Concept {
    pub fn ambition(&self) -> FeatureProfile {
        let playful = self.tone.score();
        FeatureProfile::nonfiction([
            Some(playful * 0.9),
            Some(playful),
            None,
            Some(0.7),
            None,
            None,
            Some(0.35 + playful * 0.5),
            Some(match self.subject {
                Subject::Science => 0.97,
                Subject::EverydayLife => 0.58,
                Subject::Entertainment => 0.25,
            }),
            None,
            None,
        ])
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Draft,
    Booked,
    Recording,
    PostProduction,
    Completed,
}
impl Phase {
    pub fn label(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Booked => "booked",
            Self::Recording => "recording",
            Self::PostProduction => "post_production",
            Self::Completed => "completed",
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Production {
    pub id: ProductionId,
    pub concept: Concept,
    pub fees: Money,
    pub recording_end: SimTick,
    pub ready_at: SimTick,
    pub phase: Phase,
    pub programme: Option<ProgrammeId>,
    pub booked_at: Option<SimTick>,
    pub studio: Option<StudioId>,
    pub exceptional_person: Option<PersonId>,
    pub cooperation_sway: u32,
    output: Option<Programme>,
    pub outcome: Option<ProductionOutcome>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct QuestAcceptance {
    pub accepted_at: SimTick,
    pub deadline: SimTick,
    pub production: Option<ProductionId>,
    pub fulfilled_at: Option<SimTick>,
}
/// Closed, deliberately small request vocabulary for the discussion experiment.
#[derive(Clone, Debug, PartialEq)]
pub enum RequestCondition {
    CoGuests { first: PersonId, second: PersonId },
    CalmEverydayDiscussion { guest: PersonId },
}
impl RequestCondition {
    pub fn matches(&self, c: &Concept) -> bool {
        match self {
            Self::CoGuests { first, second } => {
                c.guests.contains(first) && c.guests.contains(second)
            }
            Self::CalmEverydayDiscussion { guest } => {
                c.guests.contains(guest)
                    && c.subject == Subject::EverydayLife
                    && c.tone == Tone::Calm
            }
        }
    }
    pub fn achievement(&self) -> &'static str {
        match self {
            Self::CoGuests { .. } => "recording_completed",
            Self::CalmEverydayDiscussion { .. } => "production_completed",
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProductionRequest {
    pub id: RequestId,
    pub proposer: PersonId,
    pub condition: RequestCondition,
    pub statement: String,
    pub reward: u32,
    pub time_limit_ticks: u32,
    pub acceptance: Option<QuestAcceptance>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum SwayReason {
    RequestReward(RequestId),
    ExceptionalParticipation(PersonId),
    Cooperation,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwayEntry {
    pub at: SimTick,
    pub production: ProductionId,
    pub change: i32,
    pub balance: u32,
    pub reason: SwayReason,
}
pub const PARTICIPATION_COST: u32 = 2;
/// Authored collaboration difficulty, independent of individual ability.
#[derive(Clone, Debug, PartialEq)]
pub struct CooperationIssue {
    pub people: [PersonId; 2],
    pub penalty: f64,
    pub known_note: Option<String>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProductionOutcome {
    pub craft: f64,
    pub subject_fit: f64,
    pub capability: f64,
    pub ambition: FeatureProfile,
    pub achieved: FeatureProfile,
    pub resources: f64,
    pub base_execution: f64,
    pub cooperation_penalty: f64,
    pub recovered_penalty: f64,
    pub remaining_penalty: f64,
    pub execution: f64,
    pub issues: Vec<CooperationIssue>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProductionState {
    roster: BTreeMap<PersonId, Talent>,
    issues: Vec<CooperationIssue>,
    projects: BTreeMap<ProductionId, Production>,
    requests: BTreeMap<RequestId, ProductionRequest>,
    sway: u32,
    sway_history: Vec<SwayEntry>,
    pub(super) next_project: u32,
    pub(super) next_programme: u16,
}
impl Default for ProductionState {
    fn default() -> Self {
        Self {
            roster: BTreeMap::new(),
            issues: Vec::new(),
            projects: BTreeMap::new(),
            requests: BTreeMap::new(),
            sway: 0,
            sway_history: Vec::new(),
            next_project: 1,
            next_programme: 1,
        }
    }
}
impl ProductionState {
    pub fn roster(&self) -> impl Iterator<Item = &Talent> {
        self.roster.values()
    }
    pub fn cooperation_issues(&self) -> &[CooperationIssue] {
        &self.issues
    }
    pub fn projects(&self) -> impl Iterator<Item = &Production> {
        self.projects.values()
    }
    pub fn requests(&self) -> impl Iterator<Item = &ProductionRequest> {
        self.requests.values()
    }
    pub fn sway(&self) -> u32 {
        self.sway
    }
    pub fn sway_history(&self) -> &[SwayEntry] {
        &self.sway_history
    }
    fn change_sway(
        &mut self,
        at: SimTick,
        production: ProductionId,
        change: i32,
        reason: SwayReason,
    ) -> Result<(), PlanError> {
        let balance = self.sway.checked_add_signed(change).ok_or_else(|| {
            error(
                "insufficient_sway",
                "Not enough pooled Sway for this booking.",
            )
        })?;
        self.sway = balance;
        self.sway_history.push(SwayEntry {
            at,
            production,
            change,
            balance,
            reason,
        });
        Ok(())
    }
    pub fn project(&self, id: ProductionId) -> Result<&Production, PlanError> {
        self.projects.get(&id).ok_or_else(|| {
            error(
                "unknown_production",
                "Select an existing production in this session.",
            )
        })
    }
    pub(super) fn retain_ids(&mut self, other: &Self) {
        self.next_project = self.next_project.max(other.next_project);
        self.next_programme = self.next_programme.max(other.next_programme);
    }
    pub(super) fn unfinished_programme(&self, id: ProgrammeId) -> bool {
        self.projects
            .values()
            .any(|p| p.programme == Some(id) && p.phase != Phase::Completed)
    }
}
impl Planner {
    pub fn production(&self) -> &ProductionState {
        &self.production
    }
    /// Typed fixture setup, allowed only before simulation or production activity.
    pub fn configure_production(
        &mut self,
        roster: Vec<Talent>,
        requests: Vec<ProductionRequest>,
    ) -> Result<(), PlanError> {
        if self.now().0 != 0
            || !self.production.projects.is_empty()
            || self
                .production
                .requests
                .values()
                .any(|r| r.acceptance.is_some())
        {
            return Err(error(
                "production_config_locked",
                "Production configuration is fixed once play starts.",
            ));
        }
        let mut map = BTreeMap::new();
        for person in roster {
            if person.id.0 == 0
                || map.contains_key(&person.id)
                || person.fee.validate().is_err()
                || person.normal_until > self.end
                || person
                    .negotiable_from
                    .is_some_and(|t| t < person.normal_until || t >= self.end)
            {
                return Err(error(
                    "invalid_roster",
                    "Invalid person identity, fee or booking window.",
                ));
            }
            for value in [person.recognition, person.craft]
                .into_iter()
                .chain(person.subject_fit)
                .chain(person.capabilities.0.into_iter().flatten())
            {
                unit(value, "talent parameter").map_err(engine_error)?;
            }
            map.insert(person.id, person);
        }
        let mut request_map = BTreeMap::new();
        let mut total_reward = 0_u32;
        for r in requests {
            let valid = match r.condition {
                RequestCondition::CoGuests { first, second } => {
                    first != second
                        && [first, second]
                            .iter()
                            .all(|id| map.get(id).is_some_and(|t| t.role == Role::Guest))
                }
                RequestCondition::CalmEverydayDiscussion { guest } => {
                    map.get(&guest).is_some_and(|t| t.role == Role::Guest)
                }
            };
            if r.id.0 == 0
                || request_map.contains_key(&r.id)
                || !map.contains_key(&r.proposer)
                || !valid
                || r.acceptance.is_some()
                || r.time_limit_ticks == 0
                || r.reward == 0
                || r.reward > i32::MAX as u32
            {
                return Err(error("invalid_request", "Requests need unique IDs, existing people, valid conditions and positive bounded rewards, with a positive time limit and no initial acceptance."));
            }
            total_reward = total_reward.checked_add(r.reward).ok_or_else(|| {
                error(
                    "invalid_request",
                    "Total Sway rewards exceed the supported range.",
                )
            })?;
            request_map.insert(r.id, r);
        }
        if self
            .staff
            .values()
            .flatten()
            .any(|id| !map.contains_key(id))
        {
            return Err(error(
                "invalid_roster",
                "Keep every assigned staff member in the roster.",
            ));
        }
        self.production.issues.clear();
        self.production.roster = map;
        self.production.requests = request_map;
        Ok(())
    }
    /// Fixture configuration; never inferred from a portrait or public reputation.
    pub fn configure_cooperation(
        &mut self,
        issues: Vec<CooperationIssue>,
    ) -> Result<(), PlanError> {
        if self.now().0 != 0
            || !self.production.projects.is_empty()
            || self
                .production
                .requests
                .values()
                .any(|r| r.acceptance.is_some())
        {
            return Err(error(
                "production_config_locked",
                "Configure cooperation before play starts.",
            ));
        }
        let mut pairs = BTreeSet::new();
        for issue in &issues {
            let [a, b] = issue.people;
            if a == b
                || !self.production.roster.contains_key(&a)
                || !self.production.roster.contains_key(&b)
                || !pairs.insert(if a < b { [a, b] } else { [b, a] })
                || !issue.penalty.is_finite()
                || issue.penalty <= 0.0
                || issue.penalty > 0.25
            {
                return Err(error("invalid_cooperation", "Cooperation issues need unique existing pairs and a finite penalty in (0, 0.25]."));
            }
        }
        self.production.issues = issues;
        Ok(())
    }
    fn validate_concept(
        &self,
        mut c: Concept,
    ) -> Result<(Concept, Money, SimTick, SimTick), PlanError> {
        c.title = c.title.trim().to_owned();
        if c.title.is_empty()
            || c.title.chars().count() > 60
            || c.title.chars().any(char::is_control)
        {
            return Err(error(
                "invalid_title",
                "Use a title of 1–60 visible characters.",
            ));
        }
        if c.guests[0] == c.guests[1] || c.guests.contains(&c.host) {
            return Err(error(
                "duplicate_participant",
                "Choose one host and two distinct guests.",
            ));
        }
        let mut fees = 0_u64;
        for (id, role) in [
            (c.host, Role::Host),
            (c.guests[0], Role::Guest),
            (c.guests[1], Role::Guest),
        ] {
            let person =
                self.production.roster.get(&id).ok_or_else(|| {
                    error("unknown_person", "Choose participants from the roster.")
                })?;
            if self
                .staff_owner(id)
                .is_some_and(|owner| owner != self.player)
            {
                return Err(error(
                    "foreign_staff",
                    "This person works for the other station and cannot be assigned here.",
                ));
            }
            if person.role != role {
                return Err(error(
                    "wrong_role",
                    "Choose a host and guests for their listed roles.",
                ));
            }
            fees = fees
                .checked_add(person.fee.0)
                .ok_or_else(|| error("invalid_budget", "Participant fees overflow."))?;
        }
        if c.budget.validate().is_err() || c.budget.0 == 0 || c.budget.0 < fees {
            return Err(error("invalid_budget","The production budget must cover all participant fees and fit the supported credit range."));
        }
        let recording_end = SimTick(c.record_at.0.checked_add(4).ok_or_else(|| {
            error(
                "invalid_recording_time",
                "Recording time exceeds the scenario.",
            )
        })?);
        let ready_at = SimTick(recording_end.0.checked_add(8).ok_or_else(|| {
            error(
                "invalid_recording_time",
                "Production completion exceeds the scenario.",
            )
        })?);
        if c.record_at < self.now() || ready_at > self.end {
            return Err(error("invalid_recording_time","Book future recording time with room for one hour recording and two hours post-production before scenario end."));
        }
        Ok((c, Money(fees), recording_end, ready_at))
    }
    pub fn create_production(&mut self, concept: Concept) -> Result<ProductionId, PlanError> {
        let (concept, fees, recording_end, ready_at) = self.validate_concept(concept)?;
        let id = ProductionId(self.production.next_project);
        let next = self
            .production
            .next_project
            .checked_add(1)
            .ok_or_else(|| error("capacity", "Production identifier capacity exhausted."))?;
        self.bump()?;
        self.production.projects.insert(
            id,
            Production {
                id,
                concept,
                fees,
                recording_end,
                ready_at,
                phase: Phase::Draft,
                programme: None,
                booked_at: None,
                studio: None,
                exceptional_person: None,
                cooperation_sway: 0,
                output: None,
                outcome: None,
            },
        );
        self.production.next_project = next;
        Ok(id)
    }
    pub fn revise_production(
        &mut self,
        id: ProductionId,
        concept: Concept,
    ) -> Result<(), PlanError> {
        if self.production.project(id)?.phase != Phase::Draft {
            return Err(error(
                "production_locked",
                "Booked productions cannot be revised or cancelled in this experiment.",
            ));
        }
        let (concept, fees, recording_end, ready_at) = self.validate_concept(concept)?;
        self.bump()?;
        let p = self.production.projects.get_mut(&id).unwrap();
        p.concept = concept;
        p.fees = fees;
        p.recording_end = recording_end;
        p.ready_at = ready_at;
        Ok(())
    }
    pub fn discard_production(&mut self, id: ProductionId) -> Result<(), PlanError> {
        if self.production.project(id)?.phase != Phase::Draft {
            return Err(error(
                "production_locked",
                "Only unbooked drafts can be discarded.",
            ));
        }
        self.bump()?;
        self.production.projects.remove(&id);
        Ok(())
    }
    pub fn book_production(
        &mut self,
        id: ProductionId,
        exceptional_person: Option<PersonId>,
        cooperation_sway: u32,
    ) -> Result<(), PlanError> {
        let p = self.production.project(id)?.clone();
        if p.phase != Phase::Draft {
            return Err(error(
                "production_locked",
                "This production is already booked.",
            ));
        }
        self.validate_concept(p.concept.clone())?;
        let people = [p.concept.host, p.concept.guests[0], p.concept.guests[1]];
        if let Some(person) = exceptional_person {
            if !people.contains(&person) {
                return Err(error(
                    "participation_mismatch",
                    "Exceptional participation must name someone in this production.",
                ));
            }
            let talent = &self.production.roster[&person];
            if p.recording_end <= talent.normal_until {
                return Err(error(
                    "participation_not_needed",
                    "This person is normally available; no Sway is needed.",
                ));
            }
            if !talent
                .negotiable_from
                .is_some_and(|from| p.concept.record_at >= from)
            {
                return Err(error(
                    "participation_not_negotiable",
                    "This date is not explicitly negotiable, even with Sway.",
                ));
            }
        }
        for person in people {
            let talent = &self.production.roster[&person];
            if p.recording_end > talent.normal_until && exceptional_person != Some(person) {
                return Err(error(
                    "exceptional_participation_required",
                    "Select paid Sway negotiation for this participant's exceptional availability.",
                ));
            }
            if self.production.projects.values().any(|other| {
                other.phase != Phase::Draft
                    && p.concept.record_at < other.recording_end
                    && p.recording_end > other.concept.record_at
                    && [
                        other.concept.host,
                        other.concept.guests[0],
                        other.concept.guests[1],
                    ]
                    .contains(&person)
            }) {
                return Err(error("talent_conflict","A selected participant is already reserved for another recording at that time."));
            }
        }
        let studio = (1..=self.studio_count(self.player))
            .map(StudioId)
            .find(|number| {
                !self.production.projects.values().any(|other| {
                    other.studio == Some(*number)
                        && p.concept.record_at < other.recording_end
                        && p.recording_end > other.concept.record_at
                })
            })
            .ok_or_else(|| {
                error(
                    "studio_unavailable",
                    "All of your studios are reserved for another recording at that time.",
                )
            })?;
        let sway_cost = cooperation_sway.checked_add(if exceptional_person.is_some() {
            PARTICIPATION_COST
        } else {
            0
        });
        if cooperation_sway > i32::MAX as u32
            || sway_cost.is_none_or(|cost| self.production.sway < cost)
        {
            return Err(error(
                "insufficient_sway",
                "Not enough pooled Sway for all selected uses.",
            ));
        }
        if self.state().economy().cash(self.player).unwrap() < p.concept.budget {
            return Err(error(
                "insufficient_production_funds",
                "Station cash cannot cover the whole production budget.",
            ));
        }
        let next_programme = super::rights::original_programme_number(
            &self.engine.world.programmes,
            self.rights.as_ref(),
            self.production.next_programme,
        )?;
        let following = next_programme
            .checked_add(1)
            .ok_or_else(|| error("capacity", "Programme identifiers exhausted."))?;
        let (output, outcome) =
            self.make_discussion(&p, ProgrammeId(next_programme), cooperation_sway);
        let mut candidate = self.clone();
        candidate
            .engine
            .state
            .economy
            .reserve_production(self.now(), self.player, id, p.concept.budget)
            .map_err(engine_error)?;
        candidate.bump()?;
        candidate.production.next_programme = following;
        let booked = candidate.production.projects.get_mut(&id).unwrap();
        booked.phase = Phase::Booked;
        booked.programme = Some(output.id);
        booked.output = Some(output);
        booked.booked_at = Some(self.now());
        booked.studio = Some(studio);
        booked.exceptional_person = exceptional_person;
        booked.cooperation_sway = cooperation_sway;
        booked.outcome = Some(outcome);
        if let Some(person) = exceptional_person {
            candidate.production.change_sway(
                self.now(),
                id,
                -(PARTICIPATION_COST as i32),
                SwayReason::ExceptionalParticipation(person),
            )?;
        }
        if cooperation_sway > 0 {
            candidate.production.change_sway(
                self.now(),
                id,
                -(cooperation_sway as i32),
                SwayReason::Cooperation,
            )?;
        }
        *self = candidate;
        Ok(())
    }
    fn make_discussion(
        &self,
        p: &Production,
        id: ProgrammeId,
        support: u32,
    ) -> (Programme, ProductionOutcome) {
        let crew = [
            (&self.production.roster[&p.concept.host], 0.5),
            (&self.production.roster[&p.concept.guests[0]], 0.25),
            (&self.production.roster[&p.concept.guests[1]], 0.25),
        ];
        let craft = crew.iter().map(|(t, w)| t.craft * w).sum::<f64>();
        let fit = crew
            .iter()
            .map(|(t, w)| t.subject_fit[p.concept.subject.index()] * w)
            .sum::<f64>();
        let ambition = p.concept.ambition();
        let delivery: [f64; 10] = std::array::from_fn(|i| {
            crew.iter()
                .map(|(person, weight)| person.capabilities.0[i].unwrap_or(0.0) * weight)
                .sum()
        });
        let capability = ambition
            .values
            .iter()
            .zip(delivery)
            .filter_map(|(target, skill)| target.map(|_| skill))
            .sum::<f64>()
            / ambition.values.iter().flatten().count() as f64;
        let achieved = FeatureProfile::nonfiction(std::array::from_fn(|i| {
            ambition.values[i].map(|target| {
                let ceiling = 0.30 + 0.40 * delivery[i] + 0.20 * craft + 0.10 * fit;
                let baseline = if i == 1 { 0.5 } else { 0.0 };
                baseline + (target - baseline) * ceiling
            })
        }));
        let resources = (((p.concept.budget.0 - p.fees.0) as f64 / 1000.0).clamp(0.0, 1.0)).sqrt();
        let cast = crew.iter().map(|(t, w)| t.recognition * w).sum::<f64>();
        let base_execution =
            (0.15 + 0.45 * craft + 0.25 * fit + 0.10 * capability + 0.05 * resources)
                .clamp(0.0, 1.0);
        let people = [p.concept.host, p.concept.guests[0], p.concept.guests[1]];
        let issues = self
            .production
            .issues
            .iter()
            .filter(|i| i.people.iter().all(|id| people.contains(id)))
            .cloned()
            .collect::<Vec<_>>();
        let cooperation_penalty = issues
            .iter()
            .fold(0.0_f64, |total, issue| total + issue.penalty)
            .min(0.25)
            .min(base_execution);
        let remaining_penalty = cooperation_penalty * 2.0 / (2.0 + f64::from(support));
        let recovered_penalty = cooperation_penalty - remaining_penalty;
        let execution = (base_execution - remaining_penalty).clamp(0.0, 1.0);
        let outcome = ProductionOutcome {
            craft,
            subject_fit: fit,
            capability,
            ambition,
            achieved,
            resources,
            base_execution,
            cooperation_penalty,
            recovered_penalty,
            remaining_penalty,
            execution,
            issues,
        };
        (
            Programme {
                id,
                kind: ProgrammeKind::Standalone,
                name: p.concept.title.clone(),
                profile: achieved,
                awareness: 0.55,
                cast_appeal: cast,
                execution: execution.clamp(0.0, 1.0),
                broadcast_cost: Money(50),
                duration_minutes: 45,
            },
            outcome,
        )
    }
    pub(super) fn start_recordings(&mut self, at: SimTick) -> Result<(), PlanError> {
        let starting = self
            .production
            .projects
            .values()
            .filter(|p| p.phase == Phase::Booked && p.concept.record_at == at)
            .map(|p| p.id)
            .collect::<Vec<_>>();
        for id in starting {
            self.engine
                .state
                .economy
                .spend_production(at, id)
                .map_err(engine_error)?;
            self.production.projects.get_mut(&id).unwrap().phase = Phase::Recording;
        }
        Ok(())
    }
    pub(super) fn finish_productions(&mut self, at: SimTick) -> Result<(), PlanError> {
        let recorded = self
            .production
            .projects
            .values()
            .filter(|p| p.phase == Phase::Recording && p.recording_end == at)
            .map(|p| p.id)
            .collect::<Vec<_>>();
        for id in recorded {
            self.production.projects.get_mut(&id).unwrap().phase = Phase::PostProduction;
        }
        self.fulfil_requests(at, false)?;
        let ready = self
            .production
            .projects
            .values()
            .filter(|p| p.phase == Phase::PostProduction && p.ready_at == at)
            .map(|p| p.id)
            .collect::<Vec<_>>();
        for id in ready {
            let output = self.production.projects[&id].output.clone().unwrap();
            self.engine.world.programmes.push(output.clone());
            self.engine.world.validate().map_err(engine_error)?;
            self.libraries
                .entry(self.player)
                .or_default()
                .insert(output.id);
            for audience in &self.engine.world.audiences {
                self.engine
                    .state
                    .familiarity
                    .insert((audience.id, output.id), 0.0);
            }
            self.production.projects.get_mut(&id).unwrap().phase = Phase::Completed;
        }
        self.fulfil_requests(at, true)?;
        Ok(())
    }
    /// Accept a finite request independently of any production. No retroactive credit.
    pub fn accept_request(&mut self, id: RequestId) -> Result<(), PlanError> {
        let request = self
            .production
            .requests
            .get(&id)
            .ok_or_else(|| error("unknown_request", "Choose a listed opportunity."))?;
        if request.acceptance.is_some() {
            return Err(error(
                "quest_already_accepted",
                "This finite quest has already been accepted, completed or expired.",
            ));
        }
        if self.now() >= self.end {
            return Err(error(
                "scenario_ended",
                "No time remains to accept a quest.",
            ));
        }
        let deadline = SimTick(
            self.now()
                .0
                .saturating_add(request.time_limit_ticks)
                .min(self.end.0),
        );
        let accepted_at = self.now();
        self.bump()?;
        self.production.requests.get_mut(&id).unwrap().acceptance = Some(QuestAcceptance {
            accepted_at,
            deadline,
            production: None,
            fulfilled_at: None,
        });
        Ok(())
    }
    fn fulfil_requests(&mut self, at: SimTick, completed: bool) -> Result<(), PlanError> {
        let earned = self
            .production
            .requests
            .values()
            .filter_map(|r| {
                let acceptance = r.acceptance.as_ref()?;
                if acceptance.fulfilled_at.is_some() || at > acceptance.deadline {
                    return None;
                }
                self.production
                    .projects
                    .values()
                    .find(|p| {
                        let achieved = match r.condition {
                            RequestCondition::CoGuests { .. } => {
                                !completed
                                    && p.recording_end == at
                                    && p.phase == Phase::PostProduction
                            }
                            RequestCondition::CalmEverydayDiscussion { .. } => {
                                completed && p.ready_at == at && p.phase == Phase::Completed
                            }
                        };
                        achieved
                            && p.concept.record_at >= acceptance.accepted_at
                            && r.condition.matches(&p.concept)
                    })
                    .map(|p| (r.id, p.id, r.reward))
            })
            .collect::<Vec<_>>();
        for (request, project, reward) in earned {
            self.production.change_sway(
                at,
                project,
                reward as i32,
                SwayReason::RequestReward(request),
            )?;
            let acceptance = self
                .production
                .requests
                .get_mut(&request)
                .unwrap()
                .acceptance
                .as_mut()
                .unwrap();
            acceptance.fulfilled_at = Some(at);
            acceptance.production = Some(project);
        }
        Ok(())
    }
}

/// Small fictional content, separate from the production rules.
pub fn demo_roster() -> (Vec<Talent>, Vec<ProductionRequest>) {
    let roster=vec![
        Talent { id:PersonId(1),name:"Elian North".into(),role:Role::Host,fee:Money(220),description:"A careful science presenter, strongest with calm explanations.".into(),recognition:0.55,craft:0.90,subject_fit:[0.95,0.7,0.25],capabilities:Capabilities::discussion([0.25,0.64,0.50,0.55,0.97]),normal_until:SimTick(672),negotiable_from:None },
        Talent { id:PersonId(2),name:"Mina Sol".into(),role:Role::Host,fee:Money(120),description:"A familiar entertainment host with a lively conversational style.".into(),recognition:0.85,craft:0.72,subject_fit:[0.25,0.6,0.95],capabilities:Capabilities::discussion([0.96,0.94,0.65,0.96,0.40]),normal_until:SimTick(672),negotiable_from:None },
        Talent { id:PersonId(3),name:"Nora Vale".into(),role:Role::Guest,fee:Money(220),description:"An inventor who enjoys explaining practical science. Available days 1–2; later by explicit Sway negotiation.".into(),recognition:0.50,craft:0.85,subject_fit:[0.95,0.7,0.3],capabilities:Capabilities::discussion([0.28,0.73,0.65,0.53,0.98]),normal_until:SimTick(192),negotiable_from:Some(SimTick(192)) },
        Talent { id:PersonId(4),name:"Felix Reed".into(),role:Role::Guest,fee:Money(180),description:"A hands-on engineer, comfortable with detailed technical discussion.".into(),recognition:0.30,craft:0.88,subject_fit:[1.0,0.6,0.2],capabilities:Capabilities::discussion([0.19,0.68,0.40,0.52,0.95]),normal_until:SimTick(672),negotiable_from:None },
        Talent { id:PersonId(5),name:"Jules Hart".into(),role:Role::Guest,fee:Money(140),description:"A well-known performer with show-business stories and a theatrical delivery. Available days 1–2; later by explicit Sway negotiation.".into(),recognition:0.90,craft:0.65,subject_fit:[0.15,0.5,1.0],capabilities:Capabilities::discussion([0.98,0.87,0.85,0.95,0.32]),normal_until:SimTick(192),negotiable_from:Some(SimTick(192)) },
        Talent { id:PersonId(6),name:"Rafi Linden".into(),role:Role::Guest,fee:Money(160),description:"A community storyteller who connects everyday life and popular culture.".into(),recognition:0.65,craft:0.80,subject_fit:[0.45,0.95,0.9],capabilities:Capabilities::discussion([0.80,0.96,0.95,0.82,0.73]),normal_until:SimTick(672),negotiable_from:None },
    ];
    (
        roster,
        vec![
            ProductionRequest {
                id: RequestId(1),
                proposer: PersonId(3),
                condition: RequestCondition::CoGuests {
                    first: PersonId(3),
                    second: PersonId(4),
                },
                statement: "Record a discussion with me and Felix Reed as the two guests.".into(),
                reward: 4,
                time_limit_ticks: 192,
                acceptance: None,
            },
            ProductionRequest {
                id: RequestId(2),
                proposer: PersonId(6),
                condition: RequestCondition::CalmEverydayDiscussion { guest: PersonId(6) },
                statement: "Complete a calm discussion about everyday life with me as a guest."
                    .into(),
                reward: 3,
                time_limit_ticks: 192,
                acceptance: None,
            },
        ],
    )
}

/// Two disclosed fixture concerns; no dynamic relationship graph.
pub fn demo_cooperation() -> Vec<CooperationIssue> {
    vec![
        CooperationIssue { people: [PersonId(2), PersonId(5)], penalty: 0.12, known_note: Some("Booking notes: Mina and Jules have previously disagreed over who leads the conversation.".into()) },
        CooperationIssue { people: [PersonId(3), PersonId(6)], penalty: 0.08, known_note: Some("Booking notes: Nora and Rafi have found it difficult to share the floor.".into()) },
    ]
}
