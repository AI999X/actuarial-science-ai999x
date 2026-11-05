//! Actuarial Science's own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model
//! computes: value maps over two inputs (every cell one run of the model), curves of one output against one input,
//! loss triangles beside the vectors returned from them, the inputs and results of a life-table question, cash flows
//! with the present value the model gives each of them. docs/FIGURE_PARITY.md.

use ax_engine::figkit::{self as k, num, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::{Answer, Input, Val};

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let mut v: Vec<Option<Figure>> = vec![];
    if let Some(f) = FIELDS.iter().find(|f| f.key == s.key) {
        v.push(field(s, f));
    }
    if let Some(c) = CURVES.iter().find(|c| c.key == s.key) {
        v.push(curve(s, c));
    }
    if let Some(m) = MATRICES.iter().find(|m| m.0 == s.key) {
        v.push(matrices(s, m.1, m.2));
    }
    match s.key {
        "act_deferred_death_probability" | "act_life_expectancy" | "act_survival_probability"
        | "act_temporary_expectation" => v.push(io(s)),
        "act_duration_convexity" | "act_present_value_cash_flows" => v.push(cash_flows(s)),
        "act_expected_claims_method" => v.push(origin_years(s)),
        "act_exponential_mle" => v.push(observed_times(s)),
        _ => {}
    }
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ helpers
/// A number input as read (a number or a whole number).
fn num_in(s: &Solved, sym: &str) -> Option<f64> {
    s.inp(sym).or_else(|| match s.val(sym)? {
        Val::Num(x) if x.is_finite() => Some(x),
        Val::Int(i) => Some(i as f64),
        _ => None,
    })
}

fn spec_ty(s: &Solved, sym: &str) -> String {
    s.p.procedure(s.key).and_then(|p| p.inputs.iter().find(|i| i.symbol == sym)).map(|i| i.ty.clone())
        .unwrap_or_default()
}

/// "sym, meaning" for an axis (the meaning without its parenthesised example), at most 52 characters.
fn axis(s: &Solved, sym: &str) -> String {
    let m = s.p.procedure(s.key).and_then(|p| p.inputs.iter().find(|i| i.symbol == sym)
        .map(|i| i.meaning.clone()).or_else(|| p.outputs.iter().find(|o| o.symbol == sym).map(|o| o.meaning.clone())))
        .unwrap_or_default();
    let m = m.split(" (").next().unwrap_or("").trim().to_string();
    let t = if m.is_empty() || m == sym { sym.to_string() } else { format!("{sym}, {m}") };
    clip(&t, 52)
}

fn clip(t: &str, n: usize) -> String {
    if t.chars().count() <= n { t.to_string() } else { format!("{}…", t.chars().take(n - 1).collect::<String>()) }
}

/// This capability solved again with number inputs set (a whole-number input sent as one).
fn at(s: &Solved, set: &[(&str, f64)]) -> Option<Answer> {
    let mut v: Vec<(&str, Input)> = vec![];
    for (sym, x) in set {
        if !x.is_finite() {
            return None;
        }
        if spec_ty(s, sym) == "integer" {
            if (x - x.round()).abs() > 1e-9 {
                return None;
            }
            v.push((sym, Input::Int(x.round() as i64)));
        } else {
            v.push((sym, Input::Float(*x)));
        }
    }
    s.with_input(&v)
}

fn list_json(v: &[f64]) -> Input {
    Input::Json(serde_json::Value::Array(v.iter().map(|x| serde_json::json!(x)).collect()))
}

/// The values of an input axis.
#[derive(Clone, Copy)]
enum Ax {
    /// from lo to hi (widened to hold the visitor's value)
    Lin(f64, f64),
    /// the visitor's value divided and multiplied by a factor (logarithmic)
    Log(f64),
    /// the visitor's value times (1 - f) to (1 + f)
    Around(f64),
    /// whole numbers from lo to at least hi (and twice the visitor's value)
    Int(f64, f64),
}

fn axis_values(a: Ax, v: f64, n: usize) -> Option<(Vec<f64>, bool)> {
    match a {
        Ax::Lin(lo, hi) => Some((k::span(lo.min(v), hi.max(v), n, false), false)),
        Ax::Log(f) if v > 0.0 => Some((k::span(v / f, v * f, n, true), true)),
        Ax::Around(f) if v != 0.0 => {
            let (a, b) = (v * (1.0 - f), v * (1.0 + f));
            Some((k::span(a.min(b), a.max(b), n, false), false))
        }
        Ax::Int(lo, hi) => {
            let hi = hi.max(2.0 * v).max(v + 1.0).round();
            let lo = lo.min(v).round();
            let step = ((hi - lo) / (n as f64 - 1.0)).ceil().max(1.0);
            let mut xs = vec![];
            let mut x = lo;
            while x <= hi + 1e-9 {
                xs.push(x);
                x += step;
            }
            if !xs.iter().any(|x| (x - v).abs() < 1e-9) {
                xs.push(v);
                xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
            }
            Some((xs, false))
        }
        _ => None,
    }
}

// ------------------------------------------------------------------------------------------------ value maps
struct FieldSpec {
    key: &'static str,
    out: &'static str,
    x: &'static str,
    xa: Ax,
    y: &'static str,
    ya: Ax,
    /// shade by the logarithm of the value (values spanning many orders of magnitude)
    log_shade: bool,
}

const FIELDS: &[FieldSpec] = &[
    FieldSpec { key: "act_annuity_certain", out: "a_n", x: "n", xa: Ax::Int(1.0, 20.0), y: "i", ya: Ax::Log(10.0), log_shade: false },
    FieldSpec { key: "act_constant_force_mortality", out: "tpx", x: "mu", xa: Ax::Log(10.0), y: "t", ya: Ax::Log(10.0), log_shade: false },
    FieldSpec { key: "act_exponential_deductible", out: "per_loss", x: "theta", xa: Ax::Log(10.0), y: "d", ya: Ax::Log(10.0), log_shade: false },
    FieldSpec { key: "act_fractional_age", out: "spx", x: "q", xa: Ax::Lin(0.02, 0.98), y: "s", ya: Ax::Lin(0.02, 0.98), log_shade: false },
    FieldSpec { key: "act_increasing_decreasing_annuity", out: "Ia", x: "n", xa: Ax::Int(1.0, 20.0), y: "i", ya: Ax::Log(10.0), log_shade: false },
    FieldSpec { key: "act_limited_fluctuation_full_credibility", out: "n_full", x: "k", xa: Ax::Log(4.0), y: "p", ya: Ax::Lin(0.5, 0.99), log_shade: true },
    FieldSpec { key: "act_mthly_annuity_udd", out: "a_m", x: "i", xa: Ax::Log(10.0), y: "m", ya: Ax::Int(1.0, 24.0), log_shade: false },
    FieldSpec { key: "act_net_premium_whole_life", out: "P", x: "S", xa: Ax::Around(0.5), y: "i", ya: Ax::Log(10.0), log_shade: false },
    FieldSpec { key: "act_nominal_to_effective_rate", out: "i", x: "rate", xa: Ax::Around(0.5), y: "m", ya: Ax::Int(1.0, 24.0), log_shade: false },
    FieldSpec { key: "act_poisson_frequency", out: "pmf", x: "k", xa: Ax::Int(0.0, 12.0), y: "lam", ya: Ax::Log(10.0), log_shade: false },
    FieldSpec { key: "act_poisson_gamma_credibility", out: "post_mean", x: "alpha", xa: Ax::Around(0.5), y: "theta", ya: Ax::Log(10.0), log_shade: false },
    FieldSpec { key: "act_single_to_multiple_decrement", out: "q1", x: "q1s", xa: Ax::Lin(0.02, 0.98), y: "q2s", ya: Ax::Lin(0.02, 0.98), log_shade: false },
    FieldSpec { key: "act_stop_loss_premium", out: "premium", x: "lam", xa: Ax::Log(10.0), y: "d", ya: Ax::Int(0.0, 10.0), log_shade: true },
];

fn field(s: &Solved, f: &FieldSpec) -> Option<Figure> {
    let (vx, vy, vz) = (num_in(s, f.x)?, num_in(s, f.y)?, s.out(f.out)?);
    let (xs, lx) = axis_values(f.xa, vx, 17)?;
    let (ys, ly) = axis_values(f.ya, vy, 13)?;
    if xs.len() * ys.len() > s.left().saturating_sub(40) {
        return None;
    }
    let mut vals: Vec<Vec<Option<f64>>> = vec![];
    let mut n = 0;
    for x in &xs {
        let mut col = vec![];
        for y in &ys {
            let z = at(s, &[(f.x, *x), (f.y, *y)]).and_then(|a| k::num_of(&a, f.out));
            let z = z.filter(|z| !f.log_shade || *z > 0.0);
            n += z.is_some() as usize;
            col.push(z);
        }
        vals.push(col);
    }
    if n < 4 {
        return None;
    }
    let all: Vec<f64> = vals.iter().flatten().flatten().copied().collect();
    let (lo, hi) = all.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |a, z| (a.0.min(*z), a.1.max(*z)));
    let shaded: Vec<Vec<Option<f64>>> = if f.log_shade {
        vals.iter().map(|c| c.iter().map(|z| z.map(|z| z.log10())).collect()).collect()
    } else {
        vals.clone()
    };
    let header = format!("{} = {}", f.out, num(vz));
    let mut body = k::field(&xs, &ys, &shaded, (vx, vy), &axis(s, f.x), &axis(s, f.y), &header, (lx, ly));
    if body.is_empty() {
        return None;
    }
    if f.log_shade {
        // the scale's ends as values, not their logarithms
        let (llo, lhi) = (lo.log10(), hi.log10());
        let shown = format!("light {} to dark {}</text>", num(k::snap(llo, lhi)), num(lhi));
        let real = format!("light {} to dark {} (log)</text>", num(lo), num(hi));
        if !body.contains(&shown) {
            return None;
        }
        body = body.replacen(&shown, &real, 1);
    }
    let title = cap1(&format!("{} over {} and {}", out_meaning(s, f.out), f.x, f.y));
    let caption = format!("{} ({}) over {} from {} to {} and {} from {} to {}{}: each of the {} cells is one run of the \
                           model, shaded by its value (light {} to dark {}{}); refused combinations are left blank. \
                           Marked: these inputs, {} = {}, {} = {}, {} = {}.",
                          f.out, out_meaning(s, f.out), f.x, num(xs[0]), num(xs[xs.len() - 1]), f.y, num(ys[0]),
                          num(ys[ys.len() - 1]), match (lx, ly) {
                              (true, true) => " (logarithmic axes)",
                              (true, false) => " (logarithmic horizontal axis)",
                              (false, true) => " (logarithmic vertical axis)",
                              _ => "",
                          }, n, num(lo), num(hi), if f.log_shade { ", logarithmic shading" } else { "" },
                          f.x, num(vx), f.y, num(vy), f.out, num(vz));
    Some(s.chart("field", &title, &caption, &body))
}

