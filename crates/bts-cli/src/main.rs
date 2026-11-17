use bts_sim::{
    demo::{self, Case},
    developer::{self, DeveloperReport},
};
use std::{env, error::Error, fs, path::PathBuf};

const HELP:&str="Usage: bts-cli demo [--case all|broad|targeted|disappointing] [--json PATH] [--trace]\n\nDeterministic developer experiment. JSON contains full inputs, rules and traces.\nCounts are viewer-slot impressions; they are not cumulative unique reach.";

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
    match args.next().as_deref() {
        Some("--help" | "-h") => {
            println!("{HELP}");
            return Ok(());
        }
        Some("demo") => {}
        _ => return Err(HELP.into()),
    }
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
