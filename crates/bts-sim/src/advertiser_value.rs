//! Controlled developer counterfactuals for deficit, saturation and repetition.
//! Stock interventions are confined to this fixture, not a public state-loading API.
use crate::{economy::*, *};

#[derive(Clone, Debug, PartialEq)]
pub struct CaseResult {
    pub name: String,
    pub world: World,
    pub parameters: Parameters,
    pub initial_hype: f64,
    pub initial_repetition: f64,
    pub offer: Offer,
    pub event: SlotEvent,
    pub post_campaign_demand: Demand,
    pub next_quote: Offer,
    pub final_state: State,
}

fn world(creative: f64) -> World {
    let mut world = demo::world();
    world.audiences.truncate(1);
    world.stations.truncate(1);
    world.programmes.truncate(1);
    world.brands.truncate(1);
    let audience = &mut world.audiences[0];
    audience.population = 10000;
    audience.genre_tastes = [1.0; 3];
    audience.availability = [1.0; 24];
    audience.preferred_tone = 0.8;
    audience.purchasing_power = 1.0;
    audience.product_interests = [1.0; 2];
    let programme = &mut world.programmes[0];
    programme.tone = 0.8;
    programme.awareness = 1.0;
    programme.cast_appeal = 1.0;
    programme.execution = 1.0;
    programme.broadcast_cost = Money(10);
    world.stations[0].initial_cash = Money(100000);
    let brand = &mut world.brands[0];
    brand.creative_quality = creative;
    brand.marketing_budget = Money(100000);
    brand.maximum_offer = Money(100000);
    brand.rate_per_thousand = Money(1000);
    brand.targets = vec![Target {
        audience: audience.id,
        priority: 1.0,
        desired_hype: 90.0,
    }];
    world
}

/// Six one-hour counterfactuals; no optimization, randomness, or elapsed wall time.
pub fn run() -> Result<Vec<CaseResult>, Error> {
    let mut cases = Vec::new();
    for (name, hype, repetition, creative) in [
        ("low-hype/fresh", 5.0, 0.0, 0.8),
        ("high-hype/fresh", 65.0, 0.0, 0.8),
        ("low-hype/repeated", 5.0, 4.0, 0.8),
        ("high-hype/repeated", 65.0, 4.0, 0.8),
        ("low-hype/weak-creative", 5.0, 0.0, 0.2),
        ("low-hype/weak-repeated", 5.0, 4.0, 0.2),
    ] {
        let world = world(creative);
        let parameters = Parameters {
            loyalty_learning: 0.0,
            ..Parameters::default()
        };
        let mut simulation = Simulation::new(world.clone(), parameters)?;
        let station = world.stations[0].id;
        let brand = world.brands[0].id;
        let audience = world.audiences[0].id;
        // Explicit initial-state interventions for a controlled developer experiment.
        simulation.state.hype.insert((brand, audience), hype);
        simulation
            .state
            .repetition
            .insert((brand, audience), repetition);
        let action = Slot {
            at: SimHour(1),
            broadcasts: vec![Broadcast {
                station,
                programme: world.programmes[0].id,
                advertisement: Some(brand),
            }],
        };
        let offer = simulation.quote(&action, station, brand, action.at)?;
        simulation.accept_offer(&offer)?;
        let event = simulation.step(&action)?;
        let post_campaign_demand = simulation.demand(brand)?;
        let mut future = action;
        future.at = SimHour(2);
        let next_quote = simulation.quote(&future, station, brand, future.at)?;
        cases.push(CaseResult {
            name: name.into(),
            world,
            parameters,
            initial_hype: hype,
            initial_repetition: repetition,
            offer,
            event,
            post_campaign_demand,
            next_quote,
            final_state: simulation.state().clone(),
        });
    }
    Ok(cases)
}