fn cap1(t: &str) -> String {
    let mut c = t.chars();
    c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
}

fn out_meaning(s: &Solved, sym: &str) -> String {
    let m = s.p.procedure(s.key).and_then(|p| p.outputs.iter().find(|o| o.symbol == sym).map(|o| o.meaning.clone()))
        .unwrap_or_default();
    if m.is_empty() { sym.to_string() } else { clip(&m, 60) }
}

// ------------------------------------------------------------------------------------------------ curves
struct CurveSpec {
    key: &'static str,
    kind: &'static str,
    out: &'static str,
    x: &'static str,
    xa: Ax,
}

const CURVES: &[CurveSpec] = &[
    CurveSpec { key: "act_annuity_certain", kind: "a-n-vs-n", out: "a_n", x: "n", xa: Ax::Int(1.0, 15.0) },
    CurveSpec { key: "act_limited_fluctuation_full_credibility", kind: "n-full-vs-p", out: "n_full", x: "p", xa: Ax::Lin(0.5, 0.99) },
    CurveSpec { key: "act_multiple_decrement_to_single", kind: "q1-single-vs-q1", out: "q1_single", x: "q1", xa: Ax::Around(0.5) },
    CurveSpec { key: "act_pareto_deductible", kind: "per-loss-vs-alpha", out: "per_loss", x: "alpha", xa: Ax::Around(0.5) },
];

