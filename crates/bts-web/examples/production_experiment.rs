//! Controlled public reports plus a replayable native reference for browser parity.
use bts_web::Session;
use serde_json::{json, Value};
fn dispatch(
    s: &mut Session,
    command: Value,
    requests: &mut Vec<Value>,
    responses: &mut Vec<Value>,
) -> Value {
    let request = json!({"api_version":4,"command":command});
    let response: Value = serde_json::from_str(&s.dispatch_json(&request.to_string())).unwrap();
    requests.push(request);
    responses.push(response.clone());
    response
}
fn concept(
    title: &str,
    host: u16,
    guests: [u16; 2],
    subject: &str,
    tone: &str,
    budget: &str,
    record_at: u32,
) -> Value {
    json!({"title":title,"host":host,"guests":guests,"subject":subject,"tone":tone,"budget":budget,"record_at":record_at})
}
fn comparison(entertainment: bool) -> Value {
    let mut s = Session::new().unwrap();
    let mut requests = vec![];
    let mut responses = vec![];
    let c = if entertainment {
        concept(
            "Culture at the Table",
            2,
            [5, 6],
            "entertainment",
            "lively",
            "600",
            40,
        )
    } else {
        concept(
            "Workshop Conversations",
            1,
            [3, 4],
            "science",
            "calm",
            "1000",
            40,
        )
    };
    let mut send = |command| dispatch(&mut s, command, &mut requests, &mut responses);
    send(json!({"type":"create_production","concept":c}));
    send(
        json!({"type":"book_production","production_id":1,"promise_request":null,"exceptional_person":null,"smooth_cooperation":false}),
    );
    send(json!({"type":"advance","to_tick":52}));
    send(json!({"type":"place","start_tick":72,"programme":8,"advertiser":1}));
    let quote = send(json!({"type":"request_offer","airing_id":57}));
    let token = quote["report"]["offer"]["offer_id"].clone();
    send(json!({"type":"accept_offer","airing_id":57,"offer_id":token}));
    let final_report = send(json!({"type":"advance","to_tick":75}));
    assert_eq!(final_report["status"], "ok");
    json!({"case":if entertainment { "popular_culture" } else { "technical_science" },"report":final_report["report"]})
}
fn main() {
    let comparisons = vec![comparison(false), comparison(true)];
    let mut s = Session::new().unwrap();
    let mut requests = vec![];
    let mut responses = vec![];
    let mut send = |command| dispatch(&mut s, command, &mut requests, &mut responses);
    send(json!({"type":"inspect"}));
    send(
        json!({"type":"create_production","concept":concept("Workshop Conversations",1,[3,4],"science","calm","1000",40)}),
    );
    send(
        json!({"type":"book_production","production_id":1,"promise_request":1,"exceptional_person":null,"smooth_cooperation":false}),
    );
    let early = send(json!({"type":"place","start_tick":72,"programme":8,"advertiser":null}));
    assert_eq!(early["error"]["code"], "production_not_ready");
    send(json!({"type":"advance","to_tick":52}));
    send(json!({"type":"place","start_tick":72,"programme":8,"advertiser":2}));
    let quote = send(json!({"type":"request_offer","airing_id":57}));
    let token = quote["report"]["offer"]["offer_id"].clone();
    send(json!({"type":"accept_offer","airing_id":57,"offer_id":token}));
    send(json!({"type":"advance","to_tick":75}));
    send(
        json!({"type":"create_production","concept":concept("Neighbourhood Conversations",2,[5,6],"everyday_life","conversational","1000",232)}),
    );
    let without = send(
        json!({"type":"book_production","production_id":2,"promise_request":null,"exceptional_person":null,"smooth_cooperation":false}),
    );
    assert_eq!(
        without["error"]["code"],
        "exceptional_participation_required"
    );
    send(
        json!({"type":"book_production","production_id":2,"promise_request":null,"exceptional_person":5,"smooth_cooperation":true}),
    );
    send(json!({"type":"advance","to_tick":244}));
    send(json!({"type":"place","start_tick":264,"programme":9,"advertiser":1}));
    let quote = send(json!({"type":"request_offer","airing_id":58}));
    let token = quote["report"]["offer"]["offer_id"].clone();
    send(json!({"type":"accept_offer","airing_id":58,"offer_id":token}));
    send(json!({"type":"advance","to_tick":267}));
    send(
        json!({"type":"create_production","concept":concept("Everyday workshop",1,[4,6],"everyday_life","calm","1000",280)}),
    );
    send(
        json!({"type":"book_production","production_id":3,"promise_request":2,"exceptional_person":null,"smooth_cooperation":false}),
    );
    let final_report = send(json!({"type":"advance","to_tick":292}));
    assert_eq!(final_report["status"], "ok");
    if std::env::args().any(|arg| arg == "--sequence") {
        println!("{}", json!({"requests":requests,"responses":responses}));
    } else {
        println!("{}",serde_json::to_string_pretty(&json!({"experiment":"pooled_sway_v1","comparisons":comparisons,"sway_chain":final_report["report"],"rejected_without_sway":without["error"]})).unwrap());
    }
}
