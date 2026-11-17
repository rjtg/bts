use serde_json::Value;
use std::{fs, process::Command};

#[test]
fn exported_cases_have_readable_summaries_full_traces_and_reproduce() {
    let path = std::env::temp_dir().join(format!("bts-cli-test-{}.json", std::process::id()));
    let execute = || {
        Command::new(env!("CARGO_BIN_EXE_bts-cli"))
            .args(["demo", "--json"])
            .arg(&path)
            .output()
            .unwrap()
    };
    let first = execute();
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    let summary = String::from_utf8(first.stdout).unwrap();
    for phrase in [
        "broad",
        "targeted",
        "disappointing",
        "viewer-slot impressions",
        "willingness",
        "after 48h idle",
        "not revenue",
    ] {
        assert!(summary.contains(phrase), "missing {phrase}");
    }
    let data = fs::read(&path).unwrap();
    let reports: Value = serde_json::from_slice(&data).unwrap();
    assert_eq!(reports.as_array().unwrap().len(), 3);
    for report in reports.as_array().unwrap() {
        assert_eq!(report["schema_version"], 4);
        assert_eq!(report["slots"].as_array().unwrap().len(), 13);
        assert_eq!(
            report["initial_world"]["audiences"]
                .as_array()
                .unwrap()
                .len(),
            3
        );
        let viewing = &report["slots"][0]["allocations"][0]["viewing"][0];
        assert!(viewing["attraction"]["inputs"].is_array());
        assert!(viewing["attraction"]["rules"][0]["condition"].is_object());
        assert!(viewing["attraction"]["rules"][0]["effective_weight"].is_number());
        assert!(viewing["satisfaction"]["output"].is_number());
        let cooldown = report["slots"].as_array().unwrap().last().unwrap();
        assert_eq!(cooldown["elapsed_hours"], 48);
        for h in cooldown["hype"].as_array().unwrap() {
            assert_eq!(h["campaign_generated"], 0.0);
            assert_eq!(h["expected_exposures"], 0.0);
        }
    }
    assert!(execute().status.success());
    assert_eq!(fs::read(&path).unwrap(), data);
    fs::remove_file(&path).unwrap();
}