fn curve(s: &Solved, c: &CurveSpec) -> Option<Figure> {
    let (vx, vy) = (num_in(s, c.x)?, s.out(c.out)?);
    let (xs, _) = axis_values(c.xa, vx, 31)?;
    let pts: Vec<(f64, f64)> = xs.iter()
        .filter_map(|x| at(s, &[(c.x, *x)]).and_then(|a| k::num_of(&a, c.out)).map(|y| (*x, y))).collect();
    if pts.len() < 3 {
        return None;
    }
    let fixed: Vec<String> = s.p.procedure(s.key)?.inputs.iter().filter(|i| i.symbol != c.x)
        .filter_map(|i| num_in(s, &i.symbol).map(|v| format!("{} = {}", i.symbol, num(v)))).collect();
    let series = vec![(String::new(), pts.clone())];
    let body = k::lines_y(&series, &axis(s, c.x), &axis(s, c.out), &format!("{} = {}", c.out, num(vy)), Some((vx, vy)),
                          &format!("{} = {}", c.x, num(vx)), false, true);
    if body.is_empty() {
        return None;
    }
    let fixed_t = if fixed.is_empty() { String::new() } else { format!(", with {}", fixed.join(", ")) };
    Some(s.chart(c.kind, &format!("{} against {}", c.out, c.x),
                 &format!("{} ({}) against {} from {} to {}{}: {} points, each computed by the model. Marked: {} = {}, \
                           {} = {}.", c.out, out_meaning(s, c.out), c.x, num(pts[0].0), num(pts[pts.len() - 1].0),
                          fixed_t, pts.len(), c.x, num(vx), c.out, num(vy)), &body))
}

