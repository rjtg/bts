use bts_sim::{
    demo::{self, Case},
    developer::{self, DeveloperReport},
};
use std::{env, error::Error, fs, path::PathBuf};

const HELP: &str = "Usage: bts-cli demo [--case all|broad|targeted|disappointing] [--json PATH] [--trace]\n       bts-cli economy [--case all|mass-broad|mass-targeted|specialist-broad|specialist-targeted] [--json PATH] [--trace]\n\nDeterministic developer experiments. JSON includes inputs, traces and economy accounts.\nCounts are viewer-slot impressions, not cumulative unique reach.";

fn summary(report: &DeveloperReport) {
    println!("\n{} (identical initial state and competitor)", report.case);
    for station in &report.summary.stations {
        let name = &report
            .initial_world
            .stations
            .iter()
            .find(|s| s.id == station.station)
            .unwrap()
            .name;
        println!(
            "  {name}: {:.0} viewer-slot impressions, mean satisfaction {:.3}",
            station.viewer_slot_impressions,
            station.mean_satisfaction.unwrap_or(0.0)
        );
    }
    for audience in &report.summary.first_station_return {
        let name = &report
            .initial_world
            .audiences
            .iter()
            .find(|a| a.id == audience.audience)
            .unwrap()
            .name;
        println!(
            "  {name}: early-slot viewers {:.0} -> {:.0}; willingness {:.3} -> {:.3}",
            audience.first_early_viewers,
            audience.last_early_viewers,
            audience.initial_willingness,
            audience.final_willingness
        );
    }
    for brand in &report.summary.brands {
        let name = &report
            .initial_world
            .brands
            .iter()
            .find(|b| b.id == brand.brand)
            .unwrap()
            .name;
        println!(
            "  {name}: population-weighted hype {:.2}, after 48h idle {:.2}; specialist hype {:.2}",
            brand.mean_hype_after_broadcasts,
            brand.mean_hype_after_cooling,
            brand.specialist_hype_after_broadcasts
        );
    }
}
fn trace(report: &DeveloperReport) {
    for slot in &report.slots {
        println!("  Hour {} (+{}h)", slot.action.at.0, slot.elapsed_hours);
        for audience in &slot.allocations {
            println!(
                "    {:?}: available {:.1}, not watching {:.1}",
                audience.audience, audience.available, audience.total_not_watching
            );
            for viewing in &audience.viewing {
                println!("      {:?}/{:?}: viewers {:.1}, attraction {:.3}, satisfaction {:.3}, willingness {:.3}->{:.3}",viewing.station,viewing.programme,viewing.expected_viewers,viewing.attraction.output,viewing.satisfaction.output,viewing.willingness_before,viewing.willingness_after);
                for (name, model) in [
                    ("attraction", &viewing.attraction),
                    ("satisfaction", &viewing.satisfaction),
                ] {
                    println!("        {name} inputs {:?}; numerator {:.3}, denominator {:.3}, output {:.3}",model.inputs,model.total_contribution,model.total_effective_weight,model.output);
                    for rule in &model.rules {
                        println!("          {}: activation {:.3}, configured weight {:.3}, effective {:.3}, contribution {:.3}; {:?}",rule.name,rule.activation,rule.configured_weight,rule.effective_weight,rule.contribution,rule.condition);
                    }
                }
            }
        }
        for hype in &slot.hype {
            println!("    {:?}/{:?}: exposures {:.1}, relevant/person {:.4}, hype {:.3} -> decayed {:.3} + campaign {:.3} = {:.3}, repetition {:.3}->{:.3}",hype.brand,hype.audience,hype.expected_exposures,hype.relevant_exposure_per_person,hype.before,hype.after_decay,hype.campaign_generated,hype.after,hype.repetition_before,hype.repetition_after);
            for ad in &hype.advertisements {
                println!(
                    "      {:?}: effectiveness {:.3}, inputs {:?}, rules {:?}",
                    ad.station,
                    ad.effectiveness.output,
                    ad.effectiveness.inputs,
                    ad.effectiveness.rules
                );
            }
        }
    }
}
fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let economy = match args.next().as_deref() {
        Some("--help" | "-h") => {
            println!("{HELP}");
            return Ok(());
        }
        Some("demo") => false,
        Some("economy") => true,
        _ => return Err(HELP.into()),
    };
    let mut selection = "all".to_string();
    let mut json: Option<PathBuf> = None;
    let mut verbose = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--case" => selection = args.next().ok_or("--case needs a value")?,
            "--json" => json = Some(args.next().ok_or("--json needs a path")?.into()),
            "--trace" => verbose = true,
            "--help" | "-h" => {
                println!("{HELP}");
                return Ok(());
            }
            _ => return Err(format!("unknown argument {arg:?}\n{HELP}").into()),
        }
    }
    if economy {
        use bts_sim::economy_demo;
        let cases: Vec<_> = economy_demo::Case::ALL
            .into_iter()
            .filter(|c| selection == "all" || selection == c.name())
            .collect();
        if cases.is_empty() {
            return Err(format!("unknown economy case {selection:?}\n{HELP}").into());
        }
        let reports = cases
            .into_iter()
            .map(|case| economy_demo::run(case).map(|run| developer::economy_report(&run)))
            .collect::<Result<Vec<_>, _>>()?;
        println!("Developer-only advertiser-funded economy. Integer credits; provisional coefficients.\nActual targeting/hype are internal evidence, not player research.");
        for report in &reports {
            summary(&report.broadcasts);
            for station in &report.finances.stations {
                println!(
                    "  {:?}: revenue {}, broadcast costs {}, profit {}, cash {}",
                    station.station,
                    station.revenue,
                    station.broadcast_costs,
                    station.profit,
                    station.cash.0
                );
            }
            for brand in &report.finances.brands {
                println!(
                    "  {:?}: spent {}, remaining budget {}, escrow {}",
                    brand.brand, brand.spent, brand.remaining_budget.0, brand.unsettled_escrow
                );
            }
            if let (Some(first), Some(last)) = (report.offers.first(), report.offers.last()) {
                println!(
                    "  Demand pressure {:.4} -> {:.4}; recovery after 48h {:.4} (budget {})",
                    first.demand.pressure,
                    report.demand_after_campaigns.pressure,
                    report.recovery_quote.demand.pressure,
                    report.recovery_quote.demand.available_budget.0
                );
                println!("  First forecast {:.2} relevant impressions, promised {}, offer {} credits; final offer {}; recovery offer {}",first.forecast_relevant_delivery,first.promised_whole_delivery,first.payment.0,last.payment.0,report.recovery_quote.payment.0);
            }
            println!("  Same future-slot quote before/after decay: {} -> {} credits; forecast delivery unchanged at {:.2}",report.before_cooling_quote.payment.0,report.recovery_quote.payment.0,report.recovery_quote.forecast_relevant_delivery);
            let forecast_total = report
                .contracts
                .iter()
                .map(|c| c.offer.forecast_relevant_delivery)
                .sum::<f64>();
            let actual_total = report
                .contracts
                .iter()
                .map(|c| c.actual.relevant_delivery)
                .sum::<f64>();
            println!("  Accepted campaign target delivery: forecast {:.2}, actual {:.2} relevant impressions",forecast_total,actual_total);
            if verbose {
                trace(&report.broadcasts);
                for offer in &report.offers {
                    println!(
                        "  Quote hour {}: demand {:?}, price {:.3} -> {} credits",
                        offer.proposal.at.0, offer.demand, offer.uncapped_price, offer.payment.0
                    );
                    for audience in &offer.audiences {
                        println!(
                            "    forecast {:?}: priority {}, viewers {:.2}, relevant {:.2}; {:?}",
                            audience.audience,
                            audience.priority,
                            audience.expected_viewers,
                            audience.relevant_delivery,
                            audience.effectiveness
                        );
                    }
                }
                for contract in &report.contracts {
                    println!(
                        "  Contract {:?}: actual {:?}, status {:?}",
                        contract.id, contract.actual, contract.status
                    );
                }
                for entry in &report.finances.ledger {
                    println!("  Ledger {:?}", entry);
                }
            }
        }
        println!("\nControlled comparisons: reach, target delivery and station profit can favour different schedules.\nDemand recovery cannot replenish spent budget. Hype is not advertiser sales; no universal winner is claimed.");
        if let Some(path) = json {
            fs::write(&path, serde_json::to_vec_pretty(&reports)?)?;
            println!("\nWrote full developer JSON to {}", path.display());
        }
        return Ok(());
    }
    let cases: Vec<_> = Case::ALL
        .into_iter()
        .filter(|case| selection == "all" || selection == case.name())
        .collect();
    if cases.is_empty() {
        return Err(format!("unknown case {selection:?}\n{HELP}").into());
    }
    let reports = cases
        .into_iter()
        .map(|case| demo::run(case).map(|run| developer::report(&run)))
        .collect::<Result<Vec<_>, _>>()?;
    println!("Developer-only broadcasting experiment; all coefficients provisional.\nExpected viewer-slot impressions, not unique reach. Hype is bounded [0,100].");
    for report in &reports {
        summary(report);
        if verbose {
            trace(report);
        }
    }
    println!("\nTradeoffs: compare total viewing, specialist delivery, satisfaction, and return willingness.\nHype is not revenue. These fixed cases do not establish balance or a universal winner.");
    if let Some(path) = json {
        fs::write(&path, serde_json::to_vec_pretty(&reports)?)?;
        println!("\nWrote full developer JSON to {}", path.display());
    }
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
