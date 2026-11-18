//! Paired forecasts from different evidence, against identical actual airings.
use bts_sim::planning::research::Polling;
use bts_sim::{AudienceId, BrandId, ProgrammeId, SimTick};
use serde_json::json;

fn main() {
    let mut runs = Vec::new();
    for (label, polling, insight) in [
        ("stale", Polling::Off, 0),
        ("fresh", Polling::Daily, 0),
        ("researched", Polling::Daily, 8),
    ] {
        let mut planner = bts_web::scenario::planner().unwrap();
        let group = AudienceId(1);
        planner.set_polling(polling).unwrap();
        if insight > 0 {
            planner.spend_insight(group, insight).unwrap();
        }
        let airing = planner
            .place(ProgrammeId(1), SimTick(168), Some(BrandId(1)))
            .unwrap();
        planner.advance_to(SimTick(96)).unwrap();
        let research = serde_json::to_value(planner.research_view().unwrap()).unwrap();
        let quote = planner.quote(airing).unwrap();
        planner.accept(&quote).unwrap();
        let events = planner.advance_to(SimTick(172)).unwrap();
        let deliveries: Vec<_> = events
            .iter()
            .flat_map(|event| &event.financial.deliveries)
            .collect();
        runs.push(json!({
            "case": label,
            "research_at_quote": research,
            "forecast_relevant_delivery": quote.offer.forecast.relevant_delivery,
            "maximum_payment": quote.offer.payment.0,
            "actual_deliveries": deliveries,
            "cash": planner.state().economy().cash(planner.player()).unwrap().0,
            "total_money": planner.state().economy().total_money(),
        }));
    }
    println!("{}", serde_json::to_string_pretty(&runs).unwrap());
}