// ------------------------------------------------------------------------------------------------ matrices
/// (capability, input matrices, returned lists)
const MATRICES: &[(&str, &[&str], &[&str])] = &[
    ("act_bornhuetter_ferguson", &["T"], &["ultimate"]),
    ("act_buhlmann_empirical", &["X"], &["premiums"]),
    ("act_buhlmann_straub", &["X", "m"], &["Z"]),
    ("act_cape_cod", &["T"], &["ultimate"]),
    ("act_chain_ladder_factors", &["T"], &["cdf"]),
    ("act_chain_ladder_reserve", &["T"], &["ultimate"]),
    ("act_chain_ladder_simple_average", &["T"], &["ultimate"]),
    ("act_mack_standard_error", &["T"], &["se"]),
];

/// Panels side by side, each shaded by its own largest magnitude: the input matrices, then each returned list as a
/// column (one entry per row) or a row (one per column).
fn matrices(s: &Solved, ins: &[&str], outs: &[&str]) -> Option<Figure> {
    let mut panels: Vec<(String, Vec<Vec<f64>>)> = vec![];
    for m in ins {
        panels.push((format!("{m} (input)"), s.mat(m)?));
    }
    let (r0, c0) = (panels[0].1.len(), panels[0].1[0].len());
    for o in outs {
        let v = s.out_list(o)?;
        let m = if v.len() == r0 { v.iter().map(|x| vec![*x]).collect() } else if v.len() == c0 { vec![v] } else {
            return None;
        };
        panels.push((format!("{o} (returned)"), m));
    }
    let rows = panels.iter().map(|p| p.1.len()).max()?;
    let cols: usize = panels.iter().map(|p| p.1[0].len()).sum();
    let gap = 26.0;
    let left = 30.0;
    let avail = 520.0 - left - 12.0 - gap * (panels.len() - 1) as f64;
    let cell = (avail / cols as f64).min(300.0 / rows as f64).min(64.0);
    if cell < 6.0 {
        return None;
    }
    let width = cell * cols as f64 + gap * (panels.len() - 1) as f64;
    let mut x = left + (520.0 - left - 12.0 - width) / 2.0;
    let y0 = 34.0;
    let font = (cell * 0.3).clamp(7.0, 13.0);
    let show = cell >= 26.0;
    let mut c = Canvas::new(y0 + cell * rows as f64 + 50.0);
    for i in 0..rows {
        c.name(x - 6.0, y0 + cell * (i as f64 + 0.5) + 4.0, "end", &(i + 1).to_string());
    }
    for (name, m) in &panels {
        let mx = m.iter().flatten().fold(0.0f64, |a, v| a.max(v.abs()));
        let w = m[0].len();
        for (i, row) in m.iter().enumerate() {
            for (j, v) in row.iter().enumerate() {
                let (cx, cy) = (x + cell * j as f64, y0 + cell * i as f64);
                let op = if mx > 0.0 { 0.08 + 0.62 * v.abs() / mx } else { 0.1 };
                c.raw(&format!("<rect class=\"{}\" x=\"{cx:.1}\" y=\"{cy:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
                                fill-opacity=\"{op:.2}\"/>", if *v < 0.0 { "axp-hn" } else { "axp-hp" }, cell - 1.5,
                               cell - 1.5));
                if show {
                    let t = num_short(*v, ((cell - 4.0) / (font * 0.6)) as usize);
                    c.raw(&format!("<text class=\"axp-ht\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" \
                                    font-size=\"{font:.0}\">{t}</text>", cx + cell / 2.0, cy + cell / 2.0 + font * 0.35));
                }
            }
        }
        for j in 0..w {
            if w <= 12 || j % 2 == 0 {
                c.name(x + cell * (j as f64 + 0.5), y0 + cell * m.len() as f64 + 15.0, "middle", &(j + 1).to_string());
            }
        }
        c.bold(x + cell * w as f64 / 2.0, y0 + cell * m.len() as f64 + 36.0, "middle", name);
        x += cell * w as f64 + gap;
    }
    let title = s.p.procedure(s.key).map(|p| format!("{}: the matrices", p.name)).unwrap_or_default();
    let others: Vec<String> = s.a.values.iter()
        .filter(|(k_, v, _)| !outs.contains(&k_.as_str()) && v.as_f64().is_some())
        .map(|(k_, v, _)| format!("{k_} = {}", num(v.as_f64().unwrap_or(f64::NAN)))).collect();
    let caption = format!("{} as heat maps (rows and columns numbered; each panel shaded by its own largest magnitude, \
                           negative values in the second colour), beside {} as the model returns {} for these inputs.{}",
                          ins.join(" and "), outs.join(" and "), if outs.len() == 1 { "it" } else { "them" },
                          if others.is_empty() { String::new() } else { format!(" Also returned: {}.", others.join(", ")) });
    Some(s.figure("matrix", &title, &caption, c.body(), c.h))
}

