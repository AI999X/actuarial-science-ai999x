# Actuarial Science and Insurance (actuarial-science-ai999x)

[![tests](https://github.com/AI999X/actuarial-science-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/actuarial-science-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/actuarial-science-ai999x)](https://pypi.org/project/actuarial-science-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/actuarial-science-ai999x)

Actuarial Science and Insurance covers life tables and survival models, life insurances and annuities, net and gross premiums, reserves and Thiele's equation, interest theory, loss models with deductibles and limits, Panjer's recursion, VaR and TVaR, credibility and loss reserving (chain-ladder, Bornhuetter-Ferguson, Cape Cod, Mack): 93 calculations in 15 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Actuarial Science and Insurance desktop app: Normal approximation to aggregate losses, with its worked example and live scene](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/hero.webp)](https://www.ai999x.com/models/actuarial-science-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `actuarial-science` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/actuarial-science-ai999x/#try) |
| **Python** 3.10+ | `pip install actuarial-science-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

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

## Screenshots

[![The Actuarial Science and Insurance desktop app: Normal approximation to aggregate losses, with its worked example and live scene](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/hero-dark.webp)](https://www.ai999x.com/models/actuarial-science-ai999x/)

[![Effective rates from a constant force of interest: the live scene in the Actuarial Science and Insurance desktop app](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/see-1.webp)](https://www.ai999x.com/models/actuarial-science-ai999x/)

*Effective rates from a constant force of interest*

[![Buhlmann credibility premium (known structural parameters): the live scene in the Actuarial Science and Insurance desktop app](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/see-2.webp)](https://www.ai999x.com/models/actuarial-science-ai999x/)

*Buhlmann credibility premium (known structural parameters)*

[![Loss at issue of whole life insurance: mean, standard deviation, probability of loss: the live scene in the Actuarial Science and Insurance desktop app](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/see-3.webp)](https://www.ai999x.com/models/actuarial-science-ai999x/)

*Loss at issue of whole life insurance: mean, standard deviation, probability of loss*

Every calculation, with its formula, a worked example and its figures: **[Actuarial Science and Insurance on ai999x.com](https://www.ai999x.com/models/actuarial-science-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/actuarial-science-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/actuarial-science-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `actuarial-science` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/actuarial-science`) and its data (`rust/models/actuarial_science/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p actuarial-science-ai999x
./target/release/actuarial-science --help
cargo test -p actuarial-science-ai999x
```
