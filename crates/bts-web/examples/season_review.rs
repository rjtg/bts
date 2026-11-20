//! Reproducible six-week scenario and a playable checkpoint file.
use bts_web::protocol::PlayerReport;
use bts_web::Session;
use serde_json::{json, Value};

fn send(session: &mut Session, command: Value) -> PlayerReport {
    let request = serde_json::from_value::<bts_web::protocol::Command>(command).unwrap();
    session.execute(request).unwrap()
}

fn main() {
    let mut station = Session::new().unwrap();
    send(
        &mut station,
        json!({
            "type": "preview_weekly",
            "plan": {"programme": 1, "weekday": 5, "minute_of_day": 1080, "count": 3, "ad_minutes": 3, "exceptions": []}
        }),
    );
    let planned = send(&mut station, json!({"type":"confirm_weekly"}));
    let first = planned
        .schedule
        .iter()
        .find(|airing| airing.weekly_id.is_some())
        .unwrap()
        .airing_id;
    send(
        &mut station,
        json!({"type":"set_autosell","airing_id":first,"policy":{"floor":"1","max_minutes":3}}),
    );
    send(
        &mut station,
        json!({"type":"place_exact","programme":2,"start_minute":21 * 1440 + 1080,"ad_minutes":0}),
    );
    send(&mut station, json!({"type":"advance","to_tick":7 * 96}));
    if std::env::args().any(|arg| arg == "--save") {
        println!("{}", station.save_game().unwrap());
        return;
    }

    let end = station.report().end_tick;
    let closed = send(&mut station, json!({"type":"advance","to_tick":end}));
    assert_eq!(closed.now_tick, closed.end_tick);
    assert!(closed
        .station_goals
        .iter()
        .all(|goal| goal.progress >= goal.target));
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "scenario_days": end / 96,
            "closed_tick": closed.now_tick,
            "station_cash": closed.cash,
            "goals": closed.station_goals,
            "paid_ad_sales": closed.minute_sales,
        }))
        .unwrap()
    );
}