/// A cell value written in at most `n` characters (4 significant digits, fewer when the cell is narrow).
fn num_short(v: f64, n: usize) -> String {
    for d in (1..=4).rev() {
        let t = ax_engine::num::py_g(v, d);
        if t.chars().count() <= n.max(3) {
            return t;
        }
    }
    ax_engine::num::py_g(v, 1)
}

// ------------------------------------------------------------------------------------------------ inputs and results
fn val_text(v: &Val) -> Option<String> {
    Some(match v {
        Val::Num(x) => num(*x),
        Val::Int(i) => i.to_string(),
        Val::Text(t) => t.clone(),
        Val::Bool(b) => b.to_string(),
        Val::Vec(v) => {
            let mut t = v.iter().take(4).map(|x| num(*x)).collect::<Vec<_>>().join(", ");
            if v.len() > 4 {
                t.push_str(", …");
            }
            format!("[{t}]")
        }
        _ => return None,
    })
}

fn io(s: &Solved) -> Option<Figure> {
    let p = s.p.procedure(s.key)?;
    let ins: Vec<String> = p.inputs.iter().filter_map(|i| Some(clip(&format!("{} = {}", i.symbol, val_text(&s.val(&i.symbol)?)?), 30)))
        .collect();
    let outs: Vec<String> = s.a.values.iter().filter_map(|(k_, v, _)| v.as_f64().map(|x| format!("{k_} = {}", num(x))))
        .collect();
    if ins.is_empty() || outs.is_empty() {
        return None;
    }
    let half = ins.len().div_ceil(2);
    let rows_in = half as f64;
    let mut c = Canvas::new(0.0);
    let mut y = 26.0;
    c.h = 1000.0;
    c.bold(16.0, y, "start", "Stated");
    y += 22.0;
    for (j, t) in ins.iter().enumerate() {
        let (col, row) = (j / half, j % half);
        c.value(30.0 + 250.0 * col as f64, y + 20.0 * row as f64, "start", t);
    }
    y += 20.0 * rows_in + 4.0;
    c.arrow((260.0, y), (260.0, y + 26.0));
    y += 32.0;
    c.block(60.0, y, 400.0, 40.0, &clip(&p.name, 48));
    y += 46.0;
    c.arrow((260.0, y), (260.0, y + 26.0));
    y += 44.0;
    c.bold(16.0, y, "start", "Returned");
    y += 22.0;
    let half_o = outs.len().div_ceil(2);
    for (j, t) in outs.iter().enumerate() {
        let (col, row) = (j / half_o, j % half_o);
        c.value(30.0 + 250.0 * col as f64, y + 20.0 * row as f64, "start", t);
    }
    y += 20.0 * half_o as f64;
    c.h = y + 6.0;
    let caption = format!("The question as stated ({}) and what the model returns for it ({}).", ins.join(", "),
                          outs.join(", "));
    Some(s.figure("io", &format!("{}: inputs and results", p.name), &caption, c.body(), c.h))
}

