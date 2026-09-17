//! End-to-end checks of the CLI binary: JSON output never contains `null`
//! for a numeric field, invalid numeric input is rejected with a non-zero
//! exit code, and non-validated models print their caveat in text mode.

use std::process::Command;

fn run(args: &[&str]) -> (bool, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_pcb-toolkit"))
        .args(args)
        .output()
        .unwrap();
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn assert_numeric_json(stdout: &str) {
    let v: serde_json::Value = serde_json::from_str(stdout).expect("valid JSON");
    for (k, x) in v.as_object().unwrap() {
        assert!(
            x.is_number() || x.is_null() && (k.ends_with("_db") || k == "xc_ohms"),
            "{k} = {x}"
        );
    }
}

#[test]
fn json_outputs_are_numeric() {
    let cases: Vec<Vec<&str>> = vec![
        vec![
            "--json",
            "impedance",
            "microstrip",
            "-w",
            "10mil",
            "--height",
            "5mil",
            "-f",
            "1GHz",
        ],
        vec![
            "--json",
            "impedance",
            "stripline",
            "-w",
            "100mil",
            "--height",
            "5mil",
        ],
        vec![
            "--json",
            "impedance",
            "embedded",
            "-w",
            "10mil",
            "--height",
            "5mil",
            "--cover-height",
            "1mil",
        ],
        vec![
            "--json",
            "impedance",
            "coplanar",
            "-w",
            "10mil",
            "-g",
            "5mil",
            "--height",
            "5mil",
            "-t",
            "0",
        ],
        vec![
            "--json",
            "differential",
            "edge-coupled-internal-asym",
            "-w",
            "10",
            "-s",
            "5",
            "--height1",
            "1",
            "--height2",
            "19",
        ],
        vec![
            "--json",
            "differential",
            "broadside-coupled",
            "-w",
            "10",
            "--separation",
            "0.1",
            "--height-total",
            "30",
            "-t",
            "0.01",
            "--shielded",
        ],
        vec![
            "--json",
            "current",
            "-w",
            "10mil",
            "-l",
            "1in",
            "--ambient",
            "60",
        ],
        vec![
            "--json",
            "via",
            "--hole",
            "10",
            "--pad",
            "20",
            "--antipad",
            "40",
            "--height",
            "62",
        ],
        vec![
            "--json",
            "ohms-law",
            "pi-pad",
            "--attenuation",
            "20",
            "--impedance",
            "50",
        ],
        vec!["--json", "ohms-law", "capacitors-series", "10pF", "10pF"],
        vec!["--json", "spacing", "-v", "10", "-d", "b4"],
        vec![
            "--json",
            "pdn",
            "--voltage",
            "5",
            "--current",
            "2",
            "--i-step",
            "50",
            "--v-ripple",
            "5",
            "--area-sq-in",
            "5",
            "--er",
            "4.6",
            "--distance",
            "2mil",
            "--freq",
            "1MHz",
        ],
    ];
    for args in cases {
        let (ok, stdout, stderr) = run(&args);
        assert!(ok, "{args:?} failed: {stderr}");
        assert_numeric_json(&stdout);
    }
}

#[test]
fn nan_and_overflow_inputs_are_rejected() {
    let (ok, _, stderr) = run(&["spacing", "-v", "NaN", "-d", "b1"]);
    assert!(!ok, "NaN voltage must be rejected");
    assert!(
        stderr.contains("not finite") || stderr.contains("out of range"),
        "{stderr}"
    );
    let (ok, _, _) = run(&["impedance", "microstrip", "-w", "1e308in", "--height", "5"]);
    assert!(!ok);
    let (ok, _, _) = run(&["current", "-w", "1mil", "-l", "1in", "-e", "1:1"]);
    assert!(!ok, "impossible etched trapezoid must be rejected");
    let (ok, _, stderr) = run(&[
        "differential",
        "broadside-coupled",
        "-w",
        "10",
        "--separation",
        "5",
        "--height-total",
        "30",
    ]);
    assert!(!ok);
    assert!(stderr.contains("unsupported"), "{stderr}");
}

#[test]
fn non_validated_models_print_caveats() {
    let (ok, stdout, _) = run(&[
        "crosstalk",
        "--rise-time",
        "1",
        "--voltage",
        "5",
        "-l",
        "1000",
        "-s",
        "10",
        "--height",
        "5",
        "-w",
        "10",
    ]);
    assert!(ok);
    assert!(stdout.contains("[experimental]"), "{stdout}");
    let (ok, stdout, _) = run(&[
        "differential",
        "edge-coupled-external",
        "-w",
        "10",
        "-s",
        "5",
        "--height",
        "15",
        "-t",
        "2.1",
    ]);
    assert!(ok);
    assert!(stdout.contains("[compatibility]"), "{stdout}");
    let (ok, stdout, _) = run(&["impedance", "microstrip", "-w", "10", "--height", "5"]);
    assert!(ok);
    assert!(!stdout.contains("Model:"), "{stdout}");
}

#[test]
fn pi_pad_prints_corrected_series_resistor() {
    let (ok, stdout, _) = run(&[
        "ohms-law",
        "pi-pad",
        "--attenuation",
        "20",
        "--impedance",
        "50",
    ]);
    assert!(ok);
    assert!(stdout.contains("247.5000"), "{stdout}");
}
