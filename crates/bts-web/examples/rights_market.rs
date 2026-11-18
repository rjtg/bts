//! Controlled market comparisons and a replayable native reference.
use bts_sim::economy::Money;
use bts_sim::planning::Planner;
use bts_sim::{demo, Parameters, ProgrammeId, SimTick};
use bts_web::Session;
use serde_json::{json, Value};

const CORNER: ProgrammeId = ProgrammeId(60000);
const OBSERVATORY: ProgrammeId = ProgrammeId(60001);

fn comparisons() -> Value {
    let reference = bts_web::scenario::planner().unwrap();
    let seeds = reference
        .titles()
        .map(|t| t.seed.clone())
        .collect::<Vec<_>>();
    let compare = |rival_cash| {
        let mut world = bts_web::scenario::world();
        world.stations[1].initial_cash = Money(rival_cash);
        let ages = world.audiences.iter().map(|group| (group.id, 40)).collect();
        let mut planner =
            Planner::new(world, Parameters::default(), demo::MAIN, SimTick(672), &[]).unwrap();
        planner.configure_research(719, 12, ages).unwrap();
        planner
            .configure_rights(demo::RIVAL, seeds.clone())
            .unwrap();
        let total = planner.state().economy().total_money();
        let market = |planner: &Planner, id| {
            let title = planner
                .titles()
                .find(|t| t.seed.programme.id == id)
                .unwrap();
            let assessment = planner.title_forecast(id).unwrap();
            json!({
                "title":title.seed.programme.name,
                "price":title.acquisition.as_ref().unwrap().price,
                "viewers":assessment.expected_viewers,
                "uncertainty":assessment.relative_uncertainty,
                "groups":assessment.groups,
            })
        };
        let before = market(&planner, CORNER);
        let specialist = market(&planner, OBSERVATORY);
        planner.commission_title_assessment(CORNER, 6).unwrap();
        planner.advance_to(SimTick(12)).unwrap();
        let after = market(&planner, CORNER);
        planner.advance_to(SimTick(144)).unwrap();
        assert_eq!(planner.state().economy().total_money(), total);
        let winner = planner
            .titles()
            .find(|title| title.seed.programme.id == CORNER)
            .unwrap()
            .owner;
        json!({"rival_opening_cash":rival_cash,"broad_before_assessment":before,"specialist":specialist,
            "broad_after_assessment":after,"buyer_at_deadline":winner,"money_total":total})
    };
    json!([compare(20000), compare(150)])
}

fn replay() -> Value {
    let mut session = Session::new().unwrap();
    let mut requests = Vec::new();
    let mut results = Vec::new();
    let mut send = |command: Value| {
        let request = json!({"api_version":8,"command":command});
        let response: Value =
            serde_json::from_str(&session.dispatch_json(&request.to_string())).unwrap();
        assert_eq!(response["status"], "ok", "{response}");
        requests.push(request);
        let report = &response["report"];
        results.push(json!({"market":report["rights_market"],"cash":report["cash"],
            "finances":report["finances"],"catalogue":report["catalogue"],"schedule":report["schedule"]}));
        response
    };
    send(json!({"type":"assess_title","programme":CORNER.0,"insight":6}));
    let assessment = send(json!({"type":"advance","to_tick":12}));
    let token = &assessment["report"]["rights_market"]["titles"][0]["acquisition"]["token"];
    send(json!({"type":"buy_rights","programme":CORNER.0,"token":token}));
    send(json!({"type":"advance","to_tick":192}));
    let resale = send(json!({"type":"list_resale","programme":CORNER.0}));
    let token = &resale["report"]["rights_market"]["titles"][0]["resale"]["token"];
    send(json!({"type":"accept_resale","programme":CORNER.0,"token":token}));
    send(json!({"type":"advance","to_tick":216}));
    json!({"requests":requests,"results":results})
}

fn main() {
    let result = if std::env::args().any(|arg| arg == "--replay") {
        replay()
    } else {
        comparisons()
    };
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
