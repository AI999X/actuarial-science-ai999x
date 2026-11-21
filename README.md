# Actuarial Science and Insurance (actuarial-science-ai999x)

[![tests](https://github.com/AI999X/actuarial-science-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/actuarial-science-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/actuarial-science-ai999x)](https://pypi.org/project/actuarial-science-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/actuarial-science-ai999x)

Actuarial Science and Insurance covers life tables and survival models, life insurances and annuities, net and gross premiums, reserves and Thiele's equation, interest theory, loss models with deductibles and limits, Panjer's recursion, VaR and TVaR, credibility and loss reserving (chain-ladder, Bornhuetter-Ferguson, Cape Cod, Mack): 93 calculations in 15 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Actuarial Science and Insurance desktop app: Net annual premium for whole life insurance, with its worked example and live scene](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/hero.webp)](https://www.ai999x.com/models/actuarial-science-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-actuarial-science-1.6.5-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-actuarial-science-1.6.5-linux-x86_64.tar.gz) |
| **Command line** `actuarial-science` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/actuarial-science-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install actuarial-science-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Actuarial Science and Insurance desktop app: every scene and chart is drawn from the computed answer.

[![Actuarial Science and Insurance desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/clip.mp4)*

| | |
|---|---|
| [![Net annual premium for whole life insurance: the live scene in the Actuarial Science and Insurance desktop app](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/scene-1.webp)](https://www.ai999x.com/models/actuarial-science-ai999x/#cap-act_net_premium_whole_life)<br>Net annual premium for whole life insurance | [![Chain-ladder ultimate losses and reserves: the live scene in the Actuarial Science and Insurance desktop app](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/scene-2.webp)](https://www.ai999x.com/models/actuarial-science-ai999x/#cap-act_chain_ladder_reserve)<br>Chain-ladder ultimate losses and reserves |
| [![Annuity-certain present and accumulated values: the live scene in the Actuarial Science and Insurance desktop app](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/scene-3.webp)](https://www.ai999x.com/models/actuarial-science-ai999x/#cap-act_annuity_certain)<br>Annuity-certain present and accumulated values | [![Buhlmann credibility premium (known structural parameters): the live scene in the Actuarial Science and Insurance desktop app](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/scene-4.webp)](https://www.ai999x.com/models/actuarial-science-ai999x/#cap-act_buhlmann_credibility)<br>Buhlmann credibility premium (known structural parameters) |
| [![Ordinary deductible with Pareto losses: the live scene in the Actuarial Science and Insurance desktop app](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/scene-5.webp)](https://www.ai999x.com/models/actuarial-science-ai999x/#cap-act_pareto_deductible)<br>Ordinary deductible with Pareto losses | [![Deferred whole life annuity-due: the live scene in the Actuarial Science and Insurance desktop app](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/scene-6.webp)](https://www.ai999x.com/models/actuarial-science-ai999x/#cap-act_deferred_annuity)<br>Deferred whole life annuity-due |
| [![Continuous whole life insurance and annuity under UDD: the live scene in the Actuarial Science and Insurance desktop app](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/scene-7.webp)](https://www.ai999x.com/models/actuarial-science-ai999x/#cap-act_continuous_insurance_udd)<br>Continuous whole life insurance and annuity under UDD | [![Death probability from the central death rate: the live scene in the Actuarial Science and Insurance desktop app](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/scene-8.webp)](https://www.ai999x.com/models/actuarial-science-ai999x/#cap-act_central_death_rate)<br>Death probability from the central death rate |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Net annual premium for whole life insurance: a chart in the Actuarial Science and Insurance desktop app](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/chart-1.webp)](https://www.ai999x.com/models/actuarial-science-ai999x/#cap-act_net_premium_whole_life)<br>Net annual premium for whole life insurance | [![Chain-ladder ultimate losses and reserves: a chart in the Actuarial Science and Insurance desktop app](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/chart-2.webp)](https://www.ai999x.com/models/actuarial-science-ai999x/#cap-act_chain_ladder_reserve)<br>Chain-ladder ultimate losses and reserves | [![Annuity-certain present and accumulated values: a chart in the Actuarial Science and Insurance desktop app](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/chart-3.webp)](https://www.ai999x.com/models/actuarial-science-ai999x/#cap-act_annuity_certain)<br>Annuity-certain present and accumulated values | [![Buhlmann credibility premium (known structural parameters): a chart in the Actuarial Science and Insurance desktop app](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/chart-4.webp)](https://www.ai999x.com/models/actuarial-science-ai999x/#cap-act_buhlmann_credibility)<br>Buhlmann credibility premium (known structural parameters) |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import actuarial_science_ai999x as actuarial_science

a = actuarial_science.ask(
    "Buhlmann credibility premium: mu = 1000, EPV = 50000, VHM = "
    "10000, n = 5, xbar = 1200"
)
print(a.outcome, a.values)
# SOLVED {'k': 5.0, 'Z': 0.5, 'premium': 1100.0}

a = actuarial_science.ask(
    "Deferred annuity: q = [0.1, 0.2, 0.5, 1], i = 0.05, u = 2, P = "
    "1000"
)
print(a.outcome, a.values)
# SOLVED {'ua_due': 0.964042759961, 'EPV': 964.042759961}

a = actuarial_science.ask(
    "Compound Poisson moments: lam = 3, m1 = 2, m2 = 6, m3 = 24"
)
print(a.outcome, a.values)
# SOLVED {'mean': 6.0, 'var': 18.0, 'skew': 0.942809041582}

a = actuarial_science.ask(
    "Pure premium rate: L = 500000, X = 2000, Fe = 30, V = 0.15, Q = "
    "0.05"
)
print(a.outcome, a.values)
# SOLVED {'pure_premium': 250.0, 'rate': 350.0}

a = actuarial_science.ask(
    "VaR and TVaR of a lognormal loss: mu = 7, sigma = 1, p = 0.95"
)
print(a.outcome, a.values)
# SOLVED {'VaR': 5680.83567601, 'TVaR': 9384.13872632}

a = actuarial_science.ask(
    "Expectation of life: lx = [1000, 900, 700, 400, 0], x0 = 95, x = "
    "95"
)
print(a.outcome, a.values)
# SOLVED {'e_curtate': 2.0, 'e_complete': 2.5}

a = actuarial_science.ask(
    "Multiple decrement probabilities: q1s = 0.1, q2s = 0.2"
)
print(a.outcome, a.values)
# SOLVED {'q1': 0.09, 'q2': 0.19, 'q_total': 0.28}

a = actuarial_science.ask(
    "Reversionary annuity: qx = [0.1, 0.2, 0.5, 1], qy = [0.05, 0.1, "
    "0.3, 0.6, 1], i = 0.05, P = 1000"
)
print(a.outcome, a.values)
# SOLVED {'a_rev': 0.835458373826, 'EPV': 835.458373826}

a = actuarial_science.ask(
    "Loss at issue: q = [0.1, 0.2, 0.5, 1], i = 0.05, S = 1000, P = "
    "450"
)
print(a.outcome, a.values)
# SOLVED {'mean': -403.87569994, 'sd': 434.100555804, 'prob_loss': 0.28}

a = actuarial_science.ask(
    "Loss ratio rate change: L = 650000, EP = 1000000, F = 0.1, V = "
    "0.15, Q = 0.05"
)
print(a.outcome, a.values)
# SOLVED {'change': -0.0625, 'loss_ratio': 0.65, 'permissible': 0.8}

a = actuarial_science.ask(
    "Gross premium reserve: q = [0.1, 0.2, 0.5, 1], i = 0.05, S = "
    "1000, e0 = 0.4, r = 0.05, c = 10, E = 20, t = 1"
)
print(a.outcome, a.values)
# SOLVED {'V': 148.731075614, 'G': 391.044393734}

a = actuarial_science.ask(
    "VaR and TVaR of a normal loss: mu = 1000, sigma = 200, p = 0.95"
)
print(a.outcome, a.values)
# SOLVED {'VaR': 1328.97072539, 'TVaR': 1412.5425615}

a = actuarial_science.ask(
    "Kaplan-Meier estimate: times = [1, 2, 2, 3, 5], events = [1, 1, "
    "0, 1, 0], t = 3"
)
print(a.outcome, a.values)
# SOLVED {'S': 0.3, 'F': 0.7, 'deaths': 3}

a = actuarial_science.ask(
    "Death probability from the central death rate: m = 0.02, "
    "assumption = udd"
)
print(a.outcome, a.values)
# SOLVED {'q': 0.019801980198, 'p': 0.980198019802}

a = actuarial_science.ask(
    "Net premium for endowment insurance: q = [0.1, 0.2, 0.5, 1], i = "
    "0.05, n = 3, S = 1000"
)
print(a.outcome, a.values)
# SOLVED {'P': 350.754936121, 'A': 0.880466472303, 'a_due': 2.51020408163}
```

## Command line: 15 examples

Each command below was run with `actuarial-science` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ actuarial-science solve act_buhlmann_credibility mu=1000 EPV=50000 VHM=10000 n=5 xbar=1200
k = 5
Z = 0.5
premium = 1100

$ actuarial-science solve act_accumulated_value P=1000 i=0.05 t=10
AV = 1628.894627
PV = 613.9132535
factor = 1.628894627

$ actuarial-science solve act_deferred_annuity "q=[0.1, 0.2, 0.5, 1]" i=0.05 u=2 P=1000
ua_due = 0.96404276
EPV = 964.04276

$ actuarial-science solve act_continuous_insurance_udd "q=[0.1, 0.2, 0.5, 1]" i=0.05
A_bar = 0.8871232988
A = 0.8656578278
a_bar = 2.313513453

$ actuarial-science solve act_commutation_functions "lx=[1000, 900, 700, 400, 0]" x0=95 x=95 i=0.05
D_x = 9.705470376
N_x = 27.54022849
C_x = 0.924330512
M_x = 8.394030925
A_x = 0.8648762604
a_due_x = 2.837598531

$ actuarial-science solve act_compound_poisson_moments lam=3 m1=2 m2=6 m3=24
mean = 6
var = 18
skew = 0.9428090416

$ actuarial-science solve act_bornhuetter_ferguson "T=[[100, 150, 175], [110, 168, 0], [120, 0, 0]]" "P=[200, 210, 220]" elr=0.8
ultimate = [175.0,192.0,196.377358491]
reserve = [0.0,24.0,76.3773584906]
total_reserve = 100.3773585

$ actuarial-science solve act_multiple_decrement_to_single q1=0.1 q2=0.2
q1_single = 0.1120959983
q2_single = 0.2116264837
q_total = 0.3

$ actuarial-science solve act_joint_life_last_survivor "qx=[0.1, 0.2, 0.5, 1]" "qy=[0.05, 0.1, 0.3, 0.6, 1]" i=0.05
a_joint = 2.55877551
a_last = 3.656643991
A_joint = 0.8781535471
A_last = 0.8258740957

$ actuarial-science solve act_gross_premium "q=[0.1, 0.2, 0.5, 1]" i=0.05 S=1000 e0=0.4 r=0.05 c=10 E=20
G = 391.0443937
P = 306.8418549
loading = 84.20253884

$ actuarial-science solve act_pure_premium_rate L=500000 X=2000 Fe=30 V=0.15 Q=0.05
pure_premium = 250
rate = 350

$ actuarial-science solve act_makeham_continuous_whole_life A=0.00022 B=2.7e-06 c=1.124 x=50 i=0.05 S=100000 t=10
a_bar = 16.52037321
A_bar = 0.1939682791
P = 1174.1156
V = 12836.47174

$ actuarial-science solve act_var_tvar_lognormal mu=7 sigma=1 p=0.95
VaR = 5680.835676
TVaR = 9384.138726

$ actuarial-science solve act_exponential_mle "times=[1, 2, 2, 3, 5]" "events=[1, 1, 0, 1, 0]" t=3
hazard = 0.2307692308
mean = 4.333333333
S = 0.5004199204

$ actuarial-science solve act_constant_force_mortality mu=0.02 t=10
tpx = 0.8187307531
tqx = 0.1812692469
e_complete = 50
e_curtate = 49.50166666
var_T = 2500
```

## Rust library: 15 examples

The crate `actuarial-science-ai999x` (in `rust/` of the GitHub repository) answers a JSON request with `actuarial_science_ai999x::solve_json`. Each call below was compiled and run on this release; the comment under it is what it printed. The whole program is `rust/actuarial-science/examples/readme.rs`: `cd rust && cargo run --release --example readme -p actuarial-science-ai999x`.

```rust
use serde_json::{json, Value};

fn show(request: Value) {
    let answer: Value = serde_json::from_str(&actuarial_science_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

show(json!({"key": "act_buhlmann_credibility", "inputs": {"mu": "1000", "EPV": "50000", "VHM": "10000", "n": "5", "xbar": "1200"}}));
// SOLVED
// k = 5.0
// Z = 0.5
// premium = 1100.0

show(json!({"key": "act_accumulated_value", "inputs": {"P": "1000", "i": "0.05", "t": "10"}}));
// SOLVED
// AV = 1628.89462678
// PV = 613.913253541
// factor = 1.62889462678

show(json!({"key": "act_deferred_annuity", "inputs": {"q": [0.1, 0.2, 0.5, 1], "i": "0.05", "u": "2", "P": "1000"}}));
// SOLVED
// ua_due = 0.964042759961
// EPV = 964.042759961

show(json!({"key": "act_continuous_insurance_udd", "inputs": {"q": [0.1, 0.2, 0.5, 1], "i": "0.05"}}));
// SOLVED
// A_bar = 0.887123298818
// A = 0.865657827757
// a_bar = 2.31351345304

show(json!({"key": "act_commutation_functions", "inputs": {"lx": [1000, 900, 700, 400, 0], "x0": "95", "x": "95", "i": "0.05"}}));
// SOLVED
// D_x = 9.7054703765
// N_x = 27.5402284876
// C_x = 0.924330512048
// M_x = 8.39403092471
// A_x = 0.864876260406
// a_due_x = 2.83759853148

show(json!({"key": "act_compound_poisson_moments", "inputs": {"lam": "3", "m1": "2", "m2": "6", "m3": "24"}}));
// SOLVED
// mean = 6.0
// var = 18.0
// skew = 0.942809041582

show(json!({"key": "act_bornhuetter_ferguson", "inputs": {"T": [[100, 150, 175], [110, 168, 0], [120, 0, 0]], "P": [200, 210, 220], "elr": "0.8"}}));
// SOLVED
// ultimate = [175.0,192.0,196.377358491]
// reserve = [0.0,24.0,76.3773584906]
// total_reserve = 100.377358491

show(json!({"key": "act_multiple_decrement_to_single", "inputs": {"q1": "0.1", "q2": "0.2"}}));
// SOLVED
// q1_single = 0.112095998257
// q2_single = 0.211626483689
// q_total = 0.3

show(json!({"key": "act_joint_life_last_survivor", "inputs": {"qx": [0.1, 0.2, 0.5, 1], "qy": [0.05, 0.1, 0.3, 0.6, 1], "i": "0.05"}}));
// SOLVED
// a_joint = 2.5587755102
// a_last = 3.65664399093
// A_joint = 0.878153547133
// A_last = 0.82587409567

show(json!({"key": "act_gross_premium", "inputs": {"q": [0.1, 0.2, 0.5, 1], "i": "0.05", "S": "1000", "e0": "0.4", "r": "0.05", "c": "10", "E": "20"}}));
// SOLVED
// G = 391.044393734
// P = 306.841854896
// loading = 84.2025388389

show(json!({"key": "act_pure_premium_rate", "inputs": {"L": "500000", "X": "2000", "Fe": "30", "V": "0.15", "Q": "0.05"}}));
// SOLVED
// pure_premium = 250.0
// rate = 350.0

show(json!({"key": "act_makeham_continuous_whole_life", "inputs": {"A": "0.00022", "B": "2.7e-06", "c": "1.124", "x": "50", "i": "0.05", "S": "100000", "t": "10"}}));
// SOLVED
// a_bar = 16.5203732076
// A_bar = 0.193968279062
// P = 1174.11560033
// V = 12836.4717411

show(json!({"key": "act_var_tvar_lognormal", "inputs": {"mu": "7", "sigma": "1", "p": "0.95"}}));
// SOLVED
// VaR = 5680.83567601
// TVaR = 9384.13872632

show(json!({"key": "act_exponential_mle", "inputs": {"times": [1, 2, 2, 3, 5], "events": [1, 1, 0, 1, 0], "t": "3"}}));
// SOLVED
// hazard = 0.230769230769
// mean = 4.33333333333
// S = 0.500419920361

show(json!({"key": "act_constant_force_mortality", "inputs": {"mu": "0.02", "t": "10"}}));
// SOLVED
// tpx = 0.818730753078
// tqx = 0.181269246922
// e_complete = 50.0
// e_curtate = 49.5016666556
// var_T = 2500.0
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/actuarial-science-ai999x/blob/main/docs/actuarial_science_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/actuarial-science-ai999x/blob/main/docs/actuarial_science_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/actuarial-science-ai999x/blob/main/docs/actuarial_science_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/actuarial-science-ai999x/blob/main/docs/actuarial_science_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/actuarial-science-ai999x/blob/main/docs/actuarial_science_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/actuarial-science-ai999x/blob/main/docs/actuarial_science_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/actuarial-science-ai999x/blob/main/docs/actuarial_science_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/actuarial-science-ai999x/blob/main/docs/actuarial_science_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/actuarial-science-ai999x/blob/main/docs/actuarial_science_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Actuarial Science and Insurance on ai999x.com](https://www.ai999x.com/models/actuarial-science-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/actuarial-science-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.6`; the [GitHub release](https://github.com/AI999X/actuarial-science-ai999x/releases/tag/v1.6.6) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `actuarial-science` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/actuarial-science`) and its data (`rust/models/actuarial_science/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p actuarial-science-ai999x
./target/release/actuarial-science --help
cargo test -p actuarial-science-ai999x
```