// ------------------------------------------------------------------------------------------------ cash flows
/// Each payment and the present value the model gives it alone (the same rate), as bars by payment time.
fn cash_flows(s: &Solved) -> Option<Figure> {
    let (am, tm, i) = (s.vec("amounts")?, s.vec("times")?, num_in(s, "i")?);
    if am.len() != tm.len() || am.is_empty() || am.len() > 24 {
        return None;
    }
    let mut pv = vec![];
    for (a, t) in am.iter().zip(&tm) {
        let b = s.with_input(&[("amounts", list_json(&[*a])), ("times", list_json(&[*t]))])?;
        pv.push(k::num_of(&b, "PV")?);
    }
    let labels: Vec<String> = tm.iter().map(|t| format!("t = {}", num(*t))).collect();
    let groups = vec![("amount (input)".to_string(), am.iter().map(|x| Some(*x)).collect()),
                      ("present value".to_string(), pv.iter().map(|x| Some(*x)).collect())];
    let total = s.out("PV")?;
    let body = k::bar_groups(&labels, &groups, "payment time (years)", "amount", &format!("PV = {} at i = {}", num(total), num(i)));
    let also: Vec<String> = s.a.values.iter().filter_map(|(k_, v, _)| v.as_f64().map(|x| format!("{k_} = {}", num(x)))).collect();
    Some(s.chart("xy", "Cash flows and their present values",
                 &format!("The payments {} at times {} (years) and, beside each, its present value at i = {} as the model \
                           computes it for that payment alone: {}. Returned for all of them: {}.",
                          am.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "),
                          tm.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "), num(i),
                          pv.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "), also.join(", ")), &body))
}

/// Paid to date, the returned ultimate and reserve, by origin year (earned premium as the label).
fn origin_years(s: &Solved) -> Option<Figure> {
    let (pr, paid, ult, res) = (s.vec("P")?, s.vec("paid")?, s.out_list("ultimate")?, s.out_list("reserve")?);
    let n = pr.len();
    if paid.len() != n || ult.len() != n || res.len() != n || n > 20 {
        return None;
    }
    let labels: Vec<String> = (0..n).map(|j| format!("{} (P {})", j + 1, num(pr[j]))).collect();
    let g = |v: &[f64]| v.iter().map(|x| Some(*x)).collect::<Vec<_>>();
    let groups = vec![("paid (input)".to_string(), g(&paid)), ("ultimate (returned)".to_string(), g(&ult)),
                      ("reserve (returned)".to_string(), g(&res))];
    let tot = s.out("total_reserve")?;
    let body = k::bar_groups(&labels, &groups, "origin year (earned premium)", "losses", &format!("total reserve = {}", num(tot)));
    Some(s.chart("xy", "Paid losses, expected ultimate and reserve by origin year",
                 &format!("For each origin year: losses paid to date (input), and the ultimate and reserve the model \
                           returns: ultimate {}; reserve {}; total reserve {}.",
                          ult.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "),
                          res.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "), num(tot)), &body))
}

/// The observed times, deaths in the accent colour and censored times in grey.
fn observed_times(s: &Solved) -> Option<Figure> {
    let (tm, ev) = (s.vec("times")?, s.vec("events")?);
    if tm.len() != ev.len() || tm.is_empty() || tm.len() > 60 {
        return None;
    }
    let (mean, hz) = (s.out("mean")?, s.out("hazard")?);
    let labels: Vec<String> = (1..=tm.len()).map(|j| j.to_string()).collect();
    let hl: Vec<usize> = ev.iter().enumerate().filter(|(_, e)| **e != 0.0).map(|(j, _)| j).collect();
    let body = k::bars(&labels, &tm, &hl, "observation (accent: event, grey: censored)", "observed time",
                       &format!("mean = {}, hazard = {}", num(mean), num(hz)));
    let also: Vec<String> = s.a.values.iter().filter_map(|(k_, v, _)| v.as_f64().map(|x| format!("{k_} = {}", num(x)))).collect();
    Some(s.chart("data", "Observed times, events and censoring",
                 &format!("The observed times {} in order, an event (death) in the accent colour and a censored time in \
                           grey; the model returns {}.", tm.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "),
                          also.join(", ")), &body))
}
