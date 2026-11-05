//! Every Actuarial Science capability's published figure kinds, live (src/figures.rs and the generic engine): its
//! worked example gives each kind, a changed input changes the drawing, and no drawing holds a non-finite number or
//! text outside its canvas.

use actuarial_science_ai999x::pack;
use ax_engine::figures::{figures, Figure};
use ax_engine::Input;

/// The published figure kinds of every capability (publish/website/models/actuarial-science-ai999x/figures, except
/// the generic "curve"), as tools/figure_parity.py lists them.
const KINDS: &[(&str, &[&str])] = &[
    ("act_annuity_certain", &["a-n-vs-n", "field"]),
    ("act_bornhuetter_ferguson", &["matrix", "series"]),
    ("act_buhlmann_empirical", &["matrix"]),
    ("act_buhlmann_straub", &["matrix", "series"]),
    ("act_cape_cod", &["matrix", "series"]),
    ("act_chain_ladder_factors", &["matrix", "series"]),
    ("act_chain_ladder_reserve", &["matrix", "series"]),
    ("act_chain_ladder_simple_average", &["matrix", "series"]),
    ("act_constant_force_mortality", &["field"]),
    ("act_deferred_death_probability", &["io"]),
    ("act_duration_convexity", &["xy"]),
    ("act_expected_claims_method", &["series", "xy"]),
    ("act_exponential_deductible", &["field"]),
    ("act_exponential_mle", &["data"]),
    ("act_fractional_age", &["field"]),
    ("act_increasing_decreasing_annuity", &["field"]),
    ("act_individual_risk_model", &["sensitivity"]),
    ("act_life_expectancy", &["io"]),
    ("act_life_table_functions", &["series"]),
    ("act_limited_fluctuation_full_credibility", &["field", "n-full-vs-p"]),
    ("act_mack_standard_error", &["matrix", "series"]),
    ("act_mthly_annuity_udd", &["field"]),
    ("act_multiple_decrement_to_single", &["q1-single-vs-q1"]),
    ("act_net_premium_whole_life", &["field"]),
    ("act_nominal_to_effective_rate", &["field"]),
    ("act_panjer_negative_binomial", &["series"]),
    ("act_panjer_poisson", &["series"]),
    ("act_pareto_deductible", &["per-loss-vs-alpha"]),
    ("act_poisson_frequency", &["field"]),
    ("act_poisson_gamma_credibility", &["field"]),
    ("act_present_value_cash_flows", &["xy"]),
    ("act_single_to_multiple_decrement", &["field"]),
    ("act_stop_loss_premium", &["field"]),
    ("act_survival_probability", &["io"]),
    ("act_temporary_expectation", &["io"]),
];

/// The arguments of a structured example: solve('key', a=1, b=[1.0, 2.0], c='x') (lists kept whole, as text).
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let body = ex.split_once(',').map(|x| x.1).unwrap_or("");
    let body = body.trim().trim_end_matches(')');
    let mut parts = vec![];
    let (mut depth, mut cur, mut quote) = (0i32, String::new(), None::<char>);
    for c in body.chars() {
        match (quote, c) {
            (Some(q), _) if c == q => quote = None,
            (Some(_), _) => cur.push(c),
            (None, '\'' | '"') => quote = Some(c),
            (None, '[') => { depth += 1; cur.push(c) }
            (None, ']') => { depth -= 1; cur.push(c) }
            (None, ',') if depth == 0 => parts.push(std::mem::take(&mut cur)),
            _ => cur.push(c),
        }
    }
    parts.push(cur);
    parts.iter().filter_map(|p| p.split_once('='))
        .map(|(n, v)| (n.trim().to_string(), Input::Text(v.trim().to_string()))).collect()
}

fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

fn own(f: &Figure) -> bool {
    !f.slug.contains("-live-")
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        assert!(ax_engine::solve(p, key, &given, None).solved(), "{key}: the worked example is not solved");
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| f.kind == *kind) {
                Some(f) => {
                    if own(f) {
                        assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    }
                    assert!(!f.svg.contains("NaN") && !f.svg.contains("inf"), "{key} {kind}: a non-finite number");
                    if own(f) {
                        check_texts(f);   // the generic engine's figures are checked by its own tests
                    }
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        if let Some(i) = figs.iter().position(|f| !own(f)) {
            assert!(figs[i..].iter().all(|f| !own(f)), "{key}: generic figures before own ones");
        }
        for f in figs.iter().filter(|f| own(f)) {
            let low = f.caption.to_lowercase();
            for bad in [" we ", " our ", " us ", "—", "package", "open" ] {
                if bad == "open" {
                    assert!(!low.contains(&format!("{bad}{}", "stax")), "{key}: {bad}");
                } else {
                    assert!(!low.contains(bad), "{key}: '{bad}' in a caption");
                }
            }
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// Numbers in a text multiplied by f ("12" -> "16.44"; "3, 6" -> "4.11, 8.22").
fn scale_text(t: &str, f: f64) -> Option<String> {
    let mut out = String::new();
    let mut cur = String::new();
    let mut changed = false;
    let flush = |cur: &mut String, out: &mut String, changed: &mut bool| {
        if !cur.is_empty() {
            match cur.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    *changed = true;
                }
                _ => out.push_str(cur),
            }
            cur.clear();
        }
    };
    for c in t.chars() {
        if c.is_ascii_digit() || c == '.' || (c == '-' && cur.is_empty()) || ((c == 'e' || c == 'E') && !cur.is_empty()) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out, &mut changed);
            out.push(c);
        }
    }
    flush(&mut cur, &mut out, &mut changed);
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        for f in [1.37, 0.73, 2.0, 0.5] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = kinds.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| own(f)) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn a_refused_input_draws_nothing() {
    // a negative term is refused by the model: no figure at all, never a wrong one
    let p = pack();
    let given: Vec<(String, Input)> = [("n", "-3"), ("i", "0.05")].iter()
        .map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    let a = ax_engine::solve(p, "act_annuity_certain", &given, None);
    let figs = figures(p, "act_annuity_certain", &given, None, None);
    if !a.solved() {
        assert!(figs.is_empty());
    }
    // a triangle and a premium list of different lengths: the matrix figure is left out, not drawn wrong
    let given: Vec<(String, Input)> = [("T", "[[100, 150], [110, 0]]"), ("P", "[200, 210, 220]"), ("elr", "0.8")].iter()
        .map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    let a = ax_engine::solve(p, "act_bornhuetter_ferguson", &given, None);
    if !a.solved() {
        assert!(figures(p, "act_bornhuetter_ferguson", &given, None, None).is_empty());
    }
}
