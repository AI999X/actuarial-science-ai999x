//! The question reader on Actuarial Science's documented refused questions (ask.rs): the documented question itself,
//! word for word, keeps its documented outcome even when its words, numbers free, are also a capability's documented
//! question ("Interest rate conversion: i = 5, m = 12" is UNRECOGNIZED_FORMAT, as the package answers it, not
//! AMBIGUOUS); and no documented refused question is ever answered with a number. Found by the release gate on the
//! Rust-engine wheel (tools/contract_acceptance.py --wheel, templates 385/386, 2026-11-03).

use actuarial_science_ai999x::pack;
use serde_json::Value;

fn ask(q: &str) -> Value {
    serde_json::from_str(&ax_engine::ask::ask_json(pack(), q)).unwrap()
}

#[test]
fn the_documented_refused_question_keeps_its_outcome() {
    let a = ask("Interest rate conversion: i = 5, m = 12");
    assert_eq!(a["outcome"], "UNRECOGNIZED_FORMAT", "{a}");
}

#[test]
fn every_documented_refused_question_has_its_documented_outcome() {
    let doc: Value = serde_json::from_str(actuarial_science_ai999x::PACK_JSON).unwrap();
    let mut n = 0;
    for f in doc["forms"].as_array().unwrap() {
        let want = f["outcome"].as_str().unwrap_or("");
        if f.get("key").is_some() || want == "SOLVED" || want.is_empty() {
            continue;
        }
        n += 1;
        let a = ask(f["q"].as_str().unwrap());
        assert_eq!(a["outcome"], want, "{}: {a}", f["q"]);
    }
    assert!(n >= 10, "only {n} documented refusals");
}
