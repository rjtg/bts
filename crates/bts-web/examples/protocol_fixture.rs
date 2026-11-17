//! Native reference sequence used by the browser parity test.
use bts_web::Session;
use serde_json::{json, Value};
fn main() {
    let mut session = Session::new().unwrap();
    let mut reports = vec![];
    for command in [
        json!({"type":"inspect"}),
        json!({"type":"plan","slot":0,"programme":1,"advertiser":1}),
        json!({"type":"request_offer"}),
    ] {
        reports.push(
            serde_json::from_str::<Value>(
                &session.dispatch_json(&json!({"api_version":1,"command":command}).to_string()),
            )
            .unwrap(),
        );
    }
    let token = reports.last().unwrap()["report"]["offer"]["offer_id"]
        .as_str()
        .unwrap();
    for command in [
        json!({"type":"accept_offer","offer_id":token}),
        json!({"type":"advance"}),
    ] {
        reports.push(
            serde_json::from_str::<Value>(
                &session.dispatch_json(&json!({"api_version":1,"command":command}).to_string()),
            )
            .unwrap(),
        );
    }
    println!("{}", serde_json::to_string(&reports).unwrap());
}
