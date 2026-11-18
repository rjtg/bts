use bts_sim::demo::{self, MAIN, RIVAL, YOUNG};
use bts_sim::planning::weekly::Weekday;
use bts_sim::planning::{CompetitorAiring, Planner};
use bts_sim::{Parameters, ProgrammeId, SimTick};
use bts_web::Session;
use serde_json::{json, Value};

fn sequence() -> Value {
    let mut session = Session::new().unwrap();
    let requests = [
        json!({"type":"place","programme":2,"start_tick":744,"advertiser":null}),
        json!({"type":"preview_weekly","plan":{"programme":1,"weekday":5,"time":72,"count":3,"ad_minutes":10,"exceptions":[]}}),
        json!({"type":"preview_weekly","plan":{"programme":1,"weekday":5,"time":72,"count":3,"ad_minutes":10,"exceptions":[{"occurrence":1,"start_tick":752}]}}),
        json!({"type":"confirm_weekly"}),
    ];
    let responses: Vec<_> = requests.iter().map(|command| {
        let response: Value = serde_json::from_str(&session.dispatch_json(&json!({"api_version":8,"command":command}).to_string())).unwrap();
        assert_eq!(response["status"], "ok", "{response}");
        let report = &response["report"];
        let airings: Vec<_> = report["schedule"].as_array().unwrap().iter().filter(|a| a["station"] == 1).collect();
        json!({"review":report["weekly_review"],"airings":airings,"calendar":report["first_weekday"]})
    }).collect();
    json!({"requests":requests,"responses":responses})
}

fn comparison() -> Value {
    json!([("Saturday", 72), ("Monday", 264)].map(|(day, start)| {
        let mut world = demo::world();
        for group in &mut world.audiences { group.availability = [0.8;24]; }
        let patterns = world.audiences.iter().map(|group| (group.id, [[0.15;3],[0.85;3]])).collect();
        let rival = [CompetitorAiring { station: RIVAL, programme: ProgrammeId(4), start: SimTick(start) }];
        let mut planner = Planner::new(world, Parameters::default(), MAIN, SimTick(2016), &rival).unwrap();
        planner.configure_viewing_calendar(Weekday::SATURDAY, patterns).unwrap();
        let airing = planner.place(ProgrammeId(1), SimTick(start), None).unwrap();
        planner.advance_to(SimTick(start+4)).unwrap();
        let snapshot = planner.airing(airing).unwrap().result.as_ref().unwrap();
        let young = snapshot.allocations.iter().find(|a| a.audience == YOUNG).unwrap();
        let at = |station| young.viewing.iter().find(|v| v.station==station).unwrap().viewers;
        json!({"day":day,"hour":18,"available":young.available,"viewer_preference":if day=="Saturday" {0.85} else {0.15},
            "player_expected_viewers":at(MAIN),"rival_expected_viewers":at(RIVAL),"not_watching":young.not_watching,"population":young.population})
    }))
}
fn main() {
    let result = if std::env::args().any(|arg| arg == "--sequence") {
        sequence()
    } else {
        comparison()
    };
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
