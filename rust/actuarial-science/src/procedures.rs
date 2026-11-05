//! actuarial_science: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/actuarial_science/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "act_accumulated_value",
    "act_annuity_certain",
    "act_annuity_yield",
    "act_bornhuetter_ferguson",
    "act_buhlmann_credibility",
    "act_buhlmann_empirical",
    "act_buhlmann_straub",
    "act_cape_cod",
    "act_central_death_rate",
    "act_chain_ladder_factors",
    "act_chain_ladder_reserve",
    "act_chain_ladder_simple_average",
    "act_commutation_functions",
    "act_compound_distribution_moments",
    "act_compound_poisson_moments",
    "act_constant_force_mortality",
    "act_continuous_insurance_udd",
    "act_de_moivre_law",
    "act_deferred_annuity",
    "act_deferred_death_probability",
    "act_deferred_insurance",
    "act_duration_convexity",
    "act_endowment_insurance",
    "act_expected_claims_method",
    "act_exponential_coverage_modifications",
    "act_exponential_deductible",
    "act_exponential_mle",
    "act_force_of_interest",
    "act_fractional_age",
    "act_gompertz_law",
    "act_gross_premium",
    "act_gross_premium_reserve",
    "act_increasing_decreasing_annuity",
    "act_individual_risk_model",
    "act_interest_rate_conversion",
    "act_joint_life_last_survivor",
    "act_kaplan_meier",
    "act_life_expectancy",
    "act_life_table_functions",
    "act_limited_fluctuation_full_credibility",
    "act_limited_fluctuation_severity_aggregate",
    "act_loan_amortization",
    "act_lognormal_deductible",
    "act_loss_at_issue",
    "act_loss_ratio_rate_change",
    "act_mack_standard_error",
    "act_makeham_continuous_whole_life",
    "act_makeham_law",
    "act_mthly_annuity_udd",
    "act_multiple_decrement_to_single",
    "act_negative_binomial_frequency",
    "act_nelson_aalen",
    "act_net_premium_endowment",
    "act_net_premium_limited_pay",
    "act_net_premium_term",
    "act_net_premium_whole_life",
    "act_nominal_to_effective_rate",
    "act_normal_approximation_aggregate",
    "act_panjer_negative_binomial",
    "act_panjer_poisson",
    "act_pareto_coverage_modifications",
    "act_pareto_deductible",
    "act_partial_credibility",
    "act_perpetuity",
    "act_poisson_frequency",
    "act_poisson_gamma_credibility",
    "act_policy_value_recursion",
    "act_premium_principles",
    "act_present_value_cash_flows",
    "act_pure_endowment",
    "act_pure_premium_rate",
    "act_reserve_endowment",
    "act_reserve_term",
    "act_reserve_whole_life",
    "act_retrospective_reserve",
    "act_reversionary_annuity",
    "act_single_loss_payment",
    "act_single_to_multiple_decrement",
    "act_stop_loss_premium",
    "act_survival_probability",
    "act_temporary_annuity",
    "act_temporary_expectation",
    "act_term_insurance",
    "act_thiele_endowment",
    "act_var_tvar_exponential",
    "act_var_tvar_lognormal",
    "act_var_tvar_normal",
    "act_var_tvar_pareto",
    "act_whole_life_annuity",
    "act_whole_life_insurance",
    "act_woolhouse_approximation",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "act_accumulated_value" => {
            let r = f_accumulated_value(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_24(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_annuity_certain" => {
            let r = f_annuity_certain(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_75(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_annuity_yield" => {
            let r = f_annuity_yield(match v.get("PV") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_annuity_yield(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_bornhuetter_ferguson" => {
            let r = f_bornhuetter_ferguson(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("elr") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_225(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_buhlmann_credibility" => {
            let r = f_buhlmann_premium(match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("EPV") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VHM") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xbar") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_273(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_buhlmann_empirical" => {
            let r = f_buhlmann_empirical(match v.get("X") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_332(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_buhlmann_straub" => {
            let r = f_buhlmann_straub(match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_401(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_cape_cod" => {
            let r = f_cape_cod(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_474(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_central_death_rate" => {
            let r = f_central_death_rate(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("assumption") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_523(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_chain_ladder_factors" => {
            let r = f_chain_ladder_factors(match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_584(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_chain_ladder_reserve" => {
            let r = f_chain_ladder_reserve(match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_651(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_chain_ladder_simple_average" => {
            let r = f_chain_ladder_simple(match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_717(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_commutation_functions" => {
            let r = f_commutation_functions(match v.get("lx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_792(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_compound_distribution_moments" => {
            let r = f_compound_moments(match v.get("EN") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VarN") { Some(x) => py::from_val(x), None => { return None } }, match v.get("EX") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VarX") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_837(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_compound_poisson_moments" => {
            let r = f_compound_poisson_moments(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m3") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_893(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_constant_force_mortality" => {
            let r = f_constant_force(match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_940(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_continuous_insurance_udd" => {
            let r = f_continuous_insurance_udd(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1003(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_de_moivre_law" => {
            let r = f_de_moivre(match v.get("omega") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1055(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_deferred_annuity" => {
            let r = f_deferred_annuity(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1124(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_deferred_death_probability" => {
            let r = f_deferred_death_probability(match v.get("lx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1191(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_deferred_insurance" => {
            let r = f_deferred_insurance(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1260(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_duration_convexity" => {
            let r = f_duration_convexity(match v.get("amounts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1320(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_endowment_insurance" => {
            let r = f_endowment_insurance(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1387(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_expected_claims_method" => {
            let r = f_expected_claims(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("elr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("paid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1445(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_exponential_coverage_modifications" => {
            let r = f_exponential_coverage(match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1526(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_exponential_deductible" => {
            let r = f_exponential_deductible(match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1596(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_exponential_mle" => {
            let r = f_exponential_mle(match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1658(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_force_of_interest" => {
            let r = f_force_of_interest(match v.get("delta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1702(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_fractional_age" => {
            let r = f_fractional_age(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("assumption") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1756(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_gompertz_law" => {
            let r = f_gompertz(match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1824(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_gross_premium" => {
            let r = f_gross_premium(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1909(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_gross_premium_reserve" => {
            let r = f_gross_premium_reserve(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1998(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_increasing_decreasing_annuity" => {
            let r = f_increasing_annuity(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2049(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_individual_risk_model" => {
            let r = f_individual_risk_model(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2103(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_interest_rate_conversion" => {
            let r = f_rate_conversion(match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2153(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_joint_life_last_survivor" => {
            let r = f_joint_life(match v.get("qx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("qy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2222(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_kaplan_meier" => {
            let r = f_kaplan_meier(match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2297(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_life_expectancy" => {
            let r = f_life_expectancy(match v.get("lx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2363(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_life_table_functions" => {
            let r = f_life_table_functions(match v.get("lx") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2418(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_limited_fluctuation_full_credibility" => {
            let r = f_lf_full_credibility(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2477(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_limited_fluctuation_severity_aggregate" => {
            let r = f_lf_full_credibility_severity(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mean_x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd_x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2540(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_loan_amortization" => {
            let r = f_loan_amortization(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2596(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_lognormal_deductible" => {
            let r = f_lognormal_deductible(match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2670(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_loss_at_issue" => {
            let r = f_loss_at_issue(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2741(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_loss_ratio_rate_change" => {
            let r = f_loss_ratio_rate_change(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("EP") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2805(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_mack_standard_error" => {
            let r = f_mack_standard_error(match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2886(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_makeham_continuous_whole_life" => {
            let r = f_makeham_continuous(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2962(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_makeham_law" => {
            let r = f_makeham(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3030(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_mthly_annuity_udd" => {
            let r = f_mthly_annuity_udd(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3095(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_multiple_decrement_to_single" => {
            let r = f_multiple_to_single(match v.get("q1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3144(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_negative_binomial_frequency" => {
            let r = f_negative_binomial_frequency(match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3199(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_nelson_aalen" => {
            let r = f_nelson_aalen(match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3274(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_net_premium_endowment" => {
            let r = f_net_premium_endowment(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3342(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_net_premium_limited_pay" => {
            let r = f_net_premium_limited_pay(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3412(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_net_premium_term" => {
            let r = f_net_premium_term(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3480(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_net_premium_whole_life" => {
            let r = f_net_premium_whole_life(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3548(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_nominal_to_effective_rate" => {
            let r = f_nominal_to_effective(match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("kind") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3605(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_normal_approximation_aggregate" => {
            let r = f_normal_approx_aggregate(match v.get("mean") { Some(x) => py::from_val(x), None => { return None } }, match v.get("var") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3650(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_panjer_negative_binomial" => {
            let r = f_panjer_negative_binomial(match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("smax") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3716(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_panjer_poisson" => {
            let r = f_panjer_poisson(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("smax") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3778(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_pareto_coverage_modifications" => {
            let r = f_pareto_coverage(match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3863(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_pareto_deductible" => {
            let r = f_pareto_deductible(match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3937(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_partial_credibility" => {
            let r = f_partial_credibility(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_full") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xbar") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3986(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_perpetuity" => {
            let r = f_perpetuity(match v.get("i") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4030(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_poisson_frequency" => {
            let r = f_poisson_frequency(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4081(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_poisson_gamma_credibility" => {
            let r = f_poisson_gamma(match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("counts") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4132(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_policy_value_recursion" => {
            let r = f_policy_value_recursion(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4189(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_premium_principles" => {
            let r = f_premium_principle(match v.get("mean") { Some(x) => py::from_val(x), None => { return None } }, match v.get("var") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("principle") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4239(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_present_value_cash_flows" => {
            let r = f_pv_cash_flows(match v.get("amounts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4292(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_pure_endowment" => {
            let r = f_pure_endowment_lx(match v.get("lx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4363(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_pure_premium_rate" => {
            let r = f_pure_premium_rate(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Fe") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4427(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_reserve_endowment" => {
            let r = f_reserve_endowment(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4499(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_reserve_term" => {
            let r = f_reserve_term(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4571(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_reserve_whole_life" => {
            let r = f_reserve_whole_life(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4641(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_retrospective_reserve" => {
            let r = f_retrospective_reserve(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4713(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_reversionary_annuity" => {
            let r = f_reversionary_annuity(match v.get("qx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("qy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4784(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_single_loss_payment" => {
            let r = f_single_loss_payment(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4835(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_single_to_multiple_decrement" => {
            let r = f_single_to_multiple(match v.get("q1s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q2s") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4885(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_stop_loss_premium" => {
            let r = f_stop_loss_compound_poisson(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4950(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_survival_probability" => {
            let r = f_survival_probability(match v.get("lx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5015(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_temporary_annuity" => {
            let r = f_temporary_annuity(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5082(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_temporary_expectation" => {
            let r = f_temporary_expectation(match v.get("lx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5152(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_term_insurance" => {
            let r = f_term_insurance(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5219(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_thiele_endowment" => {
            let r = f_thiele_endowment(match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("delta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5283(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_var_tvar_exponential" => {
            let r = f_var_tvar_exponential(match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5331(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_var_tvar_lognormal" => {
            let r = f_var_tvar_lognormal(match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5394(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_var_tvar_normal" => {
            let r = f_var_tvar_normal(match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5454(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_var_tvar_pareto" => {
            let r = f_var_tvar_pareto(match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5506(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_whole_life_annuity" => {
            let r = f_whole_life_annuity(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5574(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_whole_life_insurance" => {
            let r = f_whole_life_insurance(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5641(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "act_woolhouse_approximation" => {
            let r = f_woolhouse(match v.get("a_due") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5698(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c__GLW() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::Float(0.0666713443086881), Py::Float(0.1494513491505806), Py::Float(0.219086362515982), Py::Float(0.2692667193099963), Py::Float(0.2955242247147529), Py::Float(0.2955242247147529), Py::Float(0.2692667193099963), Py::Float(0.219086362515982), Py::Float(0.1494513491505806), Py::Float(0.0666713443086881)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__GLX() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::Float(-0.9739065285171717), Py::Float(-0.8650633666889845), Py::Float(-0.6794095682990244), Py::Float(-0.4333953941292472), Py::Float(-0.1488743389816312), Py::Float(0.1488743389816312), Py::Float(0.4333953941292472), Py::Float(0.6794095682990244), Py::Float(0.8650633666889845), Py::Float(0.9739065285171717)])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ann(mut v_q: Py, mut v_v: Py, mut v_n: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_kp: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_kp = f__kp(v_q.clone())?;
    v_s = Py::Float(0.0);
    for t5560 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t5560;
        v_s = py::iadd(&v_s, &py::mul(&py::pow(&v_v.clone(), &v_k.clone())?, &py::getitem(&v_kp.clone(), &v_k.clone())?)?)?;
    }
    return Ok(v_s.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__annuity_values(mut v_n: Py, mut v_i: Py) -> R {
    let mut v_an: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_an = py::truediv(&py::sub(&Py::Float(1.0), &py::pow(&v_v.clone(), &v_n.clone())?)?, &v_i.clone())?;
    v_d = py::mul(&v_i.clone(), &v_v.clone())?;
    return Ok(Py::tuple(vec![v_v.clone(), v_an.clone(), py::mul(&v_an.clone(), &py::add(&Py::Float(1.0), &v_i.clone())?)?, py::truediv(&py::sub(&py::pow(&py::add(&Py::Float(1.0), &v_i.clone())?, &v_n.clone())?, &Py::Float(1.0))?, &v_i.clone())?, py::truediv(&py::sub(&py::pow(&py::add(&Py::Float(1.0), &v_i.clone())?, &v_n.clone())?, &Py::Float(1.0))?, &v_d.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bisect(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_fa: Py = Py::None;
    let mut v_fb: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t117 = (Py::tuple(vec![py::call(&v_f.clone(), &[v_a.clone()])?, py::call(&v_f.clone(), &[v_b.clone()])?])).items()?;
    if t117.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t117[0].clone();
    v_fb = t117[1].clone();
    if ({ let t118 = v_fa.clone(); Py::Bool({ let t119 = Py::Int(0i64); if py::eq(&t118, &t119) { true } else { false } }) }).truthy() {
        return Ok(v_a.clone());
    }
    if ({ let t120 = v_fb.clone(); Py::Bool({ let t121 = Py::Int(0i64); if py::eq(&t120, &t121) { true } else { false } }) }).truthy() {
        return Ok(v_b.clone());
    }
    let _ = f__need({ let t122 = { let t124 = v_fa.clone(); Py::Bool({ let t125 = Py::Int(0i64); if py::gt(&t124, &t125)? { true } else { false } }) }; Py::Bool({ let t123 = { let t126 = v_fb.clone(); Py::Bool({ let t127 = Py::Int(0i64); if py::gt(&t126, &t127)? { true } else { false } }) }; if py::ne(&t122, &t123) { true } else { false } }) }, Py::str("no root in the interval"))?;
    for t128 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t128;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?;
        if ({ let t133 = { let t129 = v_m.clone(); Py::Bool({ let t130 = v_a.clone(); if py::eq(&t129, &t130) { true } else { false } }) }; if t133.truthy() { t133 } else { { let t131 = v_m.clone(); Py::Bool({ let t132 = v_b.clone(); if py::eq(&t131, &t132) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_m.clone()])?;
        if ({ let t134 = v_fm.clone(); Py::Bool({ let t135 = Py::Int(0i64); if py::eq(&t134, &t135) { true } else { false } }) }).truthy() {
            return Ok(v_m.clone());
        }
        if ({ let t136 = { let t138 = v_fm.clone(); Py::Bool({ let t139 = Py::Int(0i64); if py::gt(&t138, &t139)? { true } else { false } }) }; Py::Bool({ let t137 = { let t140 = v_fa.clone(); Py::Bool({ let t141 = Py::Int(0i64); if py::gt(&t140, &t141)? { true } else { false } }) }; if py::eq(&t136, &t137) { true } else { false } }) }).truthy() {
            let t142 = (Py::tuple(vec![v_m.clone(), v_fm.clone()])).items()?;
            if t142.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t142[0].clone();
            v_fa = t142[1].clone();
        } else {
            v_b = v_m.clone();
        }
    }
    return Ok(py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cl_factors(mut v_T: Py, mut v_simple: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_cdf: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_N = py::len(&v_T.clone())?;
    v_f = Py::list(vec![]);
    for t2850 in py::range_iter(&[(py::sub(&v_N.clone(), &Py::Int(1i64))?).i()?]) {
        v_j = t2850;
        if (v_simple.clone()).truthy() {
            v_s = Py::Float(0.0);
            for t2851 in py::range_iter(&[(py::sub(&py::sub(&v_N.clone(), &Py::Int(1i64))?, &v_j.clone())?).i()?]) {
                v_i = t2851;
                v_s = py::iadd(&v_s, &py::truediv(&py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &py::add(&v_j.clone(), &Py::Int(1i64))?)?, &py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_j.clone())?)?)?;
            }
            let _ = py::method(&v_f.clone(), "append", &[py::truediv(&v_s.clone(), &py::sub(&py::sub(&v_N.clone(), &Py::Int(1i64))?, &v_j.clone())?)?])?;
        } else {
            let t2852 = Py::Float(0.0);
            v_a = t2852.clone();
            v_b = t2852.clone();
            for t2853 in py::range_iter(&[(py::sub(&py::sub(&v_N.clone(), &Py::Int(1i64))?, &v_j.clone())?).i()?]) {
                v_i = t2853;
                v_a = py::iadd(&v_a, &py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &py::add(&v_j.clone(), &Py::Int(1i64))?)?)?;
                v_b = py::iadd(&v_b, &py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_j.clone())?)?;
            }
            let _ = py::method(&v_f.clone(), "append", &[py::truediv(&v_a.clone(), &v_b.clone())?])?;
        }
    }
    v_cdf = py::mul(&Py::list(vec![Py::Float(1.0)]), &v_N.clone())?;
    for t2854 in py::range_iter(&[(py::sub(&v_N.clone(), &Py::Int(2i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
        v_j = t2854;
        py::setitem(&v_cdf.clone(), &v_j.clone(), py::mul(&py::getitem(&v_f.clone(), &v_j.clone())?, &py::getitem(&v_cdf.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?)?)?;
    }
    return Ok(Py::tuple(vec![v_f.clone(), v_cdf.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cl_projection(mut v_T: Py, mut v_cdf: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_latest: Py = Py::None;
    let mut v_ult: Py = Py::None;
    v_N = py::len(&v_T.clone())?;
    v_latest = { let mut out = vec![]; for t700 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_i: Py; 
v_i = t700;
out.push(py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &py::sub(&py::sub(&v_N.clone(), &Py::Int(1i64))?, &v_i.clone())?)?); } Py::list(out) };
    v_ult = { let mut out = vec![]; for t701 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_i: Py; 
v_i = t701;
out.push(py::mul(&py::getitem(&v_latest.clone(), &v_i.clone())?, &py::getitem(&v_cdf.clone(), &py::sub(&py::sub(&v_N.clone(), &Py::Int(1i64))?, &v_i.clone())?)?)?); } Py::list(out) };
    return Ok(Py::tuple(vec![v_latest.clone(), v_ult.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__coverage(mut v_dist: Py, mut v_p1: Py, mut v_p2: Py, mut v_d: Py, mut v_u: Py, mut v_c: Py, mut v_r: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_per_loss: Py = Py::None;
    let mut v_sfd: Py = Py::None;
    let _ = f__nonneg(Py::str("the deductible d"), Py::tuple(vec![v_d.clone()]))?;
    let _ = f__need({ let t3820 = f__isnum(v_u.clone())?; if !t3820.truthy() { t3820 } else { { let t3818 = v_u.clone(); Py::Bool({ let t3819 = v_d.clone(); if py::gt(&t3818, &t3819)? { true } else { false } }) } } }, Py::str("the policy limit u must exceed the deductible d"))?;
    let _ = f__need({ let t3824 = f__isnum(v_c.clone())?; if !t3824.truthy() { t3824 } else { { let t3821 = Py::Int(0i64); Py::Bool({ let t3822 = v_c.clone(); if py::lt(&t3821, &t3822)? { { let t3823 = Py::Int(1i64); if py::le(&t3822, &t3823)? { true } else { false } } } else { false } }) } } }, Py::str("the coinsurance c must lie between 0 and 100%"))?;
    let _ = f__need({ let t3828 = f__isnum(v_r.clone())?; if !t3828.truthy() { t3828 } else { { let t3825 = Py::Int(0i64); Py::Bool({ let t3826 = v_r.clone(); if py::le(&t3825, &t3826)? { { let t3827 = Py::Int(1i64); if py::lt(&t3826, &t3827)? { true } else { false } } } else { false } }) } } }, Py::str("the inflation r must lie between 0 and 100%"))?;
    v_f = py::add(&Py::Float(1.0), &v_r.clone())?;
    v_per_loss = py::mul(&py::mul(&v_c.clone(), &v_f.clone())?, &py::sub(&f__lev(v_dist.clone(), v_p1.clone(), v_p2.clone(), py::truediv(&v_u.clone(), &v_f.clone())?)?, &f__lev(v_dist.clone(), v_p1.clone(), v_p2.clone(), py::truediv(&v_d.clone(), &v_f.clone())?)?)?)?;
    v_sfd = f__sf(v_dist.clone(), v_p1.clone(), v_p2.clone(), py::truediv(&v_d.clone(), &v_f.clone())?)?;
    let _ = f__need({ let t3849 = v_sfd.clone(); Py::Bool({ let t3850 = Py::Float(1e-300); if py::gt(&t3849, &t3850)? { true } else { false } }) }, Py::str("the deductible is beyond the range of the losses"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("per_loss"), v_per_loss.clone()), (Py::str("per_payment"), py::truediv(&v_per_loss.clone(), &v_sfd.clone())?), (Py::str("max_payment"), py::mul(&v_c.clone(), &py::sub(&v_u.clone(), &v_d.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__deductible(mut v_dist: Py, mut v_p1: Py, mut v_p2: Py, mut v_d: Py) -> R {
    let mut v_lev: Py = Py::None;
    let mut v_mean: Py = Py::None;
    let mut v_per_loss: Py = Py::None;
    let mut v_sfd: Py = Py::None;
    let _ = f__nonneg(Py::str("the deductible d"), Py::tuple(vec![v_d.clone()]))?;
    v_mean = f__lev(v_dist.clone(), v_p1.clone(), v_p2.clone(), Py::Float(f64::INFINITY))?;
    v_lev = f__lev(v_dist.clone(), v_p1.clone(), v_p2.clone(), v_d.clone())?;
    v_sfd = f__sf(v_dist.clone(), v_p1.clone(), v_p2.clone(), v_d.clone())?;
    let _ = f__need({ let t3923 = v_sfd.clone(); Py::Bool({ let t3924 = Py::Float(1e-300); if py::gt(&t3923, &t3924)? { true } else { false } }) }, Py::str("the deductible is beyond the range of the losses"))?;
    v_per_loss = py::sub(&v_mean.clone(), &v_lev.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("mean"), v_mean.clone()), (Py::str("lev_d"), v_lev.clone()), (Py::str("per_loss"), v_per_loss.clone()), (Py::str("per_payment"), py::truediv(&v_per_loss.clone(), &v_sfd.clone())?), (Py::str("LER"), py::truediv(&v_lev.clone(), &v_mean.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__exp_check(mut v_S: Py, mut v_e0: Py, mut v_r: Py, mut v_c: Py, mut v_E: Py) -> R {
    let _ = f__pos(Py::str("the sum insured S"), Py::tuple(vec![v_S.clone()]))?;
    let _ = f__frac(Py::str("the initial expense fraction e0"), v_e0.clone(), Py::Bool(false), Py::Bool(true))?;
    let _ = f__frac(Py::str("the renewal expense fraction r"), v_r.clone(), Py::Bool(false), Py::Bool(true))?;
    let _ = f__nonneg(Py::str("the per-policy expense c and the settlement expense E"), Py::tuple(vec![v_c.clone(), v_E.clone()]))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t5420 in py::iter(&(v_vals.clone()))? {
        v_v = t5420;
        let _ = f__need(f__isnum(v_v.clone())?, Py::str("every input must be a finite number"))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__frac(mut v_name: Py, mut v_v: Py, mut v_lo_open: Py, mut v_hi_open: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_v.clone()]))?;
    let _ = f__need({ let t4872 = (if (v_lo_open.clone()).truthy() { { let t4864 = v_v.clone(); Py::Bool({ let t4865 = Py::Int(0i64); if py::gt(&t4864, &t4865)? { true } else { false } }) } } else { { let t4866 = v_v.clone(); Py::Bool({ let t4867 = Py::Int(0i64); if py::ge(&t4866, &t4867)? { true } else { false } }) } }); if !t4872.truthy() { t4872 } else { (if (v_hi_open.clone()).truthy() { { let t4868 = v_v.clone(); Py::Bool({ let t4869 = Py::Int(1i64); if py::lt(&t4868, &t4869)? { true } else { false } }) } } else { { let t4870 = v_v.clone(); Py::Bool({ let t4871 = Py::Int(1i64); if py::le(&t4870, &t4871)? { true } else { false } }) } }) } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a fraction between 0 and 1".to_string()].concat()))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gross_G(mut v_q: Py, mut v_i: Py, mut v_S: Py, mut v_e0: Py, mut v_r: Py, mut v_c: Py, mut v_E: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_A = f__ins(v_q.clone(), v_v.clone(), py::len(&v_q.clone())?)?;
    v_a = f__ann(v_q.clone(), v_v.clone(), py::len(&v_q.clone())?)?;
    v_den = py::sub(&py::mul(&v_a.clone(), &py::sub(&Py::Float(1.0), &v_r.clone())?)?, &py::sub(&v_e0.clone(), &v_r.clone())?)?;
    let _ = f__need({ let t1981 = v_den.clone(); Py::Bool({ let t1982 = Py::Int(0i64); if py::gt(&t1981, &t1982)? { true } else { false } }) }, Py::str("the expense loadings take the whole premium"))?;
    return Ok(Py::tuple(vec![py::truediv(&py::add(&py::mul(&py::add(&v_S.clone(), &v_E.clone())?, &v_A.clone())?, &py::mul(&v_c.clone(), &v_a.clone())?)?, &v_den.clone())?, v_A.clone(), v_a.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__idx(mut v_lx: Py, mut v_x0: Py, mut v_x: Py, mut v_steps: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__whole(Py::str("the starting age x0"), v_x0.clone(), Py::Int(0i64))?;
    let _ = f__whole(Py::str("the age x"), v_x.clone(), Py::Int(0i64))?;
    v_k = py::sub(&v_x.clone(), &v_x0.clone())?;
    let _ = f__need({ let t5129 = Py::Int(0i64); Py::Bool({ let t5130 = v_k.clone(); if py::le(&t5129, &t5130)? { { let t5131 = py::len(&v_lx.clone())?; if py::lt(&t5130, &t5131)? { true } else { false } } } else { false } }) }, Py::str("the age x must lie inside the table (from x0 to the last age listed)"))?;
    let _ = f__need({ let t5132 = py::getitem(&v_lx.clone(), &v_k.clone())?; Py::Bool({ let t5133 = Py::Int(0i64); if py::gt(&t5132, &t5133)? { true } else { false } }) }, Py::str("nobody is alive at age x in the table"))?;
    for t5134 in py::iter(&(v_steps.clone()))? {
        v_s = t5134;
        let _ = f__whole(Py::str("every number of years"), v_s.clone(), Py::Int(0i64))?;
    }
    let _ = f__need({ let t5135 = py::add(&v_k.clone(), &py::sum(&v_steps.clone(), None)?)?; Py::Bool({ let t5136 = py::len(&v_lx.clone())?; if py::lt(&t5135, &t5136)? { true } else { false } }) }, Py::str("the ages asked about run past the end of the table"))?;
    return Ok(v_k.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ins(mut v_q: Py, mut v_v: Py, mut v_n: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_kp: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_kp = f__kp(v_q.clone())?;
    v_s = Py::Float(0.0);
    for t5628 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t5628;
        v_s = py::iadd(&v_s, &py::mul(&py::mul(&py::pow(&v_v.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_kp.clone(), &v_k.clone())?)?, &py::getitem(&v_q.clone(), &v_k.clone())?)?)?;
    }
    return Ok(v_s.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__isnum(mut v_v: Py) -> R {
    return Ok({ let t5668 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5668.truthy() { t5668 } else { { let t5667 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5667.truthy() { t5667 } else { py::math1("isfinite", &v_v.clone())? } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__kp(mut v_q: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_out = Py::list(vec![Py::Float(1.0)]);
    for t5627 in py::iter(&(v_q.clone()))? {
        v_v = t5627;
        let _ = py::method(&v_out.clone(), "append", &[py::mul(&py::getitem(&v_out.clone(), &py::neg(&Py::Int(1i64))?)?, &py::sub(&Py::Float(1.0), &v_v.clone())?)?])?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lev(mut v_dist: Py, mut v_p1: Py, mut v_p2: Py, mut v_u: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_lu: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_sg: Py = Py::None;
    let mut v_th: Py = Py::None;
    if ({ let t3903 = v_dist.clone(); Py::Bool({ let t3904 = Py::str("exponential"); if py::eq(&t3903, &t3904) { true } else { false } }) }).truthy() {
        return Ok((if ({ let t3905 = v_u.clone(); Py::Bool({ let t3906 = Py::Float(f64::INFINITY); if py::eq(&t3905, &t3906) { true } else { false } }) }).truthy() { v_p1.clone() } else { py::mul(&v_p1.clone(), &py::sub(&Py::Float(1.0), &py::math1("exp", &py::truediv(&py::neg(&v_u.clone())?, &v_p1.clone())?)?)?)? }));
    }
    if ({ let t3907 = v_dist.clone(); Py::Bool({ let t3908 = Py::str("pareto"); if py::eq(&t3907, &t3908) { true } else { false } }) }).truthy() {
        let t3909 = (Py::tuple(vec![v_p1.clone(), v_p2.clone()])).items()?;
        if t3909.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t3909[0].clone();
        v_th = t3909[1].clone();
        if ({ let t3910 = v_u.clone(); Py::Bool({ let t3911 = Py::Float(f64::INFINITY); if py::eq(&t3910, &t3911) { true } else { false } }) }).truthy() {
            return Ok(py::truediv(&v_th.clone(), &py::sub(&v_a.clone(), &Py::Float(1.0))?)?);
        }
        return Ok(py::mul(&py::truediv(&v_th.clone(), &py::sub(&v_a.clone(), &Py::Float(1.0))?)?, &py::sub(&Py::Float(1.0), &py::pow(&py::truediv(&v_th.clone(), &py::add(&v_th.clone(), &v_u.clone())?)?, &py::sub(&v_a.clone(), &Py::Float(1.0))?)?)?)?);
    }
    let t3912 = (Py::tuple(vec![v_p1.clone(), v_p2.clone()])).items()?;
    if t3912.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mu = t3912[0].clone();
    v_sg = t3912[1].clone();
    v_m = py::math1("exp", &py::add(&v_mu.clone(), &py::truediv(&py::mul(&v_sg.clone(), &v_sg.clone())?, &Py::Float(2.0))?)?)?;
    if ({ let t3913 = v_u.clone(); Py::Bool({ let t3914 = Py::Float(f64::INFINITY); if py::eq(&t3913, &t3914) { true } else { false } }) }).truthy() {
        return Ok(v_m.clone());
    }
    if ({ let t3915 = v_u.clone(); Py::Bool({ let t3916 = Py::Int(0i64); if py::le(&t3915, &t3916)? { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    v_lu = py::math1("log", &v_u.clone())?;
    return Ok(py::add(&py::mul(&v_m.clone(), &f__norm_cdf(py::truediv(&py::sub(&py::sub(&v_lu.clone(), &v_mu.clone())?, &py::mul(&v_sg.clone(), &v_sg.clone())?)?, &v_sg.clone())?)?)?, &py::mul(&v_u.clone(), &py::sub(&Py::Float(1.0), &f__norm_cdf(py::truediv(&py::sub(&v_lu.clone(), &v_mu.clone())?, &v_sg.clone())?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lx_check(mut v_lx: Py, mut v_closed: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t5108 = py::len(&v_lx.clone())?; Py::Bool({ let t5109 = Py::Int(2i64); if py::ge(&t5108, &t5109)? { true } else { false } }) }, Py::str("a life table needs at least two ages"))?;
    for t5110 in py::iter(&(v_lx.clone()))? {
        v_v = t5110;
        let _ = f__need({ let t5115 = f__isnum(v_v.clone())?; if !t5115.truthy() { t5115 } else { { let t5113 = v_v.clone(); Py::Bool({ let t5114 = Py::Int(0i64); if py::ge(&t5113, &t5114)? { true } else { false } }) } } }, Py::str("survivor numbers lx must be zero or positive"))?;
    }
    for t5116 in py::range_iter(&[(py::sub(&py::len(&v_lx.clone())?, &Py::Int(1i64))?).i()?]) {
        v_k = t5116;
        let _ = f__need({ let t5117 = py::getitem(&v_lx.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t5118 = py::getitem(&v_lx.clone(), &v_k.clone())?; if py::le(&t5117, &t5118)? { true } else { false } }) }, Py::str("survivor numbers lx must not increase with age"))?;
        let _ = f__need({ let t5119 = py::getitem(&v_lx.clone(), &v_k.clone())?; Py::Bool({ let t5120 = Py::Int(0i64); if py::gt(&t5119, &t5120)? { true } else { false } }) }, Py::str("only the last survivor number may be 0"))?;
    }
    if (v_closed.clone()).truthy() {
        let _ = f__need({ let t5121 = py::getitem(&v_lx.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t5122 = Py::Int(0i64); if py::eq(&t5121, &t5122) { true } else { false } }) }, Py::str("this needs a complete table: the last survivor number must be 0"))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mat_check(mut v_X: Py, mut v_name: Py) -> R {
    let mut v_row: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t362 = { let t358 = py::len(&v_X.clone())?; Py::Bool({ let t359 = Py::Int(2i64); if py::ge(&t358, &t359)? { true } else { false } }) }; if !t362.truthy() { t362 } else { { let t360 = py::len(&py::getitem(&v_X.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t361 = Py::Int(2i64); if py::ge(&t360, &t361)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " needs at least two risks (rows) and two periods (columns)".to_string()].concat()))?;
    for t363 in py::iter(&(v_X.clone()))? {
        v_row = t363;
        for t364 in py::iter(&(v_row.clone()))? {
            v_v = t364;
            let _ = f__need(f__isnum(v_v.clone())?, Py::str(&["every entry of ".to_string(), py::format(&v_name.clone(), "")?, " must be a finite number".to_string()].concat()))?;
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mk_check(mut v_A: Py, mut v_B: Py, mut v_c: Py, mut v_x: Py, mut v_t: Py) -> R {
    let _ = f__nonneg(Py::str("Makeham's A"), Py::tuple(vec![v_A.clone()]))?;
    let _ = f__need({ let t2997 = f__isnum(v_B.clone())?; if !t2997.truthy() { t2997 } else { { let t2994 = Py::Int(0i64); Py::Bool({ let t2995 = v_B.clone(); if py::lt(&t2994, &t2995)? { { let t2996 = Py::Float(0.1); if py::lt(&t2995, &t2996)? { true } else { false } } } else { false } }) } } }, Py::str("B must lie between 0 and 0.1"))?;
    let _ = f__need({ let t3001 = f__isnum(v_c.clone())?; if !t3001.truthy() { t3001 } else { { let t2998 = Py::Int(1i64); Py::Bool({ let t2999 = v_c.clone(); if py::lt(&t2998, &t2999)? { { let t3000 = Py::Float(1.5); if py::lt(&t2999, &t3000)? { true } else { false } } } else { false } }) } } }, Py::str("c must lie between 1 and 1.5"))?;
    let _ = f__need({ let t3004 = f__isnum(v_A.clone())?; if !t3004.truthy() { t3004 } else { { let t3002 = v_A.clone(); Py::Bool({ let t3003 = Py::Float(0.1); if py::lt(&t3002, &t3003)? { true } else { false } }) } } }, Py::str("A must lie below 0.1"))?;
    let _ = f__need({ let t3008 = f__isnum(v_x.clone())?; if !t3008.truthy() { t3008 } else { { let t3005 = Py::Int(0i64); Py::Bool({ let t3006 = v_x.clone(); if py::le(&t3005, &t3006)? { { let t3007 = Py::Int(120i64); if py::le(&t3006, &t3007)? { true } else { false } } } else { false } }) } } }, Py::str("the age x must lie between 0 and 120"))?;
    let _ = f__need({ let t3012 = f__isnum(v_t.clone())?; if !t3012.truthy() { t3012 } else { { let t3009 = Py::Int(0i64); Py::Bool({ let t3010 = v_t.clone(); if py::le(&t3009, &t3010)? { { let t3011 = Py::Int(100i64); if py::le(&t3010, &t3011)? { true } else { false } } } else { false } }) } } }, Py::str("t must lie between 0 and 100 years"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mk_integral(mut v_A: Py, mut v_B: Py, mut v_c: Py, mut v_x: Py, mut v_delta: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_part: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_wi: Py = Py::None;
    let mut v_xi: Py = Py::None;
    v_tot = Py::Float(0.0);
    v_a = Py::Float(0.0);
    v_h = Py::Float(0.5);
    for t3025 in py::range_iter(&[(Py::Int(4000i64)).i()?]) {
        v__ = t3025;
        v_part = Py::Float(0.0);
        for t3026 in py::iter(&(py::zip(&[c__GLX()?, c__GLW()?])?))? {
            let t3027 = (t3026).items()?;
            if t3027.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_xi = t3027[0].clone();
            v_wi = t3027[1].clone();
            v_u = py::add(&v_a.clone(), &py::truediv(&py::mul(&v_h.clone(), &py::add(&v_xi.clone(), &Py::Float(1.0))?)?, &Py::Float(2.0))?)?;
            v_part = py::iadd(&v_part, &py::mul(&py::mul(&v_wi.clone(), &py::math1("exp", &py::mul(&py::neg(&v_delta.clone())?, &v_u.clone())?)?)?, &f__mk_tp(v_A.clone(), v_B.clone(), v_c.clone(), v_x.clone(), v_u.clone())?)?)?;
        }
        v_tot = py::iadd(&v_tot, &py::truediv(&py::mul(&v_part.clone(), &v_h.clone())?, &Py::Float(2.0))?)?;
        v_a = py::iadd(&v_a, &v_h.clone())?;
        if ({ let t3028 = py::mul(&py::math1("exp", &py::mul(&py::neg(&v_delta.clone())?, &v_a.clone())?)?, &f__mk_tp(v_A.clone(), v_B.clone(), v_c.clone(), v_x.clone(), v_a.clone())?)?; Py::Bool({ let t3029 = py::mul(&Py::Float(1e-18), &v_tot.clone())?; if py::lt(&t3028, &t3029)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    return Ok(v_tot.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mk_tp(mut v_A: Py, mut v_B: Py, mut v_c: Py, mut v_x: Py, mut v_t: Py) -> R {
    return Ok(py::math1("exp", &py::sub(&py::mul(&py::neg(&v_A.clone())?, &v_t.clone())?, &py::truediv(&py::mul(&py::mul(&v_B.clone(), &py::pow(&v_c.clone(), &v_x.clone())?)?, &py::sub(&py::pow(&v_c.clone(), &v_t.clone())?, &Py::Float(1.0))?)?, &py::math1("log", &v_c.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need(mut v_ok: Py, mut v_msg: Py) -> R {
    if (py::not(&v_ok.clone())).truthy() {
        return Err(py::value_error(py::to_str(&v_msg.clone())));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonneg(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t4810 in py::iter(&(v_vals.clone()))? {
        v_v = t4810;
        let _ = f__need({ let t4815 = f__isnum(v_v.clone())?; if !t4815.truthy() { t4815 } else { { let t4813 = v_v.clone(); Py::Bool({ let t4814 = Py::Int(0i64); if py::ge(&t4813, &t4814)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__norm_cdf(mut v_x: Py) -> R {
    return Ok(py::mul(&Py::Float(0.5), &py::math1("erfc", &py::truediv(&py::neg(&v_x.clone())?, &py::math1("sqrt", &Py::Float(2.0))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__norm_pdf(mut v_x: Py) -> R {
    return Ok(py::truediv(&py::math1("exp", &py::mul(&py::mul(&py::neg(&Py::Float(0.5))?, &v_x.clone())?, &v_x.clone())?)?, &py::math1("sqrt", &py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__norm_ppf(mut v_p: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_dd: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__need({ let t5434 = f__isnum(v_p.clone())?; if !t5434.truthy() { t5434 } else { { let t5431 = Py::Int(0i64); Py::Bool({ let t5432 = v_p.clone(); if py::lt(&t5431, &t5432)? { { let t5433 = Py::Int(1i64); if py::lt(&t5432, &t5433)? { true } else { false } } } else { false } }) } } }, Py::str("a probability must lie strictly between 0 and 1"))?;
    v_a = Py::tuple(vec![py::neg(&Py::Float(39.69683028665376))?, Py::Float(220.9460984245205), py::neg(&Py::Float(275.9285104469687))?, Py::Float(138.357751867269), py::neg(&Py::Float(30.66479806614716))?, Py::Float(2.506628277459239)]);
    v_b = Py::tuple(vec![py::neg(&Py::Float(54.47609879822406))?, Py::Float(161.5858368580409), py::neg(&Py::Float(155.6989798598866))?, Py::Float(66.80131188771972), py::neg(&Py::Float(13.28068155288572))?]);
    v_c = Py::tuple(vec![py::neg(&Py::Float(0.007784894002430293))?, py::neg(&Py::Float(0.3223964580411365))?, py::neg(&Py::Float(2.400758277161838))?, py::neg(&Py::Float(2.549732539343734))?, Py::Float(4.374664141464968), Py::Float(2.938163982698783)]);
    v_dd = Py::tuple(vec![Py::Float(0.007784695709041462), Py::Float(0.3224671290700398), Py::Float(2.445134137142996), Py::Float(3.754408661907416)]);
    if ({ let t5435 = v_p.clone(); Py::Bool({ let t5436 = Py::Float(0.02425); if py::lt(&t5435, &t5436)? { true } else { false } }) }).truthy() {
        v_s = py::math1("sqrt", &py::mul(&py::neg(&Py::Float(2.0))?, &py::math1("log", &v_p.clone())?)?)?;
        v_x = py::truediv(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &v_s.clone())?, &py::getitem(&v_c.clone(), &Py::Int(1i64))?)?, &v_s.clone())?, &py::getitem(&v_c.clone(), &Py::Int(2i64))?)?, &v_s.clone())?, &py::getitem(&v_c.clone(), &Py::Int(3i64))?)?, &v_s.clone())?, &py::getitem(&v_c.clone(), &Py::Int(4i64))?)?, &v_s.clone())?, &py::getitem(&v_c.clone(), &Py::Int(5i64))?)?, &py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::getitem(&v_dd.clone(), &Py::Int(0i64))?, &v_s.clone())?, &py::getitem(&v_dd.clone(), &Py::Int(1i64))?)?, &v_s.clone())?, &py::getitem(&v_dd.clone(), &Py::Int(2i64))?)?, &v_s.clone())?, &py::getitem(&v_dd.clone(), &Py::Int(3i64))?)?, &v_s.clone())?, &Py::Float(1.0))?)?;
    } else {
        if ({ let t5437 = v_p.clone(); Py::Bool({ let t5438 = py::sub(&Py::Float(1.0), &Py::Float(0.02425))?; if py::gt(&t5437, &t5438)? { true } else { false } }) }).truthy() {
            v_s = py::math1("sqrt", &py::mul(&py::neg(&Py::Float(2.0))?, &py::math1("log", &py::sub(&Py::Float(1.0), &v_p.clone())?)?)?)?;
            v_x = py::truediv(&py::neg(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &v_s.clone())?, &py::getitem(&v_c.clone(), &Py::Int(1i64))?)?, &v_s.clone())?, &py::getitem(&v_c.clone(), &Py::Int(2i64))?)?, &v_s.clone())?, &py::getitem(&v_c.clone(), &Py::Int(3i64))?)?, &v_s.clone())?, &py::getitem(&v_c.clone(), &Py::Int(4i64))?)?, &v_s.clone())?, &py::getitem(&v_c.clone(), &Py::Int(5i64))?)?)?, &py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::getitem(&v_dd.clone(), &Py::Int(0i64))?, &v_s.clone())?, &py::getitem(&v_dd.clone(), &Py::Int(1i64))?)?, &v_s.clone())?, &py::getitem(&v_dd.clone(), &Py::Int(2i64))?)?, &v_s.clone())?, &py::getitem(&v_dd.clone(), &Py::Int(3i64))?)?, &v_s.clone())?, &Py::Float(1.0))?)?;
        } else {
            v_s = py::sub(&v_p.clone(), &Py::Float(0.5))?;
            v_t = py::mul(&v_s.clone(), &v_s.clone())?;
            v_x = py::truediv(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &v_t.clone())?, &py::getitem(&v_a.clone(), &Py::Int(1i64))?)?, &v_t.clone())?, &py::getitem(&v_a.clone(), &Py::Int(2i64))?)?, &v_t.clone())?, &py::getitem(&v_a.clone(), &Py::Int(3i64))?)?, &v_t.clone())?, &py::getitem(&v_a.clone(), &Py::Int(4i64))?)?, &v_t.clone())?, &py::getitem(&v_a.clone(), &Py::Int(5i64))?)?, &v_s.clone())?, &py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::add(&py::mul(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &v_t.clone())?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?, &v_t.clone())?, &py::getitem(&v_b.clone(), &Py::Int(2i64))?)?, &v_t.clone())?, &py::getitem(&v_b.clone(), &Py::Int(3i64))?)?, &v_t.clone())?, &py::getitem(&v_b.clone(), &Py::Int(4i64))?)?, &v_t.clone())?, &Py::Float(1.0))?)?;
        }
    }
    for t5439 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
        v__ = t5439;
        if ({ let t5440 = v_p.clone(); Py::Bool({ let t5441 = Py::Float(0.5); if py::lt(&t5440, &t5441)? { true } else { false } }) }).truthy() {
            v_e = py::sub(&py::mul(&Py::Float(0.5), &py::math1("erfc", &py::truediv(&py::neg(&v_x.clone())?, &py::math1("sqrt", &Py::Float(2.0))?)?)?)?, &v_p.clone())?;
        } else {
            v_e = py::sub(&py::sub(&Py::Float(1.0), &v_p.clone())?, &py::mul(&Py::Float(0.5), &py::math1("erfc", &py::truediv(&v_x.clone(), &py::math1("sqrt", &Py::Float(2.0))?)?)?)?)?;
        }
        v_u = py::mul(&py::mul(&v_e.clone(), &py::math1("sqrt", &py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?)?)?, &py::math1("exp", &py::truediv(&py::mul(&v_x.clone(), &v_x.clone())?, &Py::Float(2.0))?)?)?;
        v_x = py::sub(&v_x.clone(), &py::truediv(&v_u.clone(), &py::add(&Py::Float(1.0), &py::truediv(&py::mul(&v_x.clone(), &v_u.clone())?, &Py::Float(2.0))?)?)?)?;
    }
    return Ok(v_x.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out(mut v_kw: Py) -> R {
    return Ok({ let mut out = vec![]; for t5686 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5687 = (t5686).items()?;
if t5687.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5687[0].clone();
v_v = t5687[1].clone();
out.push((v_k.clone(), f__r(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__p_check(mut v_p: Py) -> R {
    let _ = f__need({ let t5493 = f__isnum(v_p.clone())?; if !t5493.truthy() { t5493 } else { { let t5490 = Py::Int(0i64); Py::Bool({ let t5491 = v_p.clone(); if py::lt(&t5490, &t5491)? { { let t5492 = Py::Int(1i64); if py::lt(&t5491, &t5492)? { true } else { false } } } else { false } }) } } }, Py::str("the security level p must lie strictly between 0 and 1"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__panjer(mut v_a: Py, mut v_b: Py, mut v_g0: Py, mut v_f: Py, mut v_smax: Py) -> R {
    let mut v_acc: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_g = Py::list(vec![v_g0.clone()]);
    v_c = py::sub(&Py::Float(1.0), &py::mul(&v_a.clone(), &py::getitem(&v_f.clone(), &Py::Int(0i64))?)?)?;
    for t4947 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_smax.clone(), &Py::Int(1i64))?).i()?]) {
        v_s = t4947;
        v_acc = Py::Float(0.0);
        for t4948 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::minmax(&[v_s.clone(), py::sub(&py::len(&v_f.clone())?, &Py::Int(1i64))?], false)?, &Py::Int(1i64))?).i()?]) {
            v_j = t4948;
            v_acc = py::iadd(&v_acc, &py::mul(&py::mul(&py::add(&v_a.clone(), &py::truediv(&py::mul(&v_b.clone(), &v_j.clone())?, &v_s.clone())?)?, &py::getitem(&v_f.clone(), &v_j.clone())?)?, &py::getitem(&v_g.clone(), &py::sub(&v_s.clone(), &v_j.clone())?)?)?)?;
        }
        let _ = py::method(&v_g.clone(), "append", &[py::truediv(&v_acc.clone(), &v_c.clone())?])?;
    }
    return Ok(v_g.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pe(mut v_q: Py, mut v_v: Py, mut v_n: Py) -> R {
    return Ok(py::mul(&py::pow(&v_v.clone(), &v_n.clone())?, &py::getitem(&f__kp(v_q.clone())?, &v_n.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t5623 in py::iter(&(v_vals.clone()))? {
        v_v = t5623;
        let _ = f__need({ let t5626 = f__isnum(v_v.clone())?; if !t5626.truthy() { t5626 } else { { let t5624 = v_v.clone(); Py::Bool({ let t5625 = Py::Int(0i64); if py::gt(&t5624, &t5625)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive number".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__prem_check(mut v_P: Py, mut v_N: Py) -> R {
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t444 = py::len(&v_P.clone())?; Py::Bool({ let t445 = v_N.clone(); if py::eq(&t444, &t445) { true } else { false } }) }, Py::str("give one premium (exposure) for every origin year"))?;
    for t446 in py::iter(&(v_P.clone()))? {
        v_v = t446;
        let _ = f__need({ let t449 = f__isnum(v_v.clone())?; if !t449.truthy() { t449 } else { { let t447 = v_v.clone(); Py::Bool({ let t448 = Py::Int(0i64); if py::gt(&t447, &t448)? { true } else { false } }) } } }, Py::str("every premium must be positive"))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__q_check(mut v_q: Py, mut v_whole: Py, mut v_n: Py) -> R {
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t5600 = py::len(&v_q.clone())?; Py::Bool({ let t5601 = Py::Int(1i64); if py::ge(&t5600, &t5601)? { true } else { false } }) }, Py::str("give at least one death probability"))?;
    for t5602 in py::iter(&(v_q.clone()))? {
        v_v = t5602;
        let _ = f__need({ let t5608 = f__isnum(v_v.clone())?; if !t5608.truthy() { t5608 } else { { let t5605 = Py::Int(0i64); Py::Bool({ let t5606 = v_v.clone(); if py::le(&t5605, &t5606)? { { let t5607 = Py::Int(1i64); if py::le(&t5606, &t5607)? { true } else { false } } } else { false } }) } } }, Py::str("every death probability q must lie between 0 and 1"))?;
    }
    if (v_whole.clone()).truthy() {
        let _ = f__need({ let t5609 = py::getitem(&v_q.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t5610 = Py::Int(1i64); if py::eq(&t5609, &t5610) { true } else { false } }) }, Py::str("a whole-life table must close: its last q must be 1"))?;
    }
    if ({ let t5611 = v_n.clone(); Py::Bool({ let t5612 = Py::None; if !py::is(&t5611, &t5612) { true } else { false } }) }).truthy() {
        let _ = f__whole(Py::str("the term n"), v_n.clone(), Py::Int(1i64))?;
        let _ = f__need({ let t5617 = v_n.clone(); Py::Bool({ let t5618 = py::len(&v_q.clone())?; if py::le(&t5617, &t5618)? { true } else { false } }) }, Py::str("the table must cover the whole term (at least n death probabilities)"))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if ({ let t5691 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t5691.truthy() { t5691 } else { { let t5690 = Py::Bool(matches!(v_x.clone(), Py::Str(_))); if t5690.truthy() { t5690 } else { { let t5688 = v_x.clone(); Py::Bool({ let t5689 = Py::None; if py::is(&t5688, &t5689) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t5692 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t5692;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    if ({ let t5695 = { let t5693 = v_x.clone(); Py::Bool({ let t5694 = Py::Int(0i64); if py::eq(&t5693, &t5694) { true } else { false } }) }; if t5695.truthy() { t5695 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t5696 = v_x.clone(); Py::Bool({ let t5697 = Py::Int(0i64); if py::eq(&t5696, &t5697) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rate(mut v_i: Py) -> R {
    let _ = f__need({ let t5681 = f__isnum(v_i.clone())?; if !t5681.truthy() { t5681 } else { { let t5678 = Py::Int(0i64); Py::Bool({ let t5679 = v_i.clone(); if py::lt(&t5678, &t5679)? { { let t5680 = Py::Int(1i64); if py::lt(&t5679, &t5680)? { true } else { false } } } else { false } }) } } }, Py::str("the interest rate i must lie between 0 and 100% (write 5% or 0.05)"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    if ({ let t5704 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t5704.truthy() { t5704 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t5708 = py::not(&{ let t5705 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t5705.truthy() { t5705 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t5708.truthy() { t5708 } else { { let t5706 = py::len(&v_a.clone())?; Py::Bool({ let t5707 = py::len(&v_b.clone())?; if py::ne(&t5706, &t5707) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t5709 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t5710 = (t5709).items()?;
if t5710.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t5710[0].clone();
v_y = t5710[1].clone();
out.push(f__rel(v_x.clone(), v_y.clone())?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?], true)?);
    }
    if ({ let t5711 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t5711.truthy() { t5711 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))) } }).truthy() {
        return Ok((if ({ let t5712 = v_a.clone(); Py::Bool({ let t5713 = v_b.clone(); if py::eq(&t5712, &t5713) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    let t5715: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t5714 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t5714.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t5714[0].clone();
            v_b = t5714[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5715 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t5716 = v_d.clone(); Py::Bool({ let t5717 = Py::Int(0i64); if py::eq(&t5716, &t5717) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5722 = { let t5718 = py::abs(&v_a.clone())?; Py::Bool({ let t5719 = Py::Float(1e-15); if py::lt(&t5718, &t5719)? { true } else { false } }) }; if !t5722.truthy() { t5722 } else { { let t5720 = py::abs(&v_b.clone())?; Py::Bool({ let t5721 = Py::Float(1e-15); if py::lt(&t5720, &t5721)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(0.0));
    }
    return Ok(py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__risk_table(mut v_times: Py, mut v_events: Py, mut v_t: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_tj: Py = Py::None;
    let mut v_ts: Py = Py::None;
    v_ts = py::sorted(&py::set(&{ let mut out = vec![]; for t3245 in py::range_iter(&[(py::len(&v_times.clone())?).i()?]) { let mut v_k: Py; 
v_k = t3245;
if ({ let t3250 = { let t3246 = py::getitem(&v_events.clone(), &v_k.clone())?; Py::Bool({ let t3247 = Py::Int(1i64); if py::eq(&t3246, &t3247) { true } else { false } }) }; if !t3250.truthy() { t3250 } else { { let t3248 = py::getitem(&v_times.clone(), &v_k.clone())?; Py::Bool({ let t3249 = v_t.clone(); if py::le(&t3248, &t3249)? { true } else { false } }) } } }).truthy() { out.push(py::getitem(&v_times.clone(), &v_k.clone())?); }} Py::list(out) })?, (Py::Bool(false)).truthy())?;
    v_out = Py::list(vec![]);
    for t3251 in py::iter(&(v_ts.clone()))? {
        v_tj = t3251;
        v_d = Py::Int(0i64);
        v_n = Py::Int(0i64);
        for t3252 in py::range_iter(&[(py::len(&v_times.clone())?).i()?]) {
            v_k = t3252;
            if ({ let t3253 = py::getitem(&v_times.clone(), &v_k.clone())?; Py::Bool({ let t3254 = v_tj.clone(); if py::ge(&t3253, &t3254)? { true } else { false } }) }).truthy() {
                v_n = py::iadd(&v_n, &Py::Int(1i64))?;
            }
            if ({ let t3259 = { let t3255 = py::getitem(&v_times.clone(), &v_k.clone())?; Py::Bool({ let t3256 = v_tj.clone(); if py::eq(&t3255, &t3256) { true } else { false } }) }; if !t3259.truthy() { t3259 } else { { let t3257 = py::getitem(&v_events.clone(), &v_k.clone())?; Py::Bool({ let t3258 = Py::Int(1i64); if py::eq(&t3257, &t3258) { true } else { false } }) } } }).truthy() {
                v_d = py::iadd(&v_d, &Py::Int(1i64))?;
            }
        }
        let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![v_tj.clone(), v_d.clone(), v_n.clone()])])?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sev_check(mut v_f: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let _ = f__need({ let t4917 = py::len(&v_f.clone())?; Py::Bool({ let t4918 = Py::Int(2i64); if py::ge(&t4917, &t4918)? { true } else { false } }) }, Py::str("give the severity probabilities for amounts 0, 1, 2, ..."))?;
    v_tot = Py::Float(0.0);
    for t4919 in py::iter(&(v_f.clone()))? {
        v_p = t4919;
        let _ = f__need({ let t4923 = f__isnum(v_p.clone())?; if !t4923.truthy() { t4923 } else { { let t4920 = Py::Int(0i64); Py::Bool({ let t4921 = v_p.clone(); if py::le(&t4920, &t4921)? { { let t4922 = Py::Int(1i64); if py::le(&t4921, &t4922)? { true } else { false } } } else { false } }) } } }, Py::str("every severity probability must lie between 0 and 1"))?;
        v_tot = py::iadd(&v_tot, &v_p.clone())?;
    }
    let _ = f__need({ let t4924 = py::abs(&py::sub(&v_tot.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t4925 = Py::Float(1e-09); if py::le(&t4924, &t4925)? { true } else { false } }) }, Py::str("the severity probabilities must add up to 1"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sf(mut v_dist: Py, mut v_p1: Py, mut v_p2: Py, mut v_x: Py) -> R {
    if ({ let t3917 = v_x.clone(); Py::Bool({ let t3918 = Py::Int(0i64); if py::le(&t3917, &t3918)? { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    if ({ let t3919 = v_dist.clone(); Py::Bool({ let t3920 = Py::str("exponential"); if py::eq(&t3919, &t3920) { true } else { false } }) }).truthy() {
        return Ok(py::math1("exp", &py::truediv(&py::neg(&v_x.clone())?, &v_p1.clone())?)?);
    }
    if ({ let t3921 = v_dist.clone(); Py::Bool({ let t3922 = Py::str("pareto"); if py::eq(&t3921, &t3922) { true } else { false } }) }).truthy() {
        return Ok(py::pow(&py::truediv(&v_p2.clone(), &py::add(&v_p2.clone(), &v_x.clone())?)?, &v_p1.clone())?);
    }
    return Ok(py::mul(&Py::Float(0.5), &py::math1("erfc", &py::truediv(&py::truediv(&py::sub(&py::math1("log", &v_x.clone())?, &v_p1.clone())?, &v_p2.clone())?, &py::math1("sqrt", &Py::Float(2.0))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__surv_check(mut v_times: Py, mut v_events: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__need({ let t3229 = { let t3225 = py::len(&v_times.clone())?; Py::Bool({ let t3226 = py::len(&v_events.clone())?; if py::eq(&t3225, &t3226) { true } else { false } }) }; if !t3229.truthy() { t3229 } else { { let t3227 = py::len(&v_times.clone())?; Py::Bool({ let t3228 = Py::Int(1i64); if py::ge(&t3227, &t3228)? { true } else { false } }) } } }, Py::str("give one event indicator for every time"))?;
    for t3230 in py::iter(&(v_times.clone()))? {
        v_t = t3230;
        let _ = f__need({ let t3235 = f__isnum(v_t.clone())?; if !t3235.truthy() { t3235 } else { { let t3233 = v_t.clone(); Py::Bool({ let t3234 = Py::Int(0i64); if py::gt(&t3233, &t3234)? { true } else { false } }) } } }, Py::str("every time must be positive"))?;
    }
    for t3236 in py::iter(&(v_events.clone()))? {
        v_e = t3236;
        let _ = f__need({ let t3241 = { let t3237 = v_e.clone(); Py::Bool({ let t3238 = Py::Int(0i64); if py::eq(&t3237, &t3238) { true } else { false } }) }; if t3241.truthy() { t3241 } else { { let t3239 = v_e.clone(); Py::Bool({ let t3240 = Py::Int(1i64); if py::eq(&t3239, &t3240) { true } else { false } }) } } }, Py::str("every event indicator must be 1 (death or failure) or 0 (censored)"))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tri(mut v_T: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_N = py::len(&v_T.clone())?;
    let _ = f__need({ let t2835 = { let t2831 = v_N.clone(); Py::Bool({ let t2832 = Py::Int(2i64); if py::ge(&t2831, &t2832)? { true } else { false } }) }; if !t2835.truthy() { t2835 } else { { let t2833 = v_N.clone(); Py::Bool({ let t2834 = Py::Int(30i64); if py::le(&t2833, &t2834)? { true } else { false } }) } } }, Py::str("the triangle needs between 2 and 30 origin years"))?;
    for t2836 in py::range_iter(&[(v_N.clone()).i()?]) {
        v_i = t2836;
        let _ = f__need({ let t2837 = py::len(&py::getitem(&v_T.clone(), &v_i.clone())?)?; Py::Bool({ let t2838 = v_N.clone(); if py::eq(&t2837, &t2838) { true } else { false } }) }, Py::str("the triangle must be square (unknown cells written as 0)"))?;
        for t2839 in py::range_iter(&[(v_N.clone()).i()?]) {
            v_j = t2839;
            v_v = py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_j.clone())?;
            let _ = f__need(f__isnum(v_v.clone())?, Py::str("every cell must be a number"))?;
            if ({ let t2842 = v_j.clone(); Py::Bool({ let t2843 = py::sub(&v_N.clone(), &v_i.clone())?; if py::lt(&t2842, &t2843)? { true } else { false } }) }).truthy() {
                let _ = f__need({ let t2844 = v_v.clone(); Py::Bool({ let t2845 = Py::Int(0i64); if py::gt(&t2844, &t2845)? { true } else { false } }) }, Py::str("every known cumulative amount must be positive"))?;
            } else {
                let _ = f__need({ let t2846 = v_v.clone(); Py::Bool({ let t2847 = Py::Int(0i64); if py::eq(&t2846, &t2847) { true } else { false } }) }, Py::str("the cells below the latest diagonal must be written as 0"))?;
            }
        }
    }
    return Ok(v_N.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_annuity_yield(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_PV: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_val: Py = Py::None;
    let t175: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_j = py::getitem(&v_outputs.clone(), &Py::str("i"))?;
            let t155 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("n"))?, py::getitem(&v_inputs.clone(), &Py::str("R"))?, py::getitem(&v_inputs.clone(), &Py::str("PV"))?])).items()?;
            if t155.len() != 3 { return Err(PyErr::Value("unpack".into())); }
            v_n = t155[0].clone();
            v_R = t155[1].clone();
            v_PV = t155[2].clone();
            v_val = py::truediv(&py::mul(&v_R.clone(), &py::sub(&Py::Float(1.0), &py::pow(&py::add(&Py::Float(1.0), &v_j.clone())?, &py::neg(&v_n.clone())?)?)?)?, &v_j.clone())?;
            return Ok(Some(py::minmax(&[py::truediv(&py::abs(&py::sub(&v_val.clone(), &v_PV.clone())?)?, &v_PV.clone())?, f__rel(py::getitem(&v_outputs.clone(), &Py::str("a_n"))?, py::truediv(&v_PV.clone(), &v_R.clone())?)?], true)?));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t175 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__whole(mut v_name: Py, mut v_n: Py, mut v_lo: Py) -> R {
    let _ = f__need({ let t5675 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t5675.truthy() { t5675 } else { { let t5674 = py::not(&Py::Bool(matches!(v_n.clone(), Py::Bool(_)))); if !t5674.truthy() { t5674 } else { { let t5672 = v_n.clone(); Py::Bool({ let t5673 = v_lo.clone(); if py::ge(&t5672, &t5673)? { true } else { false } }) } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number of at least ".to_string(), py::format(&v_lo.clone(), "")?].concat()))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_accumulated_value(mut v_P: Py, mut v_i: Py, mut v_t: Py) -> R {
    let mut v_f: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_P.clone()]))?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__need({ let t11 = f__isnum(v_t.clone())?; if !t11.truthy() { t11 } else { { let t8 = Py::Int(0i64); Py::Bool({ let t9 = v_t.clone(); if py::le(&t8, &t9)? { { let t10 = Py::Int(200i64); if py::le(&t9, &t10)? { true } else { false } } } else { false } }) } } }, Py::str("the time t must lie between 0 and 200 years"))?;
    v_f = py::pow(&py::add(&Py::Float(1.0), &v_i.clone())?, &v_t.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("AV"), py::mul(&v_P.clone(), &v_f.clone())?), (Py::str("PV"), py::truediv(&v_P.clone(), &v_f.clone())?), (Py::str("factor"), v_f.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_annuity_certain(mut v_n: Py, mut v_i: Py) -> R {
    let mut v_adn: Py = Py::None;
    let mut v_an: Py = Py::None;
    let mut v_delta: Py = Py::None;
    let mut v_sdn: Py = Py::None;
    let mut v_sn: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__whole(Py::str("the number of payments n"), v_n.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t54 = v_n.clone(); Py::Bool({ let t55 = Py::Int(1000i64); if py::le(&t54, &t55)? { true } else { false } }) }, Py::str("n must be at most 1000"))?;
    let _ = f__rate(v_i.clone())?;
    let t62 = (f__annuity_values(v_n.clone(), v_i.clone())?).items()?;
    if t62.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_v = t62[0].clone();
    v_an = t62[1].clone();
    v_adn = t62[2].clone();
    v_sn = t62[3].clone();
    v_sdn = t62[4].clone();
    v_delta = py::math1("log1p", &v_i.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("a_n"), v_an.clone()), (Py::str("a_due"), v_adn.clone()), (Py::str("s_n"), v_sn.clone()), (Py::str("s_due"), v_sdn.clone()), (Py::str("a_cont"), py::truediv(&py::sub(&Py::Float(1.0), &py::pow(&v_v.clone(), &v_n.clone())?)?, &v_delta.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_annuity_yield(mut v_PV: Py, mut v_R: Py, mut v_n: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_j: Py = Py::None;
    let _ = f__pos(Py::str("the present value and the payment"), Py::tuple(vec![v_PV.clone(), v_R.clone()]))?;
    let _ = f__whole(Py::str("the number of payments n"), v_n.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t111 = v_n.clone(); Py::Bool({ let t112 = Py::Int(1000i64); if py::le(&t111, &t112)? { true } else { false } }) }, Py::str("n must be at most 1000"))?;
    let _ = f__need({ let t113 = v_PV.clone(); Py::Bool({ let t114 = py::mul(&v_n.clone(), &v_R.clone())?; if py::lt(&t113, &t114)? { true } else { false } }) }, Py::str("the present value must be below the total of the payments (a positive yield)"))?;
    let _ = f__need({ let t115 = v_PV.clone(); Py::Bool({ let t116 = py::truediv(&py::mul(&v_R.clone(), &py::sub(&Py::Float(1.0), &py::pow(&Py::Float(1.99), &py::neg(&v_n.clone())?)?)?)?, &Py::Float(0.99))?; if py::gt(&t115, &t116)? { true } else { false } }) }, Py::str("the yield would exceed 99%"))?;
    v_f = { let v_PV = v_PV.clone(); let v_R = v_R.clone(); let v_f = v_f.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::truediv(&py::mul(&v_R.clone(), &py::sub(&Py::Float(1.0), &py::pow(&py::add(&Py::Float(1.0), &v_j.clone())?, &py::neg(&v_n.clone())?)?)?)?, &v_j.clone())?, &v_PV.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_j = f__bisect(v_f.clone(), Py::Float(1e-12), Py::Float(0.99), Py::Int(500i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("i"), v_j.clone()), (Py::str("a_n"), py::truediv(&v_PV.clone(), &v_R.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bornhuetter_ferguson(mut v_T: Py, mut v_P: Py, mut v_elr: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_cdf: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_latest: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_N = f__tri(v_T.clone())?;
    let _ = f__prem_check(v_P.clone(), v_N.clone())?;
    let _ = f__need({ let t202 = f__isnum(v_elr.clone())?; if !t202.truthy() { t202 } else { { let t199 = Py::Int(0i64); Py::Bool({ let t200 = v_elr.clone(); if py::lt(&t199, &t200)? { { let t201 = Py::Int(5i64); if py::le(&t200, &t201)? { true } else { false } } } else { false } }) } } }, Py::str("the expected loss ratio must lie between 0 and 500%"))?;
    let t208 = (f__cl_factors(v_T.clone(), Py::Bool(false))?).items()?;
    if t208.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f = t208[0].clone();
    v_cdf = t208[1].clone();
    v_latest = { let mut out = vec![]; for t209 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_i: Py; 
v_i = t209;
out.push(py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &py::sub(&py::sub(&v_N.clone(), &Py::Int(1i64))?, &v_i.clone())?)?); } Py::list(out) };
    v_res = { let mut out = vec![]; for t210 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_i: Py; 
v_i = t210;
out.push(py::mul(&py::mul(&py::getitem(&v_P.clone(), &v_i.clone())?, &v_elr.clone())?, &py::sub(&Py::Float(1.0), &py::truediv(&Py::Float(1.0), &py::getitem(&v_cdf.clone(), &py::sub(&py::sub(&v_N.clone(), &Py::Int(1i64))?, &v_i.clone())?)?)?)?)?); } Py::list(out) };
    v_tot = Py::Float(0.0);
    for t211 in py::iter(&(v_res.clone()))? {
        v_v = t211;
        v_tot = py::iadd(&v_tot, &v_v.clone())?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("ultimate"), { let mut out = vec![]; for t224 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_i: Py; 
v_i = t224;
out.push(py::add(&py::getitem(&v_latest.clone(), &v_i.clone())?, &py::getitem(&v_res.clone(), &v_i.clone())?)?); } Py::list(out) }), (Py::str("reserve"), v_res.clone()), (Py::str("total_reserve"), v_tot.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_buhlmann_empirical(mut v_X: Py) -> R {
    let mut v_Z: Py = Py::None;
    let mut v_epv: Py = Py::None;
    let mut v_grand: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m_: Py = Py::None;
    let mut v_means: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vb: Py = Py::None;
    let mut v_vhm: Py = Py::None;
    let _ = f__mat_check(v_X.clone(), Py::str("the data"))?;
    let t308 = (Py::tuple(vec![py::len(&v_X.clone())?, py::len(&py::getitem(&v_X.clone(), &Py::Int(0i64))?)?])).items()?;
    if t308.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t308[0].clone();
    v_n = t308[1].clone();
    v_means = Py::list(vec![]);
    for t309 in py::iter(&(v_X.clone()))? {
        v_row = t309;
        v_s = Py::Float(0.0);
        for t310 in py::iter(&(v_row.clone()))? {
            v_v = t310;
            v_s = py::iadd(&v_s, &v_v.clone())?;
        }
        let _ = py::method(&v_means.clone(), "append", &[py::truediv(&v_s.clone(), &v_n.clone())?])?;
    }
    v_grand = Py::Float(0.0);
    for t311 in py::iter(&(v_means.clone()))? {
        v_m_ = t311;
        v_grand = py::iadd(&v_grand, &v_m_.clone())?;
    }
    v_grand = py::truediv(&v_grand, &v_r.clone())?;
    v_epv = Py::Float(0.0);
    for t312 in py::range_iter(&[(v_r.clone()).i()?]) {
        v_i = t312;
        for t313 in py::iter(&(py::getitem(&v_X.clone(), &v_i.clone())?))? {
            v_v = t313;
            v_epv = py::iadd(&v_epv, &py::pow(&py::sub(&v_v.clone(), &py::getitem(&v_means.clone(), &v_i.clone())?)?, &Py::Int(2i64))?)?;
        }
    }
    v_epv = py::truediv(&v_epv, &py::mul(&v_r.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?;
    v_vb = Py::Float(0.0);
    for t314 in py::iter(&(v_means.clone()))? {
        v_m_ = t314;
        v_vb = py::iadd(&v_vb, &py::pow(&py::sub(&v_m_.clone(), &v_grand.clone())?, &Py::Int(2i64))?)?;
    }
    v_vhm = py::sub(&py::truediv(&v_vb.clone(), &py::sub(&v_r.clone(), &Py::Int(1i64))?)?, &py::truediv(&v_epv.clone(), &v_n.clone())?)?;
    let _ = f__need({ let t315 = v_vhm.clone(); Py::Bool({ let t316 = Py::Int(0i64); if py::gt(&t315, &t316)? { true } else { false } }) }, Py::str("the estimated variance of the hypothetical means is not positive: no credibility (Z = 0)"))?;
    let _ = f__need({ let t317 = v_epv.clone(); Py::Bool({ let t318 = Py::Int(0i64); if py::gt(&t317, &t318)? { true } else { false } }) }, Py::str("the expected process variance estimate is zero"))?;
    v_k = py::truediv(&v_epv.clone(), &v_vhm.clone())?;
    v_Z = py::truediv(&v_n.clone(), &py::add(&v_n.clone(), &v_k.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("EPV"), v_epv.clone()), (Py::str("VHM"), v_vhm.clone()), (Py::str("k"), v_k.clone()), (Py::str("Z"), v_Z.clone()), (Py::str("premiums"), { let mut out = vec![]; for t331 in py::iter(&(v_means.clone()))? { let mut v_m_: Py; 
v_m_ = t331;
out.push(py::add(&py::mul(&v_Z.clone(), &v_m_.clone())?, &py::mul(&py::sub(&Py::Float(1.0), &v_Z.clone())?, &v_grand.clone())?)?); } Py::list(out) }), (Py::str("mean"), v_grand.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_buhlmann_premium(mut v_mu: Py, mut v_EPV: Py, mut v_VHM: Py, mut v_n: Py, mut v_xbar: Py) -> R {
    let mut v_Z: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_mu.clone(), v_xbar.clone()]))?;
    let _ = f__pos(Py::str("the expected process variance and the variance of hypothetical means"), Py::tuple(vec![v_EPV.clone(), v_VHM.clone()]))?;
    let _ = f__need({ let t260 = f__isnum(v_n.clone())?; if !t260.truthy() { t260 } else { { let t258 = v_n.clone(); Py::Bool({ let t259 = Py::Int(0i64); if py::gt(&t258, &t259)? { true } else { false } }) } } }, Py::str("the number of observations n must be positive"))?;
    v_k = py::truediv(&v_EPV.clone(), &v_VHM.clone())?;
    v_Z = py::truediv(&v_n.clone(), &py::add(&v_n.clone(), &v_k.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("k"), v_k.clone()), (Py::str("Z"), v_Z.clone()), (Py::str("premium"), py::add(&py::mul(&v_Z.clone(), &v_xbar.clone())?, &py::mul(&py::sub(&Py::Float(1.0), &v_Z.clone())?, &v_mu.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_buhlmann_straub(mut v_X: Py, mut v_m: Py) -> R {
    let mut v_Z: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_mi: Py = Py::None;
    let mut v_mt: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sq: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_xbar: Py = Py::None;
    let mut v_xi: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let _ = f__mat_check(v_X.clone(), Py::str("the losses per exposure"))?;
    let _ = f__mat_check(v_m.clone(), Py::str("the exposures"))?;
    let _ = f__need({ let t371 = { let t367 = py::len(&v_m.clone())?; Py::Bool({ let t368 = py::len(&v_X.clone())?; if py::eq(&t367, &t368) { true } else { false } }) }; if !t371.truthy() { t371 } else { { let t369 = py::len(&py::getitem(&v_m.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t370 = py::len(&py::getitem(&v_X.clone(), &Py::Int(0i64))?)?; if py::eq(&t369, &t370) { true } else { false } }) } } }, Py::str("the exposures must have the same shape as the losses"))?;
    let t372 = (Py::tuple(vec![py::len(&v_X.clone())?, py::len(&py::getitem(&v_X.clone(), &Py::Int(0i64))?)?])).items()?;
    if t372.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t372[0].clone();
    v_n = t372[1].clone();
    let t373 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t373.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mi = t373[0].clone();
    v_xi = t373[1].clone();
    for t374 in py::range_iter(&[(v_r.clone()).i()?]) {
        v_i = t374;
        let t375 = Py::Float(0.0);
        v_s = t375.clone();
        v_w = t375.clone();
        for t376 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_j = t376;
            let _ = f__need({ let t377 = py::getitem(&py::getitem(&v_m.clone(), &v_i.clone())?, &v_j.clone())?; Py::Bool({ let t378 = Py::Int(0i64); if py::gt(&t377, &t378)? { true } else { false } }) }, Py::str("every exposure must be positive"))?;
            v_s = py::iadd(&v_s, &py::mul(&py::getitem(&py::getitem(&v_m.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_j.clone())?)?)?;
            v_w = py::iadd(&v_w, &py::getitem(&py::getitem(&v_m.clone(), &v_i.clone())?, &v_j.clone())?)?;
        }
        let _ = py::method(&v_mi.clone(), "append", &[v_w.clone()])?;
        let _ = py::method(&v_xi.clone(), "append", &[py::truediv(&v_s.clone(), &v_w.clone())?])?;
    }
    v_mt = Py::Float(0.0);
    v_xs = Py::Float(0.0);
    for t379 in py::range_iter(&[(v_r.clone()).i()?]) {
        v_i = t379;
        v_mt = py::iadd(&v_mt, &py::getitem(&v_mi.clone(), &v_i.clone())?)?;
        v_xs = py::iadd(&v_xs, &py::mul(&py::getitem(&v_mi.clone(), &v_i.clone())?, &py::getitem(&v_xi.clone(), &v_i.clone())?)?)?;
    }
    v_xbar = py::truediv(&v_xs.clone(), &v_mt.clone())?;
    v_v = Py::Float(0.0);
    for t380 in py::range_iter(&[(v_r.clone()).i()?]) {
        v_i = t380;
        for t381 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_j = t381;
            v_v = py::iadd(&v_v, &py::mul(&py::getitem(&py::getitem(&v_m.clone(), &v_i.clone())?, &v_j.clone())?, &py::pow(&py::sub(&py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_xi.clone(), &v_i.clone())?)?, &Py::Int(2i64))?)?)?;
        }
    }
    v_v = py::truediv(&v_v, &py::mul(&v_r.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?;
    v_num = Py::Float(0.0);
    v_sq = Py::Float(0.0);
    for t382 in py::range_iter(&[(v_r.clone()).i()?]) {
        v_i = t382;
        v_num = py::iadd(&v_num, &py::mul(&py::getitem(&v_mi.clone(), &v_i.clone())?, &py::pow(&py::sub(&py::getitem(&v_xi.clone(), &v_i.clone())?, &v_xbar.clone())?, &Py::Int(2i64))?)?)?;
        v_sq = py::iadd(&v_sq, &py::mul(&py::getitem(&v_mi.clone(), &v_i.clone())?, &py::getitem(&v_mi.clone(), &v_i.clone())?)?)?;
    }
    v_a = py::truediv(&py::sub(&v_num.clone(), &py::mul(&v_v.clone(), &py::sub(&v_r.clone(), &Py::Int(1i64))?)?)?, &py::sub(&v_mt.clone(), &py::truediv(&v_sq.clone(), &v_mt.clone())?)?)?;
    let _ = f__need({ let t383 = v_a.clone(); Py::Bool({ let t384 = Py::Int(0i64); if py::gt(&t383, &t384)? { true } else { false } }) }, Py::str("the estimated variance of the hypothetical means is not positive: no credibility (Z = 0)"))?;
    let _ = f__need({ let t385 = v_v.clone(); Py::Bool({ let t386 = Py::Int(0i64); if py::gt(&t385, &t386)? { true } else { false } }) }, Py::str("the expected process variance estimate is zero"))?;
    v_k = py::truediv(&v_v.clone(), &v_a.clone())?;
    v_Z = { let mut out = vec![]; for t387 in py::iter(&(v_mi.clone()))? { let mut v_w: Py; 
v_w = t387;
out.push(py::truediv(&v_w.clone(), &py::add(&v_w.clone(), &v_k.clone())?)?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("EPV"), v_v.clone()), (Py::str("VHM"), v_a.clone()), (Py::str("k"), v_k.clone()), (Py::str("Z"), v_Z.clone()), (Py::str("premiums"), { let mut out = vec![]; for t400 in py::range_iter(&[(v_r.clone()).i()?]) { let mut v_i: Py; 
v_i = t400;
out.push(py::add(&py::mul(&py::getitem(&v_Z.clone(), &v_i.clone())?, &py::getitem(&v_xi.clone(), &v_i.clone())?)?, &py::mul(&py::sub(&Py::Float(1.0), &py::getitem(&v_Z.clone(), &v_i.clone())?)?, &v_xbar.clone())?)?); } Py::list(out) }), (Py::str("mean"), v_xbar.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cape_cod(mut v_T: Py, mut v_P: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_cdf: Py = Py::None;
    let mut v_elr: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_latest: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_N = f__tri(v_T.clone())?;
    let _ = f__prem_check(v_P.clone(), v_N.clone())?;
    let t455 = (f__cl_factors(v_T.clone(), Py::Bool(false))?).items()?;
    if t455.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f = t455[0].clone();
    v_cdf = t455[1].clone();
    v_latest = { let mut out = vec![]; for t456 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_i: Py; 
v_i = t456;
out.push(py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &py::sub(&py::sub(&v_N.clone(), &Py::Int(1i64))?, &v_i.clone())?)?); } Py::list(out) };
    let t457 = Py::Float(0.0);
    v_a = t457.clone();
    v_b = t457.clone();
    for t458 in py::range_iter(&[(v_N.clone()).i()?]) {
        v_i = t458;
        v_a = py::iadd(&v_a, &py::getitem(&v_latest.clone(), &v_i.clone())?)?;
        v_b = py::iadd(&v_b, &py::truediv(&py::getitem(&v_P.clone(), &v_i.clone())?, &py::getitem(&v_cdf.clone(), &py::sub(&py::sub(&v_N.clone(), &Py::Int(1i64))?, &v_i.clone())?)?)?)?;
    }
    v_elr = py::truediv(&v_a.clone(), &v_b.clone())?;
    v_res = { let mut out = vec![]; for t459 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_i: Py; 
v_i = t459;
out.push(py::mul(&py::mul(&py::getitem(&v_P.clone(), &v_i.clone())?, &v_elr.clone())?, &py::sub(&Py::Float(1.0), &py::truediv(&Py::Float(1.0), &py::getitem(&v_cdf.clone(), &py::sub(&py::sub(&v_N.clone(), &Py::Int(1i64))?, &v_i.clone())?)?)?)?)?); } Py::list(out) };
    v_tot = Py::Float(0.0);
    for t460 in py::iter(&(v_res.clone()))? {
        v_v = t460;
        v_tot = py::iadd(&v_tot, &v_v.clone())?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("elr"), v_elr.clone()), (Py::str("ultimate"), { let mut out = vec![]; for t473 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_i: Py; 
v_i = t473;
out.push(py::add(&py::getitem(&v_latest.clone(), &v_i.clone())?, &py::getitem(&v_res.clone(), &v_i.clone())?)?); } Py::list(out) }), (Py::str("reserve"), v_res.clone()), (Py::str("total_reserve"), v_tot.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_central_death_rate(mut v_m: Py, mut v_assumption: Py) -> R {
    let mut v_q: Py = Py::None;
    let _ = f__need({ let t500 = v_assumption.clone(); Py::Bool({ let t501 = Py::tuple(vec![Py::str("udd"), Py::str("constant_force")]); if py::contains(&t501, &t500)? { true } else { false } }) }, Py::str("assumption: udd or constant_force"))?;
    let _ = f__need({ let t506 = f__isnum(v_m.clone())?; if !t506.truthy() { t506 } else { { let t504 = v_m.clone(); Py::Bool({ let t505 = Py::Int(0i64); if py::gt(&t504, &t505)? { true } else { false } }) } } }, Py::str("the central death rate must be positive"))?;
    if ({ let t507 = v_assumption.clone(); Py::Bool({ let t508 = Py::str("udd"); if py::eq(&t507, &t508) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t509 = v_m.clone(); Py::Bool({ let t510 = Py::Int(2i64); if py::lt(&t509, &t510)? { true } else { false } }) }, Py::str("under UDD the central death rate must lie below 2"))?;
        v_q = py::truediv(&py::mul(&Py::Float(2.0), &v_m.clone())?, &py::add(&Py::Float(2.0), &v_m.clone())?)?;
    } else {
        v_q = py::sub(&Py::Float(1.0), &py::math1("exp", &py::neg(&v_m.clone())?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("q"), v_q.clone()), (Py::str("p"), py::sub(&Py::Float(1.0), &v_q.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_chain_ladder_factors(mut v_T: Py) -> R {
    let mut v_cdf: Py = Py::None;
    let mut v_f: Py = Py::None;
    let _ = f__tri(v_T.clone())?;
    let t571 = (f__cl_factors(v_T.clone(), Py::Bool(false))?).items()?;
    if t571.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f = t571[0].clone();
    v_cdf = t571[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("ldf"), v_f.clone()), (Py::str("cdf"), v_cdf.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_chain_ladder_reserve(mut v_T: Py) -> R {
    let mut v_cdf: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_latest: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_ult: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__tri(v_T.clone())?;
    let t632 = (f__cl_factors(v_T.clone(), Py::Bool(false))?).items()?;
    if t632.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f = t632[0].clone();
    v_cdf = t632[1].clone();
    let t635 = (f__cl_projection(v_T.clone(), v_cdf.clone())?).items()?;
    if t635.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_latest = t635[0].clone();
    v_ult = t635[1].clone();
    v_res = { let mut out = vec![]; for t636 in py::iter(&(py::zip(&[v_ult.clone(), v_latest.clone()])?))? { let mut v_l: Py; let mut v_u: Py; 
let t637 = (t636).items()?;
if t637.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t637[0].clone();
v_l = t637[1].clone();
out.push(py::sub(&v_u.clone(), &v_l.clone())?); } Py::list(out) };
    v_tot = Py::Float(0.0);
    for t638 in py::iter(&(v_res.clone()))? {
        v_v = t638;
        v_tot = py::iadd(&v_tot, &v_v.clone())?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("ultimate"), v_ult.clone()), (Py::str("reserve"), v_res.clone()), (Py::str("total_reserve"), v_tot.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_chain_ladder_simple(mut v_T: Py) -> R {
    let mut v_cdf: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_l: Py = Py::None;
    let mut v_latest: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_ult: Py = Py::None;
    let _ = f__tri(v_T.clone())?;
    let t699 = (f__cl_factors(v_T.clone(), Py::Bool(true))?).items()?;
    if t699.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f = t699[0].clone();
    v_cdf = t699[1].clone();
    let t702 = (f__cl_projection(v_T.clone(), v_cdf.clone())?).items()?;
    if t702.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_latest = t702[0].clone();
    v_ult = t702[1].clone();
    v_tot = Py::Float(0.0);
    for t703 in py::iter(&(py::zip(&[v_ult.clone(), v_latest.clone()])?))? {
        let t704 = (t703).items()?;
        if t704.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t704[0].clone();
        v_l = t704[1].clone();
        v_tot = py::iadd(&v_tot, &py::sub(&v_u.clone(), &v_l.clone())?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("ldf"), v_f.clone()), (Py::str("ultimate"), v_ult.clone()), (Py::str("total_reserve"), v_tot.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_commutation_functions(mut v_lx: Py, mut v_x0: Py, mut v_x: Py, mut v_i: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__lx_check(v_lx.clone(), Py::Bool(true))?;
    let _ = f__rate(v_i.clone())?;
    v_k = f__idx(v_lx.clone(), v_x0.clone(), v_x.clone(), Py::tuple(vec![]))?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_D = { let mut out = vec![]; for t774 in py::range_iter(&[(py::len(&v_lx.clone())?).i()?]) { let mut v_j: Py; 
v_j = t774;
out.push(py::mul(&py::getitem(&v_lx.clone(), &v_j.clone())?, &py::pow(&v_v.clone(), &py::add(&v_x0.clone(), &v_j.clone())?)?)?); } Py::list(out) };
    v_C = { let mut out = vec![]; for t775 in py::range_iter(&[(py::sub(&py::len(&v_lx.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t775;
out.push(py::mul(&py::sub(&py::getitem(&v_lx.clone(), &v_j.clone())?, &py::getitem(&v_lx.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?)?, &py::pow(&v_v.clone(), &py::add(&py::add(&v_x0.clone(), &v_j.clone())?, &Py::Int(1i64))?)?)?); } Py::list(out) };
    v_N = Py::Float(0.0);
    for t776 in py::range_iter(&[(v_k.clone()).i()?, (py::len(&v_lx.clone())?).i()?]) {
        v_j = t776;
        v_N = py::iadd(&v_N, &py::getitem(&v_D.clone(), &v_j.clone())?)?;
    }
    v_M = Py::Float(0.0);
    for t777 in py::range_iter(&[(v_k.clone()).i()?, (py::sub(&py::len(&v_lx.clone())?, &Py::Int(1i64))?).i()?]) {
        v_j = t777;
        v_M = py::iadd(&v_M, &py::getitem(&v_C.clone(), &v_j.clone())?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("D_x"), py::getitem(&v_D.clone(), &v_k.clone())?), (Py::str("N_x"), v_N.clone()), (Py::str("C_x"), (if ({ let t790 = v_k.clone(); Py::Bool({ let t791 = py::len(&v_C.clone())?; if py::lt(&t790, &t791)? { true } else { false } }) }).truthy() { py::getitem(&v_C.clone(), &v_k.clone())? } else { Py::Float(0.0) })), (Py::str("M_x"), v_M.clone()), (Py::str("A_x"), py::truediv(&v_M.clone(), &py::getitem(&v_D.clone(), &v_k.clone())?)?), (Py::str("a_due_x"), py::truediv(&v_N.clone(), &py::getitem(&v_D.clone(), &v_k.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_compound_moments(mut v_EN: Py, mut v_VarN: Py, mut v_EX: Py, mut v_VarX: Py) -> R {
    let _ = f__nonneg(Py::str("the moments"), Py::tuple(vec![v_EN.clone(), v_VarN.clone(), v_VarX.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_EX.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("ES"), py::mul(&v_EN.clone(), &v_EX.clone())?), (Py::str("VarS"), py::add(&py::mul(&v_EN.clone(), &v_VarX.clone())?, &py::mul(&py::mul(&v_VarN.clone(), &v_EX.clone())?, &v_EX.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_compound_poisson_moments(mut v_lam: Py, mut v_m1: Py, mut v_m2: Py, mut v_m3: Py) -> R {
    let mut v_var: Py = Py::None;
    let _ = f__need({ let t867 = f__isnum(v_lam.clone())?; if !t867.truthy() { t867 } else { { let t865 = v_lam.clone(); Py::Bool({ let t866 = Py::Int(0i64); if py::gt(&t865, &t866)? { true } else { false } }) } } }, Py::str("lambda must be positive"))?;
    let _ = f__need({ let t878 = f__isnum(v_m1.clone())?; if !t878.truthy() { t878 } else { { let t877 = f__isnum(v_m2.clone())?; if !t877.truthy() { t877 } else { { let t876 = f__isnum(v_m3.clone())?; if !t876.truthy() { t876 } else { { let t875 = { let t868 = v_m1.clone(); Py::Bool({ let t869 = Py::Int(0i64); if py::gt(&t868, &t869)? { true } else { false } }) }; if !t875.truthy() { t875 } else { { let t874 = { let t870 = v_m2.clone(); Py::Bool({ let t871 = Py::Int(0i64); if py::gt(&t870, &t871)? { true } else { false } }) }; if !t874.truthy() { t874 } else { { let t872 = v_m3.clone(); Py::Bool({ let t873 = Py::Int(0i64); if py::gt(&t872, &t873)? { true } else { false } }) } } } } } } } } } } }, Py::str("the severity moments must be positive"))?;
    let _ = f__need({ let t879 = v_m2.clone(); Py::Bool({ let t880 = py::mul(&py::mul(&v_m1.clone(), &v_m1.clone())?, &py::sub(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::ge(&t879, &t880)? { true } else { false } }) }, Py::str("the second moment cannot be below the squared mean"))?;
    v_var = py::mul(&v_lam.clone(), &v_m2.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("mean"), py::mul(&v_lam.clone(), &v_m1.clone())?), (Py::str("var"), v_var.clone()), (Py::str("skew"), py::truediv(&py::mul(&v_lam.clone(), &v_m3.clone())?, &py::pow(&v_var.clone(), &Py::Float(1.5))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_constant_force(mut v_mu: Py, mut v_t: Py) -> R {
    let mut v_p1: Py = Py::None;
    let _ = f__need({ let t924 = f__isnum(v_mu.clone())?; if !t924.truthy() { t924 } else { { let t921 = Py::Int(0i64); Py::Bool({ let t922 = v_mu.clone(); if py::lt(&t921, &t922)? { { let t923 = Py::Int(5i64); if py::lt(&t922, &t923)? { true } else { false } } } else { false } }) } } }, Py::str("the force of mortality must lie between 0 and 5 per year"))?;
    let _ = f__need({ let t927 = f__isnum(v_t.clone())?; if !t927.truthy() { t927 } else { { let t925 = v_t.clone(); Py::Bool({ let t926 = Py::Int(0i64); if py::ge(&t925, &t926)? { true } else { false } }) } } }, Py::str("t must be zero or positive"))?;
    v_p1 = py::math1("exp", &py::neg(&v_mu.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("tpx"), py::math1("exp", &py::mul(&py::neg(&v_mu.clone())?, &v_t.clone())?)?), (Py::str("tqx"), py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_mu.clone())?, &v_t.clone())?)?)?), (Py::str("e_complete"), py::truediv(&Py::Float(1.0), &v_mu.clone())?), (Py::str("e_curtate"), py::truediv(&v_p1.clone(), &py::sub(&Py::Float(1.0), &v_p1.clone())?)?), (Py::str("var_T"), py::truediv(&Py::Float(1.0), &py::mul(&v_mu.clone(), &v_mu.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_continuous_insurance_udd(mut v_q: Py, mut v_i: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_Ab: Py = Py::None;
    let mut v_delta: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__q_check(v_q.clone(), Py::Bool(true), Py::None)?;
    let _ = f__rate(v_i.clone())?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_delta = py::math1("log1p", &v_i.clone())?;
    v_A = f__ins(v_q.clone(), v_v.clone(), py::len(&v_q.clone())?)?;
    v_Ab = py::mul(&py::truediv(&v_i.clone(), &v_delta.clone())?, &v_A.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("A_bar"), v_Ab.clone()), (Py::str("A"), v_A.clone()), (Py::str("a_bar"), py::truediv(&py::sub(&Py::Float(1.0), &v_Ab.clone())?, &v_delta.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_de_moivre(mut v_omega: Py, mut v_x: Py, mut v_t: Py) -> R {
    let mut v_r: Py = Py::None;
    let _ = f__pos(Py::str("the limiting age omega"), Py::tuple(vec![v_omega.clone()]))?;
    let _ = f__need({ let t1038 = f__isnum(v_x.clone())?; if !t1038.truthy() { t1038 } else { { let t1035 = Py::Int(0i64); Py::Bool({ let t1036 = v_x.clone(); if py::le(&t1035, &t1036)? { { let t1037 = v_omega.clone(); if py::lt(&t1036, &t1037)? { true } else { false } } } else { false } }) } } }, Py::str("the age x must lie between 0 and omega"))?;
    let _ = f__need({ let t1042 = f__isnum(v_t.clone())?; if !t1042.truthy() { t1042 } else { { let t1039 = Py::Int(0i64); Py::Bool({ let t1040 = v_t.clone(); if py::le(&t1039, &t1040)? { { let t1041 = py::sub(&v_omega.clone(), &v_x.clone())?; if py::lt(&t1040, &t1041)? { true } else { false } } } else { false } }) } } }, Py::str("t must lie between 0 and omega - x"))?;
    v_r = py::sub(&v_omega.clone(), &v_x.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("tpx"), py::truediv(&py::sub(&v_r.clone(), &v_t.clone())?, &v_r.clone())?), (Py::str("tqx"), py::truediv(&v_t.clone(), &v_r.clone())?), (Py::str("mu"), py::truediv(&Py::Float(1.0), &py::sub(&v_r.clone(), &v_t.clone())?)?), (Py::str("e_complete"), py::truediv(&v_r.clone(), &Py::Float(2.0))?), (Py::str("var_T"), py::truediv(&py::mul(&v_r.clone(), &v_r.clone())?, &Py::Float(12.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_deferred_annuity(mut v_q: Py, mut v_i: Py, mut v_u: Py, mut v_P: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_kp: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__q_check(v_q.clone(), Py::Bool(true), Py::None)?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__pos(Py::str("the annual payment P"), Py::tuple(vec![v_P.clone()]))?;
    let _ = f__whole(Py::str("the deferment u"), v_u.clone(), Py::Int(0i64))?;
    let _ = f__need({ let t1108 = v_u.clone(); Py::Bool({ let t1109 = py::len(&v_q.clone())?; if py::lt(&t1108, &t1109)? { true } else { false } }) }, Py::str("the deferment must end before the table closes"))?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_kp = f__kp(v_q.clone())?;
    v_s = Py::Float(0.0);
    for t1111 in py::range_iter(&[(v_u.clone()).i()?, (py::len(&v_q.clone())?).i()?]) {
        v_k = t1111;
        v_s = py::iadd(&v_s, &py::mul(&py::pow(&v_v.clone(), &v_k.clone())?, &py::getitem(&v_kp.clone(), &v_k.clone())?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("ua_due"), v_s.clone()), (Py::str("EPV"), py::mul(&v_P.clone(), &v_s.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_deferred_death_probability(mut v_lx: Py, mut v_x0: Py, mut v_x: Py, mut v_u: Py, mut v_t: Py) -> R {
    let mut v_k: Py = Py::None;
    let _ = f__lx_check(v_lx.clone(), Py::Bool(false))?;
    v_k = f__idx(v_lx.clone(), v_x0.clone(), v_x.clone(), Py::tuple(vec![v_u.clone(), v_t.clone()]))?;
    let _ = f__need({ let t1177 = v_t.clone(); Py::Bool({ let t1178 = Py::Int(1i64); if py::ge(&t1177, &t1178)? { true } else { false } }) }, Py::str("the death interval t must be at least one year"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("prob"), py::truediv(&py::sub(&py::getitem(&v_lx.clone(), &py::add(&v_k.clone(), &v_u.clone())?)?, &py::getitem(&v_lx.clone(), &py::add(&py::add(&v_k.clone(), &v_u.clone())?, &v_t.clone())?)?)?, &py::getitem(&v_lx.clone(), &v_k.clone())?)?), (Py::str("upx"), py::truediv(&py::getitem(&v_lx.clone(), &py::add(&v_k.clone(), &v_u.clone())?)?, &py::getitem(&v_lx.clone(), &v_k.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_deferred_insurance(mut v_q: Py, mut v_i: Py, mut v_u: Py, mut v_S: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_kp: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__q_check(v_q.clone(), Py::Bool(true), Py::None)?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__pos(Py::str("the sum insured S"), Py::tuple(vec![v_S.clone()]))?;
    let _ = f__whole(Py::str("the deferment u"), v_u.clone(), Py::Int(0i64))?;
    let _ = f__need({ let t1244 = v_u.clone(); Py::Bool({ let t1245 = py::len(&v_q.clone())?; if py::lt(&t1244, &t1245)? { true } else { false } }) }, Py::str("the deferment must end before the table closes"))?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_kp = f__kp(v_q.clone())?;
    v_s = Py::Float(0.0);
    for t1247 in py::range_iter(&[(v_u.clone()).i()?, (py::len(&v_q.clone())?).i()?]) {
        v_k = t1247;
        v_s = py::iadd(&v_s, &py::mul(&py::mul(&py::pow(&v_v.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_kp.clone(), &v_k.clone())?)?, &py::getitem(&v_q.clone(), &v_k.clone())?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("uA"), v_s.clone()), (Py::str("EPV"), py::mul(&v_S.clone(), &v_s.clone())?), (Py::str("uEx"), py::mul(&py::pow(&v_v.clone(), &v_u.clone())?, &py::getitem(&v_kp.clone(), &v_u.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_duration_convexity(mut v_amounts: Py, mut v_times: Py, mut v_i: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_mac: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__rate(v_i.clone())?;
    let _ = f__need({ let t1292 = py::len(&v_amounts.clone())?; Py::Bool({ let t1293 = py::len(&v_times.clone())?; if py::eq(&t1292, &t1293) { true } else { false } }) }, Py::str("give one time for every amount"))?;
    for t1294 in py::iter(&(v_amounts.clone()))? {
        v_c = t1294;
        let _ = f__need({ let t1297 = f__isnum(v_c.clone())?; if !t1297.truthy() { t1297 } else { { let t1295 = v_c.clone(); Py::Bool({ let t1296 = Py::Int(0i64); if py::ge(&t1295, &t1296)? { true } else { false } }) } } }, Py::str("the cash flows must be zero or positive (liability or bond cash flows)"))?;
    }
    for t1298 in py::iter(&(v_times.clone()))? {
        v_t = t1298;
        let _ = f__need({ let t1302 = f__isnum(v_t.clone())?; if !t1302.truthy() { t1302 } else { { let t1299 = Py::Int(0i64); Py::Bool({ let t1300 = v_t.clone(); if py::le(&t1299, &t1300)? { { let t1301 = Py::Int(200i64); if py::le(&t1300, &t1301)? { true } else { false } } } else { false } }) } } }, Py::str("every time must lie between 0 and 200 years"))?;
    }
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    let t1303 = Py::Float(0.0);
    v_p = t1303.clone();
    v_s1 = t1303.clone();
    v_s2 = t1303.clone();
    for t1304 in py::iter(&(py::zip(&[v_amounts.clone(), v_times.clone()])?))? {
        let t1305 = (t1304).items()?;
        if t1305.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_c = t1305[0].clone();
        v_t = t1305[1].clone();
        v_w = py::mul(&v_c.clone(), &py::pow(&v_v.clone(), &v_t.clone())?)?;
        v_p = py::iadd(&v_p, &v_w.clone())?;
        v_s1 = py::iadd(&v_s1, &py::mul(&v_t.clone(), &v_w.clone())?)?;
        v_s2 = py::iadd(&v_s2, &py::mul(&py::mul(&v_t.clone(), &py::add(&v_t.clone(), &Py::Float(1.0))?)?, &v_w.clone())?)?;
    }
    let _ = f__need({ let t1306 = v_p.clone(); Py::Bool({ let t1307 = Py::Int(0i64); if py::gt(&t1306, &t1307)? { true } else { false } }) }, Py::str("the present value must be positive"))?;
    v_mac = py::truediv(&v_s1.clone(), &v_p.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("PV"), v_p.clone()), (Py::str("macaulay"), v_mac.clone()), (Py::str("modified"), py::mul(&v_mac.clone(), &v_v.clone())?), (Py::str("convexity"), py::truediv(&py::mul(&py::mul(&v_s2.clone(), &v_v.clone())?, &v_v.clone())?, &v_p.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_endowment_insurance(mut v_q: Py, mut v_i: Py, mut v_n: Py, mut v_S: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_A1: Py = Py::None;
    let mut v_A2: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__q_check(v_q.clone(), Py::Bool(false), v_n.clone())?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__pos(Py::str("the sum insured S"), Py::tuple(vec![v_S.clone()]))?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_A1 = f__ins(v_q.clone(), v_v.clone(), v_n.clone())?;
    v_E = f__pe(v_q.clone(), v_v.clone(), v_n.clone())?;
    v_A2 = py::add(&f__ins(v_q.clone(), py::mul(&v_v.clone(), &v_v.clone())?, v_n.clone())?, &f__pe(v_q.clone(), py::mul(&v_v.clone(), &v_v.clone())?, v_n.clone())?)?;
    v_A = py::add(&v_A1.clone(), &v_E.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("A1"), v_A1.clone()), (Py::str("nEx"), v_E.clone()), (Py::str("A2"), v_A2.clone()), (Py::str("EPV"), py::mul(&v_S.clone(), &v_A.clone())?), (Py::str("sd"), py::mul(&v_S.clone(), &py::math1("sqrt", &py::minmax(&[py::sub(&v_A2.clone(), &py::mul(&v_A.clone(), &v_A.clone())?)?, Py::Float(0.0)], true)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_expected_claims(mut v_P: Py, mut v_elr: Py, mut v_paid: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_ult: Py = Py::None;
    let _ = f__need({ let t1417 = { let t1413 = py::len(&v_P.clone())?; Py::Bool({ let t1414 = py::len(&v_paid.clone())?; if py::eq(&t1413, &t1414) { true } else { false } }) }; if !t1417.truthy() { t1417 } else { { let t1415 = py::len(&v_P.clone())?; Py::Bool({ let t1416 = Py::Int(1i64); if py::ge(&t1415, &t1416)? { true } else { false } }) } } }, Py::str("give one premium and one paid amount for every origin year"))?;
    let _ = f__need({ let t1423 = f__isnum(v_elr.clone())?; if !t1423.truthy() { t1423 } else { { let t1420 = Py::Int(0i64); Py::Bool({ let t1421 = v_elr.clone(); if py::lt(&t1420, &t1421)? { { let t1422 = Py::Int(5i64); if py::le(&t1421, &t1422)? { true } else { false } } } else { false } }) } } }, Py::str("the expected loss ratio must lie between 0 and 500%"))?;
    let t1424 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t1424.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ult = t1424[0].clone();
    v_res = t1424[1].clone();
    v_tot = Py::Float(0.0);
    for t1425 in py::iter(&(py::zip(&[v_P.clone(), v_paid.clone()])?))? {
        let t1432 = (t1425).items()?;
        if t1432.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p = t1432[0].clone();
        v_c = t1432[1].clone();
        let _ = f__need({ let t1428 = f__isnum(v_p.clone())?; if !t1428.truthy() { t1428 } else { { let t1426 = v_p.clone(); Py::Bool({ let t1427 = Py::Int(0i64); if py::gt(&t1426, &t1427)? { true } else { false } }) } } }, Py::str("every premium must be positive"))?;
        let _ = f__need({ let t1431 = f__isnum(v_c.clone())?; if !t1431.truthy() { t1431 } else { { let t1429 = v_c.clone(); Py::Bool({ let t1430 = Py::Int(0i64); if py::ge(&t1429, &t1430)? { true } else { false } }) } } }, Py::str("every paid amount must be zero or positive"))?;
        v_u = py::mul(&v_p.clone(), &v_elr.clone())?;
        let _ = py::method(&v_ult.clone(), "append", &[v_u.clone()])?;
        let _ = py::method(&v_res.clone(), "append", &[py::sub(&v_u.clone(), &v_c.clone())?])?;
        v_tot = py::iadd(&v_tot, &py::sub(&v_u.clone(), &v_c.clone())?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("ultimate"), v_ult.clone()), (Py::str("reserve"), v_res.clone()), (Py::str("total_reserve"), v_tot.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_exponential_coverage(mut v_theta: Py, mut v_d: Py, mut v_u: Py, mut v_c: Py, mut v_r: Py) -> R {
    let _ = f__pos(Py::str("the mean theta"), Py::tuple(vec![v_theta.clone()]))?;
    return Ok(f__coverage(Py::str("exponential"), v_theta.clone(), Py::Float(0.0), v_d.clone(), v_u.clone(), v_c.clone(), v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_exponential_deductible(mut v_theta: Py, mut v_d: Py) -> R {
    let _ = f__pos(Py::str("the mean theta"), Py::tuple(vec![v_theta.clone()]))?;
    return Ok(f__deductible(Py::str("exponential"), v_theta.clone(), Py::Float(0.0), v_d.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_exponential_mle(mut v_times: Py, mut v_events: Py, mut v_t: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let _ = f__surv_check(v_times.clone(), v_events.clone())?;
    let _ = f__need({ let t1641 = f__isnum(v_t.clone())?; if !t1641.truthy() { t1641 } else { { let t1639 = v_t.clone(); Py::Bool({ let t1640 = Py::Int(0i64); if py::ge(&t1639, &t1640)? { true } else { false } }) } } }, Py::str("the time t must be zero or positive"))?;
    v_d = Py::Float(0.0);
    v_tot = Py::Float(0.0);
    for t1642 in py::iter(&(py::zip(&[v_times.clone(), v_events.clone()])?))? {
        let t1643 = (t1642).items()?;
        if t1643.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t1643[0].clone();
        v_b = t1643[1].clone();
        v_d = py::iadd(&v_d, &v_b.clone())?;
        v_tot = py::iadd(&v_tot, &v_a.clone())?;
    }
    let _ = f__need({ let t1644 = v_d.clone(); Py::Bool({ let t1645 = Py::Int(0i64); if py::gt(&t1644, &t1645)? { true } else { false } }) }, Py::str("at least one death is needed to estimate the hazard"))?;
    v_lam = py::truediv(&v_d.clone(), &v_tot.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("hazard"), v_lam.clone()), (Py::str("mean"), py::truediv(&Py::Float(1.0), &v_lam.clone())?), (Py::str("S"), py::math1("exp", &py::mul(&py::neg(&v_lam.clone())?, &v_t.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_force_of_interest(mut v_delta: Py) -> R {
    let mut v_i: Py = Py::None;
    let _ = f__need({ let t1689 = f__isnum(v_delta.clone())?; if !t1689.truthy() { t1689 } else { { let t1686 = Py::Int(0i64); Py::Bool({ let t1687 = v_delta.clone(); if py::lt(&t1686, &t1687)? { { let t1688 = Py::Int(1i64); if py::lt(&t1687, &t1688)? { true } else { false } } } else { false } }) } } }, Py::str("the force of interest must lie between 0 and 1"))?;
    v_i = py::sub(&py::math1("exp", &v_delta.clone())?, &Py::Float(1.0))?;
    return Ok(f__out(Py::dict(vec![(Py::str("i"), v_i.clone()), (Py::str("d"), py::sub(&Py::Float(1.0), &py::math1("exp", &py::neg(&v_delta.clone())?)?)?), (Py::str("v"), py::math1("exp", &py::neg(&v_delta.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fractional_age(mut v_q: Py, mut v_s: Py, mut v_assumption: Py) -> R {
    let mut v_mu: Py = Py::None;
    let mut v_sp: Py = Py::None;
    let _ = f__need({ let t1728 = v_assumption.clone(); Py::Bool({ let t1729 = Py::tuple(vec![Py::str("udd"), Py::str("constant_force"), Py::str("balducci")]); if py::contains(&t1729, &t1728)? { true } else { false } }) }, Py::str("assumption: udd, constant_force or balducci"))?;
    let _ = f__need({ let t1735 = f__isnum(v_q.clone())?; if !t1735.truthy() { t1735 } else { { let t1732 = Py::Int(0i64); Py::Bool({ let t1733 = v_q.clone(); if py::lt(&t1732, &t1733)? { { let t1734 = Py::Int(1i64); if py::lt(&t1733, &t1734)? { true } else { false } } } else { false } }) } } }, Py::str("q must lie strictly between 0 and 1"))?;
    let _ = f__need({ let t1739 = f__isnum(v_s.clone())?; if !t1739.truthy() { t1739 } else { { let t1736 = Py::Int(0i64); Py::Bool({ let t1737 = v_s.clone(); if py::lt(&t1736, &t1737)? { { let t1738 = Py::Int(1i64); if py::lt(&t1737, &t1738)? { true } else { false } } } else { false } }) } } }, Py::str("the fraction of a year s must lie strictly between 0 and 1"))?;
    if ({ let t1740 = v_assumption.clone(); Py::Bool({ let t1741 = Py::str("udd"); if py::eq(&t1740, &t1741) { true } else { false } }) }).truthy() {
        v_sp = py::sub(&Py::Float(1.0), &py::mul(&v_s.clone(), &v_q.clone())?)?;
        v_mu = py::truediv(&v_q.clone(), &py::sub(&Py::Float(1.0), &py::mul(&v_s.clone(), &v_q.clone())?)?)?;
    } else {
        if ({ let t1742 = v_assumption.clone(); Py::Bool({ let t1743 = Py::str("constant_force"); if py::eq(&t1742, &t1743) { true } else { false } }) }).truthy() {
            v_sp = py::pow(&py::sub(&Py::Float(1.0), &v_q.clone())?, &v_s.clone())?;
            v_mu = py::neg(&py::math1("log", &py::sub(&Py::Float(1.0), &v_q.clone())?)?)?;
        } else {
            v_sp = py::truediv(&py::sub(&Py::Float(1.0), &v_q.clone())?, &py::sub(&Py::Float(1.0), &py::mul(&py::sub(&Py::Float(1.0), &v_s.clone())?, &v_q.clone())?)?)?;
            v_mu = py::truediv(&v_q.clone(), &py::sub(&Py::Float(1.0), &py::mul(&py::sub(&Py::Float(1.0), &v_s.clone())?, &v_q.clone())?)?)?;
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("spx"), v_sp.clone()), (Py::str("sqx"), py::sub(&Py::Float(1.0), &v_sp.clone())?), (Py::str("mu"), v_mu.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gompertz(mut v_B: Py, mut v_c: Py, mut v_x: Py, mut v_t: Py) -> R {
    let mut v_tp: Py = Py::None;
    let _ = f__mk_check(Py::Float(0.0), v_B.clone(), v_c.clone(), v_x.clone(), v_t.clone())?;
    v_tp = f__mk_tp(Py::Float(0.0), v_B.clone(), v_c.clone(), v_x.clone(), v_t.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("tpx"), v_tp.clone()), (Py::str("tqx"), py::sub(&Py::Float(1.0), &v_tp.clone())?), (Py::str("mu_x"), py::mul(&v_B.clone(), &py::pow(&v_c.clone(), &v_x.clone())?)?), (Py::str("mu_xt"), py::mul(&v_B.clone(), &py::pow(&v_c.clone(), &py::add(&v_x.clone(), &v_t.clone())?)?)?), (Py::str("e_complete"), f__mk_integral(Py::Float(0.0), v_B.clone(), v_c.clone(), v_x.clone(), Py::Float(0.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gross_premium(mut v_q: Py, mut v_i: Py, mut v_S: Py, mut v_e0: Py, mut v_r: Py, mut v_c: Py, mut v_E: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_G: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_a: Py = Py::None;
    let _ = f__q_check(v_q.clone(), Py::Bool(true), Py::None)?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__exp_check(v_S.clone(), v_e0.clone(), v_r.clone(), v_c.clone(), v_E.clone())?;
    let t1896 = (f__gross_G(v_q.clone(), v_i.clone(), v_S.clone(), v_e0.clone(), v_r.clone(), v_c.clone(), v_E.clone())?).items()?;
    if t1896.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_G = t1896[0].clone();
    v_A = t1896[1].clone();
    v_a = t1896[2].clone();
    v_P = py::truediv(&py::mul(&v_S.clone(), &v_A.clone())?, &v_a.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("G"), v_G.clone()), (Py::str("P"), v_P.clone()), (Py::str("loading"), py::sub(&v_G.clone(), &v_P.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gross_premium_reserve(mut v_q: Py, mut v_i: Py, mut v_S: Py, mut v_e0: Py, mut v_r: Py, mut v_c: Py, mut v_E: Py, mut v_t: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_G: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_qt: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__q_check(v_q.clone(), Py::Bool(true), Py::None)?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__exp_check(v_S.clone(), v_e0.clone(), v_r.clone(), v_c.clone(), v_E.clone())?;
    let _ = f__whole(Py::str("the policy duration t"), v_t.clone(), Py::Int(0i64))?;
    let _ = f__need({ let t1976 = v_t.clone(); Py::Bool({ let t1977 = py::len(&v_q.clone())?; if py::lt(&t1976, &t1977)? { true } else { false } }) }, Py::str("the duration t must lie inside the table"))?;
    let t1983 = (f__gross_G(v_q.clone(), v_i.clone(), v_S.clone(), v_e0.clone(), v_r.clone(), v_c.clone(), v_E.clone())?).items()?;
    if t1983.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_G = t1983[0].clone();
    v__ = t1983[1].clone();
    v__ = t1983[2].clone();
    if ({ let t1984 = v_t.clone(); Py::Bool({ let t1985 = Py::Int(0i64); if py::eq(&t1984, &t1985) { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("V"), Py::Float(0.0)), (Py::str("G"), v_G.clone())]))?);
    }
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_qt = py::slice(&v_q.clone(), Some((v_t.clone()).i()?), None, None)?;
    v_A = f__ins(v_qt.clone(), v_v.clone(), py::len(&v_qt.clone())?)?;
    v_a = f__ann(v_qt.clone(), v_v.clone(), py::len(&v_qt.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), py::sub(&py::add(&py::mul(&py::add(&v_S.clone(), &v_E.clone())?, &v_A.clone())?, &py::mul(&v_c.clone(), &v_a.clone())?)?, &py::mul(&py::mul(&v_G.clone(), &py::sub(&Py::Float(1.0), &v_r.clone())?)?, &v_a.clone())?)?), (Py::str("G"), v_G.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_increasing_annuity(mut v_n: Py, mut v_i: Py) -> R {
    let mut v_adn: Py = Py::None;
    let mut v_an: Py = Py::None;
    let mut v_da: Py = Py::None;
    let mut v_ia: Py = Py::None;
    let mut v_sdn: Py = Py::None;
    let mut v_sn: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__whole(Py::str("the term n"), v_n.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t2028 = v_n.clone(); Py::Bool({ let t2029 = Py::Int(1000i64); if py::le(&t2028, &t2029)? { true } else { false } }) }, Py::str("n must be at most 1000"))?;
    let _ = f__rate(v_i.clone())?;
    let t2036 = (f__annuity_values(v_n.clone(), v_i.clone())?).items()?;
    if t2036.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_v = t2036[0].clone();
    v_an = t2036[1].clone();
    v_adn = t2036[2].clone();
    v_sn = t2036[3].clone();
    v_sdn = t2036[4].clone();
    v_ia = py::truediv(&py::sub(&v_adn.clone(), &py::mul(&v_n.clone(), &py::pow(&v_v.clone(), &v_n.clone())?)?)?, &v_i.clone())?;
    v_da = py::truediv(&py::sub(&v_n.clone(), &v_an.clone())?, &v_i.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Ia"), v_ia.clone()), (Py::str("Ia_due"), py::mul(&v_ia.clone(), &py::add(&Py::Float(1.0), &v_i.clone())?)?), (Py::str("Da"), v_da.clone()), (Py::str("Da_due"), py::mul(&v_da.clone(), &py::add(&Py::Float(1.0), &v_i.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_individual_risk_model(mut v_q: Py, mut v_b: Py) -> R {
    let mut v_bi: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_p0: Py = Py::None;
    let mut v_qi: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t2075 = py::len(&v_q.clone())?; Py::Bool({ let t2076 = py::len(&v_b.clone())?; if py::eq(&t2075, &t2076) { true } else { false } }) }, Py::str("give one benefit for every probability"))?;
    let _ = f__need({ let t2077 = py::len(&v_q.clone())?; Py::Bool({ let t2078 = Py::Int(200i64); if py::le(&t2077, &t2078)? { true } else { false } }) }, Py::str("at most 200 policies"))?;
    let t2079 = Py::Float(0.0);
    v_m = t2079.clone();
    v_v = t2079.clone();
    v_p0 = Py::Float(1.0);
    for t2080 in py::iter(&(py::zip(&[v_q.clone(), v_b.clone()])?))? {
        let t2090 = (t2080).items()?;
        if t2090.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_qi = t2090[0].clone();
        v_bi = t2090[1].clone();
        let _ = f__need({ let t2086 = f__isnum(v_qi.clone())?; if !t2086.truthy() { t2086 } else { { let t2083 = Py::Int(0i64); Py::Bool({ let t2084 = v_qi.clone(); if py::le(&t2083, &t2084)? { { let t2085 = Py::Int(1i64); if py::le(&t2084, &t2085)? { true } else { false } } } else { false } }) } } }, Py::str("every claim probability must lie between 0 and 1"))?;
        let _ = f__need({ let t2089 = f__isnum(v_bi.clone())?; if !t2089.truthy() { t2089 } else { { let t2087 = v_bi.clone(); Py::Bool({ let t2088 = Py::Int(0i64); if py::gt(&t2087, &t2088)? { true } else { false } }) } } }, Py::str("every benefit must be positive"))?;
        v_m = py::iadd(&v_m, &py::mul(&v_qi.clone(), &v_bi.clone())?)?;
        v_v = py::iadd(&v_v, &py::mul(&py::mul(&py::mul(&v_qi.clone(), &py::sub(&Py::Float(1.0), &v_qi.clone())?)?, &v_bi.clone())?, &v_bi.clone())?)?;
        v_p0 = py::mul(&v_p0, &py::sub(&Py::Float(1.0), &v_qi.clone())?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("mean"), v_m.clone()), (Py::str("var"), v_v.clone()), (Py::str("prob_no_claim"), v_p0.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_joint_life(mut v_qx: Py, mut v_qy: Py, mut v_i: Py) -> R {
    let mut v_ax: Py = Py::None;
    let mut v_axy: Py = Py::None;
    let mut v_ay: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_last: Py = Py::None;
    let mut v_px: Py = Py::None;
    let mut v_py: Py = Py::None;
    let mut v_sx: Py = Py::None;
    let mut v_sy: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__q_check(v_qx.clone(), Py::Bool(true), Py::None)?;
    let _ = f__q_check(v_qy.clone(), Py::Bool(true), Py::None)?;
    let _ = f__rate(v_i.clone())?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_d = py::mul(&v_i.clone(), &v_v.clone())?;
    let t2203 = (Py::tuple(vec![f__kp(v_qx.clone())?, f__kp(v_qy.clone())?])).items()?;
    if t2203.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_px = t2203[0].clone();
    v_py = t2203[1].clone();
    let t2204 = Py::Float(0.0);
    v_axy = t2204.clone();
    v_ax = t2204.clone();
    v_ay = t2204.clone();
    for t2205 in py::range_iter(&[(py::minmax(&[py::len(&v_qx.clone())?, py::len(&v_qy.clone())?], true)?).i()?]) {
        v_k = t2205;
        v_sx = (if ({ let t2206 = v_k.clone(); Py::Bool({ let t2207 = py::len(&v_px.clone())?; if py::lt(&t2206, &t2207)? { true } else { false } }) }).truthy() { py::getitem(&v_px.clone(), &v_k.clone())? } else { Py::Float(0.0) });
        v_sy = (if ({ let t2208 = v_k.clone(); Py::Bool({ let t2209 = py::len(&v_py.clone())?; if py::lt(&t2208, &t2209)? { true } else { false } }) }).truthy() { py::getitem(&v_py.clone(), &v_k.clone())? } else { Py::Float(0.0) });
        v_axy = py::iadd(&v_axy, &py::mul(&py::mul(&py::pow(&v_v.clone(), &v_k.clone())?, &v_sx.clone())?, &v_sy.clone())?)?;
        v_ax = py::iadd(&v_ax, &py::mul(&py::pow(&v_v.clone(), &v_k.clone())?, &v_sx.clone())?)?;
        v_ay = py::iadd(&v_ay, &py::mul(&py::pow(&v_v.clone(), &v_k.clone())?, &v_sy.clone())?)?;
    }
    v_last = py::sub(&py::add(&v_ax.clone(), &v_ay.clone())?, &v_axy.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("a_joint"), v_axy.clone()), (Py::str("a_last"), v_last.clone()), (Py::str("A_joint"), py::sub(&Py::Float(1.0), &py::mul(&v_d.clone(), &v_axy.clone())?)?), (Py::str("A_last"), py::sub(&Py::Float(1.0), &py::mul(&v_d.clone(), &v_last.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_kaplan_meier(mut v_times: Py, mut v_events: Py, mut v_t: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_deaths: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_tj: Py = Py::None;
    let _ = f__surv_check(v_times.clone(), v_events.clone())?;
    let _ = f__need({ let t2267 = f__isnum(v_t.clone())?; if !t2267.truthy() { t2267 } else { { let t2265 = v_t.clone(); Py::Bool({ let t2266 = Py::Int(0i64); if py::ge(&t2265, &t2266)? { true } else { false } }) } } }, Py::str("the time t must be zero or positive"))?;
    v_S = Py::Float(1.0);
    v_deaths = Py::Int(0i64);
    for t2283 in py::iter(&(f__risk_table(v_times.clone(), v_events.clone(), v_t.clone())?))? {
        let t2284 = (t2283).items()?;
        if t2284.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_tj = t2284[0].clone();
        v_d = t2284[1].clone();
        v_n = t2284[2].clone();
        v_S = py::mul(&v_S, &py::sub(&Py::Float(1.0), &py::truediv(&v_d.clone(), &v_n.clone())?)?)?;
        v_deaths = py::iadd(&v_deaths, &v_d.clone())?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("S"), v_S.clone()), (Py::str("F"), py::sub(&Py::Float(1.0), &v_S.clone())?), (Py::str("deaths"), v_deaths.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lf_full_credibility(mut v_p: Py, mut v_k: Py) -> R {
    let mut v_z: Py = Py::None;
    let _ = f__p_check(v_p.clone())?;
    let _ = f__need({ let t2453 = f__isnum(v_k.clone())?; if !t2453.truthy() { t2453 } else { { let t2450 = Py::Int(0i64); Py::Bool({ let t2451 = v_k.clone(); if py::lt(&t2450, &t2451)? { { let t2452 = Py::Int(1i64); if py::lt(&t2451, &t2452)? { true } else { false } } } else { false } }) } } }, Py::str("the tolerance k must lie strictly between 0 and 1"))?;
    v_z = f__norm_ppf(py::truediv(&py::add(&Py::Float(1.0), &v_p.clone())?, &Py::Float(2.0))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("n_full"), py::pow(&py::truediv(&v_z.clone(), &v_k.clone())?, &Py::Int(2i64))?), (Py::str("z"), v_z.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lf_full_credibility_severity(mut v_p: Py, mut v_k: Py, mut v_mean_x: Py, mut v_sd_x: Py) -> R {
    let mut v_cv2: Py = Py::None;
    let mut v_n0: Py = Py::None;
    let _ = f__p_check(v_p.clone())?;
    let _ = f__need({ let t2512 = f__isnum(v_k.clone())?; if !t2512.truthy() { t2512 } else { { let t2509 = Py::Int(0i64); Py::Bool({ let t2510 = v_k.clone(); if py::lt(&t2509, &t2510)? { { let t2511 = Py::Int(1i64); if py::lt(&t2510, &t2511)? { true } else { false } } } else { false } }) } } }, Py::str("the tolerance k must lie strictly between 0 and 1"))?;
    let _ = f__pos(Py::str("the severity mean and standard deviation"), Py::tuple(vec![v_mean_x.clone(), v_sd_x.clone()]))?;
    v_n0 = py::pow(&py::truediv(&f__norm_ppf(py::truediv(&py::add(&Py::Float(1.0), &v_p.clone())?, &Py::Float(2.0))?)?, &v_k.clone())?, &Py::Int(2i64))?;
    v_cv2 = py::pow(&py::truediv(&v_sd_x.clone(), &v_mean_x.clone())?, &Py::Int(2i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("claims_aggregate"), py::mul(&v_n0.clone(), &py::add(&Py::Float(1.0), &v_cv2.clone())?)?), (Py::str("claims_severity"), py::mul(&v_n0.clone(), &v_cv2.clone())?), (Py::str("claims_frequency"), v_n0.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_life_expectancy(mut v_lx: Py, mut v_x0: Py, mut v_x: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let _ = f__lx_check(v_lx.clone(), Py::Bool(true))?;
    v_k = f__idx(v_lx.clone(), v_x0.clone(), v_x.clone(), Py::tuple(vec![]))?;
    v_tot = Py::Float(0.0);
    for t2350 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?, (py::len(&v_lx.clone())?).i()?]) {
        v_j = t2350;
        v_tot = py::iadd(&v_tot, &py::getitem(&v_lx.clone(), &v_j.clone())?)?;
    }
    v_e = py::truediv(&v_tot.clone(), &py::getitem(&v_lx.clone(), &v_k.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("e_curtate"), v_e.clone()), (Py::str("e_complete"), py::add(&v_e.clone(), &Py::Float(0.5))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_life_table_functions(mut v_lx: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_dx: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_px: Py = Py::None;
    let mut v_qx: Py = Py::None;
    let _ = f__lx_check(v_lx.clone(), Py::Bool(false))?;
    let t2404 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![]), Py::list(vec![])])).items()?;
    if t2404.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_dx = t2404[0].clone();
    v_qx = t2404[1].clone();
    v_px = t2404[2].clone();
    for t2405 in py::range_iter(&[(py::sub(&py::len(&v_lx.clone())?, &Py::Int(1i64))?).i()?]) {
        v_k = t2405;
        v_d = py::sub(&py::getitem(&v_lx.clone(), &v_k.clone())?, &py::getitem(&v_lx.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?;
        let _ = py::method(&v_dx.clone(), "append", &[v_d.clone()])?;
        let _ = py::method(&v_qx.clone(), "append", &[py::truediv(&v_d.clone(), &py::getitem(&v_lx.clone(), &v_k.clone())?)?])?;
        let _ = py::method(&v_px.clone(), "append", &[py::truediv(&py::getitem(&v_lx.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_lx.clone(), &v_k.clone())?)?])?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("dx"), v_dx.clone()), (Py::str("qx"), v_qx.clone()), (Py::str("px"), v_px.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_loan_amortization(mut v_L: Py, mut v_i: Py, mut v_n: Py, mut v_t: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_an: Py = Py::None;
    let mut v_bal_after: Py = Py::None;
    let mut v_bal_before: Py = Py::None;
    let mut v_interest: Py = Py::None;
    let mut v_principal: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::str("the loan amount L"), Py::tuple(vec![v_L.clone()]))?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__whole(Py::str("the term n"), v_n.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t2580 = v_n.clone(); Py::Bool({ let t2581 = Py::Int(1000i64); if py::le(&t2580, &t2581)? { true } else { false } }) }, Py::str("n must be at most 1000"))?;
    let _ = f__whole(Py::str("the payment number t"), v_t.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t2582 = v_t.clone(); Py::Bool({ let t2583 = v_n.clone(); if py::le(&t2582, &t2583)? { true } else { false } }) }, Py::str("the payment number t must lie between 1 and n"))?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_an = py::truediv(&py::sub(&Py::Float(1.0), &py::pow(&v_v.clone(), &v_n.clone())?)?, &v_i.clone())?;
    v_R = py::truediv(&v_L.clone(), &v_an.clone())?;
    v_bal_before = py::truediv(&py::mul(&v_R.clone(), &py::sub(&Py::Float(1.0), &py::pow(&v_v.clone(), &py::add(&py::sub(&v_n.clone(), &v_t.clone())?, &Py::Int(1i64))?)?)?)?, &v_i.clone())?;
    v_interest = py::mul(&v_i.clone(), &v_bal_before.clone())?;
    v_principal = py::sub(&v_R.clone(), &v_interest.clone())?;
    v_bal_after = py::truediv(&py::mul(&v_R.clone(), &py::sub(&Py::Float(1.0), &py::pow(&v_v.clone(), &py::sub(&v_n.clone(), &v_t.clone())?)?)?)?, &v_i.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("interest_t"), v_interest.clone()), (Py::str("principal_t"), v_principal.clone()), (Py::str("balance_t"), v_bal_after.clone()), (Py::str("total_interest"), py::sub(&py::mul(&v_n.clone(), &v_R.clone())?, &v_L.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lognormal_deductible(mut v_mu: Py, mut v_sigma: Py, mut v_d: Py) -> R {
    let _ = f__need({ let t2627 = f__isnum(v_mu.clone())?; if !t2627.truthy() { t2627 } else { { let t2624 = py::neg(&Py::Int(20i64))?; Py::Bool({ let t2625 = v_mu.clone(); if py::lt(&t2624, &t2625)? { { let t2626 = Py::Int(30i64); if py::lt(&t2625, &t2626)? { true } else { false } } } else { false } }) } } }, Py::str("mu must lie between -20 and 30"))?;
    let _ = f__need({ let t2631 = f__isnum(v_sigma.clone())?; if !t2631.truthy() { t2631 } else { { let t2628 = Py::Int(0i64); Py::Bool({ let t2629 = v_sigma.clone(); if py::lt(&t2628, &t2629)? { { let t2630 = Py::Int(5i64); if py::le(&t2629, &t2630)? { true } else { false } } } else { false } }) } } }, Py::str("sigma must lie between 0 and 5"))?;
    return Ok(f__deductible(Py::str("lognormal"), v_mu.clone(), v_sigma.clone(), v_d.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_loss_at_issue(mut v_q: Py, mut v_i: Py, mut v_S: Py, mut v_P: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_A2: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_kp: Py = Py::None;
    let mut v_pos: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__q_check(v_q.clone(), Py::Bool(true), Py::None)?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__pos(Py::str("the sum insured S and the premium P"), Py::tuple(vec![v_S.clone(), v_P.clone()]))?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_d = py::mul(&v_i.clone(), &v_v.clone())?;
    v_A = f__ins(v_q.clone(), v_v.clone(), py::len(&v_q.clone())?)?;
    v_A2 = f__ins(v_q.clone(), py::mul(&v_v.clone(), &v_v.clone())?, py::len(&v_q.clone())?)?;
    v_a = f__ann(v_q.clone(), v_v.clone(), py::len(&v_q.clone())?)?;
    v_kp = f__kp(v_q.clone())?;
    v_pos = Py::Float(0.0);
    for t2726 in py::range_iter(&[(py::len(&v_q.clone())?).i()?]) {
        v_k = t2726;
        if ({ let t2727 = py::sub(&py::mul(&v_S.clone(), &py::pow(&v_v.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?, &py::truediv(&py::mul(&v_P.clone(), &py::sub(&Py::Float(1.0), &py::pow(&v_v.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?)?, &v_d.clone())?)?; Py::Bool({ let t2728 = Py::Int(0i64); if py::gt(&t2727, &t2728)? { true } else { false } }) }).truthy() {
            v_pos = py::iadd(&v_pos, &py::mul(&py::getitem(&v_kp.clone(), &v_k.clone())?, &py::getitem(&v_q.clone(), &v_k.clone())?)?)?;
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("mean"), py::sub(&py::mul(&v_S.clone(), &v_A.clone())?, &py::mul(&v_P.clone(), &v_a.clone())?)?), (Py::str("sd"), py::mul(&py::add(&v_S.clone(), &py::truediv(&v_P.clone(), &v_d.clone())?)?, &py::math1("sqrt", &py::minmax(&[py::sub(&v_A2.clone(), &py::mul(&v_A.clone(), &v_A.clone())?)?, Py::Float(0.0)], true)?)?)?), (Py::str("prob_loss"), v_pos.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_loss_ratio_rate_change(mut v_L: Py, mut v_EP: Py, mut v_F: Py, mut v_V: Py, mut v_Q: Py) -> R {
    let mut v_lr: Py = Py::None;
    let mut v_perm: Py = Py::None;
    let _ = f__nonneg(Py::str("the losses"), Py::tuple(vec![v_L.clone()]))?;
    let _ = f__pos(Py::str("the earned premium at current rates"), Py::tuple(vec![v_EP.clone()]))?;
    let _ = f__frac(Py::str("the fixed expense ratio F"), v_F.clone(), Py::Bool(false), Py::Bool(true))?;
    let _ = f__frac(Py::str("the variable expense ratio V"), v_V.clone(), Py::Bool(false), Py::Bool(true))?;
    let _ = f__need({ let t2790 = f__isnum(v_Q.clone())?; if !t2790.truthy() { t2790 } else { { let t2787 = py::neg(&Py::Float(0.5))?; Py::Bool({ let t2788 = v_Q.clone(); if py::lt(&t2787, &t2788)? { { let t2789 = Py::Int(1i64); if py::lt(&t2788, &t2789)? { true } else { false } } } else { false } }) } } }, Py::str("the profit provision Q must lie between -50% and 100%"))?;
    v_perm = py::sub(&py::sub(&Py::Float(1.0), &v_V.clone())?, &v_Q.clone())?;
    let _ = f__need({ let t2791 = v_perm.clone(); Py::Bool({ let t2792 = Py::Int(0i64); if py::gt(&t2791, &t2792)? { true } else { false } }) }, Py::str("the variable expenses and profit leave nothing for losses"))?;
    v_lr = py::truediv(&v_L.clone(), &v_EP.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("change"), py::sub(&py::truediv(&py::add(&v_lr.clone(), &v_F.clone())?, &v_perm.clone())?, &Py::Float(1.0))?), (Py::str("loss_ratio"), v_lr.clone()), (Py::str("permissible"), v_perm.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mack_standard_error(mut v_T: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_acc: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_cdf: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_full: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_later: Py = Py::None;
    let mut v_mse: Py = Py::None;
    let mut v_nj: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_tot: Py = Py::None;
    v_N = f__tri(v_T.clone())?;
    let _ = f__need({ let t2848 = v_N.clone(); Py::Bool({ let t2849 = Py::Int(4i64); if py::ge(&t2848, &t2849)? { true } else { false } }) }, Py::str("Mack's method needs at least 4 origin years"))?;
    let t2855 = (f__cl_factors(v_T.clone(), Py::Bool(false))?).items()?;
    if t2855.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f = t2855[0].clone();
    v_cdf = t2855[1].clone();
    v_s2 = Py::list(vec![]);
    for t2856 in py::range_iter(&[(py::sub(&v_N.clone(), &Py::Int(1i64))?).i()?]) {
        v_j = t2856;
        v_nj = py::sub(&py::sub(&v_N.clone(), &Py::Int(1i64))?, &v_j.clone())?;
        if ({ let t2857 = v_nj.clone(); Py::Bool({ let t2858 = Py::Int(2i64); if py::ge(&t2857, &t2858)? { true } else { false } }) }).truthy() {
            v_acc = Py::Float(0.0);
            for t2859 in py::range_iter(&[(v_nj.clone()).i()?]) {
                v_i = t2859;
                v_acc = py::iadd(&v_acc, &py::mul(&py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_j.clone())?, &py::pow(&py::sub(&py::truediv(&py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &py::add(&v_j.clone(), &Py::Int(1i64))?)?, &py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&v_f.clone(), &v_j.clone())?)?, &Py::Int(2i64))?)?)?;
            }
            let _ = py::method(&v_s2.clone(), "append", &[py::truediv(&v_acc.clone(), &py::sub(&v_nj.clone(), &Py::Int(1i64))?)?])?;
        } else {
            let t2860 = (Py::tuple(vec![py::getitem(&v_s2.clone(), &py::neg(&Py::Int(1i64))?)?, py::getitem(&v_s2.clone(), &py::neg(&Py::Int(2i64))?)?])).items()?;
            if t2860.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t2860[0].clone();
            v_b = t2860[1].clone();
            let _ = py::method(&v_s2.clone(), "append", &[(if ({ let t2861 = v_b.clone(); Py::Bool({ let t2862 = Py::Int(0i64); if py::gt(&t2861, &t2862)? { true } else { false } }) }).truthy() { py::minmax(&[py::truediv(&py::mul(&v_a.clone(), &v_a.clone())?, &v_b.clone())?, v_a.clone(), v_b.clone()], false)? } else { Py::Float(0.0) })])?;
        }
    }
    v_S = Py::list(vec![]);
    for t2863 in py::range_iter(&[(py::sub(&v_N.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t2863;
        v_acc = Py::Float(0.0);
        for t2864 in py::range_iter(&[(py::sub(&py::sub(&v_N.clone(), &Py::Int(1i64))?, &v_k.clone())?).i()?]) {
            v_i = t2864;
            v_acc = py::iadd(&v_acc, &py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_k.clone())?)?;
        }
        let _ = py::method(&v_S.clone(), "append", &[v_acc.clone()])?;
    }
    v_full = Py::list(vec![]);
    for t2865 in py::range_iter(&[(v_N.clone()).i()?]) {
        v_i = t2865;
        v_row = { let mut out = vec![]; for t2866 in py::range_iter(&[(py::sub(&v_N.clone(), &v_i.clone())?).i()?]) { let mut v_j: Py; 
v_j = t2866;
out.push(py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) };
        for t2867 in py::range_iter(&[(py::sub(&v_N.clone(), &v_i.clone())?).i()?, (v_N.clone()).i()?]) {
            v_j = t2867;
            let _ = py::method(&v_row.clone(), "append", &[py::mul(&py::getitem(&v_row.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_f.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?)?])?;
        }
        let _ = py::method(&v_full.clone(), "append", &[v_row.clone()])?;
    }
    v_mse = Py::list(vec![]);
    for t2868 in py::range_iter(&[(v_N.clone()).i()?]) {
        v_i = t2868;
        v_acc = Py::Float(0.0);
        for t2869 in py::range_iter(&[(py::sub(&py::sub(&v_N.clone(), &Py::Int(1i64))?, &v_i.clone())?).i()?, (py::sub(&v_N.clone(), &Py::Int(1i64))?).i()?]) {
            v_k = t2869;
            v_acc = py::iadd(&v_acc, &py::mul(&py::truediv(&py::getitem(&v_s2.clone(), &v_k.clone())?, &py::mul(&py::getitem(&v_f.clone(), &v_k.clone())?, &py::getitem(&v_f.clone(), &v_k.clone())?)?)?, &py::add(&py::truediv(&Py::Float(1.0), &py::getitem(&py::getitem(&v_full.clone(), &v_i.clone())?, &v_k.clone())?)?, &py::truediv(&Py::Float(1.0), &py::getitem(&v_S.clone(), &v_k.clone())?)?)?)?)?;
        }
        let _ = py::method(&v_mse.clone(), "append", &[py::mul(&py::pow(&py::getitem(&py::getitem(&v_full.clone(), &v_i.clone())?, &py::sub(&v_N.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?, &v_acc.clone())?])?;
    }
    v_tot = Py::Float(0.0);
    for t2870 in py::range_iter(&[(v_N.clone()).i()?]) {
        v_i = t2870;
        v_later = Py::Float(0.0);
        for t2871 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_N.clone()).i()?]) {
            v_j = t2871;
            v_later = py::iadd(&v_later, &py::getitem(&py::getitem(&v_full.clone(), &v_j.clone())?, &py::sub(&v_N.clone(), &Py::Int(1i64))?)?)?;
        }
        v_acc = Py::Float(0.0);
        for t2872 in py::range_iter(&[(py::sub(&py::sub(&v_N.clone(), &Py::Int(1i64))?, &v_i.clone())?).i()?, (py::sub(&v_N.clone(), &Py::Int(1i64))?).i()?]) {
            v_k = t2872;
            v_acc = py::iadd(&v_acc, &py::truediv(&py::truediv(&py::mul(&Py::Float(2.0), &py::getitem(&v_s2.clone(), &v_k.clone())?)?, &py::mul(&py::getitem(&v_f.clone(), &v_k.clone())?, &py::getitem(&v_f.clone(), &v_k.clone())?)?)?, &py::getitem(&v_S.clone(), &v_k.clone())?)?)?;
        }
        v_tot = py::iadd(&v_tot, &py::add(&py::getitem(&v_mse.clone(), &v_i.clone())?, &py::mul(&py::mul(&py::getitem(&py::getitem(&v_full.clone(), &v_i.clone())?, &py::sub(&v_N.clone(), &Py::Int(1i64))?)?, &v_later.clone())?, &v_acc.clone())?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("se"), { let mut out = vec![]; for t2885 in py::iter(&(v_mse.clone()))? { let mut v_m_: Py; 
v_m_ = t2885;
out.push(py::math1("sqrt", &v_m_.clone())?); } Py::list(out) }), (Py::str("total_se"), py::math1("sqrt", &v_tot.clone())?), (Py::str("sigma2"), v_s2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_makeham(mut v_A: Py, mut v_B: Py, mut v_c: Py, mut v_x: Py, mut v_t: Py) -> R {
    let mut v_tp: Py = Py::None;
    let _ = f__mk_check(v_A.clone(), v_B.clone(), v_c.clone(), v_x.clone(), v_t.clone())?;
    v_tp = f__mk_tp(v_A.clone(), v_B.clone(), v_c.clone(), v_x.clone(), v_t.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("tpx"), v_tp.clone()), (Py::str("tqx"), py::sub(&Py::Float(1.0), &v_tp.clone())?), (Py::str("mu_x"), py::add(&v_A.clone(), &py::mul(&v_B.clone(), &py::pow(&v_c.clone(), &v_x.clone())?)?)?), (Py::str("mu_xt"), py::add(&v_A.clone(), &py::mul(&v_B.clone(), &py::pow(&v_c.clone(), &py::add(&v_x.clone(), &v_t.clone())?)?)?)?), (Py::str("e_complete"), f__mk_integral(v_A.clone(), v_B.clone(), v_c.clone(), v_x.clone(), Py::Float(0.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_makeham_continuous(mut v_A: Py, mut v_B: Py, mut v_c: Py, mut v_x: Py, mut v_i: Py, mut v_S: Py, mut v_t: Py) -> R {
    let mut v_Abar: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_a0: Py = Py::None;
    let mut v_at: Py = Py::None;
    let mut v_delta: Py = Py::None;
    let _ = f__mk_check(v_A.clone(), v_B.clone(), v_c.clone(), v_x.clone(), v_t.clone())?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__pos(Py::str("the sum insured S"), Py::tuple(vec![v_S.clone()]))?;
    v_delta = py::math1("log1p", &v_i.clone())?;
    v_a0 = f__mk_integral(v_A.clone(), v_B.clone(), v_c.clone(), v_x.clone(), v_delta.clone())?;
    v_at = f__mk_integral(v_A.clone(), v_B.clone(), v_c.clone(), py::add(&v_x.clone(), &v_t.clone())?, v_delta.clone())?;
    v_Abar = py::sub(&Py::Float(1.0), &py::mul(&v_delta.clone(), &v_a0.clone())?)?;
    v_P = py::truediv(&py::mul(&v_S.clone(), &v_Abar.clone())?, &v_a0.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("a_bar"), v_a0.clone()), (Py::str("A_bar"), v_Abar.clone()), (Py::str("P"), v_P.clone()), (Py::str("V"), py::mul(&v_S.clone(), &py::sub(&Py::Float(1.0), &py::truediv(&v_at.clone(), &v_a0.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mthly_annuity_udd(mut v_q: Py, mut v_i: Py, mut v_m: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_alpha: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dm: Py = Py::None;
    let mut v_im: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__q_check(v_q.clone(), Py::Bool(true), Py::None)?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__whole(Py::str("the number of payments per year m"), v_m.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t3079 = v_m.clone(); Py::Bool({ let t3080 = Py::Int(365i64); if py::le(&t3079, &t3080)? { true } else { false } }) }, Py::str("m must be at most 365"))?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_d = py::mul(&v_i.clone(), &v_v.clone())?;
    v_im = py::mul(&v_m.clone(), &py::sub(&py::pow(&py::add(&Py::Float(1.0), &v_i.clone())?, &py::truediv(&Py::Float(1.0), &v_m.clone())?)?, &Py::Float(1.0))?)?;
    v_dm = py::mul(&v_m.clone(), &py::sub(&Py::Float(1.0), &py::pow(&v_v.clone(), &py::truediv(&Py::Float(1.0), &v_m.clone())?)?)?)?;
    v_alpha = py::truediv(&py::mul(&v_i.clone(), &v_d.clone())?, &py::mul(&v_im.clone(), &v_dm.clone())?)?;
    v_beta = py::truediv(&py::sub(&v_i.clone(), &v_im.clone())?, &py::mul(&v_im.clone(), &v_dm.clone())?)?;
    v_a = f__ann(v_q.clone(), v_v.clone(), py::len(&v_q.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("a_m"), py::sub(&py::mul(&v_alpha.clone(), &v_a.clone())?, &v_beta.clone())?), (Py::str("alpha"), v_alpha.clone()), (Py::str("beta"), v_beta.clone()), (Py::str("a_due"), v_a.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_multiple_to_single(mut v_q1: Py, mut v_q2: Py) -> R {
    let mut v_pt: Py = Py::None;
    let mut v_qt: Py = Py::None;
    let _ = f__need({ let t3129 = f__isnum(v_q1.clone())?; if !t3129.truthy() { t3129 } else { { let t3128 = f__isnum(v_q2.clone())?; if !t3128.truthy() { t3128 } else { { let t3127 = { let t3123 = v_q1.clone(); Py::Bool({ let t3124 = Py::Int(0i64); if py::gt(&t3123, &t3124)? { true } else { false } }) }; if !t3127.truthy() { t3127 } else { { let t3125 = v_q2.clone(); Py::Bool({ let t3126 = Py::Int(0i64); if py::gt(&t3125, &t3126)? { true } else { false } }) } } } } } } }, Py::str("both decrement probabilities must be positive"))?;
    v_qt = py::add(&v_q1.clone(), &v_q2.clone())?;
    let _ = f__need({ let t3130 = v_qt.clone(); Py::Bool({ let t3131 = Py::Int(1i64); if py::lt(&t3130, &t3131)? { true } else { false } }) }, Py::str("the total decrement probability must lie below 1"))?;
    v_pt = py::sub(&Py::Float(1.0), &v_qt.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("q1_single"), py::sub(&Py::Float(1.0), &py::pow(&v_pt.clone(), &py::truediv(&v_q1.clone(), &v_qt.clone())?)?)?), (Py::str("q2_single"), py::sub(&Py::Float(1.0), &py::pow(&v_pt.clone(), &py::truediv(&v_q2.clone(), &v_qt.clone())?)?)?), (Py::str("q_total"), v_qt.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_negative_binomial_frequency(mut v_r: Py, mut v_beta: Py, mut v_k: Py) -> R {
    let mut v_cdf: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_pk: Py = Py::None;
    let _ = f__need({ let t3175 = f__isnum(v_r.clone())?; if !t3175.truthy() { t3175 } else { { let t3172 = Py::Int(0i64); Py::Bool({ let t3173 = v_r.clone(); if py::lt(&t3172, &t3173)? { { let t3174 = Py::Int(1000i64); if py::le(&t3173, &t3174)? { true } else { false } } } else { false } }) } } }, Py::str("r must lie between 0 and 1000"))?;
    let _ = f__need({ let t3179 = f__isnum(v_beta.clone())?; if !t3179.truthy() { t3179 } else { { let t3176 = Py::Int(0i64); Py::Bool({ let t3177 = v_beta.clone(); if py::lt(&t3176, &t3177)? { { let t3178 = Py::Int(1000i64); if py::le(&t3177, &t3178)? { true } else { false } } } else { false } }) } } }, Py::str("beta must lie between 0 and 1000"))?;
    let _ = f__whole(Py::str("the number of claims k"), v_k.clone(), Py::Int(0i64))?;
    let _ = f__need({ let t3184 = v_k.clone(); Py::Bool({ let t3185 = Py::Int(2000i64); if py::le(&t3184, &t3185)? { true } else { false } }) }, Py::str("k must be at most 2000"))?;
    v_pk = { let v_beta = v_beta.clone(); let v_k = v_k.clone(); let v_pk = v_pk.clone(); let v_r = v_r.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::math1("exp", &py::add(&py::sub(&py::sub(&py::sub(&py::math1("lgamma", &py::add(&v_r.clone(), &v_j.clone())?)?, &py::math1("lgamma", &v_r.clone())?)?, &py::math1("lgamma", &py::add(&v_j.clone(), &Py::Float(1.0))?)?)?, &py::mul(&v_r.clone(), &py::math1("log1p", &v_beta.clone())?)?)?, &py::mul(&v_j.clone(), &py::sub(&py::math1("log", &v_beta.clone())?, &py::math1("log1p", &v_beta.clone())?)?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_cdf = Py::Float(0.0);
    for t3186 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) {
        v_j = t3186;
        v_cdf = py::iadd(&v_cdf, &py::call(&v_pk.clone(), &[v_j.clone()])?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("pmf"), py::call(&v_pk.clone(), &[v_k.clone()])?), (Py::str("cdf"), py::minmax(&[v_cdf.clone(), Py::Float(1.0)], false)?), (Py::str("mean"), py::mul(&v_r.clone(), &v_beta.clone())?), (Py::str("var"), py::mul(&py::mul(&v_r.clone(), &v_beta.clone())?, &py::add(&Py::Float(1.0), &v_beta.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nelson_aalen(mut v_times: Py, mut v_events: Py, mut v_t: Py) -> R {
    let mut v_H: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_tj: Py = Py::None;
    let _ = f__surv_check(v_times.clone(), v_events.clone())?;
    let _ = f__need({ let t3244 = f__isnum(v_t.clone())?; if !t3244.truthy() { t3244 } else { { let t3242 = v_t.clone(); Py::Bool({ let t3243 = Py::Int(0i64); if py::ge(&t3242, &t3243)? { true } else { false } }) } } }, Py::str("the time t must be zero or positive"))?;
    v_H = Py::Float(0.0);
    for t3260 in py::iter(&(f__risk_table(v_times.clone(), v_events.clone(), v_t.clone())?))? {
        let t3261 = (t3260).items()?;
        if t3261.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_tj = t3261[0].clone();
        v_d = t3261[1].clone();
        v_n = t3261[2].clone();
        v_H = py::iadd(&v_H, &py::truediv(&v_d.clone(), &v_n.clone())?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("H"), v_H.clone()), (Py::str("S"), py::math1("exp", &py::neg(&v_H.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_net_premium_endowment(mut v_q: Py, mut v_i: Py, mut v_n: Py, mut v_S: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__q_check(v_q.clone(), Py::Bool(false), v_n.clone())?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__pos(Py::str("the sum insured S"), Py::tuple(vec![v_S.clone()]))?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_A = py::add(&f__ins(v_q.clone(), v_v.clone(), v_n.clone())?, &f__pe(v_q.clone(), v_v.clone(), v_n.clone())?)?;
    v_a = f__ann(v_q.clone(), v_v.clone(), v_n.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("P"), py::truediv(&py::mul(&v_S.clone(), &v_A.clone())?, &v_a.clone())?), (Py::str("A"), v_A.clone()), (Py::str("a_due"), v_a.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_net_premium_limited_pay(mut v_q: Py, mut v_i: Py, mut v_h: Py, mut v_S: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__q_check(v_q.clone(), Py::Bool(true), Py::None)?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__pos(Py::str("the sum insured S"), Py::tuple(vec![v_S.clone()]))?;
    let _ = f__whole(Py::str("the premium paying period h"), v_h.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t3395 = v_h.clone(); Py::Bool({ let t3396 = py::len(&v_q.clone())?; if py::le(&t3395, &t3396)? { true } else { false } }) }, Py::str("the premium paying period cannot outlast the table"))?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_A = f__ins(v_q.clone(), v_v.clone(), py::len(&v_q.clone())?)?;
    v_a = f__ann(v_q.clone(), v_v.clone(), v_h.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("P"), py::truediv(&py::mul(&v_S.clone(), &v_A.clone())?, &v_a.clone())?), (Py::str("A"), v_A.clone()), (Py::str("a_due_h"), v_a.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_net_premium_term(mut v_q: Py, mut v_i: Py, mut v_n: Py, mut v_S: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__q_check(v_q.clone(), Py::Bool(false), v_n.clone())?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__pos(Py::str("the sum insured S"), Py::tuple(vec![v_S.clone()]))?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_A = f__ins(v_q.clone(), v_v.clone(), v_n.clone())?;
    v_a = f__ann(v_q.clone(), v_v.clone(), v_n.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("P"), py::truediv(&py::mul(&v_S.clone(), &v_A.clone())?, &v_a.clone())?), (Py::str("A1"), v_A.clone()), (Py::str("a_due"), v_a.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_net_premium_whole_life(mut v_q: Py, mut v_i: Py, mut v_S: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__q_check(v_q.clone(), Py::Bool(true), Py::None)?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__pos(Py::str("the sum insured S"), Py::tuple(vec![v_S.clone()]))?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_A = f__ins(v_q.clone(), v_v.clone(), py::len(&v_q.clone())?)?;
    v_a = f__ann(v_q.clone(), v_v.clone(), py::len(&v_q.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("P"), py::truediv(&py::mul(&v_S.clone(), &v_A.clone())?, &v_a.clone())?), (Py::str("A"), v_A.clone()), (Py::str("a_due"), v_a.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nominal_to_effective(mut v_rate: Py, mut v_m: Py, mut v_kind: Py) -> R {
    let mut v_i: Py = Py::None;
    let _ = f__need({ let t3574 = v_kind.clone(); Py::Bool({ let t3575 = Py::tuple(vec![Py::str("interest"), Py::str("discount")]); if py::contains(&t3575, &t3574)? { true } else { false } }) }, Py::str("kind must be interest or discount"))?;
    let _ = f__whole(Py::str("the number of conversion periods per year m"), v_m.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t3580 = v_m.clone(); Py::Bool({ let t3581 = Py::Int(365i64); if py::le(&t3580, &t3581)? { true } else { false } }) }, Py::str("m must be at most 365"))?;
    let _ = f__need({ let t3586 = f__isnum(v_rate.clone())?; if !t3586.truthy() { t3586 } else { { let t3584 = v_rate.clone(); Py::Bool({ let t3585 = Py::Int(0i64); if py::gt(&t3584, &t3585)? { true } else { false } }) } } }, Py::str("the nominal rate must be positive"))?;
    if ({ let t3587 = v_kind.clone(); Py::Bool({ let t3588 = Py::str("interest"); if py::eq(&t3587, &t3588) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t3589 = v_rate.clone(); Py::Bool({ let t3590 = Py::Int(1i64); if py::lt(&t3589, &t3590)? { true } else { false } }) }, Py::str("the nominal rate must lie below 100%"))?;
        v_i = py::sub(&py::pow(&py::add(&Py::Float(1.0), &py::truediv(&v_rate.clone(), &v_m.clone())?)?, &v_m.clone())?, &Py::Float(1.0))?;
    } else {
        let _ = f__need({ let t3591 = v_rate.clone(); Py::Bool({ let t3592 = py::minmax(&[Py::Float(1.0), v_m.clone()], false)?; if py::lt(&t3591, &t3592)? { true } else { false } }) }, Py::str("the nominal discount rate must lie below 100%"))?;
        v_i = py::sub(&py::pow(&py::sub(&Py::Float(1.0), &py::truediv(&v_rate.clone(), &v_m.clone())?)?, &py::neg(&v_m.clone())?)?, &Py::Float(1.0))?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("i"), v_i.clone()), (Py::str("d"), py::truediv(&v_i.clone(), &py::add(&Py::Float(1.0), &v_i.clone())?)?), (Py::str("delta"), py::math1("log1p", &v_i.clone())?), (Py::str("v"), py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_normal_approx_aggregate(mut v_mean: Py, mut v_var: Py, mut v_s: Py) -> R {
    let mut v_z: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_mean.clone(), v_s.clone()]))?;
    let _ = f__pos(Py::str("the variance"), Py::tuple(vec![v_var.clone()]))?;
    v_z = py::truediv(&py::sub(&v_s.clone(), &v_mean.clone())?, &py::math1("sqrt", &v_var.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("prob_exceed"), py::mul(&Py::Float(0.5), &py::math1("erfc", &py::truediv(&v_z.clone(), &py::math1("sqrt", &Py::Float(2.0))?)?)?)?), (Py::str("z"), v_z.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_panjer_negative_binomial(mut v_r: Py, mut v_beta: Py, mut v_f: Py, mut v_smax: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t3681 = f__isnum(v_r.clone())?; if !t3681.truthy() { t3681 } else { { let t3678 = Py::Int(0i64); Py::Bool({ let t3679 = v_r.clone(); if py::lt(&t3678, &t3679)? { { let t3680 = Py::Int(200i64); if py::le(&t3679, &t3680)? { true } else { false } } } else { false } }) } } }, Py::str("r must lie between 0 and 200"))?;
    let _ = f__need({ let t3685 = f__isnum(v_beta.clone())?; if !t3685.truthy() { t3685 } else { { let t3682 = Py::Int(0i64); Py::Bool({ let t3683 = v_beta.clone(); if py::lt(&t3682, &t3683)? { { let t3684 = Py::Int(50i64); if py::le(&t3683, &t3684)? { true } else { false } } } else { false } }) } } }, Py::str("beta must lie between 0 and 50"))?;
    let _ = f__sev_check(v_f.clone())?;
    let _ = f__whole(Py::str("the largest amount smax"), v_smax.clone(), Py::Int(0i64))?;
    let _ = f__need({ let t3699 = v_smax.clone(); Py::Bool({ let t3700 = Py::Int(300i64); if py::le(&t3699, &t3700)? { true } else { false } }) }, Py::str("smax must be at most 300"))?;
    v_a = py::truediv(&v_beta.clone(), &py::add(&Py::Float(1.0), &v_beta.clone())?)?;
    v_g = f__panjer(v_a.clone(), py::mul(&py::sub(&v_r.clone(), &Py::Float(1.0))?, &v_a.clone())?, py::pow(&py::add(&Py::Float(1.0), &py::mul(&v_beta.clone(), &py::sub(&Py::Float(1.0), &py::getitem(&v_f.clone(), &Py::Int(0i64))?)?)?)?, &py::neg(&v_r.clone())?)?, v_f.clone(), v_smax.clone())?;
    v_tot = Py::Float(0.0);
    for t3703 in py::iter(&(v_g.clone()))? {
        v_v = t3703;
        v_tot = py::iadd(&v_tot, &v_v.clone())?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("g"), v_g.clone()), (Py::str("F_smax"), py::minmax(&[v_tot.clone(), Py::Float(1.0)], false)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_panjer_poisson(mut v_lam: Py, mut v_f: Py, mut v_smax: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t3747 = f__isnum(v_lam.clone())?; if !t3747.truthy() { t3747 } else { { let t3744 = Py::Int(0i64); Py::Bool({ let t3745 = v_lam.clone(); if py::lt(&t3744, &t3745)? { { let t3746 = Py::Int(200i64); if py::le(&t3745, &t3746)? { true } else { false } } } else { false } }) } } }, Py::str("lambda must lie between 0 and 200"))?;
    let _ = f__sev_check(v_f.clone())?;
    let _ = f__whole(Py::str("the largest amount smax"), v_smax.clone(), Py::Int(0i64))?;
    let _ = f__need({ let t3761 = v_smax.clone(); Py::Bool({ let t3762 = Py::Int(300i64); if py::le(&t3761, &t3762)? { true } else { false } }) }, Py::str("smax must be at most 300"))?;
    v_g = f__panjer(Py::Float(0.0), v_lam.clone(), py::math1("exp", &py::mul(&v_lam.clone(), &py::sub(&py::getitem(&v_f.clone(), &Py::Int(0i64))?, &Py::Float(1.0))?)?)?, v_f.clone(), v_smax.clone())?;
    v_tot = Py::Float(0.0);
    for t3765 in py::iter(&(v_g.clone()))? {
        v_v = t3765;
        v_tot = py::iadd(&v_tot, &v_v.clone())?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("g"), v_g.clone()), (Py::str("F_smax"), py::minmax(&[v_tot.clone(), Py::Float(1.0)], false)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pareto_coverage(mut v_alpha: Py, mut v_theta: Py, mut v_d: Py, mut v_u: Py, mut v_c: Py, mut v_r: Py) -> R {
    let _ = f__need({ let t3809 = f__isnum(v_alpha.clone())?; if !t3809.truthy() { t3809 } else { { let t3806 = Py::Int(1i64); Py::Bool({ let t3807 = v_alpha.clone(); if py::lt(&t3806, &t3807)? { { let t3808 = Py::Int(100i64); if py::le(&t3807, &t3808)? { true } else { false } } } else { false } }) } } }, Py::str("alpha must lie between 1 and 100"))?;
    let _ = f__pos(Py::str("theta"), Py::tuple(vec![v_theta.clone()]))?;
    return Ok(f__coverage(Py::str("pareto"), v_alpha.clone(), v_theta.clone(), v_d.clone(), v_u.clone(), v_c.clone(), v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pareto_deductible(mut v_alpha: Py, mut v_theta: Py, mut v_d: Py) -> R {
    let _ = f__need({ let t3894 = f__isnum(v_alpha.clone())?; if !t3894.truthy() { t3894 } else { { let t3891 = Py::Int(1i64); Py::Bool({ let t3892 = v_alpha.clone(); if py::lt(&t3891, &t3892)? { { let t3893 = Py::Int(100i64); if py::le(&t3892, &t3893)? { true } else { false } } } else { false } }) } } }, Py::str("alpha must lie between 1 and 100 (a finite mean)"))?;
    let _ = f__pos(Py::str("theta"), Py::tuple(vec![v_theta.clone()]))?;
    return Ok(f__deductible(Py::str("pareto"), v_alpha.clone(), v_theta.clone(), v_d.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_partial_credibility(mut v_n: Py, mut v_n_full: Py, mut v_xbar: Py, mut v_M: Py) -> R {
    let mut v_Z: Py = Py::None;
    let _ = f__nonneg(Py::str("the observed claims n"), Py::tuple(vec![v_n.clone()]))?;
    let _ = f__pos(Py::str("the full credibility standard"), Py::tuple(vec![v_n_full.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_xbar.clone(), v_M.clone()]))?;
    v_Z = py::minmax(&[Py::Float(1.0), py::math1("sqrt", &py::truediv(&v_n.clone(), &v_n_full.clone())?)?], false)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Z"), v_Z.clone()), (Py::str("premium"), py::add(&py::mul(&v_Z.clone(), &v_xbar.clone())?, &py::mul(&py::sub(&Py::Float(1.0), &v_Z.clone())?, &v_M.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_perpetuity(mut v_i: Py) -> R {
    let _ = f__rate(v_i.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("a_inf"), py::truediv(&Py::Float(1.0), &v_i.clone())?), (Py::str("a_due_inf"), py::truediv(&py::add(&Py::Float(1.0), &v_i.clone())?, &v_i.clone())?), (Py::str("a_cont_inf"), py::truediv(&Py::Float(1.0), &py::math1("log1p", &v_i.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_poisson_frequency(mut v_lam: Py, mut v_k: Py) -> R {
    let mut v_cdf: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_pmf: Py = Py::None;
    let _ = f__need({ let t4061 = f__isnum(v_lam.clone())?; if !t4061.truthy() { t4061 } else { { let t4058 = Py::Int(0i64); Py::Bool({ let t4059 = v_lam.clone(); if py::lt(&t4058, &t4059)? { { let t4060 = Py::Int(500i64); if py::le(&t4059, &t4060)? { true } else { false } } } else { false } }) } } }, Py::str("the Poisson mean must lie between 0 and 500"))?;
    let _ = f__whole(Py::str("the number of claims k"), v_k.clone(), Py::Int(0i64))?;
    let _ = f__need({ let t4066 = v_k.clone(); Py::Bool({ let t4067 = Py::Int(2000i64); if py::le(&t4066, &t4067)? { true } else { false } }) }, Py::str("k must be at most 2000"))?;
    v_pmf = py::math1("exp", &py::sub(&py::add(&py::neg(&v_lam.clone())?, &py::mul(&v_k.clone(), &py::math1("log", &v_lam.clone())?)?)?, &py::math1("lgamma", &py::add(&v_k.clone(), &Py::Float(1.0))?)?)?)?;
    v_cdf = Py::Float(0.0);
    for t4068 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) {
        v_j = t4068;
        v_cdf = py::iadd(&v_cdf, &py::math1("exp", &py::sub(&py::add(&py::neg(&v_lam.clone())?, &py::mul(&v_j.clone(), &py::math1("log", &v_lam.clone())?)?)?, &py::math1("lgamma", &py::add(&v_j.clone(), &Py::Float(1.0))?)?)?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("pmf"), v_pmf.clone()), (Py::str("cdf"), py::minmax(&[v_cdf.clone(), Py::Float(1.0)], false)?), (Py::str("mean"), v_lam.clone()), (Py::str("var"), v_lam.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_poisson_gamma(mut v_alpha: Py, mut v_theta: Py, mut v_counts: Py) -> R {
    let mut v_Z: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("the gamma prior parameters"), Py::tuple(vec![v_alpha.clone(), v_theta.clone()]))?;
    v_n = py::len(&v_counts.clone())?;
    v_s = Py::Float(0.0);
    for t4113 in py::iter(&(v_counts.clone()))? {
        v_c = t4113;
        let _ = f__need({ let t4119 = f__isnum(v_c.clone())?; if !t4119.truthy() { t4119 } else { { let t4118 = { let t4114 = v_c.clone(); Py::Bool({ let t4115 = Py::Int(0i64); if py::ge(&t4114, &t4115)? { true } else { false } }) }; if !t4118.truthy() { t4118 } else { { let t4116 = v_c.clone(); Py::Bool({ let t4117 = py::int(&v_c.clone())?; if py::eq(&t4116, &t4117) { true } else { false } }) } } } } }, Py::str("the claim counts must be whole numbers"))?;
        v_s = py::iadd(&v_s, &v_c.clone())?;
    }
    v_Z = py::truediv(&py::mul(&v_n.clone(), &v_theta.clone())?, &py::add(&Py::Float(1.0), &py::mul(&v_n.clone(), &v_theta.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("post_mean"), py::truediv(&py::mul(&py::add(&v_alpha.clone(), &v_s.clone())?, &v_theta.clone())?, &py::add(&Py::Float(1.0), &py::mul(&v_n.clone(), &v_theta.clone())?)?)?), (Py::str("Z"), v_Z.clone()), (Py::str("prior_mean"), py::mul(&v_alpha.clone(), &v_theta.clone())?), (Py::str("post_alpha"), py::add(&v_alpha.clone(), &v_s.clone())?), (Py::str("post_theta"), py::truediv(&v_theta.clone(), &py::add(&Py::Float(1.0), &py::mul(&v_n.clone(), &v_theta.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_policy_value_recursion(mut v_V: Py, mut v_P: Py, mut v_e: Py, mut v_i: Py, mut v_q: Py, mut v_S: Py, mut v_E: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_V.clone(), v_P.clone(), v_e.clone()]))?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__need({ let t4168 = f__isnum(v_q.clone())?; if !t4168.truthy() { t4168 } else { { let t4165 = Py::Int(0i64); Py::Bool({ let t4166 = v_q.clone(); if py::le(&t4165, &t4166)? { { let t4167 = Py::Int(1i64); if py::lt(&t4166, &t4167)? { true } else { false } } } else { false } }) } } }, Py::str("q must lie between 0 and 1 (below 1)"))?;
    let _ = f__pos(Py::str("the sum insured S"), Py::tuple(vec![v_S.clone()]))?;
    let _ = f__nonneg(Py::str("the settlement expense E"), Py::tuple(vec![v_E.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("V_next"), py::truediv(&py::sub(&py::mul(&py::sub(&py::add(&v_V.clone(), &v_P.clone())?, &v_e.clone())?, &py::add(&Py::Float(1.0), &v_i.clone())?)?, &py::mul(&v_q.clone(), &py::add(&v_S.clone(), &v_E.clone())?)?)?, &py::sub(&Py::Float(1.0), &v_q.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_premium_principle(mut v_mean: Py, mut v_var: Py, mut v_theta: Py, mut v_principle: Py) -> R {
    let _ = f__need({ let t4215 = v_principle.clone(); Py::Bool({ let t4216 = Py::tuple(vec![Py::str("expected_value"), Py::str("variance"), Py::str("standard_deviation")]); if py::contains(&t4216, &t4215)? { true } else { false } }) }, Py::str("principle: expected_value, variance or standard_deviation"))?;
    let _ = f__nonneg(Py::str("the mean, variance and loading"), Py::tuple(vec![v_mean.clone(), v_var.clone(), v_theta.clone()]))?;
    if ({ let t4223 = v_principle.clone(); Py::Bool({ let t4224 = Py::str("expected_value"); if py::eq(&t4223, &t4224) { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("premium"), py::mul(&py::add(&Py::Float(1.0), &v_theta.clone())?, &v_mean.clone())?), (Py::str("loading"), py::mul(&v_theta.clone(), &v_mean.clone())?)]))?);
    }
    if ({ let t4237 = v_principle.clone(); Py::Bool({ let t4238 = Py::str("variance"); if py::eq(&t4237, &t4238) { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("premium"), py::add(&v_mean.clone(), &py::mul(&v_theta.clone(), &v_var.clone())?)?), (Py::str("loading"), py::mul(&v_theta.clone(), &v_var.clone())?)]))?);
    }
    return Ok(f__out(Py::dict(vec![(Py::str("premium"), py::add(&v_mean.clone(), &py::mul(&v_theta.clone(), &py::math1("sqrt", &v_var.clone())?)?)?), (Py::str("loading"), py::mul(&v_theta.clone(), &py::math1("sqrt", &v_var.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pure_endowment_lx(mut v_lx: Py, mut v_x0: Py, mut v_x: Py, mut v_n: Py, mut v_i: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__lx_check(v_lx.clone(), Py::Bool(false))?;
    let _ = f__rate(v_i.clone())?;
    v_k = f__idx(v_lx.clone(), v_x0.clone(), v_x.clone(), Py::tuple(vec![v_n.clone()]))?;
    v_E = py::truediv(&py::mul(&py::pow(&py::add(&Py::Float(1.0), &v_i.clone())?, &py::neg(&v_n.clone())?)?, &py::getitem(&v_lx.clone(), &py::add(&v_k.clone(), &v_n.clone())?)?)?, &py::getitem(&v_lx.clone(), &v_k.clone())?)?;
    let _ = f__need({ let t4349 = v_E.clone(); Py::Bool({ let t4350 = Py::Int(0i64); if py::gt(&t4349, &t4350)? { true } else { false } }) }, Py::str("nobody survives to age x+n in the table"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("nEx"), v_E.clone()), (Py::str("accumulation"), py::truediv(&Py::Float(1.0), &v_E.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pure_premium_rate(mut v_L: Py, mut v_X: Py, mut v_Fe: Py, mut v_V: Py, mut v_Q: Py) -> R {
    let mut v_perm: Py = Py::None;
    let mut v_pp: Py = Py::None;
    let _ = f__nonneg(Py::str("the losses and the fixed expense per exposure"), Py::tuple(vec![v_L.clone(), v_Fe.clone()]))?;
    let _ = f__pos(Py::str("the exposures"), Py::tuple(vec![v_X.clone()]))?;
    let _ = f__frac(Py::str("the variable expense ratio V"), v_V.clone(), Py::Bool(false), Py::Bool(true))?;
    let _ = f__need({ let t4412 = f__isnum(v_Q.clone())?; if !t4412.truthy() { t4412 } else { { let t4409 = py::neg(&Py::Float(0.5))?; Py::Bool({ let t4410 = v_Q.clone(); if py::lt(&t4409, &t4410)? { { let t4411 = Py::Int(1i64); if py::lt(&t4410, &t4411)? { true } else { false } } } else { false } }) } } }, Py::str("the profit provision Q must lie between -50% and 100%"))?;
    v_perm = py::sub(&py::sub(&Py::Float(1.0), &v_V.clone())?, &v_Q.clone())?;
    let _ = f__need({ let t4413 = v_perm.clone(); Py::Bool({ let t4414 = Py::Int(0i64); if py::gt(&t4413, &t4414)? { true } else { false } }) }, Py::str("the variable expenses and profit leave nothing for losses"))?;
    v_pp = py::truediv(&v_L.clone(), &v_X.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("pure_premium"), v_pp.clone()), (Py::str("rate"), py::truediv(&py::add(&v_pp.clone(), &v_Fe.clone())?, &v_perm.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pv_cash_flows(mut v_amounts: Py, mut v_times: Py, mut v_i: Py) -> R {
    let mut v_av: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_pv: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__rate(v_i.clone())?;
    let _ = f__need({ let t4271 = py::len(&v_amounts.clone())?; Py::Bool({ let t4272 = py::len(&v_times.clone())?; if py::eq(&t4271, &t4272) { true } else { false } }) }, Py::str("give one time for every amount"))?;
    for t4273 in py::iter(&(v_times.clone()))? {
        v_t = t4273;
        let _ = f__need({ let t4277 = f__isnum(v_t.clone())?; if !t4277.truthy() { t4277 } else { { let t4274 = Py::Int(0i64); Py::Bool({ let t4275 = v_t.clone(); if py::le(&t4274, &t4275)? { { let t4276 = Py::Int(200i64); if py::le(&t4275, &t4276)? { true } else { false } } } else { false } }) } } }, Py::str("every time must lie between 0 and 200 years"))?;
    }
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_pv = Py::Float(0.0);
    for t4278 in py::iter(&(py::zip(&[v_amounts.clone(), v_times.clone()])?))? {
        let t4279 = (t4278).items()?;
        if t4279.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_c = t4279[0].clone();
        v_t = t4279[1].clone();
        v_pv = py::iadd(&v_pv, &py::mul(&v_c.clone(), &py::pow(&v_v.clone(), &v_t.clone())?)?)?;
    }
    v_av = py::mul(&v_pv.clone(), &py::pow(&py::add(&Py::Float(1.0), &v_i.clone())?, &py::minmax(&[v_times.clone()], true)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("PV"), v_pv.clone()), (Py::str("AV_at_last"), v_av.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rate_conversion(mut v_i: Py, mut v_m: Py) -> R {
    let mut v_d_m: Py = Py::None;
    let mut v_i_m: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__rate(v_i.clone())?;
    let _ = f__whole(Py::str("the number of conversion periods per year m"), v_m.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t2139 = v_m.clone(); Py::Bool({ let t2140 = Py::Int(365i64); if py::le(&t2139, &t2140)? { true } else { false } }) }, Py::str("m must be at most 365"))?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_i_m = py::mul(&v_m.clone(), &py::sub(&py::pow(&py::add(&Py::Float(1.0), &v_i.clone())?, &py::truediv(&Py::Float(1.0), &v_m.clone())?)?, &Py::Float(1.0))?)?;
    v_d_m = py::mul(&v_m.clone(), &py::sub(&Py::Float(1.0), &py::pow(&v_v.clone(), &py::truediv(&Py::Float(1.0), &v_m.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("d"), py::mul(&v_i.clone(), &v_v.clone())?), (Py::str("v"), v_v.clone()), (Py::str("delta"), py::math1("log1p", &v_i.clone())?), (Py::str("i_m"), v_i_m.clone()), (Py::str("d_m"), v_d_m.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reserve_endowment(mut v_q: Py, mut v_i: Py, mut v_n: Py, mut v_S: Py, mut v_t: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_qt: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__q_check(v_q.clone(), Py::Bool(false), v_n.clone())?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__pos(Py::str("the sum insured S"), Py::tuple(vec![v_S.clone()]))?;
    let _ = f__whole(Py::str("the policy duration t"), v_t.clone(), Py::Int(0i64))?;
    let _ = f__need({ let t4480 = v_t.clone(); Py::Bool({ let t4481 = v_n.clone(); if py::le(&t4480, &t4481)? { true } else { false } }) }, Py::str("the duration t must lie between 0 and n"))?;
    if ({ let t4482 = v_t.clone(); Py::Bool({ let t4483 = v_n.clone(); if py::eq(&t4482, &t4483) { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("V"), v_S.clone()), (Py::str("P"), py::truediv(&py::mul(&v_S.clone(), &py::add(&f__ins(v_q.clone(), py::truediv(&Py::Int(1i64), &py::add(&Py::Int(1i64), &v_i.clone())?)?, v_n.clone())?, &f__pe(v_q.clone(), py::truediv(&Py::Int(1i64), &py::add(&Py::Int(1i64), &v_i.clone())?)?, v_n.clone())?)?)?, &f__ann(v_q.clone(), py::truediv(&Py::Int(1i64), &py::add(&Py::Int(1i64), &v_i.clone())?)?, v_n.clone())?)?)]))?);
    }
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_P = py::truediv(&py::mul(&v_S.clone(), &py::add(&f__ins(v_q.clone(), v_v.clone(), v_n.clone())?, &f__pe(v_q.clone(), v_v.clone(), v_n.clone())?)?)?, &f__ann(v_q.clone(), v_v.clone(), v_n.clone())?)?;
    v_qt = py::slice(&v_q.clone(), Some((v_t.clone()).i()?), Some((v_n.clone()).i()?), None)?;
    v_m = py::sub(&v_n.clone(), &v_t.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), py::sub(&py::mul(&v_S.clone(), &py::add(&f__ins(v_qt.clone(), v_v.clone(), v_m.clone())?, &f__pe(v_qt.clone(), v_v.clone(), v_m.clone())?)?)?, &py::mul(&v_P.clone(), &f__ann(v_qt.clone(), v_v.clone(), v_m.clone())?)?)?), (Py::str("P"), v_P.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reserve_term(mut v_q: Py, mut v_i: Py, mut v_n: Py, mut v_S: Py, mut v_t: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_qt: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__q_check(v_q.clone(), Py::Bool(false), v_n.clone())?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__pos(Py::str("the sum insured S"), Py::tuple(vec![v_S.clone()]))?;
    let _ = f__whole(Py::str("the policy duration t"), v_t.clone(), Py::Int(0i64))?;
    let _ = f__need({ let t4552 = v_t.clone(); Py::Bool({ let t4553 = v_n.clone(); if py::le(&t4552, &t4553)? { true } else { false } }) }, Py::str("the duration t must lie between 0 and n"))?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_P = py::truediv(&py::mul(&v_S.clone(), &f__ins(v_q.clone(), v_v.clone(), v_n.clone())?)?, &f__ann(v_q.clone(), v_v.clone(), v_n.clone())?)?;
    if ({ let t4557 = v_t.clone(); Py::Bool({ let t4558 = v_n.clone(); if py::eq(&t4557, &t4558) { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("V"), Py::Float(0.0)), (Py::str("P"), v_P.clone())]))?);
    }
    v_qt = py::slice(&v_q.clone(), Some((v_t.clone()).i()?), Some((v_n.clone()).i()?), None)?;
    v_m = py::sub(&v_n.clone(), &v_t.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), py::sub(&py::mul(&v_S.clone(), &f__ins(v_qt.clone(), v_v.clone(), v_m.clone())?)?, &py::mul(&v_P.clone(), &f__ann(v_qt.clone(), v_v.clone(), v_m.clone())?)?)?), (Py::str("P"), v_P.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reserve_whole_life(mut v_q: Py, mut v_i: Py, mut v_S: Py, mut v_t: Py) -> R {
    let mut v_At: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_a0: Py = Py::None;
    let mut v_at: Py = Py::None;
    let mut v_qt: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__q_check(v_q.clone(), Py::Bool(true), Py::None)?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__pos(Py::str("the sum insured S"), Py::tuple(vec![v_S.clone()]))?;
    let _ = f__whole(Py::str("the policy duration t"), v_t.clone(), Py::Int(0i64))?;
    let _ = f__need({ let t4624 = v_t.clone(); Py::Bool({ let t4625 = py::len(&v_q.clone())?; if py::lt(&t4624, &t4625)? { true } else { false } }) }, Py::str("the duration t must lie inside the table"))?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_a0 = f__ann(v_q.clone(), v_v.clone(), py::len(&v_q.clone())?)?;
    v_P = py::truediv(&py::mul(&v_S.clone(), &f__ins(v_q.clone(), v_v.clone(), py::len(&v_q.clone())?)?)?, &v_a0.clone())?;
    v_qt = py::slice(&v_q.clone(), Some((v_t.clone()).i()?), None, None)?;
    v_At = f__ins(v_qt.clone(), v_v.clone(), py::len(&v_qt.clone())?)?;
    v_at = f__ann(v_qt.clone(), v_v.clone(), py::len(&v_qt.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), py::sub(&py::mul(&v_S.clone(), &v_At.clone())?, &py::mul(&v_P.clone(), &v_at.clone())?)?), (Py::str("P"), v_P.clone()), (Py::str("A_t"), v_At.clone()), (Py::str("a_due_t"), v_at.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_retrospective_reserve(mut v_q: Py, mut v_i: Py, mut v_S: Py, mut v_P: Py, mut v_t: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__q_check(v_q.clone(), Py::Bool(false), Py::None)?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__pos(Py::str("the sum insured S and the premium P"), Py::tuple(vec![v_S.clone(), v_P.clone()]))?;
    let _ = f__whole(Py::str("the policy duration t"), v_t.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t4694 = v_t.clone(); Py::Bool({ let t4695 = py::len(&v_q.clone())?; if py::le(&t4694, &t4695)? { true } else { false } }) }, Py::str("the table must cover the first t years"))?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_E = f__pe(v_q.clone(), v_v.clone(), v_t.clone())?;
    let _ = f__need({ let t4697 = v_E.clone(); Py::Bool({ let t4698 = Py::Int(0i64); if py::gt(&t4697, &t4698)? { true } else { false } }) }, Py::str("nobody survives t years in the table"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), py::truediv(&py::sub(&py::mul(&v_P.clone(), &f__ann(v_q.clone(), v_v.clone(), v_t.clone())?)?, &py::mul(&v_S.clone(), &f__ins(v_q.clone(), v_v.clone(), v_t.clone())?)?)?, &v_E.clone())?), (Py::str("tEx"), v_E.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reversionary_annuity(mut v_qx: Py, mut v_qy: Py, mut v_i: Py, mut v_P: Py) -> R {
    let mut v_axy: Py = Py::None;
    let mut v_ay: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_px: Py = Py::None;
    let mut v_py: Py = Py::None;
    let mut v_sx: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__q_check(v_qx.clone(), Py::Bool(true), Py::None)?;
    let _ = f__q_check(v_qy.clone(), Py::Bool(true), Py::None)?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__pos(Py::str("the annual payment P"), Py::tuple(vec![v_P.clone()]))?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    let t4767 = (Py::tuple(vec![f__kp(v_qx.clone())?, f__kp(v_qy.clone())?])).items()?;
    if t4767.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_px = t4767[0].clone();
    v_py = t4767[1].clone();
    let t4768 = Py::Float(0.0);
    v_ay = t4768.clone();
    v_axy = t4768.clone();
    for t4769 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_py.clone())?).i()?]) {
        v_k = t4769;
        v_sx = (if ({ let t4770 = v_k.clone(); Py::Bool({ let t4771 = py::len(&v_px.clone())?; if py::lt(&t4770, &t4771)? { true } else { false } }) }).truthy() { py::getitem(&v_px.clone(), &v_k.clone())? } else { Py::Float(0.0) });
        v_ay = py::iadd(&v_ay, &py::mul(&py::pow(&v_v.clone(), &v_k.clone())?, &py::getitem(&v_py.clone(), &v_k.clone())?)?)?;
        v_axy = py::iadd(&v_axy, &py::mul(&py::mul(&py::pow(&v_v.clone(), &v_k.clone())?, &v_sx.clone())?, &py::getitem(&v_py.clone(), &v_k.clone())?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("a_rev"), py::sub(&v_ay.clone(), &v_axy.clone())?), (Py::str("EPV"), py::mul(&v_P.clone(), &py::sub(&v_ay.clone(), &v_axy.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_single_loss_payment(mut v_x: Py, mut v_d: Py, mut v_u: Py, mut v_c: Py) -> R {
    let mut v_covered: Py = Py::None;
    let mut v_pay: Py = Py::None;
    let _ = f__nonneg(Py::str("the loss x and the deductible d"), Py::tuple(vec![v_x.clone(), v_d.clone()]))?;
    let _ = f__need({ let t4818 = f__isnum(v_u.clone())?; if !t4818.truthy() { t4818 } else { { let t4816 = v_u.clone(); Py::Bool({ let t4817 = v_d.clone(); if py::gt(&t4816, &t4817)? { true } else { false } }) } } }, Py::str("the policy limit u must exceed the deductible d"))?;
    let _ = f__need({ let t4822 = f__isnum(v_c.clone())?; if !t4822.truthy() { t4822 } else { { let t4819 = Py::Int(0i64); Py::Bool({ let t4820 = v_c.clone(); if py::lt(&t4819, &t4820)? { { let t4821 = Py::Int(1i64); if py::le(&t4820, &t4821)? { true } else { false } } } else { false } }) } } }, Py::str("the coinsurance c must lie between 0 and 100%"))?;
    v_covered = py::minmax(&[v_x.clone(), v_u.clone()], false)?;
    v_pay = py::mul(&v_c.clone(), &py::minmax(&[py::sub(&v_covered.clone(), &v_d.clone())?, Py::Float(0.0)], true)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("payment"), v_pay.clone()), (Py::str("retained"), py::sub(&v_x.clone(), &v_pay.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_single_to_multiple(mut v_q1s: Py, mut v_q2s: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let _ = f__frac(Py::str("q'(1)"), v_q1s.clone(), Py::Bool(true), Py::Bool(true))?;
    let _ = f__frac(Py::str("q'(2)"), v_q2s.clone(), Py::Bool(true), Py::Bool(true))?;
    v_a = py::mul(&v_q1s.clone(), &py::sub(&Py::Float(1.0), &py::truediv(&v_q2s.clone(), &Py::Float(2.0))?)?)?;
    v_b = py::mul(&v_q2s.clone(), &py::sub(&Py::Float(1.0), &py::truediv(&v_q1s.clone(), &Py::Float(2.0))?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("q1"), v_a.clone()), (Py::str("q2"), v_b.clone()), (Py::str("q_total"), py::add(&v_a.clone(), &v_b.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stop_loss_compound_poisson(mut v_lam: Py, mut v_f: Py, mut v_d: Py) -> R {
    let mut v_ES: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_lim: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__need({ let t4916 = f__isnum(v_lam.clone())?; if !t4916.truthy() { t4916 } else { { let t4913 = Py::Int(0i64); Py::Bool({ let t4914 = v_lam.clone(); if py::lt(&t4913, &t4914)? { { let t4915 = Py::Int(200i64); if py::le(&t4914, &t4915)? { true } else { false } } } else { false } }) } } }, Py::str("lambda must lie between 0 and 200"))?;
    let _ = f__sev_check(v_f.clone())?;
    let _ = f__whole(Py::str("the retention d"), v_d.clone(), Py::Int(0i64))?;
    let _ = f__need({ let t4930 = v_d.clone(); Py::Bool({ let t4931 = Py::Int(300i64); if py::le(&t4930, &t4931)? { true } else { false } }) }, Py::str("the retention must be at most 300 units"))?;
    v_ES = Py::Float(0.0);
    for t4932 in py::range_iter(&[(py::len(&v_f.clone())?).i()?]) {
        v_j = t4932;
        v_ES = py::iadd(&v_ES, &py::mul(&v_j.clone(), &py::getitem(&v_f.clone(), &v_j.clone())?)?)?;
    }
    v_ES = py::mul(&v_ES, &v_lam.clone())?;
    if ({ let t4933 = v_d.clone(); Py::Bool({ let t4934 = Py::Int(0i64); if py::eq(&t4933, &t4934) { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("premium"), v_ES.clone()), (Py::str("ES"), v_ES.clone()), (Py::str("limited"), Py::Float(0.0))]))?);
    }
    v_g = f__panjer(Py::Float(0.0), v_lam.clone(), py::math1("exp", &py::mul(&v_lam.clone(), &py::sub(&py::getitem(&v_f.clone(), &Py::Int(0i64))?, &Py::Float(1.0))?)?)?, v_f.clone(), py::sub(&v_d.clone(), &Py::Int(1i64))?)?;
    v_F = Py::Float(0.0);
    v_lim = Py::Float(0.0);
    for t4949 in py::range_iter(&[(v_d.clone()).i()?]) {
        v_s = t4949;
        v_F = py::iadd(&v_F, &py::getitem(&v_g.clone(), &v_s.clone())?)?;
        v_lim = py::iadd(&v_lim, &py::sub(&Py::Float(1.0), &v_F.clone())?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("premium"), py::sub(&v_ES.clone(), &v_lim.clone())?), (Py::str("ES"), v_ES.clone()), (Py::str("limited"), v_lim.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_survival_probability(mut v_lx: Py, mut v_x0: Py, mut v_x: Py, mut v_t: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_p: Py = Py::None;
    let _ = f__lx_check(v_lx.clone(), Py::Bool(false))?;
    v_k = f__idx(v_lx.clone(), v_x0.clone(), v_x.clone(), Py::tuple(vec![v_t.clone()]))?;
    v_p = py::truediv(&py::getitem(&v_lx.clone(), &py::add(&v_k.clone(), &v_t.clone())?)?, &py::getitem(&v_lx.clone(), &v_k.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("tpx"), v_p.clone()), (Py::str("tqx"), py::truediv(&py::sub(&py::getitem(&v_lx.clone(), &v_k.clone())?, &py::getitem(&v_lx.clone(), &py::add(&v_k.clone(), &v_t.clone())?)?)?, &py::getitem(&v_lx.clone(), &v_k.clone())?)?), (Py::str("deaths"), py::sub(&py::getitem(&v_lx.clone(), &v_k.clone())?, &py::getitem(&v_lx.clone(), &py::add(&v_k.clone(), &v_t.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_temporary_annuity(mut v_q: Py, mut v_i: Py, mut v_n: Py, mut v_P: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_imm: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__q_check(v_q.clone(), Py::Bool(false), v_n.clone())?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__pos(Py::str("the annual payment P"), Py::tuple(vec![v_P.clone()]))?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_a = f__ann(v_q.clone(), v_v.clone(), v_n.clone())?;
    v_imm = py::add(&py::sub(&v_a.clone(), &Py::Float(1.0))?, &f__pe(v_q.clone(), v_v.clone(), v_n.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("a_due"), v_a.clone()), (Py::str("a_imm"), v_imm.clone()), (Py::str("EPV"), py::mul(&v_P.clone(), &v_a.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_temporary_expectation(mut v_lx: Py, mut v_x0: Py, mut v_x: Py, mut v_n: Py) -> R {
    let mut v_comp: Py = Py::None;
    let mut v_cur: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__lx_check(v_lx.clone(), Py::Bool(false))?;
    let _ = f__need({ let t5123 = v_n.clone(); Py::Bool({ let t5124 = Py::Int(1i64); if py::ge(&t5123, &t5124)? { true } else { false } }) }, Py::str("the term n must be at least one year"))?;
    v_k = f__idx(v_lx.clone(), v_x0.clone(), v_x.clone(), Py::tuple(vec![v_n.clone()]))?;
    let t5137 = Py::Float(0.0);
    v_cur = t5137.clone();
    v_comp = t5137.clone();
    for t5138 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_j = t5138;
        v_cur = py::iadd(&v_cur, &py::getitem(&v_lx.clone(), &py::add(&v_k.clone(), &v_j.clone())?)?)?;
    }
    for t5139 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_j = t5139;
        v_comp = py::iadd(&v_comp, &py::truediv(&py::add(&py::getitem(&v_lx.clone(), &py::add(&v_k.clone(), &v_j.clone())?)?, &py::getitem(&v_lx.clone(), &py::add(&py::add(&v_k.clone(), &v_j.clone())?, &Py::Int(1i64))?)?)?, &Py::Float(2.0))?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("e_curtate"), py::truediv(&v_cur.clone(), &py::getitem(&v_lx.clone(), &v_k.clone())?)?), (Py::str("e_complete"), py::truediv(&v_comp.clone(), &py::getitem(&v_lx.clone(), &v_k.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_term_insurance(mut v_q: Py, mut v_i: Py, mut v_n: Py, mut v_S: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_A2: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__q_check(v_q.clone(), Py::Bool(false), v_n.clone())?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__pos(Py::str("the sum insured S"), Py::tuple(vec![v_S.clone()]))?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_A = f__ins(v_q.clone(), v_v.clone(), v_n.clone())?;
    v_A2 = f__ins(v_q.clone(), py::mul(&v_v.clone(), &v_v.clone())?, v_n.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("A1"), v_A.clone()), (Py::str("A1_2"), v_A2.clone()), (Py::str("EPV"), py::mul(&v_S.clone(), &v_A.clone())?), (Py::str("sd"), py::mul(&v_S.clone(), &py::math1("sqrt", &py::minmax(&[py::sub(&v_A2.clone(), &py::mul(&v_A.clone(), &v_A.clone())?)?, Py::Float(0.0)], true)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thiele_endowment(mut v_mu: Py, mut v_delta: Py, mut v_n: Py, mut v_S: Py, mut v_t: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_At: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_at: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let _ = f__need({ let t5250 = f__isnum(v_mu.clone())?; if !t5250.truthy() { t5250 } else { { let t5247 = Py::Int(0i64); Py::Bool({ let t5248 = v_mu.clone(); if py::lt(&t5247, &t5248)? { { let t5249 = Py::Int(1i64); if py::lt(&t5248, &t5249)? { true } else { false } } } else { false } }) } } }, Py::str("the force of mortality must lie between 0 and 1"))?;
    let _ = f__need({ let t5254 = f__isnum(v_delta.clone())?; if !t5254.truthy() { t5254 } else { { let t5251 = Py::Int(0i64); Py::Bool({ let t5252 = v_delta.clone(); if py::lt(&t5251, &t5252)? { { let t5253 = Py::Int(1i64); if py::lt(&t5252, &t5253)? { true } else { false } } } else { false } }) } } }, Py::str("the force of interest must lie between 0 and 1"))?;
    let _ = f__need({ let t5258 = f__isnum(v_n.clone())?; if !t5258.truthy() { t5258 } else { { let t5255 = Py::Int(0i64); Py::Bool({ let t5256 = v_n.clone(); if py::lt(&t5255, &t5256)? { { let t5257 = Py::Int(100i64); if py::le(&t5256, &t5257)? { true } else { false } } } else { false } }) } } }, Py::str("the term n must lie between 0 and 100 years"))?;
    let _ = f__pos(Py::str("the sum insured S"), Py::tuple(vec![v_S.clone()]))?;
    let _ = f__need({ let t5266 = f__isnum(v_t.clone())?; if !t5266.truthy() { t5266 } else { { let t5263 = Py::Int(0i64); Py::Bool({ let t5264 = v_t.clone(); if py::le(&t5263, &t5264)? { { let t5265 = v_n.clone(); if py::le(&t5264, &t5265)? { true } else { false } } } else { false } }) } } }, Py::str("t must lie between 0 and n"))?;
    v_k = py::add(&v_mu.clone(), &v_delta.clone())?;
    v_vals = { let v_S = v_S.clone(); let v_delta = v_delta.clone(); let v_k = v_k.clone(); let v_mu = v_mu.clone(); let v_n = v_n.clone(); let v_t = v_t.clone(); let v_vals = v_vals.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_m: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_abar: Py = Py::None;
            let mut v_e: Py = Py::None;
            v_e = py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_m.clone())?)?;
            v_abar = py::truediv(&py::sub(&Py::Float(1.0), &v_e.clone())?, &v_k.clone())?;
            return Ok(Py::tuple(vec![py::add(&py::mul(&v_mu.clone(), &v_abar.clone())?, &v_e.clone())?, v_abar.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t5267 = (py::call(&v_vals.clone(), &[v_n.clone()])?).items()?;
    if t5267.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t5267[0].clone();
    v_a = t5267[1].clone();
    v_P = py::truediv(&py::mul(&v_S.clone(), &v_A.clone())?, &v_a.clone())?;
    if ({ let t5268 = v_t.clone(); Py::Bool({ let t5269 = v_n.clone(); if py::eq(&t5268, &t5269) { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("A_bar"), v_A.clone()), (Py::str("a_bar"), v_a.clone()), (Py::str("V"), v_S.clone())]))?);
    }
    let t5282 = (py::call(&v_vals.clone(), &[py::sub(&v_n.clone(), &v_t.clone())?])?).items()?;
    if t5282.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_At = t5282[0].clone();
    v_at = t5282[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("A_bar"), v_A.clone()), (Py::str("a_bar"), v_a.clone()), (Py::str("V"), py::sub(&py::mul(&v_S.clone(), &v_At.clone())?, &py::mul(&v_P.clone(), &v_at.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_var_tvar_exponential(mut v_theta: Py, mut v_p: Py) -> R {
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::str("the mean theta"), Py::tuple(vec![v_theta.clone()]))?;
    let _ = f__p_check(v_p.clone())?;
    v_v = py::mul(&py::neg(&v_theta.clone())?, &py::math1("log1p", &py::neg(&v_p.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("VaR"), v_v.clone()), (Py::str("TVaR"), py::add(&v_v.clone(), &v_theta.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_var_tvar_lognormal(mut v_mu: Py, mut v_sigma: Py, mut v_p: Py) -> R {
    let mut v_z: Py = Py::None;
    let _ = f__need({ let t5362 = f__isnum(v_mu.clone())?; if !t5362.truthy() { t5362 } else { { let t5359 = py::neg(&Py::Int(20i64))?; Py::Bool({ let t5360 = v_mu.clone(); if py::lt(&t5359, &t5360)? { { let t5361 = Py::Int(30i64); if py::lt(&t5360, &t5361)? { true } else { false } } } else { false } }) } } }, Py::str("mu must lie between -20 and 30"))?;
    let _ = f__need({ let t5366 = f__isnum(v_sigma.clone())?; if !t5366.truthy() { t5366 } else { { let t5363 = Py::Int(0i64); Py::Bool({ let t5364 = v_sigma.clone(); if py::lt(&t5363, &t5364)? { { let t5365 = Py::Int(5i64); if py::le(&t5364, &t5365)? { true } else { false } } } else { false } }) } } }, Py::str("sigma must lie between 0 and 5"))?;
    let _ = f__p_check(v_p.clone())?;
    v_z = f__norm_ppf(v_p.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("VaR"), py::math1("exp", &py::add(&v_mu.clone(), &py::mul(&v_sigma.clone(), &v_z.clone())?)?)?), (Py::str("TVaR"), py::truediv(&py::mul(&py::math1("exp", &py::add(&v_mu.clone(), &py::truediv(&py::mul(&v_sigma.clone(), &v_sigma.clone())?, &Py::Float(2.0))?)?)?, &f__norm_cdf(py::sub(&v_sigma.clone(), &v_z.clone())?)?)?, &py::sub(&Py::Float(1.0), &v_p.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_var_tvar_normal(mut v_mu: Py, mut v_sigma: Py, mut v_p: Py) -> R {
    let mut v_z: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_mu.clone()]))?;
    let _ = f__pos(Py::str("sigma"), Py::tuple(vec![v_sigma.clone()]))?;
    let _ = f__p_check(v_p.clone())?;
    v_z = f__norm_ppf(v_p.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("VaR"), py::add(&v_mu.clone(), &py::mul(&v_sigma.clone(), &v_z.clone())?)?), (Py::str("TVaR"), py::add(&v_mu.clone(), &py::truediv(&py::mul(&v_sigma.clone(), &f__norm_pdf(v_z.clone())?)?, &py::sub(&Py::Float(1.0), &v_p.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_var_tvar_pareto(mut v_alpha: Py, mut v_theta: Py, mut v_p: Py) -> R {
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t5485 = f__isnum(v_alpha.clone())?; if !t5485.truthy() { t5485 } else { { let t5482 = Py::Int(1i64); Py::Bool({ let t5483 = v_alpha.clone(); if py::lt(&t5482, &t5483)? { { let t5484 = Py::Int(100i64); if py::le(&t5483, &t5484)? { true } else { false } } } else { false } }) } } }, Py::str("alpha must lie between 1 and 100 (a finite mean)"))?;
    let _ = f__pos(Py::str("theta"), Py::tuple(vec![v_theta.clone()]))?;
    let _ = f__p_check(v_p.clone())?;
    v_v = py::mul(&v_theta.clone(), &py::sub(&py::pow(&py::sub(&Py::Float(1.0), &v_p.clone())?, &py::truediv(&py::neg(&Py::Float(1.0))?, &v_alpha.clone())?)?, &Py::Float(1.0))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("VaR"), v_v.clone()), (Py::str("TVaR"), py::add(&v_v.clone(), &py::truediv(&py::add(&v_theta.clone(), &v_v.clone())?, &py::sub(&v_alpha.clone(), &Py::Float(1.0))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_whole_life_annuity(mut v_q: Py, mut v_i: Py, mut v_P: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_A2: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__q_check(v_q.clone(), Py::Bool(true), Py::None)?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__pos(Py::str("the annual payment P"), Py::tuple(vec![v_P.clone()]))?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_a = f__ann(v_q.clone(), v_v.clone(), py::len(&v_q.clone())?)?;
    v_A = f__ins(v_q.clone(), v_v.clone(), py::len(&v_q.clone())?)?;
    v_A2 = f__ins(v_q.clone(), py::mul(&v_v.clone(), &v_v.clone())?, py::len(&v_q.clone())?)?;
    v_d = py::mul(&v_i.clone(), &v_v.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("a_due"), v_a.clone()), (Py::str("a_imm"), py::sub(&v_a.clone(), &Py::Float(1.0))?), (Py::str("EPV"), py::mul(&v_P.clone(), &v_a.clone())?), (Py::str("sd"), py::truediv(&py::mul(&v_P.clone(), &py::math1("sqrt", &py::minmax(&[py::sub(&v_A2.clone(), &py::mul(&v_A.clone(), &v_A.clone())?)?, Py::Float(0.0)], true)?)?)?, &v_d.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_whole_life_insurance(mut v_q: Py, mut v_i: Py, mut v_S: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_A2: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_var: Py = Py::None;
    let _ = f__q_check(v_q.clone(), Py::Bool(true), Py::None)?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__pos(Py::str("the sum insured S"), Py::tuple(vec![v_S.clone()]))?;
    v_v = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_i.clone())?)?;
    v_A = f__ins(v_q.clone(), v_v.clone(), py::len(&v_q.clone())?)?;
    v_A2 = f__ins(v_q.clone(), py::mul(&v_v.clone(), &v_v.clone())?, py::len(&v_q.clone())?)?;
    v_var = py::minmax(&[py::sub(&v_A2.clone(), &py::mul(&v_A.clone(), &v_A.clone())?)?, Py::Float(0.0)], true)?;
    return Ok(f__out(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("A2"), v_A2.clone()), (Py::str("EPV"), py::mul(&v_S.clone(), &v_A.clone())?), (Py::str("sd"), py::mul(&v_S.clone(), &py::math1("sqrt", &v_var.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_woolhouse(mut v_a_due: Py, mut v_m: Py, mut v_i: Py, mut v_mu: Py) -> R {
    let mut v_three: Py = Py::None;
    let mut v_two: Py = Py::None;
    let _ = f__need({ let t5671 = f__isnum(v_a_due.clone())?; if !t5671.truthy() { t5671 } else { { let t5669 = v_a_due.clone(); Py::Bool({ let t5670 = Py::Int(1i64); if py::ge(&t5669, &t5670)? { true } else { false } }) } } }, Py::str("the annual annuity-due must be at least 1"))?;
    let _ = f__whole(Py::str("the number of payments per year m"), v_m.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t5676 = v_m.clone(); Py::Bool({ let t5677 = Py::Int(365i64); if py::le(&t5676, &t5677)? { true } else { false } }) }, Py::str("m must be at most 365"))?;
    let _ = f__rate(v_i.clone())?;
    let _ = f__need({ let t5685 = f__isnum(v_mu.clone())?; if !t5685.truthy() { t5685 } else { { let t5682 = Py::Int(0i64); Py::Bool({ let t5683 = v_mu.clone(); if py::le(&t5682, &t5683)? { { let t5684 = Py::Int(5i64); if py::lt(&t5683, &t5684)? { true } else { false } } } else { false } }) } } }, Py::str("the force of mortality must lie between 0 and 5"))?;
    v_two = py::sub(&v_a_due.clone(), &py::truediv(&py::sub(&v_m.clone(), &Py::Float(1.0))?, &py::mul(&Py::Float(2.0), &v_m.clone())?)?)?;
    v_three = py::sub(&v_two.clone(), &py::mul(&py::truediv(&py::sub(&py::mul(&v_m.clone(), &v_m.clone())?, &Py::Float(1.0))?, &py::mul(&py::mul(&Py::Float(12.0), &v_m.clone())?, &v_m.clone())?)?, &py::add(&py::math1("log1p", &v_i.clone())?, &v_mu.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("a_m3"), v_three.clone()), (Py::str("a_m2"), v_two.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1003(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1005: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1004 = v_inputs.clone(); f_continuous_insurance_udd(py::getitem(&t1004, &Py::str("q"))?, py::getitem(&t1004, &Py::str("i"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1005 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1006 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1028 = (t1006).items()?;
        if t1028.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1028[0].clone();
        v_v = t1028[1].clone();
        if ({ let t1007 = v_k.clone(); Py::Bool({ let t1008 = v_outputs.clone(); if !py::contains(&t1008, &t1007)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1055(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1057: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1056 = v_inputs.clone(); f_de_moivre(py::getitem(&t1056, &Py::str("omega"))?, py::getitem(&t1056, &Py::str("x"))?, py::getitem(&t1056, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1057 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1058 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1080 = (t1058).items()?;
        if t1080.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1080[0].clone();
        v_v = t1080[1].clone();
        if ({ let t1059 = v_k.clone(); Py::Bool({ let t1060 = v_outputs.clone(); if !py::contains(&t1060, &t1059)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1124(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1126: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1125 = v_inputs.clone(); f_deferred_annuity(py::getitem(&t1125, &Py::str("q"))?, py::getitem(&t1125, &Py::str("i"))?, py::getitem(&t1125, &Py::str("u"))?, py::getitem(&t1125, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1126 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1127 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1149 = (t1127).items()?;
        if t1149.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1149[0].clone();
        v_v = t1149[1].clone();
        if ({ let t1128 = v_k.clone(); Py::Bool({ let t1129 = v_outputs.clone(); if !py::contains(&t1129, &t1128)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1191(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1193: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1192 = v_inputs.clone(); f_deferred_death_probability(py::getitem(&t1192, &Py::str("lx"))?, py::getitem(&t1192, &Py::str("x0"))?, py::getitem(&t1192, &Py::str("x"))?, py::getitem(&t1192, &Py::str("u"))?, py::getitem(&t1192, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1193 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1194 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1216 = (t1194).items()?;
        if t1216.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1216[0].clone();
        v_v = t1216[1].clone();
        if ({ let t1195 = v_k.clone(); Py::Bool({ let t1196 = v_outputs.clone(); if !py::contains(&t1196, &t1195)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1260(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1262: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1261 = v_inputs.clone(); f_deferred_insurance(py::getitem(&t1261, &Py::str("q"))?, py::getitem(&t1261, &Py::str("i"))?, py::getitem(&t1261, &Py::str("u"))?, py::getitem(&t1261, &Py::str("S"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1262 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1263 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1285 = (t1263).items()?;
        if t1285.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1285[0].clone();
        v_v = t1285[1].clone();
        if ({ let t1264 = v_k.clone(); Py::Bool({ let t1265 = v_outputs.clone(); if !py::contains(&t1265, &t1264)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1320(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1322: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1321 = v_inputs.clone(); f_duration_convexity(py::getitem(&t1321, &Py::str("amounts"))?, py::getitem(&t1321, &Py::str("times"))?, py::getitem(&t1321, &Py::str("i"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1322 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1323 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1345 = (t1323).items()?;
        if t1345.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1345[0].clone();
        v_v = t1345[1].clone();
        if ({ let t1324 = v_k.clone(); Py::Bool({ let t1325 = v_outputs.clone(); if !py::contains(&t1325, &t1324)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1387(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1389: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1388 = v_inputs.clone(); f_endowment_insurance(py::getitem(&t1388, &Py::str("q"))?, py::getitem(&t1388, &Py::str("i"))?, py::getitem(&t1388, &Py::str("n"))?, py::getitem(&t1388, &Py::str("S"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1389 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1390 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1412 = (t1390).items()?;
        if t1412.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1412[0].clone();
        v_v = t1412[1].clone();
        if ({ let t1391 = v_k.clone(); Py::Bool({ let t1392 = v_outputs.clone(); if !py::contains(&t1392, &t1391)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1445(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1447: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1446 = v_inputs.clone(); f_expected_claims(py::getitem(&t1446, &Py::str("P"))?, py::getitem(&t1446, &Py::str("elr"))?, py::getitem(&t1446, &Py::str("paid"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1447 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1448 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1470 = (t1448).items()?;
        if t1470.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1470[0].clone();
        v_v = t1470[1].clone();
        if ({ let t1449 = v_k.clone(); Py::Bool({ let t1450 = v_outputs.clone(); if !py::contains(&t1450, &t1449)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1526(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1528: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1527 = v_inputs.clone(); f_exponential_coverage(py::getitem(&t1527, &Py::str("theta"))?, py::getitem(&t1527, &Py::str("d"))?, py::getitem(&t1527, &Py::str("u"))?, py::getitem(&t1527, &Py::str("c"))?, py::getitem(&t1527, &Py::str("r"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1528 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1529 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1551 = (t1529).items()?;
        if t1551.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1551[0].clone();
        v_v = t1551[1].clone();
        if ({ let t1530 = v_k.clone(); Py::Bool({ let t1531 = v_outputs.clone(); if !py::contains(&t1531, &t1530)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1596(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1598: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1597 = v_inputs.clone(); f_exponential_deductible(py::getitem(&t1597, &Py::str("theta"))?, py::getitem(&t1597, &Py::str("d"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1598 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1599 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1621 = (t1599).items()?;
        if t1621.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1621[0].clone();
        v_v = t1621[1].clone();
        if ({ let t1600 = v_k.clone(); Py::Bool({ let t1601 = v_outputs.clone(); if !py::contains(&t1601, &t1600)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1658(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1660: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1659 = v_inputs.clone(); f_exponential_mle(py::getitem(&t1659, &Py::str("times"))?, py::getitem(&t1659, &Py::str("events"))?, py::getitem(&t1659, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1660 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1661 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1683 = (t1661).items()?;
        if t1683.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1683[0].clone();
        v_v = t1683[1].clone();
        if ({ let t1662 = v_k.clone(); Py::Bool({ let t1663 = v_outputs.clone(); if !py::contains(&t1663, &t1662)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1702(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1704: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1703 = v_inputs.clone(); f_force_of_interest(py::getitem(&t1703, &Py::str("delta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1704 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1705 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1727 = (t1705).items()?;
        if t1727.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1727[0].clone();
        v_v = t1727[1].clone();
        if ({ let t1706 = v_k.clone(); Py::Bool({ let t1707 = v_outputs.clone(); if !py::contains(&t1707, &t1706)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1756(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1758: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1757 = v_inputs.clone(); f_fractional_age(py::getitem(&t1757, &Py::str("q"))?, py::getitem(&t1757, &Py::str("s"))?, py::getitem(&t1757, &Py::str("assumption"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1758 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1759 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1781 = (t1759).items()?;
        if t1781.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1781[0].clone();
        v_v = t1781[1].clone();
        if ({ let t1760 = v_k.clone(); Py::Bool({ let t1761 = v_outputs.clone(); if !py::contains(&t1761, &t1760)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1824(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1826: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1825 = v_inputs.clone(); f_gompertz(py::getitem(&t1825, &Py::str("B"))?, py::getitem(&t1825, &Py::str("c"))?, py::getitem(&t1825, &Py::str("x"))?, py::getitem(&t1825, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1826 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1827 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1849 = (t1827).items()?;
        if t1849.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1849[0].clone();
        v_v = t1849[1].clone();
        if ({ let t1828 = v_k.clone(); Py::Bool({ let t1829 = v_outputs.clone(); if !py::contains(&t1829, &t1828)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1909(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1911: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1910 = v_inputs.clone(); f_gross_premium(py::getitem(&t1910, &Py::str("q"))?, py::getitem(&t1910, &Py::str("i"))?, py::getitem(&t1910, &Py::str("S"))?, py::getitem(&t1910, &Py::str("e0"))?, py::getitem(&t1910, &Py::str("r"))?, py::getitem(&t1910, &Py::str("c"))?, py::getitem(&t1910, &Py::str("E"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1911 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1912 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1934 = (t1912).items()?;
        if t1934.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1934[0].clone();
        v_v = t1934[1].clone();
        if ({ let t1913 = v_k.clone(); Py::Bool({ let t1914 = v_outputs.clone(); if !py::contains(&t1914, &t1913)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1998(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2000: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1999 = v_inputs.clone(); f_gross_premium_reserve(py::getitem(&t1999, &Py::str("q"))?, py::getitem(&t1999, &Py::str("i"))?, py::getitem(&t1999, &Py::str("S"))?, py::getitem(&t1999, &Py::str("e0"))?, py::getitem(&t1999, &Py::str("r"))?, py::getitem(&t1999, &Py::str("c"))?, py::getitem(&t1999, &Py::str("E"))?, py::getitem(&t1999, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2000 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2001 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2023 = (t2001).items()?;
        if t2023.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2023[0].clone();
        v_v = t2023[1].clone();
        if ({ let t2002 = v_k.clone(); Py::Bool({ let t2003 = v_outputs.clone(); if !py::contains(&t2003, &t2002)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2049(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2051: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2050 = v_inputs.clone(); f_increasing_annuity(py::getitem(&t2050, &Py::str("n"))?, py::getitem(&t2050, &Py::str("i"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2051 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2052 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2074 = (t2052).items()?;
        if t2074.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2074[0].clone();
        v_v = t2074[1].clone();
        if ({ let t2053 = v_k.clone(); Py::Bool({ let t2054 = v_outputs.clone(); if !py::contains(&t2054, &t2053)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2103(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2105: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2104 = v_inputs.clone(); f_individual_risk_model(py::getitem(&t2104, &Py::str("q"))?, py::getitem(&t2104, &Py::str("b"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2105 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2106 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2128 = (t2106).items()?;
        if t2128.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2128[0].clone();
        v_v = t2128[1].clone();
        if ({ let t2107 = v_k.clone(); Py::Bool({ let t2108 = v_outputs.clone(); if !py::contains(&t2108, &t2107)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2153(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2155: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2154 = v_inputs.clone(); f_rate_conversion(py::getitem(&t2154, &Py::str("i"))?, py::getitem(&t2154, &Py::str("m"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2155 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2156 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2178 = (t2156).items()?;
        if t2178.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2178[0].clone();
        v_v = t2178[1].clone();
        if ({ let t2157 = v_k.clone(); Py::Bool({ let t2158 = v_outputs.clone(); if !py::contains(&t2158, &t2157)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2222(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2224: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2223 = v_inputs.clone(); f_joint_life(py::getitem(&t2223, &Py::str("qx"))?, py::getitem(&t2223, &Py::str("qy"))?, py::getitem(&t2223, &Py::str("i"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2224 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2225 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2247 = (t2225).items()?;
        if t2247.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2247[0].clone();
        v_v = t2247[1].clone();
        if ({ let t2226 = v_k.clone(); Py::Bool({ let t2227 = v_outputs.clone(); if !py::contains(&t2227, &t2226)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_225(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t227: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t226 = v_inputs.clone(); f_bornhuetter_ferguson(py::getitem(&t226, &Py::str("T"))?, py::getitem(&t226, &Py::str("P"))?, py::getitem(&t226, &Py::str("elr"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t227 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t228 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t250 = (t228).items()?;
        if t250.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t250[0].clone();
        v_v = t250[1].clone();
        if ({ let t229 = v_k.clone(); Py::Bool({ let t230 = v_outputs.clone(); if !py::contains(&t230, &t229)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2297(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2299: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2298 = v_inputs.clone(); f_kaplan_meier(py::getitem(&t2298, &Py::str("times"))?, py::getitem(&t2298, &Py::str("events"))?, py::getitem(&t2298, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2299 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2300 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2322 = (t2300).items()?;
        if t2322.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2322[0].clone();
        v_v = t2322[1].clone();
        if ({ let t2301 = v_k.clone(); Py::Bool({ let t2302 = v_outputs.clone(); if !py::contains(&t2302, &t2301)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2363(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2365: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2364 = v_inputs.clone(); f_life_expectancy(py::getitem(&t2364, &Py::str("lx"))?, py::getitem(&t2364, &Py::str("x0"))?, py::getitem(&t2364, &Py::str("x"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2365 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2366 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2388 = (t2366).items()?;
        if t2388.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2388[0].clone();
        v_v = t2388[1].clone();
        if ({ let t2367 = v_k.clone(); Py::Bool({ let t2368 = v_outputs.clone(); if !py::contains(&t2368, &t2367)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_24(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t26: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t25 = v_inputs.clone(); f_accumulated_value(py::getitem(&t25, &Py::str("P"))?, py::getitem(&t25, &Py::str("i"))?, py::getitem(&t25, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t26 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t27 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t49 = (t27).items()?;
        if t49.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t49[0].clone();
        v_v = t49[1].clone();
        if ({ let t28 = v_k.clone(); Py::Bool({ let t29 = v_outputs.clone(); if !py::contains(&t29, &t28)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2418(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2420: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2419 = v_inputs.clone(); f_life_table_functions(py::getitem(&t2419, &Py::str("lx"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2420 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2421 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2443 = (t2421).items()?;
        if t2443.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2443[0].clone();
        v_v = t2443[1].clone();
        if ({ let t2422 = v_k.clone(); Py::Bool({ let t2423 = v_outputs.clone(); if !py::contains(&t2423, &t2422)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2477(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2479: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2478 = v_inputs.clone(); f_lf_full_credibility(py::getitem(&t2478, &Py::str("p"))?, py::getitem(&t2478, &Py::str("k"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2479 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2480 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2502 = (t2480).items()?;
        if t2502.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2502[0].clone();
        v_v = t2502[1].clone();
        if ({ let t2481 = v_k.clone(); Py::Bool({ let t2482 = v_outputs.clone(); if !py::contains(&t2482, &t2481)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2540(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2542: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2541 = v_inputs.clone(); f_lf_full_credibility_severity(py::getitem(&t2541, &Py::str("p"))?, py::getitem(&t2541, &Py::str("k"))?, py::getitem(&t2541, &Py::str("mean_x"))?, py::getitem(&t2541, &Py::str("sd_x"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2542 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2543 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2565 = (t2543).items()?;
        if t2565.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2565[0].clone();
        v_v = t2565[1].clone();
        if ({ let t2544 = v_k.clone(); Py::Bool({ let t2545 = v_outputs.clone(); if !py::contains(&t2545, &t2544)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2596(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2598: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2597 = v_inputs.clone(); f_loan_amortization(py::getitem(&t2597, &Py::str("L"))?, py::getitem(&t2597, &Py::str("i"))?, py::getitem(&t2597, &Py::str("n"))?, py::getitem(&t2597, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2598 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2599 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2621 = (t2599).items()?;
        if t2621.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2621[0].clone();
        v_v = t2621[1].clone();
        if ({ let t2600 = v_k.clone(); Py::Bool({ let t2601 = v_outputs.clone(); if !py::contains(&t2601, &t2600)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2670(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2672: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2671 = v_inputs.clone(); f_lognormal_deductible(py::getitem(&t2671, &Py::str("mu"))?, py::getitem(&t2671, &Py::str("sigma"))?, py::getitem(&t2671, &Py::str("d"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2672 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2673 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2695 = (t2673).items()?;
        if t2695.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2695[0].clone();
        v_v = t2695[1].clone();
        if ({ let t2674 = v_k.clone(); Py::Bool({ let t2675 = v_outputs.clone(); if !py::contains(&t2675, &t2674)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_273(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t275: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t274 = v_inputs.clone(); f_buhlmann_premium(py::getitem(&t274, &Py::str("mu"))?, py::getitem(&t274, &Py::str("EPV"))?, py::getitem(&t274, &Py::str("VHM"))?, py::getitem(&t274, &Py::str("n"))?, py::getitem(&t274, &Py::str("xbar"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t275 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t276 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t298 = (t276).items()?;
        if t298.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t298[0].clone();
        v_v = t298[1].clone();
        if ({ let t277 = v_k.clone(); Py::Bool({ let t278 = v_outputs.clone(); if !py::contains(&t278, &t277)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2741(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2743: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2742 = v_inputs.clone(); f_loss_at_issue(py::getitem(&t2742, &Py::str("q"))?, py::getitem(&t2742, &Py::str("i"))?, py::getitem(&t2742, &Py::str("S"))?, py::getitem(&t2742, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2743 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2744 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2766 = (t2744).items()?;
        if t2766.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2766[0].clone();
        v_v = t2766[1].clone();
        if ({ let t2745 = v_k.clone(); Py::Bool({ let t2746 = v_outputs.clone(); if !py::contains(&t2746, &t2745)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2805(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2807: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2806 = v_inputs.clone(); f_loss_ratio_rate_change(py::getitem(&t2806, &Py::str("L"))?, py::getitem(&t2806, &Py::str("EP"))?, py::getitem(&t2806, &Py::str("F"))?, py::getitem(&t2806, &Py::str("V"))?, py::getitem(&t2806, &Py::str("Q"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2807 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2808 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2830 = (t2808).items()?;
        if t2830.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2830[0].clone();
        v_v = t2830[1].clone();
        if ({ let t2809 = v_k.clone(); Py::Bool({ let t2810 = v_outputs.clone(); if !py::contains(&t2810, &t2809)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2886(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2888: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2887 = v_inputs.clone(); f_mack_standard_error(py::getitem(&t2887, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2888 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2889 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2911 = (t2889).items()?;
        if t2911.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2911[0].clone();
        v_v = t2911[1].clone();
        if ({ let t2890 = v_k.clone(); Py::Bool({ let t2891 = v_outputs.clone(); if !py::contains(&t2891, &t2890)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2962(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2964: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2963 = v_inputs.clone(); f_makeham_continuous(py::getitem(&t2963, &Py::str("A"))?, py::getitem(&t2963, &Py::str("B"))?, py::getitem(&t2963, &Py::str("c"))?, py::getitem(&t2963, &Py::str("x"))?, py::getitem(&t2963, &Py::str("i"))?, py::getitem(&t2963, &Py::str("S"))?, py::getitem(&t2963, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2964 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2965 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2987 = (t2965).items()?;
        if t2987.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2987[0].clone();
        v_v = t2987[1].clone();
        if ({ let t2966 = v_k.clone(); Py::Bool({ let t2967 = v_outputs.clone(); if !py::contains(&t2967, &t2966)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3030(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3032: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3031 = v_inputs.clone(); f_makeham(py::getitem(&t3031, &Py::str("A"))?, py::getitem(&t3031, &Py::str("B"))?, py::getitem(&t3031, &Py::str("c"))?, py::getitem(&t3031, &Py::str("x"))?, py::getitem(&t3031, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3032 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3033 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3055 = (t3033).items()?;
        if t3055.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3055[0].clone();
        v_v = t3055[1].clone();
        if ({ let t3034 = v_k.clone(); Py::Bool({ let t3035 = v_outputs.clone(); if !py::contains(&t3035, &t3034)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3095(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3097: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3096 = v_inputs.clone(); f_mthly_annuity_udd(py::getitem(&t3096, &Py::str("q"))?, py::getitem(&t3096, &Py::str("i"))?, py::getitem(&t3096, &Py::str("m"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3097 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3098 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3120 = (t3098).items()?;
        if t3120.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3120[0].clone();
        v_v = t3120[1].clone();
        if ({ let t3099 = v_k.clone(); Py::Bool({ let t3100 = v_outputs.clone(); if !py::contains(&t3100, &t3099)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3144(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3146: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3145 = v_inputs.clone(); f_multiple_to_single(py::getitem(&t3145, &Py::str("q1"))?, py::getitem(&t3145, &Py::str("q2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3146 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3147 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3169 = (t3147).items()?;
        if t3169.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3169[0].clone();
        v_v = t3169[1].clone();
        if ({ let t3148 = v_k.clone(); Py::Bool({ let t3149 = v_outputs.clone(); if !py::contains(&t3149, &t3148)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3199(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3201: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3200 = v_inputs.clone(); f_negative_binomial_frequency(py::getitem(&t3200, &Py::str("r"))?, py::getitem(&t3200, &Py::str("beta"))?, py::getitem(&t3200, &Py::str("k"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3201 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3202 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3224 = (t3202).items()?;
        if t3224.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3224[0].clone();
        v_v = t3224[1].clone();
        if ({ let t3203 = v_k.clone(); Py::Bool({ let t3204 = v_outputs.clone(); if !py::contains(&t3204, &t3203)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3274(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3276: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3275 = v_inputs.clone(); f_nelson_aalen(py::getitem(&t3275, &Py::str("times"))?, py::getitem(&t3275, &Py::str("events"))?, py::getitem(&t3275, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3276 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3277 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3299 = (t3277).items()?;
        if t3299.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3299[0].clone();
        v_v = t3299[1].clone();
        if ({ let t3278 = v_k.clone(); Py::Bool({ let t3279 = v_outputs.clone(); if !py::contains(&t3279, &t3278)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_332(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t334: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t333 = v_inputs.clone(); f_buhlmann_empirical(py::getitem(&t333, &Py::str("X"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t334 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t335 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t357 = (t335).items()?;
        if t357.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t357[0].clone();
        v_v = t357[1].clone();
        if ({ let t336 = v_k.clone(); Py::Bool({ let t337 = v_outputs.clone(); if !py::contains(&t337, &t336)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3342(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3344: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3343 = v_inputs.clone(); f_net_premium_endowment(py::getitem(&t3343, &Py::str("q"))?, py::getitem(&t3343, &Py::str("i"))?, py::getitem(&t3343, &Py::str("n"))?, py::getitem(&t3343, &Py::str("S"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3344 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3345 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3367 = (t3345).items()?;
        if t3367.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3367[0].clone();
        v_v = t3367[1].clone();
        if ({ let t3346 = v_k.clone(); Py::Bool({ let t3347 = v_outputs.clone(); if !py::contains(&t3347, &t3346)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3412(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3414: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3413 = v_inputs.clone(); f_net_premium_limited_pay(py::getitem(&t3413, &Py::str("q"))?, py::getitem(&t3413, &Py::str("i"))?, py::getitem(&t3413, &Py::str("h"))?, py::getitem(&t3413, &Py::str("S"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3414 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3415 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3437 = (t3415).items()?;
        if t3437.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3437[0].clone();
        v_v = t3437[1].clone();
        if ({ let t3416 = v_k.clone(); Py::Bool({ let t3417 = v_outputs.clone(); if !py::contains(&t3417, &t3416)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3480(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3482: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3481 = v_inputs.clone(); f_net_premium_term(py::getitem(&t3481, &Py::str("q"))?, py::getitem(&t3481, &Py::str("i"))?, py::getitem(&t3481, &Py::str("n"))?, py::getitem(&t3481, &Py::str("S"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3482 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3483 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3505 = (t3483).items()?;
        if t3505.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3505[0].clone();
        v_v = t3505[1].clone();
        if ({ let t3484 = v_k.clone(); Py::Bool({ let t3485 = v_outputs.clone(); if !py::contains(&t3485, &t3484)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3548(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3550: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3549 = v_inputs.clone(); f_net_premium_whole_life(py::getitem(&t3549, &Py::str("q"))?, py::getitem(&t3549, &Py::str("i"))?, py::getitem(&t3549, &Py::str("S"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3550 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3551 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3573 = (t3551).items()?;
        if t3573.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3573[0].clone();
        v_v = t3573[1].clone();
        if ({ let t3552 = v_k.clone(); Py::Bool({ let t3553 = v_outputs.clone(); if !py::contains(&t3553, &t3552)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3605(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3607: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3606 = v_inputs.clone(); f_nominal_to_effective(py::getitem(&t3606, &Py::str("rate"))?, py::getitem(&t3606, &Py::str("m"))?, py::getitem(&t3606, &Py::str("kind"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3607 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3608 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3630 = (t3608).items()?;
        if t3630.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3630[0].clone();
        v_v = t3630[1].clone();
        if ({ let t3609 = v_k.clone(); Py::Bool({ let t3610 = v_outputs.clone(); if !py::contains(&t3610, &t3609)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3650(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3652: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3651 = v_inputs.clone(); f_normal_approx_aggregate(py::getitem(&t3651, &Py::str("mean"))?, py::getitem(&t3651, &Py::str("var"))?, py::getitem(&t3651, &Py::str("s"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3652 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3653 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3675 = (t3653).items()?;
        if t3675.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3675[0].clone();
        v_v = t3675[1].clone();
        if ({ let t3654 = v_k.clone(); Py::Bool({ let t3655 = v_outputs.clone(); if !py::contains(&t3655, &t3654)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3716(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3718: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3717 = v_inputs.clone(); f_panjer_negative_binomial(py::getitem(&t3717, &Py::str("r"))?, py::getitem(&t3717, &Py::str("beta"))?, py::getitem(&t3717, &Py::str("f"))?, py::getitem(&t3717, &Py::str("smax"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3718 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3719 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3741 = (t3719).items()?;
        if t3741.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3741[0].clone();
        v_v = t3741[1].clone();
        if ({ let t3720 = v_k.clone(); Py::Bool({ let t3721 = v_outputs.clone(); if !py::contains(&t3721, &t3720)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3778(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3780: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3779 = v_inputs.clone(); f_panjer_poisson(py::getitem(&t3779, &Py::str("lam"))?, py::getitem(&t3779, &Py::str("f"))?, py::getitem(&t3779, &Py::str("smax"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3780 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3781 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3803 = (t3781).items()?;
        if t3803.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3803[0].clone();
        v_v = t3803[1].clone();
        if ({ let t3782 = v_k.clone(); Py::Bool({ let t3783 = v_outputs.clone(); if !py::contains(&t3783, &t3782)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3863(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3865: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3864 = v_inputs.clone(); f_pareto_coverage(py::getitem(&t3864, &Py::str("alpha"))?, py::getitem(&t3864, &Py::str("theta"))?, py::getitem(&t3864, &Py::str("d"))?, py::getitem(&t3864, &Py::str("u"))?, py::getitem(&t3864, &Py::str("c"))?, py::getitem(&t3864, &Py::str("r"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3865 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3866 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3888 = (t3866).items()?;
        if t3888.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3888[0].clone();
        v_v = t3888[1].clone();
        if ({ let t3867 = v_k.clone(); Py::Bool({ let t3868 = v_outputs.clone(); if !py::contains(&t3868, &t3867)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3937(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3939: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3938 = v_inputs.clone(); f_pareto_deductible(py::getitem(&t3938, &Py::str("alpha"))?, py::getitem(&t3938, &Py::str("theta"))?, py::getitem(&t3938, &Py::str("d"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3939 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3940 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3962 = (t3940).items()?;
        if t3962.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3962[0].clone();
        v_v = t3962[1].clone();
        if ({ let t3941 = v_k.clone(); Py::Bool({ let t3942 = v_outputs.clone(); if !py::contains(&t3942, &t3941)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3986(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3988: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3987 = v_inputs.clone(); f_partial_credibility(py::getitem(&t3987, &Py::str("n"))?, py::getitem(&t3987, &Py::str("n_full"))?, py::getitem(&t3987, &Py::str("xbar"))?, py::getitem(&t3987, &Py::str("M"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3988 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3989 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4011 = (t3989).items()?;
        if t4011.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4011[0].clone();
        v_v = t4011[1].clone();
        if ({ let t3990 = v_k.clone(); Py::Bool({ let t3991 = v_outputs.clone(); if !py::contains(&t3991, &t3990)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_401(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t403: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t402 = v_inputs.clone(); f_buhlmann_straub(py::getitem(&t402, &Py::str("X"))?, py::getitem(&t402, &Py::str("m"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t403 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t404 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t426 = (t404).items()?;
        if t426.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t426[0].clone();
        v_v = t426[1].clone();
        if ({ let t405 = v_k.clone(); Py::Bool({ let t406 = v_outputs.clone(); if !py::contains(&t406, &t405)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4030(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4032: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4031 = v_inputs.clone(); f_perpetuity(py::getitem(&t4031, &Py::str("i"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4032 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4033 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4055 = (t4033).items()?;
        if t4055.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4055[0].clone();
        v_v = t4055[1].clone();
        if ({ let t4034 = v_k.clone(); Py::Bool({ let t4035 = v_outputs.clone(); if !py::contains(&t4035, &t4034)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4081(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4083: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4082 = v_inputs.clone(); f_poisson_frequency(py::getitem(&t4082, &Py::str("lam"))?, py::getitem(&t4082, &Py::str("k"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4083 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4084 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4106 = (t4084).items()?;
        if t4106.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4106[0].clone();
        v_v = t4106[1].clone();
        if ({ let t4085 = v_k.clone(); Py::Bool({ let t4086 = v_outputs.clone(); if !py::contains(&t4086, &t4085)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4132(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4134: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4133 = v_inputs.clone(); f_poisson_gamma(py::getitem(&t4133, &Py::str("alpha"))?, py::getitem(&t4133, &Py::str("theta"))?, py::getitem(&t4133, &Py::str("counts"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4134 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4135 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4157 = (t4135).items()?;
        if t4157.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4157[0].clone();
        v_v = t4157[1].clone();
        if ({ let t4136 = v_k.clone(); Py::Bool({ let t4137 = v_outputs.clone(); if !py::contains(&t4137, &t4136)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4189(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4191: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4190 = v_inputs.clone(); f_policy_value_recursion(py::getitem(&t4190, &Py::str("V"))?, py::getitem(&t4190, &Py::str("P"))?, py::getitem(&t4190, &Py::str("e"))?, py::getitem(&t4190, &Py::str("i"))?, py::getitem(&t4190, &Py::str("q"))?, py::getitem(&t4190, &Py::str("S"))?, py::getitem(&t4190, &Py::str("E"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4191 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4192 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4214 = (t4192).items()?;
        if t4214.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4214[0].clone();
        v_v = t4214[1].clone();
        if ({ let t4193 = v_k.clone(); Py::Bool({ let t4194 = v_outputs.clone(); if !py::contains(&t4194, &t4193)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4239(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4241: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4240 = v_inputs.clone(); f_premium_principle(py::getitem(&t4240, &Py::str("mean"))?, py::getitem(&t4240, &Py::str("var"))?, py::getitem(&t4240, &Py::str("theta"))?, py::getitem(&t4240, &Py::str("principle"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4241 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4242 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4264 = (t4242).items()?;
        if t4264.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4264[0].clone();
        v_v = t4264[1].clone();
        if ({ let t4243 = v_k.clone(); Py::Bool({ let t4244 = v_outputs.clone(); if !py::contains(&t4244, &t4243)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4292(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4294: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4293 = v_inputs.clone(); f_pv_cash_flows(py::getitem(&t4293, &Py::str("amounts"))?, py::getitem(&t4293, &Py::str("times"))?, py::getitem(&t4293, &Py::str("i"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4294 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4295 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4317 = (t4295).items()?;
        if t4317.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4317[0].clone();
        v_v = t4317[1].clone();
        if ({ let t4296 = v_k.clone(); Py::Bool({ let t4297 = v_outputs.clone(); if !py::contains(&t4297, &t4296)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4363(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4365: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4364 = v_inputs.clone(); f_pure_endowment_lx(py::getitem(&t4364, &Py::str("lx"))?, py::getitem(&t4364, &Py::str("x0"))?, py::getitem(&t4364, &Py::str("x"))?, py::getitem(&t4364, &Py::str("n"))?, py::getitem(&t4364, &Py::str("i"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4365 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4366 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4388 = (t4366).items()?;
        if t4388.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4388[0].clone();
        v_v = t4388[1].clone();
        if ({ let t4367 = v_k.clone(); Py::Bool({ let t4368 = v_outputs.clone(); if !py::contains(&t4368, &t4367)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4427(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4429: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4428 = v_inputs.clone(); f_pure_premium_rate(py::getitem(&t4428, &Py::str("L"))?, py::getitem(&t4428, &Py::str("X"))?, py::getitem(&t4428, &Py::str("Fe"))?, py::getitem(&t4428, &Py::str("V"))?, py::getitem(&t4428, &Py::str("Q"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4429 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4430 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4452 = (t4430).items()?;
        if t4452.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4452[0].clone();
        v_v = t4452[1].clone();
        if ({ let t4431 = v_k.clone(); Py::Bool({ let t4432 = v_outputs.clone(); if !py::contains(&t4432, &t4431)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4499(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4501: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4500 = v_inputs.clone(); f_reserve_endowment(py::getitem(&t4500, &Py::str("q"))?, py::getitem(&t4500, &Py::str("i"))?, py::getitem(&t4500, &Py::str("n"))?, py::getitem(&t4500, &Py::str("S"))?, py::getitem(&t4500, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4501 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4502 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4524 = (t4502).items()?;
        if t4524.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4524[0].clone();
        v_v = t4524[1].clone();
        if ({ let t4503 = v_k.clone(); Py::Bool({ let t4504 = v_outputs.clone(); if !py::contains(&t4504, &t4503)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4571(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4573: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4572 = v_inputs.clone(); f_reserve_term(py::getitem(&t4572, &Py::str("q"))?, py::getitem(&t4572, &Py::str("i"))?, py::getitem(&t4572, &Py::str("n"))?, py::getitem(&t4572, &Py::str("S"))?, py::getitem(&t4572, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4573 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4574 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4596 = (t4574).items()?;
        if t4596.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4596[0].clone();
        v_v = t4596[1].clone();
        if ({ let t4575 = v_k.clone(); Py::Bool({ let t4576 = v_outputs.clone(); if !py::contains(&t4576, &t4575)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4641(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4643: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4642 = v_inputs.clone(); f_reserve_whole_life(py::getitem(&t4642, &Py::str("q"))?, py::getitem(&t4642, &Py::str("i"))?, py::getitem(&t4642, &Py::str("S"))?, py::getitem(&t4642, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4643 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4644 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4666 = (t4644).items()?;
        if t4666.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4666[0].clone();
        v_v = t4666[1].clone();
        if ({ let t4645 = v_k.clone(); Py::Bool({ let t4646 = v_outputs.clone(); if !py::contains(&t4646, &t4645)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4713(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4715: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4714 = v_inputs.clone(); f_retrospective_reserve(py::getitem(&t4714, &Py::str("q"))?, py::getitem(&t4714, &Py::str("i"))?, py::getitem(&t4714, &Py::str("S"))?, py::getitem(&t4714, &Py::str("P"))?, py::getitem(&t4714, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4715 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4716 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4738 = (t4716).items()?;
        if t4738.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4738[0].clone();
        v_v = t4738[1].clone();
        if ({ let t4717 = v_k.clone(); Py::Bool({ let t4718 = v_outputs.clone(); if !py::contains(&t4718, &t4717)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_474(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t476: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t475 = v_inputs.clone(); f_cape_cod(py::getitem(&t475, &Py::str("T"))?, py::getitem(&t475, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t476 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t477 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t499 = (t477).items()?;
        if t499.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t499[0].clone();
        v_v = t499[1].clone();
        if ({ let t478 = v_k.clone(); Py::Bool({ let t479 = v_outputs.clone(); if !py::contains(&t479, &t478)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4784(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4786: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4785 = v_inputs.clone(); f_reversionary_annuity(py::getitem(&t4785, &Py::str("qx"))?, py::getitem(&t4785, &Py::str("qy"))?, py::getitem(&t4785, &Py::str("i"))?, py::getitem(&t4785, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4786 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4787 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4809 = (t4787).items()?;
        if t4809.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4809[0].clone();
        v_v = t4809[1].clone();
        if ({ let t4788 = v_k.clone(); Py::Bool({ let t4789 = v_outputs.clone(); if !py::contains(&t4789, &t4788)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4835(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4837: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4836 = v_inputs.clone(); f_single_loss_payment(py::getitem(&t4836, &Py::str("x"))?, py::getitem(&t4836, &Py::str("d"))?, py::getitem(&t4836, &Py::str("u"))?, py::getitem(&t4836, &Py::str("c"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4837 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4838 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4860 = (t4838).items()?;
        if t4860.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4860[0].clone();
        v_v = t4860[1].clone();
        if ({ let t4839 = v_k.clone(); Py::Bool({ let t4840 = v_outputs.clone(); if !py::contains(&t4840, &t4839)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4885(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4887: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4886 = v_inputs.clone(); f_single_to_multiple(py::getitem(&t4886, &Py::str("q1s"))?, py::getitem(&t4886, &Py::str("q2s"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4887 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4888 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4910 = (t4888).items()?;
        if t4910.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4910[0].clone();
        v_v = t4910[1].clone();
        if ({ let t4889 = v_k.clone(); Py::Bool({ let t4890 = v_outputs.clone(); if !py::contains(&t4890, &t4889)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4950(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4952: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4951 = v_inputs.clone(); f_stop_loss_compound_poisson(py::getitem(&t4951, &Py::str("lam"))?, py::getitem(&t4951, &Py::str("f"))?, py::getitem(&t4951, &Py::str("d"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4952 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4953 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4975 = (t4953).items()?;
        if t4975.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4975[0].clone();
        v_v = t4975[1].clone();
        if ({ let t4954 = v_k.clone(); Py::Bool({ let t4955 = v_outputs.clone(); if !py::contains(&t4955, &t4954)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5015(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5017: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5016 = v_inputs.clone(); f_survival_probability(py::getitem(&t5016, &Py::str("lx"))?, py::getitem(&t5016, &Py::str("x0"))?, py::getitem(&t5016, &Py::str("x"))?, py::getitem(&t5016, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5017 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5018 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5040 = (t5018).items()?;
        if t5040.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5040[0].clone();
        v_v = t5040[1].clone();
        if ({ let t5019 = v_k.clone(); Py::Bool({ let t5020 = v_outputs.clone(); if !py::contains(&t5020, &t5019)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5082(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5084: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5083 = v_inputs.clone(); f_temporary_annuity(py::getitem(&t5083, &Py::str("q"))?, py::getitem(&t5083, &Py::str("i"))?, py::getitem(&t5083, &Py::str("n"))?, py::getitem(&t5083, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5084 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5085 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5107 = (t5085).items()?;
        if t5107.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5107[0].clone();
        v_v = t5107[1].clone();
        if ({ let t5086 = v_k.clone(); Py::Bool({ let t5087 = v_outputs.clone(); if !py::contains(&t5087, &t5086)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5152(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5154: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5153 = v_inputs.clone(); f_temporary_expectation(py::getitem(&t5153, &Py::str("lx"))?, py::getitem(&t5153, &Py::str("x0"))?, py::getitem(&t5153, &Py::str("x"))?, py::getitem(&t5153, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5154 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5155 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5177 = (t5155).items()?;
        if t5177.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5177[0].clone();
        v_v = t5177[1].clone();
        if ({ let t5156 = v_k.clone(); Py::Bool({ let t5157 = v_outputs.clone(); if !py::contains(&t5157, &t5156)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5219(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5221: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5220 = v_inputs.clone(); f_term_insurance(py::getitem(&t5220, &Py::str("q"))?, py::getitem(&t5220, &Py::str("i"))?, py::getitem(&t5220, &Py::str("n"))?, py::getitem(&t5220, &Py::str("S"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5221 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5222 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5244 = (t5222).items()?;
        if t5244.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5244[0].clone();
        v_v = t5244[1].clone();
        if ({ let t5223 = v_k.clone(); Py::Bool({ let t5224 = v_outputs.clone(); if !py::contains(&t5224, &t5223)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_523(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t525: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t524 = v_inputs.clone(); f_central_death_rate(py::getitem(&t524, &Py::str("m"))?, py::getitem(&t524, &Py::str("assumption"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t525 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t526 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t548 = (t526).items()?;
        if t548.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t548[0].clone();
        v_v = t548[1].clone();
        if ({ let t527 = v_k.clone(); Py::Bool({ let t528 = v_outputs.clone(); if !py::contains(&t528, &t527)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5283(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5285: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5284 = v_inputs.clone(); f_thiele_endowment(py::getitem(&t5284, &Py::str("mu"))?, py::getitem(&t5284, &Py::str("delta"))?, py::getitem(&t5284, &Py::str("n"))?, py::getitem(&t5284, &Py::str("S"))?, py::getitem(&t5284, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5285 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5286 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5308 = (t5286).items()?;
        if t5308.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5308[0].clone();
        v_v = t5308[1].clone();
        if ({ let t5287 = v_k.clone(); Py::Bool({ let t5288 = v_outputs.clone(); if !py::contains(&t5288, &t5287)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5331(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5333: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5332 = v_inputs.clone(); f_var_tvar_exponential(py::getitem(&t5332, &Py::str("theta"))?, py::getitem(&t5332, &Py::str("p"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5333 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5334 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5356 = (t5334).items()?;
        if t5356.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5356[0].clone();
        v_v = t5356[1].clone();
        if ({ let t5335 = v_k.clone(); Py::Bool({ let t5336 = v_outputs.clone(); if !py::contains(&t5336, &t5335)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5394(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5396: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5395 = v_inputs.clone(); f_var_tvar_lognormal(py::getitem(&t5395, &Py::str("mu"))?, py::getitem(&t5395, &Py::str("sigma"))?, py::getitem(&t5395, &Py::str("p"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5396 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5397 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5419 = (t5397).items()?;
        if t5419.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5419[0].clone();
        v_v = t5419[1].clone();
        if ({ let t5398 = v_k.clone(); Py::Bool({ let t5399 = v_outputs.clone(); if !py::contains(&t5399, &t5398)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5454(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5456: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5455 = v_inputs.clone(); f_var_tvar_normal(py::getitem(&t5455, &Py::str("mu"))?, py::getitem(&t5455, &Py::str("sigma"))?, py::getitem(&t5455, &Py::str("p"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5456 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5457 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5479 = (t5457).items()?;
        if t5479.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5479[0].clone();
        v_v = t5479[1].clone();
        if ({ let t5458 = v_k.clone(); Py::Bool({ let t5459 = v_outputs.clone(); if !py::contains(&t5459, &t5458)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5506(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5508: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5507 = v_inputs.clone(); f_var_tvar_pareto(py::getitem(&t5507, &Py::str("alpha"))?, py::getitem(&t5507, &Py::str("theta"))?, py::getitem(&t5507, &Py::str("p"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5508 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5509 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5531 = (t5509).items()?;
        if t5531.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5531[0].clone();
        v_v = t5531[1].clone();
        if ({ let t5510 = v_k.clone(); Py::Bool({ let t5511 = v_outputs.clone(); if !py::contains(&t5511, &t5510)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5574(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5576: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5575 = v_inputs.clone(); f_whole_life_annuity(py::getitem(&t5575, &Py::str("q"))?, py::getitem(&t5575, &Py::str("i"))?, py::getitem(&t5575, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5576 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5577 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5599 = (t5577).items()?;
        if t5599.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5599[0].clone();
        v_v = t5599[1].clone();
        if ({ let t5578 = v_k.clone(); Py::Bool({ let t5579 = v_outputs.clone(); if !py::contains(&t5579, &t5578)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5641(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5643: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5642 = v_inputs.clone(); f_whole_life_insurance(py::getitem(&t5642, &Py::str("q"))?, py::getitem(&t5642, &Py::str("i"))?, py::getitem(&t5642, &Py::str("S"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5643 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5644 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5666 = (t5644).items()?;
        if t5666.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5666[0].clone();
        v_v = t5666[1].clone();
        if ({ let t5645 = v_k.clone(); Py::Bool({ let t5646 = v_outputs.clone(); if !py::contains(&t5646, &t5645)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5698(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5700: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5699 = v_inputs.clone(); f_woolhouse(py::getitem(&t5699, &Py::str("a_due"))?, py::getitem(&t5699, &Py::str("m"))?, py::getitem(&t5699, &Py::str("i"))?, py::getitem(&t5699, &Py::str("mu"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5700 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5701 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5723 = (t5701).items()?;
        if t5723.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5723[0].clone();
        v_v = t5723[1].clone();
        if ({ let t5702 = v_k.clone(); Py::Bool({ let t5703 = v_outputs.clone(); if !py::contains(&t5703, &t5702)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_584(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t586: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t585 = v_inputs.clone(); f_chain_ladder_factors(py::getitem(&t585, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t586 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t587 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t609 = (t587).items()?;
        if t609.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t609[0].clone();
        v_v = t609[1].clone();
        if ({ let t588 = v_k.clone(); Py::Bool({ let t589 = v_outputs.clone(); if !py::contains(&t589, &t588)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_651(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t653: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t652 = v_inputs.clone(); f_chain_ladder_reserve(py::getitem(&t652, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t653 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t654 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t676 = (t654).items()?;
        if t676.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t676[0].clone();
        v_v = t676[1].clone();
        if ({ let t655 = v_k.clone(); Py::Bool({ let t656 = v_outputs.clone(); if !py::contains(&t656, &t655)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_717(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t719: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t718 = v_inputs.clone(); f_chain_ladder_simple(py::getitem(&t718, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t719 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t720 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t742 = (t720).items()?;
        if t742.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t742[0].clone();
        v_v = t742[1].clone();
        if ({ let t721 = v_k.clone(); Py::Bool({ let t722 = v_outputs.clone(); if !py::contains(&t722, &t721)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_75(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t77: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t76 = v_inputs.clone(); f_annuity_certain(py::getitem(&t76, &Py::str("n"))?, py::getitem(&t76, &Py::str("i"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t77 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t78 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t100 = (t78).items()?;
        if t100.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t100[0].clone();
        v_v = t100[1].clone();
        if ({ let t79 = v_k.clone(); Py::Bool({ let t80 = v_outputs.clone(); if !py::contains(&t80, &t79)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_792(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t794: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t793 = v_inputs.clone(); f_commutation_functions(py::getitem(&t793, &Py::str("lx"))?, py::getitem(&t793, &Py::str("x0"))?, py::getitem(&t793, &Py::str("x"))?, py::getitem(&t793, &Py::str("i"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t794 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t795 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t817 = (t795).items()?;
        if t817.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t817[0].clone();
        v_v = t817[1].clone();
        if ({ let t796 = v_k.clone(); Py::Bool({ let t797 = v_outputs.clone(); if !py::contains(&t797, &t796)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_837(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t839: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t838 = v_inputs.clone(); f_compound_moments(py::getitem(&t838, &Py::str("EN"))?, py::getitem(&t838, &Py::str("VarN"))?, py::getitem(&t838, &Py::str("EX"))?, py::getitem(&t838, &Py::str("VarX"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t839 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t840 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t862 = (t840).items()?;
        if t862.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t862[0].clone();
        v_v = t862[1].clone();
        if ({ let t841 = v_k.clone(); Py::Bool({ let t842 = v_outputs.clone(); if !py::contains(&t842, &t841)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_893(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t895: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t894 = v_inputs.clone(); f_compound_poisson_moments(py::getitem(&t894, &Py::str("lam"))?, py::getitem(&t894, &Py::str("m1"))?, py::getitem(&t894, &Py::str("m2"))?, py::getitem(&t894, &Py::str("m3"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t895 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t896 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t918 = (t896).items()?;
        if t918.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t918[0].clone();
        v_v = t918[1].clone();
        if ({ let t897 = v_k.clone(); Py::Bool({ let t898 = v_outputs.clone(); if !py::contains(&t898, &t897)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_940(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t942: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t941 = v_inputs.clone(); f_constant_force(py::getitem(&t941, &Py::str("mu"))?, py::getitem(&t941, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t942 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t943 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t965 = (t943).items()?;
        if t965.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t965[0].clone();
        v_v = t965[1].clone();
        if ({ let t944 = v_k.clone(); Py::Bool({ let t945 = v_outputs.clone(); if !py::contains(&t945, &t944)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
