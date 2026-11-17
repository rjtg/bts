//! One-off discussion production and one person-specific favour experiment.
//! This state is not a player-report DTO. See docs/original-production.md.
use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct ProductionId(pub u32);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct PersonId(pub u16);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct RequestId(pub u16);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct FavourId(pub u32);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
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
    fn index(self) -> usize {
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
    fn genre(self) -> Genre {
        match self {
            Self::Science => Genre::Science,
            Self::EverydayLife => Genre::Drama,
            Self::Entertainment => Genre::Comedy,
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
    pub preferred_tone: f64,
    pub normal_until: SimTick,
    pub favour_from: Option<SimTick>,
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
    pub promised_request: Option<RequestId>,
    pub called_favour: Option<FavourId>,
    output: Option<Programme>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Promise {
    pub production: ProductionId,
    pub promised_at: SimTick,
    pub fulfilled_at: Option<SimTick>,
    pub earned_favour: Option<FavourId>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CoGuestRequest {
    pub id: RequestId,
    pub person: PersonId,
    pub co_guest: PersonId,
    pub statement: String,
    pub promise: Option<Promise>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Favour {
    pub id: FavourId,
    pub owed_by: PersonId,
    pub earned_from: ProductionId,
    pub earned_at: SimTick,
    pub called_for: Option<ProductionId>,
    pub called_at: Option<SimTick>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ProductionState {
    roster: BTreeMap<PersonId, Talent>,
    projects: BTreeMap<ProductionId, Production>,
    request: Option<CoGuestRequest>,
    favours: BTreeMap<FavourId, Favour>,
    next_project: u32,
    next_programme: u16,
    next_favour: u32,
}
impl Default for ProductionState {
    fn default() -> Self {
        Self {
            roster: BTreeMap::new(),
            projects: BTreeMap::new(),
            request: None,
            favours: BTreeMap::new(),
            next_project: 1,
            next_programme: 1,
            next_favour: 1,
        }
    }
}
impl ProductionState {
    pub fn roster(&self) -> impl Iterator<Item = &Talent> {
        self.roster.values()
    }
    pub fn projects(&self) -> impl Iterator<Item = &Production> {
        self.projects.values()
    }
    pub fn request(&self) -> Option<&CoGuestRequest> {
        self.request.as_ref()
    }
    pub fn favours(&self) -> impl Iterator<Item = &Favour> {
        self.favours.values()
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
        self.next_favour = self.next_favour.max(other.next_favour);
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
        request: Option<CoGuestRequest>,
    ) -> Result<(), PlanError> {
        if self.now().0 != 0 || !self.production.projects.is_empty() {
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
                    .favour_from
                    .is_some_and(|t| t < person.normal_until || t >= self.end)
            {
                return Err(error(
                    "invalid_roster",
                    "Invalid person identity, fee or booking window.",
                ));
            }
            for value in [person.recognition, person.craft, person.preferred_tone]
                .into_iter()
                .chain(person.subject_fit)
            {
                unit(value, "talent parameter").map_err(engine_error)?;
            }
            map.insert(person.id, person);
        }
        if let Some(r) = &request {
            if r.promise.is_some()
                || r.person == r.co_guest
                || [r.person, r.co_guest]
                    .iter()
                    .any(|id| map.get(id).is_none_or(|t| t.role != Role::Guest))
            {
                return Err(error(
                    "invalid_request",
                    "A co-guest request needs two existing distinct guests and no initial promise.",
                ));
            }
        }
        self.production.roster = map;
        self.production.request = request;
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
                promised_request: None,
                called_favour: None,
                output: None,
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
        promise: Option<RequestId>,
        call: Option<FavourId>,
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
        let favour = call
            .map(|id| {
                self.production.favours.get(&id).ok_or_else(|| {
                    error(
                        "unknown_favour",
                        "Choose an earned, unused personal favour.",
                    )
                })
            })
            .transpose()?;
        if let Some(f) = favour {
            if f.called_for.is_some() {
                return Err(error(
                    "favour_already_called",
                    "This personal favour has already been called in.",
                ));
            }
            if !people.contains(&f.owed_by) {
                return Err(error(
                    "favour_wrong_person",
                    "A favour can secure only the person who owes it.",
                ));
            }
            if p.recording_end <= self.production.roster[&f.owed_by].normal_until {
                return Err(error("favour_not_needed","This person is normally available then; keep their favour for a later booking."));
            }
        }
        for person in people {
            let talent = &self.production.roster[&person];
            if p.recording_end > talent.normal_until
                && (!favour.is_some_and(|f| f.owed_by == person)
                    || !talent
                        .favour_from
                        .is_some_and(|from| p.concept.record_at >= from))
            {
                return Err(error(
                    "favour_required",
                    "This guest needs their own earned personal favour for that recording date.",
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
        if let Some(request_id) = promise {
            let request = self
                .production
                .request
                .as_ref()
                .filter(|r| r.id == request_id)
                .ok_or_else(|| error("unknown_request", "Choose the listed co-guest request."))?;
            if request.promise.is_some() {
                return Err(error(
                    "request_already_promised",
                    "This one-shot request has already been promised or fulfilled.",
                ));
            }
            if !p.concept.guests.contains(&request.person)
                || !p.concept.guests.contains(&request.co_guest)
            {
                return Err(error(
                    "promise_mismatch",
                    "The promised requester and requested co-guest must both be booked as guests.",
                ));
            }
        }
        if self.state().economy().cash(self.player).unwrap() < p.concept.budget {
            return Err(error(
                "insufficient_production_funds",
                "Station cash cannot cover the whole production budget.",
            ));
        }
        let next_programme = self
            .engine
            .world
            .programmes
            .iter()
            .map(|p| p.id.0)
            .max()
            .unwrap_or(0)
            .checked_add(1)
            .ok_or_else(|| error("capacity", "Programme identifiers exhausted."))?
            .max(self.production.next_programme);
        let following = next_programme
            .checked_add(1)
            .ok_or_else(|| error("capacity", "Programme identifiers exhausted."))?;
        let output = self.make_discussion(&p, ProgrammeId(next_programme));
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
        booked.promised_request = promise;
        booked.called_favour = call;
        if promise.is_some() {
            candidate.production.request.as_mut().unwrap().promise = Some(Promise {
                production: id,
                promised_at: self.now(),
                fulfilled_at: None,
                earned_favour: None,
            });
        }
        if let Some(f) = call {
            let credit = candidate.production.favours.get_mut(&f).unwrap();
            credit.called_for = Some(id);
            credit.called_at = Some(self.now());
        }
        *self = candidate;
        Ok(())
    }
    fn make_discussion(&self, p: &Production, id: ProgrammeId) -> Programme {
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
        let tone_fit = crew
            .iter()
            .map(|(t, w)| (1.0 - (t.preferred_tone - p.concept.tone.score()).abs()) * w)
            .sum::<f64>();
        let resources = (((p.concept.budget.0 - p.fees.0) as f64 / 1000.0).clamp(0.0, 1.0)).sqrt();
        let cast = crew.iter().map(|(t, w)| t.recognition * w).sum::<f64>();
        let execution = 0.15 + 0.45 * craft + 0.25 * fit + 0.10 * tone_fit + 0.05 * resources;
        Programme {
            id,
            kind: ProgrammeKind::Standalone,
            name: p.concept.title.clone(),
            genre: p.concept.subject.genre(),
            tone: p.concept.tone.score(),
            awareness: 0.55,
            cast_appeal: cast,
            execution: execution.clamp(0.0, 1.0),
            broadcast_cost: Money(50),
            duration_minutes: 45,
        }
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
            if self
                .production
                .request
                .as_ref()
                .and_then(|r| r.promise.as_ref())
                .is_some_and(|promise| promise.production == id && promise.fulfilled_at.is_none())
            {
                let favour_id = FavourId(self.production.next_favour);
                self.production.next_favour = self
                    .production
                    .next_favour
                    .checked_add(1)
                    .ok_or_else(|| error("capacity", "Favour identifiers exhausted."))?;
                let request = self.production.request.as_mut().unwrap();
                let promise = request.promise.as_mut().unwrap();
                promise.fulfilled_at = Some(at);
                promise.earned_favour = Some(favour_id);
                self.production.favours.insert(
                    favour_id,
                    Favour {
                        id: favour_id,
                        owed_by: request.person,
                        earned_from: id,
                        earned_at: at,
                        called_for: None,
                        called_at: None,
                    },
                );
            }
        }
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
            for audience in &self.engine.world.audiences {
                self.engine
                    .state
                    .familiarity
                    .insert((audience.id, output.id), 0.0);
            }
            self.production.projects.get_mut(&id).unwrap().phase = Phase::Completed;
        }
        Ok(())
    }
}

/// Small fictional content, separate from the production rules.
pub fn demo_roster() -> (Vec<Talent>, CoGuestRequest) {
    let roster=vec![
        Talent { id:PersonId(1),name:"Elian North".into(),role:Role::Host,fee:Money(220),description:"A careful science presenter, strongest with calm explanations.".into(),recognition:0.55,craft:0.90,subject_fit:[0.95,0.7,0.25],preferred_tone:0.2,normal_until:SimTick(672),favour_from:None },
        Talent { id:PersonId(2),name:"Mina Sol".into(),role:Role::Host,fee:Money(120),description:"A familiar entertainment host with a lively conversational style.".into(),recognition:0.85,craft:0.72,subject_fit:[0.25,0.6,0.95],preferred_tone:0.8,normal_until:SimTick(672),favour_from:None },
        Talent { id:PersonId(3),name:"Nora Vale".into(),role:Role::Guest,fee:Money(220),description:"An inventor who enjoys explaining practical science. Available days 1–2; later by personal favour.".into(),recognition:0.50,craft:0.85,subject_fit:[0.95,0.7,0.3],preferred_tone:0.25,normal_until:SimTick(192),favour_from:Some(SimTick(192)) },
        Talent { id:PersonId(4),name:"Felix Reed".into(),role:Role::Guest,fee:Money(180),description:"A hands-on engineer, comfortable with detailed technical discussion.".into(),recognition:0.30,craft:0.88,subject_fit:[1.0,0.6,0.2],preferred_tone:0.2,normal_until:SimTick(672),favour_from:None },
        Talent { id:PersonId(5),name:"Jules Hart".into(),role:Role::Guest,fee:Money(140),description:"A well-known performer with show-business stories and a theatrical delivery.".into(),recognition:0.90,craft:0.65,subject_fit:[0.15,0.5,1.0],preferred_tone:0.85,normal_until:SimTick(672),favour_from:None },
        Talent { id:PersonId(6),name:"Rafi Linden".into(),role:Role::Guest,fee:Money(160),description:"A community storyteller who connects everyday life and popular culture.".into(),recognition:0.65,craft:0.80,subject_fit:[0.45,0.95,0.9],preferred_tone:0.6,normal_until:SimTick(672),favour_from:None },
    ];
    (roster,CoGuestRequest { id:RequestId(1),person:PersonId(3),co_guest:PersonId(4),statement:"Invite Felix Reed alongside me. If you make that happen, I will make time for a later recording.".into(),promise:None })
}
