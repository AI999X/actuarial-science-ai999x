# Actuarial Science and Insurance (actuarial-science-ai999x)

[![tests](https://github.com/AI999X/actuarial-science-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/actuarial-science-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/actuarial-science-ai999x)](https://pypi.org/project/actuarial-science-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/actuarial-science-ai999x)

Actuarial Science and Insurance covers life tables and survival models, life insurances and annuities, net and gross premiums, reserves and Thiele's equation, interest theory, loss models with deductibles and limits, Panjer's recursion, VaR and TVaR, credibility and loss reserving (chain-ladder, Bornhuetter-Ferguson, Cape Cod, Mack): 93 calculations in 15 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Actuarial Science and Insurance desktop app: Normal approximation to aggregate losses, with its worked example and live scene](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/hero.webp)](https://www.ai999x.com/models/actuarial-science-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://downloads.ai999x.com/desktop/latest/ai999x-windows-x86_64.zip) · [Linux](https://downloads.ai999x.com/desktop/latest/ai999x-linux-x86_64.tar.gz) |
| **Command line** `actuarial-science` (Windows, Linux) | [Windows](https://downloads.ai999x.com/desktop/latest/ai999x-cli-windows-x86_64.zip) · [Linux](https://downloads.ai999x.com/desktop/latest/ai999x-cli-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/actuarial-science-ai999x/#try) |
| **Python** 3.10+ | `pip install actuarial-science-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Example

```python
import actuarial_science_ai999x as actuarial_science
a = actuarial_science.ask("Net premium for whole life insurance: q = [0.1, 0.2, 0.5, 1], i = 5%, S = 1000")
print(a.outcome, a.value)
```

Output:

```text
SOLVED 306.841854896
```

Every calculation, with its formula, a worked example and its figures: **[Actuarial Science and Insurance on ai999x.com](https://www.ai999x.com/models/actuarial-science-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.3 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/actuarial-science-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.3`; the [GitHub release](https://github.com/AI999X/actuarial-science-ai999x/releases/tag/v1.6.3) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `actuarial-science` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/actuarial-science`) and its data (`rust/models/actuarial_science/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p actuarial-science-ai999x
./target/release/actuarial-science --help
cargo test -p actuarial-science-ai999x
```
