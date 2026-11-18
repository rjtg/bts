//! Developer-only controlled support experiments. Player observations are separate.
use bts_sim::developer::ProductionReport;
use bts_sim::economy::Money;
use bts_sim::planning::production::*;
use bts_sim::{demo::*, SimTick};
use serde_json::json;
fn run(support: u32, weak: bool, no_issue: bool) -> serde_json::Value {
    let mut p = bts_web::scenario::planner().unwrap();
    if weak {
        let (mut roster, requests) = bts_web::scenario::production_roster();
        for t in &mut roster {
            t.craft = 0.05;
            t.subject_fit = [0.05; 3];
        }
        p.configure_production(roster, requests).unwrap();
        p.configure_cooperation(demo_cooperation()).unwrap();
    }
    p.accept_request(RequestId(1)).unwrap();
    let earning = p
        .create_production(Concept {
            title: "Workshop Conversations".into(),
            host: PersonId(1),
            guests: [PersonId(3), PersonId(4)],
            subject: Subject::Science,
            tone: Tone::Calm,
            budget: Money(1000),
            record_at: SimTick(40),
        })
        .unwrap();
    p.book_production(earning, None, 0).unwrap();
    p.advance_to(SimTick(52)).unwrap();
    let concept = Concept {
        title: "The Evening Table".into(),
        host: if no_issue { PersonId(1) } else { PersonId(2) },
        guests: if no_issue {
            [PersonId(3), PersonId(4)]
        } else {
            [PersonId(5), PersonId(6)]
        },
        subject: Subject::Entertainment,
        tone: Tone::Lively,
        budget: Money(1000),
        record_at: SimTick(60),
    };
    let id = p.create_production(concept).unwrap();
    p.book_production(id, None, support).unwrap();
    p.advance_to(SimTick(72)).unwrap();
    let project = p.production().project(id).unwrap();
    let work = project.programme.unwrap();
    let evidence = ProductionReport::from_production(project).unwrap();
    let airing = p.place(work, SimTick(72), None).unwrap();
    p.advance_to(SimTick(75)).unwrap();
    let snapshot = p.airing(airing).unwrap().result.as_ref().unwrap();
    json!({"case":if weak {"weak_skills"} else if no_issue {"no_cooperation_deficiency"} else {"working_friction"},"support":support,"production":evidence,"sway_remaining":p.production().sway(),"cash":p.state().economy().cash(MAIN).unwrap().0.to_string(),"total_money":p.state().economy().total_money().to_string(),"audiences":snapshot.allocations.iter().map(|a| { let v=a.viewing.iter().find(|v| v.station==MAIN).unwrap(); json!({"audience":a.audience,"viewers":v.viewers,"satisfaction":v.satisfaction.trace.output}) }).collect::<Vec<_>>()})
}
fn main() {
    let mut cases = vec![];
    for (weak, no_issue) in [(false, false), (true, false), (false, true)] {
        for support in [0, 1, 4] {
            cases.push(run(support, weak, no_issue));
        }
    }
    println!(
        "{}",
        serde_json::to_string_pretty(
            &json!({"experiment":"casting_cooperation_v1","developer_only":true,"cases":cases})
        )
        .unwrap()
    );
}
