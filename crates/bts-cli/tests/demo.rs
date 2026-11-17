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
        assert_eq!(report["schema_version"], 1);
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
