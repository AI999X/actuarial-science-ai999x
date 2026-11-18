//! Actuarial Science and Insurance: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p actuarial-science-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&actuarial_science_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "act_buhlmann_credibility", "inputs": {"mu": "1000", "EPV": "50000", "VHM": "10000", "n": "5", "xbar": "1200"}}));
    println!();
    show(json!({"key": "act_accumulated_value", "inputs": {"P": "1000", "i": "0.05", "t": "10"}}));
    println!();
    show(json!({"key": "act_deferred_annuity", "inputs": {"q": [0.1, 0.2, 0.5, 1], "i": "0.05", "u": "2", "P": "1000"}}));
    println!();
    show(json!({"key": "act_continuous_insurance_udd", "inputs": {"q": [0.1, 0.2, 0.5, 1], "i": "0.05"}}));
    println!();
    show(json!({"key": "act_commutation_functions", "inputs": {"lx": [1000, 900, 700, 400, 0], "x0": "95", "x": "95", "i": "0.05"}}));
    println!();
    show(json!({"key": "act_compound_poisson_moments", "inputs": {"lam": "3", "m1": "2", "m2": "6", "m3": "24"}}));
    println!();
    show(json!({"key": "act_bornhuetter_ferguson", "inputs": {"T": [[100, 150, 175], [110, 168, 0], [120, 0, 0]], "P": [200, 210, 220], "elr": "0.8"}}));
    println!();
    show(json!({"key": "act_multiple_decrement_to_single", "inputs": {"q1": "0.1", "q2": "0.2"}}));
    println!();
    show(json!({"key": "act_joint_life_last_survivor", "inputs": {"qx": [0.1, 0.2, 0.5, 1], "qy": [0.05, 0.1, 0.3, 0.6, 1], "i": "0.05"}}));
    println!();
    show(json!({"key": "act_gross_premium", "inputs": {"q": [0.1, 0.2, 0.5, 1], "i": "0.05", "S": "1000", "e0": "0.4", "r": "0.05", "c": "10", "E": "20"}}));
    println!();
    show(json!({"key": "act_pure_premium_rate", "inputs": {"L": "500000", "X": "2000", "Fe": "30", "V": "0.15", "Q": "0.05"}}));
    println!();
    show(json!({"key": "act_makeham_continuous_whole_life", "inputs": {"A": "0.00022", "B": "2.7e-06", "c": "1.124", "x": "50", "i": "0.05", "S": "100000", "t": "10"}}));
    println!();
    show(json!({"key": "act_var_tvar_lognormal", "inputs": {"mu": "7", "sigma": "1", "p": "0.95"}}));
    println!();
    show(json!({"key": "act_exponential_mle", "inputs": {"times": [1, 2, 2, 3, 5], "events": [1, 1, 0, 1, 0], "t": "3"}}));
    println!();
    show(json!({"key": "act_constant_force_mortality", "inputs": {"mu": "0.02", "t": "10"}}));
}
