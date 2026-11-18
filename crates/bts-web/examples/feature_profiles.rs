//! Developer-only comparison changing only one host capability.
use bts_sim::economy::Money;
use bts_sim::planning::production::{Concept, PersonId, Subject, Tone};
use bts_sim::{demo::MAIN, SimTick};
use serde_json::json;

fn run(depth_skill: f64) -> serde_json::Value {
    let mut planner = bts_web::scenario::planner().unwrap();
    let (mut roster, requests) = bts_web::scenario::production_roster();
    roster
        .iter_mut()
        .find(|person| person.id == PersonId(1))
        .unwrap()
        .capabilities
        .0[7] = Some(depth_skill);
    planner.configure_production(roster, requests).unwrap();
    let id = planner
        .create_production(Concept {
            title: "Inside the Workshop".into(),
            host: PersonId(1),
            guests: [PersonId(3), PersonId(4)],
            subject: Subject::Science,
            tone: Tone::Calm,
            budget: Money(1000),
            record_at: SimTick(40),
        })
        .unwrap();
    planner.book_production(id, None, 0).unwrap();
    planner.advance_to(SimTick(52)).unwrap();
    let project = planner.production().project(id).unwrap();
    let outcome = project.outcome.as_ref().unwrap();
    let work = project.programme.unwrap();
    let evidence = json!({
        "host_depth_capability": depth_skill,
        "craft": outcome.craft,
        "subject_fit": outcome.subject_fit,
        "ambition": outcome.ambition,
        "achieved": outcome.achieved,
    });
    let airing = planner.place(work, SimTick(72), None).unwrap();
    planner.advance_to(SimTick(75)).unwrap();
    let result = planner.airing(airing).unwrap().result.as_ref().unwrap();
    let groups: Vec<_> = result
        .allocations
        .iter()
        .map(|group| {
            let view = group
                .viewing
                .iter()
                .find(|view| view.station == MAIN)
                .unwrap();
            json!({
                "audience": group.audience,
                "attraction": view.attraction.trace.output,
                "satisfaction": view.satisfaction.trace.output,
                "viewers": view.viewers,
            })
        })
        .collect();
    json!({"production": evidence, "audiences": groups, "total_money": planner.state().economy().total_money()})
}

fn main() {
    let cases = [run(0.1), run(0.9)];
    assert_eq!(cases[1], run(0.9));
    println!("{}", serde_json::to_string_pretty(&cases).unwrap());
}
