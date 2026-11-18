# Actuarial Science and Insurance: the desktop app, user guide

**Actuarial Science and Insurance**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Actuarial Science and Insurance covers life tables and survival models, life insurances and annuities, net and gross premiums, reserves and Thiele's equation, interest theory, loss models with deductibles and limits, Panjer's recursion, VaR and TVaR, credibility and loss reserving (chain-ladder, Bornhuetter-Ferguson, Cape Cod, Mack). Its desktop app holds all 93 calculations in 15 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-actuarial-science-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-actuarial-science-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-actuarial-science-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-actuarial-science.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-actuarial-science-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-actuarial-science-1.6.5-linux-x86_64.tar.gz
./ai999x-actuarial-science-1.6.5-linux-x86_64/ai999x-actuarial-science
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Actuarial Science and Insurance app, opened on Net annual premium for whole life insurance with its worked example.](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/hero.webp)

*The Actuarial Science and Insurance app, opened on Net annual premium for whole life insurance with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Premiums › Net annual premium for whole life insurance (or type part of its name in the search box). It opens with a worked example filled in: “Net premium for whole life insurance: q = [0.1, 0.2, 0.5, 1], i = 0.05, S = 1000” The answer: P = 306.8418549, A = 0.8656578278, a_due = 2.821185617.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Net annual premium for whole life insurance: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/scene-1.webp)

*Net annual premium for whole life insurance: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it: the app reads the quantities and their units, picks the calculation and fills in its form.

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Chain-ladder ultimate losses and reserves](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/scene-2.webp)

*Chain-ladder ultimate losses and reserves*

![Annuity-certain present and accumulated values](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/scene-3.webp)

*Annuity-certain present and accumulated values*

![Buhlmann credibility premium (known structural parameters)](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/scene-4.webp)

*Buhlmann credibility premium (known structural parameters)*

![Ordinary deductible with Pareto losses](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/scene-5.webp)

*Ordinary deductible with Pareto losses*

![Deferred whole life annuity-due](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/scene-6.webp)

*Deferred whole life annuity-due*

![Continuous whole life insurance and annuity under UDD](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/scene-7.webp)

*Continuous whole life insurance and annuity under UDD*

![Death probability from the central death rate](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/scene-8.webp)

*Death probability from the central death rate*

![Net annual premium for whole life insurance: a chart, enlarged.](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/chart-1.webp)

*Net annual premium for whole life insurance: a chart, enlarged.*

![Chain-ladder ultimate losses and reserves: a chart, enlarged.](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/chart-2.webp)

*Chain-ladder ultimate losses and reserves: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Actuarial Science and Insurance app in the dark theme.](https://www.ai999x.com/?ax_shot=actuarial-science-ai999x/hero-dark.webp)

*The Actuarial Science and Insurance app in the dark theme.*

## 7. Every calculation

93 calculations in 15 topics; each has its own entry on the model's page (https://www.ai999x.com/models/actuarial-science-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Credibility | Buhlmann credibility premium (known structural parameters), Buhlmann-Straub credibility with exposures, Empirical Buhlmann credibility from a data table, Limited fluctuation full credibility for severity and aggregate losses, Limited fluctuation full credibility standard for claim counts, Partial credibility (square root rule), Poisson-gamma Bayesian credibility |
| Interest theory | Accumulated and present value of a single amount, Annuity-certain present and accumulated values, Effective annual rate from a nominal rate, Effective rates from a constant force of interest, Equivalent rates from an effective annual interest rate, Increasing and decreasing annuities-certain, Loan amortization: level payment, interest and principal, outstanding balance, Macaulay and modified duration and convexity of cash flows, Present value of a perpetuity, Present value of a series of cash flows, Yield rate of a level annuity |
| Life annuities | Deferred whole life annuity-due, m-thly whole life annuity-due under UDD, Temporary life annuity, Whole life annuity, Woolhouse approximation for an m-thly annuity |
| Life insurance | Continuous whole life insurance and annuity under UDD, Deferred whole life insurance, Endowment insurance (fully discrete), Term insurance (fully discrete), Whole life insurance (fully discrete) |
| Life tables | Commutation functions D, N, C and M, Curtate and complete expectation of life, Death probability from the central death rate, Deferred death probability u\|t q x, Life table functions dx, qx and px from survivor numbers, Probability of surviving t years from a life table, Pure endowment (actuarial discount factor), Survival to a fractional age (UDD, constant force, Balducci), Temporary (n-year) expectation of life |
| Loss models | Compound Poisson mean, variance and skewness, Deductible, policy limit, coinsurance and inflation with exponential losses, Deductible, policy limit, coinsurance and inflation with Pareto losses, Individual risk model: aggregate mean and variance, Insurer payment for one loss (deductible, limit, coinsurance), Mean and variance of aggregate losses (compound distribution), Negative binomial claim frequency probabilities, Normal approximation to aggregate losses, Ordinary deductible with exponential losses, Ordinary deductible with lognormal losses, Ordinary deductible with Pareto losses, Panjer recursion for a compound negative binomial distribution, Panjer recursion for a compound Poisson distribution, Poisson claim frequency probabilities, Premium principles: expected value, variance, standard deviation, Stop-loss premium for a compound Poisson distribution |
| Loss reserving | Bornhuetter-Ferguson reserve, Cape Cod (Stanard-Buhlmann) reserve, Chain-ladder age-to-age and cumulative development factors, Chain-ladder ultimate losses and reserves, Chain-ladder with simple-average development factors, Expected claims reserving method, Mack's standard error of chain-ladder reserves |
| Multiple decrements | Associated single decrement rates from a multiple decrement table (UDD), Multiple decrement probabilities from single decrement rates (UDD) |
| Multiple lives | Joint life and last survivor annuities and insurances, Reversionary annuity |
| Premiums | Gross annual premium for whole life insurance with expenses, Loss at issue of whole life insurance: mean, standard deviation, probability of loss, Net annual premium for endowment insurance, Net annual premium for limited-payment whole life insurance, Net annual premium for term insurance, Net annual premium for whole life insurance |
| Ratemaking | Indicated rate by the pure premium method, Indicated rate change by the loss ratio method, Loss and premium trend factors to a future period, On-level earned premium by the parallelogram method |
| Reserves | Continuous whole life insurance, premium and reserve under Makeham's law, Gross premium reserve for whole life insurance with expenses, Net premium reserve for endowment insurance, Net premium reserve for term insurance, Net premium reserve for whole life insurance, Policy value recursion (one year), Retrospective reserve (accumulated fund per survivor), Thiele's equation: continuous endowment insurance reserve (constant forces) |
| Risk measures | VaR and TVaR of a lognormal loss, VaR and TVaR of a normal loss, VaR and TVaR of a Pareto loss, VaR and TVaR of an exponential loss |
| Survival estimation | Exponential maximum-likelihood fit with censoring, Kaplan-Meier product-limit survival estimate, Nelson-Aalen cumulative hazard estimate |
| Survival models | Constant force of mortality (exponential lifetime), De Moivre's law of mortality, Gompertz's law of mortality, Makeham's law of mortality |

## 8. Other ways to use it

The same engine runs on the command line (`actuarial-science`, its own guide), in Python (`pip install actuarial-science-ai999x`, `import actuarial_science_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/actuarial-science-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/actuarial-science-ai999x/
