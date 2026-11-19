//! Controlled native experiments and a compact native/Wasm replay.
use bts_sim::planning::Planner;
use bts_sim::{demo, Parameters, ProgrammeId, SimTick};
use bts_web::Session;
use serde_json::{json, Value};

fn compare() -> Value {
    let mut records = Vec::new();
    for (label, programme, offered, low) in [
        ("broad", 1, 15, false),
        ("broad_after_repeats", 1, 15, false),
        ("specialist", 2, 15, false),
        ("broad_small_block", 1, 5, false),
        ("broad_poor_turnout", 1, 15, true),
    ] {
        let mut world = demo::world();
        world.programmes[0].kind = bts_sim::ProgrammeKind::Standalone;
        if low {
            for audience in &mut world.audiences {
                audience.availability = [0.02; 24];
            }
        }
        let mut planner =
            Planner::new(world, Parameters::default(), demo::MAIN, SimTick(672), &[]).unwrap();
        let total = planner.state().economy().total_money();
        let airing = planner
            .place(ProgrammeId(programme), SimTick(168), None)
            .unwrap();
        planner.resize_ad_block(airing, offered).unwrap();
        let quotes = planner.minute_offers(airing).unwrap();
        // Use a current quote after each purchase.
        for brand in [demo::SNACKS, demo::EQUIPMENT] {
            if let Some(offer) = planner
                .minute_offers(airing)
                .unwrap()
                .into_iter()
                .find(|q| q.brand == brand)
            {
                planner.accept_minutes(&offer).unwrap();
            }
        }
        if label == "broad_after_repeats" {
            for start in [72, 84] {
                planner
                    .place(ProgrammeId(programme), SimTick(start), None)
                    .unwrap();
            }
        }
        let end = planner.airing(airing).unwrap().end;
        let events = planner.advance_exact(end).unwrap();
        assert_eq!(planner.state().economy().total_money(), total);
        let start = events
            .iter()
            .find(|event| event.action.at == bts_sim::AirMinute::from_tick(SimTick(169)))
            .unwrap();
        let viewers: f64 = start
            .allocations
            .iter()
            .flat_map(|a| &a.viewing)
            .filter(|v| v.station == demo::MAIN)
            .map(|v| v.viewers)
            .sum();
        let sales: Vec<_> = planner.state().economy().minute_sales().collect();
        records.push(
            json!({"case":label, "offered":offered,"occupied_ticks":end.0 - 168,
            "quotes":quotes,"actual_expected_viewers":viewers, "sales":sales, "money_total":total}),
        );
    }
    json!(records)
}

fn replay() -> Value {
    let mut session = Session::new().unwrap();
    let mut requests = Vec::new();
    let mut results = Vec::new();
    let mut send = |command: Value| {
        let request = json!({"api_version": 8, "command": command});
        let response: Value =
            serde_json::from_str(&session.dispatch_json(&request.to_string())).unwrap();
        assert_eq!(response["status"], "ok", "{response}");
        requests.push(request);
        results.push(json!({
            "cash":response["report"]["cash"], "sales":response["report"]["minute_sales"],
            "batch":response["report"]["minute_offers"],
            "results":response["report"]["results"],
        }));
        response
    };
    let planned = send(json!({"type":"place","start_tick":72,"programme":1,"advertiser":null}));
    let id = planned["report"]["next_airing"].clone();
    send(json!({"type":"resize_ad_block","airing_id":id,"minutes":16}));
    let mut response = send(json!({"type":"request_minute_offers","airing_id":id}));
    for brand in [demo::SNACKS, demo::EQUIPMENT] {
        let token = response["report"]["minute_offers"]["token"].clone();
        response = send(
            json!({"type":"accept_minute_offer","airing_id":id,"advertiser":brand.0,"token":token}),
        );
    }
    let minutes = response["report"]["minute_sales"]
        .as_array()
        .unwrap()
        .iter()
        .map(|sale| sale["minutes"].as_u64().unwrap())
        .sum::<u64>();
    let trimmed = send(json!({"type":"resize_ad_block","airing_id":id,"minutes":minutes}));
    let end = trimmed["report"]["schedule"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["airing_id"] == id)
        .unwrap()["end_tick"]
        .clone();
    send(json!({"type":"advance","to_tick":end}));
    json!({"requests":requests,"results":results})
}

fn main() {
    let result = if std::env::args().any(|arg| arg == "--replay") {
        replay()
    } else {
        compare()
    };
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
