# actuarial-science-ai999x — offline, verified actuarial science and insurance solver with step-by-step answers

[![tests](https://github.com/AI999X/actuarial-science-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/actuarial-science-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/actuarial-science-ai999x)](https://pypi.org/project/actuarial-science-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/actuarial-science-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/actuarial-science-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/actuarial-science-ai999x/)** · **[Guides](https://huggingface.co/AI999X/actuarial-science-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/actuarial-science-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

Field: **Economics & Finance** · Tasks: Question Answering, Sentence Similarity, Feature Extraction

actuarial-science-ai999x is an offline, deterministic Python solver that answers actuarial science and insurance questions with verified results from 91 certified capabilities, gives 568 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**actuarial-science-ai999x** is a compact, deterministic actuarial science and insurance engine for Python. It reads a question in plain English (or a typed call), picks a certified actuarial science and insurance capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** actuarial-science-ai999x comes as the AI999X desktop app and as the command-line program `actuarial-science` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Actuarial Science and Insurance desktop app** 1.2.0 | Windows, Linux — this model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-actuarial-science-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-actuarial-science-1.2.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.2.0 | Windows, Linux — every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-1.2.0-linux-x86_64.tar.gz) |
| **Command line** (`actuarial-science`) | Windows, Linux — one program per model, offline, no Python | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz) |
| **In your browser** | any device — nothing to install | [Compute on the model page](https://www.ai999x.com/models/actuarial-science-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install actuarial-science-ai999x` |

Checksums: [SHA256SUMS](https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS) · every version: [https://www.ai999x.com/downloads/desktop/latest/](https://www.ai999x.com/downloads/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Rust versions

Actuarial Science and Insurance also comes as native programs compiled from Rust, version 1.2.0 of the AI999X desktop app and command line: they run offline on your computer and need no Python. Each file is in this repository's `rust/` folder, on the [GitHub release v1.0.0](https://github.com/AI999X/actuarial-science-ai999x/releases/tag/v1.0.0) and on the website.

| Program | System | File | Size | Download | Use |
|---|---|---|---|---|---|
| Desktop app (Actuarial Science and Insurance) | Windows x86-64 | `ai999x-actuarial-science-1.2.0-windows-x86_64.zip` | 7.1 MB | [Hub](https://huggingface.co/AI999X/actuarial-science-ai999x/resolve/main/rust/ai999x-actuarial-science-1.2.0-windows-x86_64.zip) · [GitHub](https://github.com/AI999X/actuarial-science-ai999x/releases/download/v1.0.0/ai999x-actuarial-science-1.2.0-windows-x86_64.zip) · [website](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-actuarial-science-1.2.0-windows-x86_64.zip) | unzip, then double-click `ai999x-actuarial-science.exe` (no installer) |
| Desktop app (Actuarial Science and Insurance) | Linux x86-64 | `ai999x-actuarial-science-1.2.0-linux-x86_64.tar.gz` | 8.1 MB | [Hub](https://huggingface.co/AI999X/actuarial-science-ai999x/resolve/main/rust/ai999x-actuarial-science-1.2.0-linux-x86_64.tar.gz) · [GitHub](https://github.com/AI999X/actuarial-science-ai999x/releases/download/v1.0.0/ai999x-actuarial-science-1.2.0-linux-x86_64.tar.gz) · [website](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-actuarial-science-1.2.0-linux-x86_64.tar.gz) | extract, then run `./ai999x-actuarial-science` (X11 or Wayland desktop) |
| Command line (`actuarial-science`) | Windows x86-64 | `ai999x-actuarial-science-cli-1.2.0-windows-x86_64.zip` | 2.9 MB | [Hub](https://huggingface.co/AI999X/actuarial-science-ai999x/resolve/main/rust/ai999x-actuarial-science-cli-1.2.0-windows-x86_64.zip) · [GitHub](https://github.com/AI999X/actuarial-science-ai999x/releases/download/v1.0.0/ai999x-actuarial-science-cli-1.2.0-windows-x86_64.zip) · [website (all models)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-windows-x86_64.zip) | unzip, then `actuarial-science.exe ask "<question>"` or `actuarial-science.exe solve <capability> name=value` |
| Command line (`actuarial-science`) | Linux x86-64 | `ai999x-actuarial-science-cli-1.2.0-linux-x86_64.tar.gz` | 2.9 MB | [Hub](https://huggingface.co/AI999X/actuarial-science-ai999x/resolve/main/rust/ai999x-actuarial-science-cli-1.2.0-linux-x86_64.tar.gz) · [GitHub](https://github.com/AI999X/actuarial-science-ai999x/releases/download/v1.0.0/ai999x-actuarial-science-cli-1.2.0-linux-x86_64.tar.gz) · [website (all models)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz) | extract, then `./actuarial-science ask "<question>"` or `./actuarial-science solve <capability> name=value` |

The command-line archive holds `actuarial-science` with a README of its commands and examples, the licence and the notice. Checksums: [`rust/SHA256SUMS`](https://huggingface.co/AI999X/actuarial-science-ai999x/resolve/main/rust/SHA256SUMS) (`sha256sum -c SHA256SUMS`).

## Quick start

### Desktop app

Download **AI999X Actuarial Science and Insurance** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Actuarial Science and Insurance among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-actuarial-science-1.2.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-actuarial-science-1.2.0-linux-x86_64.tar.gz
./ai999x-actuarial-science-1.2.0-linux-x86_64/ai999x-actuarial-science
```

### Command line

Linux:

```bash
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.2.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.2.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
actuarial-science --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.2.0-windows-x86_64.zip"
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.2.0/$F" -OutFile $F
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.2.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
actuarial-science --version
```

Every command below was run as shown; the text under it is what the program printed.

**What a capability needs** — The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ actuarial-science info act_net_premium_whole_life
Net annual premium for whole life insurance
  equivalence principle: P ä_x = S A_x
  in   q        death probabilities q_x, q_(x+1), ... ending with 1 (a list) (vector) []
  in   i        annual effective interest rate (5% or 0.05) []
  in   S        sum insured []
  out  P        annual net premium S A_x / ä_x []
  out  A        whole life insurance A_x []
  out  a_due    whole life annuity-due ä_x []
```

**Solve** — Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ actuarial-science solve act_net_premium_whole_life "q=[0.1, 0.2, 0.5, 1]" i=0.05 S=1000.0
P = 306.8418549
A = 0.8656578278
a_due = 2.821185617
  by: Net annual premium for whole life insurance  (equivalence principle: P ä_x = S A_x)
  i = 0.05
  S = 1000
  assumes: the table closes; 0 < i < 1; S > 0
```

**Many questions at once** — One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | actuarial-science batch

`cases.jsonl`:

```json
{"key": "act_net_premium_whole_life", "inputs": {"q": "[0.1, 0.2, 0.5, 1]", "i": 0.05, "S": 1000.0}}
{"key": "act_annuity_certain", "inputs": {"n": 10, "i": 0.05}}
```

```console
$ actuarial-science batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":306.841854896,"unit":"","solved_for":"P","si":[["i",0.05],["S",1000.0]],"values":[["P",306.841854896,""],["A",0.865657827757,""],["a_due",2.8211856171,""]],"target":"P","phrase":"annual net premium S A_x / ä_x","assumptions":["the table closes","0 < i < 1","S > 0"],"name":"Net annual premium for whole life insurance","equation":"equivalence principle: P ä_x = S A_x"}
{"outcome":"SOLVED","message":"","value":7.72173492918,"unit":"","solved_for":"a_n","si":[["i",0.05]],"values":[["a_n",7.72173492918,""],["a_due",8.10782167564,""],["s_n",12.5778925355,""],["s_due",13.2067871623,""],["a_cont",7.91320859505,""]],"target":"a_n","phrase":"present value of an annuity-immediate a_n","assumptions":["n >= 1","0 < i < 1"],"name":"Annuity-certain present and accumulated values","equation":"a_n = (1 - v^n)/i, a-due = (1 - v^n)/d, s_n = ((1+i)^n - 1)/i, s-due = ((1+i)^n - 1)/d, continuous = (1 - v^n)/delta"}
```

### In your browser

Open [Actuarial Science and Insurance on ai999x.com](https://www.ai999x.com/models/actuarial-science-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `actuarial_science_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install actuarial-science-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Buhlmann credibility premium (known structural parameters)

The Buhlmann credibility factor Z = n/(n + k), k = EPV/VHM, and the credibility premium Z xbar + (1 - Z) mu from the overall mean, the expected process variance, the variance of the hypothetical means and n observations with mean xbar.

Command line:

```console
$ actuarial-science solve act_buhlmann_credibility mu=1000 EPV=50000 VHM=10000 n=5 xbar=1200
k = 5
Z = 0.5
premium = 1100
  by: Buhlmann credibility premium (known structural parameters)  (k = EPV/VHM; Z = n/(n + k); P = Z xbar + (1 - Z) mu)
  mu = 1000
  EPV = 50000
  VHM = 10000
  n = 5
  xbar = 1200
  assumes: EPV, VHM > 0; n > 0
```

Python:

```python
import actuarial_science_ai999x as m

a = m.solve("act_buhlmann_credibility", mu="1000", EPV="50000", VHM="10000", n="5", xbar="1200")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/actuarial-science-ai999x/#cap-act_buhlmann_credibility) and press **Compute in your browser**.

### Buhlmann-Straub credibility with exposures

The nonparametric Buhlmann-Straub estimates of the expected process variance and of the variance of the hypothetical means from losses per unit of exposure and the exposures of r risks over n periods, each risk's credibility factor and its credibility premium (complement: the exposure-weighted overall mean).

Command line:

```console
$ actuarial-science solve act_buhlmann_straub "X=[[100, 120, 110], [200, 180, 230]]" "m=[[10, 20, 15], [30, 25, 20]]"
EPV = 7661.111111
VHM = 3834.197531
k = 1.998100267
Z = [0.957485509927,0.974050005645]
premiums = [114.590043128,200.466171022]
mean = 167.9166667
  by: Buhlmann-Straub credibility with exposures  (v = sum m_ij (X_ij - Xbar_i)^2 / (r(n-1)); a = [sum m_i (Xbar_i - Xbar)^2 - v (r-1)] / (m - sum m_i^2/m); Z_i = m_i/(m_i + v/a))
  assumes: at least 2 risks and 2 periods; positive exposures; estimated VHM > 0
```

Python:

```python
import actuarial_science_ai999x as m

a = m.solve("act_buhlmann_straub", X=[[100, 120, 110], [200, 180, 230]], m=[[10, 20, 15], [30, 25, 20]])
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/actuarial-science-ai999x/#cap-act_buhlmann_straub) and press **Compute in your browser**.

### Empirical Buhlmann credibility from a data table

The nonparametric estimates of the expected process variance and of the variance of the hypothetical means from r risks observed for n periods each, the Buhlmann k and Z, and each risk's credibility premium.

Command line:

```console
$ actuarial-science solve act_buhlmann_empirical "X=[[3, 5, 7], [6, 12, 9]]"
EPV = 6.5
VHM = 5.833333333
k = 1.114285714
Z = 0.7291666667
premiums = [5.54166666667,8.45833333333]
mean = 7
  by: Empirical Buhlmann credibility from a data table  (EPV = mean of the within-risk sample variances; VHM = sample variance of the risk means - EPV/n; Z = n/(n + EPV/VHM))
  assumes: at least 2 risks and 2 periods; estimated VHM > 0
```

Python:

```python
import actuarial_science_ai999x as m

a = m.solve("act_buhlmann_empirical", X=[[3, 5, 7], [6, 12, 9]])
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/actuarial-science-ai999x/#cap-act_buhlmann_empirical) and press **Compute in your browser**.

### Limited fluctuation full credibility for severity and aggregate losses

The expected number of claims needed for full credibility of the claim severity, of aggregate losses (pure premium) and of the claim frequency, for Poisson claim counts and a severity with the given mean and standard deviation.

Command line:

```console
$ actuarial-science solve act_limited_fluctuation_severity_aggregate p=0.9 k=0.05 mean_x=1000 sd_x=2000
claims_aggregate = 5411.086908
claims_severity = 4328.869527
claims_frequency = 1082.217382
  by: Limited fluctuation full credibility for severity and aggregate losses  (n0 = (z/k)^2; aggregate: n0 (1 + (sd/mean)^2); severity: n0 (sd/mean)^2)
  p = 0.9
  k = 0.05
  mean_x = 1000
  sd_x = 2000
  assumes: 0 < p < 1; 0 < k < 1; Poisson claim counts
```

Python:

```python
import actuarial_science_ai999x as m

a = m.solve("act_limited_fluctuation_severity_aggregate", p="0.9", k="0.05", mean_x="1000", sd_x="2000")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/actuarial-science-ai999x/#cap-act_limited_fluctuation_severity_aggregate) and press **Compute in your browser**.

## Documentation

- **Rust user guide** — the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/actuarial-science-ai999x/blob/main/docs/actuarial_science_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/actuarial-science-ai999x/blob/main/docs/actuarial_science_ai999x_rust_guide.md)
- **Python guide** — every capability with a Python example: [PDF](https://huggingface.co/AI999X/actuarial-science-ai999x/blob/main/docs/actuarial_science_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/actuarial-science-ai999x/blob/main/docs/actuarial_science_ai999x_capabilities.md)
- **Desktop app & command line** — downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page** — compute in your browser, every capability explained: https://www.ai999x.com/models/actuarial-science-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified actuarial science and insurance capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 568 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `actuarial-science` command line for Windows and Linux, and WebAssembly on the model page — no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** life contingencies, interest theory, premiums and reserves, loss models, credibility and loss reserving.

## Model overview

| Property | Value |
|---|---|
| Package | `actuarial-science-ai999x` 1.0.0 (import `actuarial_science_ai999x`) |
| Type | Deterministic neuro-symbolic actuarial science and insurance engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 91 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 568 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `actuarial-science-ai999x "<question>"`; for the native program use `actuarial-science` (above). The outputs above were produced by running this exact code on actuarial-science-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

Each question was put to `actuarial-science-ai999x`; the answer is the one it returned, with the steps it took.

**Net annual premium for whole life insurance** · _Premiums_

> Net premium for whole life insurance: q = [0.1, 0.2, 0.5, 1], i = 5%, S = 1000

→ **P = 306.8418549, A = 0.8656578278, a_due = 2.821185617**

<details><summary>Steps</summary>

- Net annual premium for whole life insurance: q = [0.1, 0.2, 0.5, 1], i = 0.05, S = 1000 → P = 306.8418549, A = 0.8656578278, a_due = 2.821185617

</details>

**Chain-ladder ultimate losses and reserves** · _Loss reserving_

> Chain-ladder reserve: T = [[100, 150, 175], [110, 168, 0], [120, 0, 0]]

→ **ultimate = [175, 196, 212], reserve = [0, 28, 92], total_reserve = 120**

<details><summary>Steps</summary>

- Chain-ladder ultimate losses and reserves: T = [100 150 175; 110 168 0; 120 0 0] → ultimate = [175, 196, 212], reserve = [0, 28, 92], total_reserve = 120

</details>

**Annuity-certain present and accumulated values** · _Interest theory_

`m.solve("act_annuity_certain", n=10, i=0.05)`

→ **a_n = 7.721734929, a_due = 8.107821676, s_n = 12.57789254, s_due = 13.20678716, a_cont = 7.913208595**

<details><summary>Steps</summary>

- Annuity-certain present and accumulated values: n = 10, i = 0.05 → a_n = 7.721734929, a_due = 8.107821676, s_n = 12.57789254, s_due = 13.20678716, a_cont = 7.913208595

</details>

**Buhlmann credibility premium (known structural parameters)** · _Credibility_

> Buhlmann credibility premium: mu = 1000, EPV = 50000, VHM = 10000, n = 5, xbar = 1200

→ **k = 5, Z = 0.5, premium = 1100**

<details><summary>Steps</summary>

- Buhlmann credibility premium (known structural parameters): mu = 1000, EPV = 50000, VHM = 10000, n = 5, xbar = 1200 → k = 5, Z = 0.5, premium = 1100

</details>

**Ordinary deductible with Pareto losses** · _Loss models_

> Pareto losses with a deductible: alpha = 3, theta = 2000, d = 500

→ **mean = 1000, lev_d = 360, per_loss = 640, per_payment = 1250, LER = 0.36**

<details><summary>Steps</summary>

- Ordinary deductible with Pareto losses: alpha = 3, theta = 2000, d = 500 → mean = 1000, lev_d = 360, per_loss = 640, per_payment = 1250, LER = 0.36

</details>

**Deferred whole life annuity-due** · _Life annuities_

> Deferred annuity: q = [0.1, 0.2, 0.5, 1], i = 0.05, u = 2, P = 1000

→ **ua_due = 0.96404276, EPV = 964.04276**

<details><summary>Steps</summary>

- Deferred whole life annuity-due: q = [0.1, 0.2, 0.5, 1], i = 0.05, u = 2, P = 1000 → ua_due = 0.96404276, EPV = 964.04276

</details>

## Figures

Every point was computed by `actuarial-science-ai999x`.

**Annuity-certain present and accumulated values**

![Annuity-certain present and accumulated values — computed by actuarial-science-ai999x](https://huggingface.co/AI999X/actuarial-science-ai999x/resolve/main/figures/act-annuity-certain-a-n-vs-n.svg)

_a_n (present value of an annuity-immediate a_n) against n (number of annual payments (term in years)), with i = 0.05. 12 points computed by actuarial-science-ai999x; marked: n = 10, a_n = 7.72173._

**Ordinary deductible with Pareto losses**

![Ordinary deductible with Pareto losses — computed by actuarial-science-ai999x](https://huggingface.co/AI999X/actuarial-science-ai999x/resolve/main/figures/act-pareto-deductible-per-loss-vs-alpha.svg)

_per_loss (expected payment per loss E[(X - d)+]) against alpha (Pareto shape alpha (> 1)), with d = 500, theta = 2000. 31 points computed by actuarial-science-ai999x; marked: alpha = 3, per_loss = 640._

**Limited fluctuation full credibility standard for claim counts**

![Limited fluctuation full credibility standard for claim counts — computed by actuarial-science-ai999x](https://huggingface.co/AI999X/actuarial-science-ai999x/resolve/main/figures/act-limited-fluctuation-full-credibility-n-full-vs-p.svg)

_n_full (expected claims needed for full credibility (z/k)^2) against p (probability level (90% or 0.9)), with k = 0.05. 19 points computed by actuarial-science-ai999x; marked: p = 0.9, n_full = 1082.22._

**Associated single decrement rates from a multiple decrement table (UDD)**

![Associated single decrement rates from a multiple decrement table (UDD) — computed by actuarial-science-ai999x](https://huggingface.co/AI999X/actuarial-science-ai999x/resolve/main/figures/act-multiple-decrement-to-single-q1-single-vs-q1.svg)

_q1_single (associated single decrement probability q'(1)) against q1 (probability of leaving by decrement 1 within the year q(1)), with q2 = 0.2. 31 points computed by actuarial-science-ai999x; marked: q1 = 0.1, q1_single = 0.112096._

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
