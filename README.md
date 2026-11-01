# actuarial-science-ai999x — offline, verified actuarial science and insurance solver with step-by-step answers

[![tests](https://github.com/AI999X/actuarial-science-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/actuarial-science-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/actuarial-science-ai999x)](https://pypi.org/project/actuarial-science-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/actuarial-science-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/actuarial-science-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/actuarial-science-ai999x/)** · **[Guides](https://huggingface.co/AI999X/actuarial-science-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/actuarial-science-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

actuarial-science-ai999x is an offline, deterministic Python solver that answers actuarial science and insurance questions with verified results from 91 certified capabilities, gives 568 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**actuarial-science-ai999x** is a compact, deterministic actuarial science and insurance engine for Python. It reads a question in plain English (or a typed call), picks a certified actuarial science and insurance capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified actuarial science and insurance capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 568 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10–3.14).
- **Coverage:** life contingencies, interest theory, premiums and reserves, loss models, credibility and loss reserving.

## Model overview

| Property | Value |
|---|---|
| Package | `actuarial-science-ai999x` 1.0.0 (import `actuarial_science_ai999x`) |
| Type | Deterministic neuro-symbolic actuarial science and insurance engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 91 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 568 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python ) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install actuarial-science-ai999x
```

```python
import actuarial_science_ai999x as actuarial_science

actuarial_science.ask("Net premium for whole life insurance: q = [0.1, 0.2, 0.5, 1], i = 5%, S = 1000").show()
# -> SOLVED — P = 306.842, A = 0.865658, a_due = 2.82119  (act_net_premium_whole_life)
actuarial_science.ask("Chain-ladder reserve: T = [[100, 150, 175], [110, 168, 0], [120, 0, 0]]").show()
# -> SOLVED — ultimate = [175.0, 196.0, 212.0], reserve = [0.0, 28.0, 92.0], total_reserve = 120  (act_chain_ladder_reserve)
actuarial_science.solve("act_annuity_certain", n=10, i=0.05).show()
# -> SOLVED — a_n = 7.72173, a_due = 8.10782, s_n = 12.5779, s_due = 13.2068  (act_annuity_certain)
actuarial_science.ask("Buhlmann credibility premium: mu = 1000, EPV = 50000, VHM = 10000, n = 5, xbar = 1200").show()
# -> SOLVED — k = 5, Z = 0.5, premium = 1100  (act_buhlmann_credibility)
actuarial_science.ask("Pareto losses with a deductible: alpha = 3, theta = 2000, d = 500").show()
# -> SOLVED — mean = 1000, lev_d = 360, per_loss = 640, per_payment = 1250  (act_pareto_deductible)
actuarial_science.ask("Who created you?").message
# -> actuarial-science-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

The Python package also installs its own command: `actuarial-science-ai999x "<question>"`, `actuarial-science-ai999x --capabilities`. The outputs above were produced by running this exact code on actuarial-science-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

91 certified procedures. A few of them:

- **Procedures:** Accumulated and present value of a single amount; Annuity-certain present and accumulated values; Associated single decrement rates from a multiple decrement table (UDD); Bornhuetter-Ferguson reserve; Buhlmann credibility premium (known structural parameters); Buhlmann-Straub credibility with exposures; Cape Cod (Stanard-Buhlmann) reserve; Chain-ladder age-to-age and cumulative development factors; Chain-ladder ultimate losses and reserves; Chain-ladder with simple-average development factors; Commutation functions D, N, C and M; Compound Poisson mean, variance and skewness; Constant force of mortality (exponential lifetime); Continuous whole life insurance and annuity under UDD; Continuous whole life insurance, premium and reserve under Makeham's law; Curtate and complete expectation of life; De Moivre's law of mortality; Death probability from the central death rate; Deductible, policy limit, coinsurance and inflation with Pareto losses; Deductible, policy limit, coinsurance and inflation with exponential losses; Deferred death probability u|t q x; Deferred whole life annuity-due; Deferred whole life insurance; Effective annual rate from a nominal rate …

`actuarial_science_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified actuarial science and insurance capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (life contingencies, interest theory, premiums and reserves, loss models, credibility and loss reserving); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{actuarial_science_ai999x_1_0_0,
  title  = {actuarial-science-ai999x: an offline, verified actuarial science and insurance solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/actuarial-science-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/actuarial-science-ai999x/discussions or visit https://ai999x.com.

## Support the project

actuarial-science-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/actuarial-science-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/actuarial-science-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/actuarial-science-ai999x/tree/main/onnx).
