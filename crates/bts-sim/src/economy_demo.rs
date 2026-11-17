//! Controlled two-schedule/two-advertiser comparisons. No optimizing market.
use crate::{
    demo::{self, EQUIPMENT, MAIN, SNACKS},
    economy::*,
    *,
};
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Case {
    MassBroad,
    MassTargeted,
    SpecialistBroad,
    SpecialistTargeted,
}
impl Case {
    pub const ALL: [Self; 4] = [
        Self::MassBroad,
        Self::MassTargeted,
        Self::SpecialistBroad,
        Self::SpecialistTargeted,
    ];
    pub fn name(self) -> &'static str {
        match self {
            Self::MassBroad => "mass-broad",
            Self::MassTargeted => "mass-targeted",
            Self::SpecialistBroad => "specialist-broad",
            Self::SpecialistTargeted => "specialist-targeted",
        }
    }
    pub fn programme_case(self) -> demo::Case {
        match self {
            Self::MassBroad | Self::SpecialistBroad => demo::Case::Broad,
            _ => demo::Case::Targeted,
        }
    }
    pub fn brand(self) -> BrandId {
        match self {
            Self::MassBroad | Self::MassTargeted => SNACKS,
            _ => EQUIPMENT,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Round {
    pub offer: Offer,
    pub accepted: Option<ContractId>,
    pub event: SlotEvent,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Run {
    pub experiment: demo::ExperimentRun,
    pub brand: BrandId,
    pub rounds: Vec<Round>,
    pub cooldown: SlotEvent,
    pub recovery: Offer,
    pub before_cooling_quote: Offer,
    pub demand_after_campaigns: Demand,
}
pub fn run(case: Case) -> Result<Run, Error> {
    let world = demo::world();
    let parameters = Parameters::default();
    let mut simulation = Simulation::new(world.clone(), parameters)?;
    let slots = demo::schedule(case.programme_case());
    let mut rounds = Vec::new();
    let mut events = Vec::new();
    for original in slots.iter().filter(|s| !s.broadcasts.is_empty()) {
        let mut action = original.clone();
        action
            .broadcasts
            .iter_mut()
            .find(|b| b.station == MAIN)
            .unwrap()
            .advertisement = Some(case.brand());
        let offer = simulation.quote(&action, MAIN, case.brand(), action.at)?;
        let accepted = if offer.payment.0 > 0 && offer.promised_delivery > 0 {
            Some(simulation.accept_offer(&offer)?)
        } else {
            action
                .broadcasts
                .iter_mut()
                .find(|b| b.station == MAIN)
                .unwrap()
                .advertisement = None;
            None
        };
        let event = simulation.step(&action)?;
        rounds.push(Round {
            offer,
            accepted,
            event: event.clone(),
        });
        events.push(event);
    }
    let mut future = rounds.last().unwrap().offer.proposal().clone();
    future.at = SimHour((slots.last().unwrap().at.0 / 24 + 1) * 24 + 18);
    let before_cooling_quote = simulation.quote(&future, MAIN, case.brand(), future.at)?;
    let demand_after_campaigns = before_cooling_quote.demand.clone();
    let cooldown = simulation.step(slots.last().unwrap())?;
    events.push(cooldown.clone());
    let recovery = simulation.quote(&future, MAIN, case.brand(), future.at)?;
    Ok(Run {
        brand: case.brand(),
        rounds,
        cooldown,
        recovery,
        before_cooling_quote,
        demand_after_campaigns,
        experiment: demo::ExperimentRun {
            case: case.name().into(),
            world,
            parameters,
            events,
            final_state: simulation.state().clone(),
        },
    })
}
