use bts_web::Session;
use serde_json::{json, Value};

fn send(session: &mut Session, command: Value) -> Value {
    let request = json!({"api_version": 8, "command": command});
    let response: Value =
        serde_json::from_str(&session.dispatch_json(&request.to_string())).unwrap();
    assert_eq!(response["status"], "ok", "{response}");
    response["report"].clone()
}

fn without_tokens(value: &mut Value) {
    match value {
        Value::Object(map) => {
            map.remove("token");
            map.remove("offer_id");
            for child in map.values_mut() {
                without_tokens(child);
            }
        }
        Value::Array(entries) => entries.iter_mut().for_each(without_tokens),
        _ => {}
    }
}

#[test]
fn restore_preserves_commitments_projects_research_and_future_outcomes() {
    let mut live = Session::new().unwrap();
    send(
        &mut live,
        json!({"type":"spend_insight","audience":1,"amount":3}),
    );
    send(
        &mut live,
        json!({"type":"start_newsroom","title":"Bulletin","first_day":2,"minute_of_day":1080,"ad_minutes":5,"staff":{"reporter":20,"anchor":22,"studio":1}}),
    );
    send(
        &mut live,
        json!({"type":"pitch_scripted_run","title":"Harbour Signals","subject":"science","standing":[0.23,0.35,0.5,0.57,0.38,0.32,0.48],"tension":[0.42,0.5,0.62]}),
    );
    send(
        &mut live,
        json!({"type":"write_film","film_id":6,"writer":101,"focus":null}),
    );
    send(
        &mut live,
        json!({"type":"create_production","concept":{"title":"Roundtable","host":1,"guests":[3,4],"subject":"science","tone":"calm","budget":"1000","record_at":40}}),
    );
    send(
        &mut live,
        json!({"type":"sign_host","host":1,"productions":[9]}),
    );
    send(
        &mut live,
        json!({"type":"book_production","production_id":9,"exceptional_person":null,"cooperation_sway":0}),
    );
    let placed = send(
        &mut live,
        json!({"type":"place_exact","programme":1,"start_minute":1080,"ad_minutes":15}),
    );
    let airing = placed["schedule"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["station"] == 1)
        .unwrap()["airing_id"]
        .clone();
    let quote = send(
        &mut live,
        json!({"type":"request_minute_offers","airing_id":airing}),
    );
    let advertiser = quote["minute_offers"]["offers"][0]["advertiser"].clone();
    let token = quote["minute_offers"]["token"].clone();
    let latest = send(
        &mut live,
        json!({"type":"accept_minute_offer","airing_id":airing,"advertiser":advertiser,"token":token}),
    );
    assert_eq!(latest["minute_sales"].as_array().unwrap().len(), 1);

    let save = live.save_game().unwrap();
    assert!(!save.contains(token.as_str().unwrap()));
    assert!(!save.contains("viewer_minutes"));
    let mut loaded = Session::new().unwrap();
    let initial = serde_json::to_value(loaded.report()).unwrap();
    assert!(loaded
        .restore_game(&save.replacen("\"version\":1", "\"version\":2", 1))
        .is_err());
    assert_eq!(serde_json::to_value(loaded.report()).unwrap(), initial);
    let mut replay_report = serde_json::to_value(loaded.restore_game(&save).unwrap()).unwrap();
    assert_ne!(
        replay_report["minute_offers"]["token"],
        latest["minute_offers"]["token"]
    );
    let pending = latest["minute_offers"].clone();
    let stale = loaded.dispatch_json(&json!({"api_version":8,"command":{
        "type":"accept_minute_offer","airing_id":airing,"advertiser":pending["offers"][0]["advertiser"],
        "token":pending["token"]
    }}).to_string());
    assert_eq!(
        serde_json::from_str::<Value>(&stale).unwrap()["error"]["code"],
        "stale_minute_offer"
    );
    let buy_pending = |session: &mut Session, report: &Value| {
        send(
            session,
            json!({
                "type":"accept_minute_offer","airing_id":airing,
                "advertiser":report["minute_offers"]["offers"][0]["advertiser"],"token":report["minute_offers"]["token"]
            }),
        )
    };
    let mut latest = buy_pending(&mut live, &latest);
    replay_report = buy_pending(&mut loaded, &replay_report);

    without_tokens(&mut latest);
    without_tokens(&mut replay_report);
    assert_eq!(replay_report, latest);
    let action = json!({"type":"advance","to_tick":4 * 96});
    let mut after_live = send(&mut live, action.clone());
    let mut after_load = send(&mut loaded, action);
    without_tokens(&mut after_live);
    without_tokens(&mut after_load);
    assert_eq!(after_load, after_live);
    assert_eq!(
        after_load["minute_sales"][0]["settlement"]["aired_minutes"],
        3
    );
}

#[test]
fn the_published_pre_office_checkpoint_keeps_its_season_accounts() {
    let mut station = Session::new().unwrap();
    let past = include_str!("../../../contrib/season-roadmap/season-checkpoint.json");
    let expected: Value = serde_json::from_str(include_str!(
        "../../../contrib/season-roadmap/season-result.json"
    ))
    .unwrap();
    let checkpoint = station.restore_game(past).unwrap();
    let closed = send(
        &mut station,
        json!({"type":"advance","to_tick":checkpoint.end_tick}),
    );
    assert_eq!(closed["cash"], expected["station_cash"]);
    assert_eq!(closed["now_tick"], expected["closed_tick"]);
    assert_eq!(closed["station_goals"], expected["goals"]);
    let receipts = |sales: &Value| {
        sales
            .as_array()
            .unwrap()
            .iter()
            .map(|sale| {
                (
                    sale["airing_id"].clone(),
                    sale["settlement"]["payment"].clone(),
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(
        receipts(&closed["minute_sales"]),
        receipts(&expected["paid_ad_sales"])
    );
}
