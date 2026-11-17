//! Native reference for the actual Wasm player-protocol parity test.
use bts_web::Session;
use serde_json::{json, Value};
fn send(session: &mut Session, command: Value, reports: &mut Vec<Value>) {
    let response = serde_json::from_str(
        &session.dispatch_json(&json!({"api_version":2,"command":command}).to_string()),
    )
    .unwrap();
    reports.push(response);
}
fn main() {
    let mut s = Session::new().unwrap();
    let mut reports = vec![];
    send(&mut s, json!({"type":"inspect"}), &mut reports);
    send(
        &mut s,
        json!({"type":"place","start_tick":72,"programme":1,"advertiser":1}),
        &mut reports,
    );
    let id = reports.last().unwrap()["report"]["schedule"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["station"] == 1)
        .unwrap()["airing_id"]
        .clone();
    send(
        &mut s,
        json!({"type":"request_offer","airing_id":id}),
        &mut reports,
    );
    let token = reports.last().unwrap()["report"]["offer"]["offer_id"].clone();
    send(
        &mut s,
        json!({"type":"accept_offer","airing_id":id,"offer_id":token}),
        &mut reports,
    );
    send(&mut s, json!({"type":"advance","to_tick":74}), &mut reports);
    println!("{}", serde_json::to_string(&reports).unwrap());
}