#[test]
fn case_selection_trace_and_invalid_arguments() {
    let output = Command::new(env!("CARGO_BIN_EXE_bts-cli"))
        .args(["demo", "--case", "targeted", "--trace"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("targeted (identical initial state"));
    assert!(!stdout.contains("broad (identical initial state"));
    assert!(stdout.contains("configured weight"));
    assert!(stdout.contains("effectiveness"));
    for args in [
        vec!["demo", "--case", "unknown"],
        vec!["demo", "--json"],
        vec!["demo", "--unknown"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_bts-cli"))
            .args(args)
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(!output.stderr.is_empty());
    }
}

#[test]
fn economy_export_keeps_forecast_actual_contracts_and_accounts_distinct() {
    let path = std::env::temp_dir().join(format!("bts-economy-test-{}.json", std::process::id()));
    let run = || {
        Command::new(env!("CARGO_BIN_EXE_bts-cli"))
            .args(["economy", "--json"])
            .arg(&path)
            .output()
            .unwrap()
    };
    let output = run();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let summary = String::from_utf8(output.stdout).unwrap();
    for phrase in [
        "mass-broad",
        "specialist-targeted",
        "profit",
        "remaining budget",
        "Demand pressure",
        "recovery after 48h",
        "forecast",
        "actual",
    ] {
        assert!(summary.contains(phrase), "missing {phrase}");
    }
    let bytes = fs::read(&path).unwrap();
    let reports: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(reports.as_array().unwrap().len(), 4);
    for report in reports.as_array().unwrap() {
        assert_eq!(report["schema_version"], 4);
        assert_eq!(report["offers"].as_array().unwrap().len(), 12);
        assert_eq!(report["finances"]["total_money"], 80000);
        assert!(report["offers"][0]["audiences"][0]["effectiveness"]["rules"].is_array());
        let budget = report["recovery_quote"]["demand"]["available_budget"]
            .as_u64()
            .unwrap();
        assert!(report["recovery_quote"]["payment"].as_u64().unwrap() <= budget);
        assert!(
            report["recovery_quote"]["demand"]["pressure"]
                .as_f64()
                .unwrap()
                > report["demand_after_campaigns"]["pressure"]
                    .as_f64()
                    .unwrap()
        );
        assert!(!report["contracts"].as_array().unwrap().is_empty());
        for contract in report["contracts"].as_array().unwrap() {
            assert!(contract["offer"]["forecast_relevant_delivery"].is_number());
            assert!(contract["actual"]["credited_units"].is_u64());
            let settlement = &contract["status"]["Settled"];
            assert_eq!(
                settlement["payment"].as_u64().unwrap() + settlement["refund"].as_u64().unwrap(),
                contract["offer"]["payment"].as_u64().unwrap()
            );
        }
    }
    assert!(run().status.success());
    assert_eq!(fs::read(&path).unwrap(), bytes);
    fs::remove_file(path).unwrap();
    let invalid = Command::new(env!("CARGO_BIN_EXE_bts-cli"))
        .args(["economy", "--case", "not-a-case"])
        .output()
        .unwrap();
    assert!(!invalid.status.success());
}

#[test]
fn advertiser_value_export_is_controlled_complete_and_reproducible() {
    let path = std::env::temp_dir().join(format!("bts-value-test-{}.json", std::process::id()));
    let execute = || {
        Command::new(env!("CARGO_BIN_EXE_bts-cli"))
            .args(["advertiser-value", "--json"])
            .arg(&path)
            .output()
            .unwrap()
    };
    let output = execute();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("Repetition is not directly priced"));
    assert!(text.contains("weak creative"));
    let bytes = fs::read(&path).unwrap();
    let reports: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(reports.as_array().unwrap().len(), 6);
    for report in reports.as_array().unwrap() {
        assert_eq!(report["schema_version"], 4);
        assert_eq!(report["total_money"], 200000);
        assert!(report["offer"]["valuation"][0]["uncapped_value"].is_number());
        assert!(report["offer"]["audiences"][0]["effectiveness"]["rules"].is_array());
        assert_eq!(
            report["slot"]["action"]["broadcasts"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            report["slot"]["aired"]["broadcasts"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
        assert!(report["slot"]["off_air"].as_array().unwrap().is_empty());
    }
    assert_eq!(
        reports[0]["offer"]["forecast_relevant_delivery"],
        reports[2]["offer"]["forecast_relevant_delivery"]
    );
    assert_eq!(reports[0]["payment"], reports[2]["payment"]);
    assert!(
        reports[0]["campaign_generated_hype"].as_f64().unwrap()
            > reports[2]["campaign_generated_hype"].as_f64().unwrap() * 3.0
    );
    assert!(execute().status.success());
    assert_eq!(fs::read(&path).unwrap(), bytes);
    fs::remove_file(path).unwrap();
}

#[test]
fn episode_exports_show_identity_familiarity_reception_and_continuity_controls() {
    let path = std::env::temp_dir().join(format!("bts-episodes-test-{}.json", std::process::id()));
    let execute = || {
        Command::new(env!("CARGO_BIN_EXE_bts-cli"))
            .args(["episodes", "--json"])
            .arg(&path)
            .output()
            .unwrap()
    };
    let output = execute();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    for s in [
        "standalone-movie",
        "strong-opening",
        "disappointing-opening",
        "earlier-repeat-control",
        "out-of-order-high",
        "fresh",
        "frontier",
    ] {
        assert!(text.contains(s));
    }
    let bytes = fs::read(&path).unwrap();
    let reports: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(reports.as_array().unwrap().len(), 8);
    assert_eq!(reports[0]["schema_version"], 4);
    assert!(reports[0]["slots"][0]["allocations"][0]["viewing"][0]["series"].is_null());
    assert_eq!(reports[3]["broadcast_history"].as_array().unwrap().len(), 4);
    let repeat = &reports[3]["slots"][2]["allocations"][0]["viewing"][0];
    assert_eq!(repeat["history"]["frontier_before"], 2);
    assert_eq!(repeat["history"]["frontier_after"], 2);
    assert!(
        repeat["fresh_viewers"].as_f64().unwrap() < repeat["expected_viewers"].as_f64().unwrap()
    );
    assert!(execute().status.success());
    assert_eq!(fs::read(&path).unwrap(), bytes);
    fs::remove_file(path).unwrap();
}
