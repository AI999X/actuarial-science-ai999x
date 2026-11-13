"""Actuarial science and insurance procedures: deterministic capabilities with typed inputs and several outputs
(domain code, loaded by the pack; enabled only once tools/certify_procedures.py has certified them).

Topics: interest theory (effective, nominal and continuous rates, accumulation, annuities-certain, perpetuities,
increasing and decreasing annuities, loan amortization, the yield of an annuity, present value, duration and
convexity of cash flows); life tables (qx, px, dx, survival and deferred death probabilities, curtate and complete
expectation of life, temporary expectation, central death rates, fractional ages under UDD, constant force and
Balducci, commutation functions, pure endowments); parametric laws (de Moivre, constant force, Gompertz, Makeham);
life insurances and annuities (whole life, term, endowment, deferred, continuous under UDD, m-thly under UDD and by
Woolhouse); premiums (net level by the equivalence principle for whole life, term, endowment and limited payment,
gross premiums with expenses, the variance of the loss at issue); reserves (prospective, retrospective, the policy
value recursion, gross premium reserves, Thiele's equation, continuous Makeham reserves); several lives and decrements
(joint life and last survivor, reversionary annuities, multiple decrement tables under UDD); loss models (Poisson and
negative binomial frequency, compound distributions, the individual risk model, the normal approximation, Panjer's
recursion, stop-loss premiums, deductibles, limits, coinsurance and inflation for exponential, Pareto and lognormal
severities, VaR and TVaR, premium principles); credibility (limited fluctuation full and partial, Buhlmann with known
and estimated parameters, Buhlmann-Straub, Poisson-gamma); loss reserving and ratemaking (chain-ladder factors and
ultimates, simple-average chain-ladder, Bornhuetter-Ferguson, Cape Cod, Mack's standard error, the expected claims
method, loss ratio and pure premium rate indications); survival estimation (Kaplan-Meier, Nelson-Aalen, the
exponential maximum-likelihood fit).

Each procedure states its inputs, outputs, conditions, method, LaTeX, the documented question form it parses, worked
examples, an INDEPENDENT reference computation (used only by certification; the checking libraries are imported inside
the reference functions) and a deliberately wrong variant that the checks must reject. The shipped code (run, verify,
parse) is plain Python (math, re).

How the references differ from the shipped code:
- chainladder (Development, Chainladder, BornhuetterFerguson, CapeCod, MackChainladder) for loss reserving;
- lifelines (KaplanMeierFitter, NelsonAalenFitter, ExponentialFitter, restricted mean survival time) for survival
  estimation and expectations of life;
- scipy.stats distributions (norm, poisson, nbinom, expon, lomax, lognorm, gamma) for frequencies, severities, VaR and
  TVaR; scipy.integrate.quad for limited expected values, continuous annuities, Gompertz-Makeham integrals, fractional
  ages and multiple decrements; scipy DOP853 integration of Thiele's equation; scipy brentq for re-solved equations;
- backward recursions (A = vq + vpA', a = 1 + vpa') and full enumeration of the curtate future lifetime (numpy) for
  insurances, annuities, premiums and reserves computed in the shipped code by direct sums; forward policy-value
  recursions for prospective reserves; direct convolution for Panjer's recursion; sympy for cumulants and derivatives;
- weaker references (the same published formula evaluated a different way) are listed in procedures_certified.json.

Conventions (fixed, so run and reference agree exactly):
- Interest: i is the annual effective rate, written as a decimal (0.05) or with % (5%); a plain number above 1 is
  refused. v = 1/(1+i), d = i/(1+i), delta = ln(1+i).
- Life tables as lists: a qx list starts at the insured's age x (q[0] = q_x, q[1] = q_(x+1), ...); a whole-life
  table must close (its last q is 1). An lx list gives survivors at consecutive integer ages starting at age x0.
- Fully discrete contracts: benefits at the end of the year of death, premiums and annuity-due payments at the start
  of each year; the sum insured S is a plain amount (any currency).
- Loss severities: exponential (mean theta), Pareto (two-parameter, Lomax: S(x) = (theta/(theta+x))^alpha, alpha > 1),
  lognormal (mu, sigma of log X). The policy limit u is the maximum covered loss. Money amounts carry no unit.
- Loss triangles: a square matrix of CUMULATIVE amounts, origin years in rows and development ages in columns, with the
  unknown cells below the latest diagonal written as 0. Factors are volume-weighted unless stated; no tail factor.
- Results built from exp, log or roots are rounded to 12 significant digits and exact zeros are written as 0.

Question form (one per procedure): the capability phrase, a colon, then `name = value` pairs separated by commas
(`Whole life insurance: q = [0.1, 0.2, 0.5, 1], i = 5%`). Lists in brackets, matrices as lists of rows, rates as
decimals or with %, choices as words. Any other number, an unknown name or a name given twice is refused, never
guessed.
"""
from __future__ import annotations

import math
import re

# ================================================================================================ shared helpers
def _r(x):
    """12 significant digits, no negative zero (identical results on every platform and Python version)."""
    if isinstance(x, bool) or isinstance(x, str) or x is None:
        return x
    if isinstance(x, int):
        return x
    if isinstance(x, (list, tuple)):
        return [_r(v) for v in x]
    if x == 0 or not math.isfinite(x):
        return 0.0 if x == 0 else x
    return float(f"{x:.12g}") + 0.0


def _out(**kw):
    return {k: _r(v) for k, v in kw.items()}


def _rel(a, b):
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        if not (isinstance(a, (list, tuple)) and isinstance(b, (list, tuple))) or len(a) != len(b):
            return math.inf
        return max([_rel(x, y) for x, y in zip(a, b)] + [0.0])
    if isinstance(a, (str, bool)) or isinstance(b, (str, bool)):
        return 0.0 if a == b else math.inf
    try:
        a, b = float(a), float(b)
    except (TypeError, ValueError):
        return math.inf
    d = abs(a - b)
    if d == 0:
        return 0.0
    if abs(a) < 1e-15 and abs(b) < 1e-15:
        return 0.0
    return d / max(abs(a), abs(b))


def _recheck(fn):
    """A verifier: every output the procedure gives is recomputed and compared (relative); a missing output fails."""
    def verify(inputs, outputs):
        try:
            want = fn(**inputs)
        except Exception:
            return math.inf
        worst = 0.0
        for k, v in want.items():
            if k not in outputs:
                return math.inf
            worst = max(worst, _rel(outputs[k], v))
        return worst
    return verify


def _safe(fn):
    """Guard a wrong variant against exceptions (a variant that raises cannot be compared)."""
    def g(**kw):
        try:
            return fn(**kw)
        except Exception:
            return {"_failed": True}
    return g


def _need(ok, msg):
    if not ok:
        raise ValueError(msg)


def _isnum(v):
    return isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v)


def _fin(*vals):
    for v in vals:
        _need(_isnum(v), "every input must be a finite number")


def _pos(name, *vals):
    for v in vals:
        _need(_isnum(v) and v > 0, f"{name} must be a positive number")


def _nonneg(name, *vals):
    for v in vals:
        _need(_isnum(v) and v >= 0, f"{name} must be zero or positive")


def _frac(name, v, lo_open=False, hi_open=False):
    _fin(v)
    _need((v > 0 if lo_open else v >= 0) and (v < 1 if hi_open else v <= 1),
          f"{name} must be a fraction between 0 and 1")


def _rate(i):
    _need(_isnum(i) and 0 < i < 1, "the interest rate i must lie between 0 and 100% (write 5% or 0.05)")


def _whole(name, n, lo=0):
    _need(isinstance(n, int) and not isinstance(n, bool) and n >= lo, f"{name} must be a whole number of at least {lo}")


def _lu(rng, a, b):
    return math.exp(rng.uniform(math.log(a), math.log(b)))


def _sig(x, n=4):
    """x rounded to n significant digits (random inputs written as a user would write them)."""
    return float(f"{x:.{n}g}")


def _bisect(f, a, b, n=500):
    """Root of f on [a, b] by bisection (f(a), f(b) of opposite signs), to full double precision."""
    fa, fb = f(a), f(b)
    if fa == 0:
        return a
    if fb == 0:
        return b
    _need((fa > 0) != (fb > 0), "no root in the interval")
    for _ in range(n):
        m = 0.5 * (a + b)
        if m == a or m == b:
            break
        fm = f(m)
        if fm == 0:
            return m
        if (fm > 0) == (fa > 0):
            a, fa = m, fm
        else:
            b = m
    return 0.5 * (a + b)


def _quad(f, a, b):
    from scipy.integrate import quad
    v, _ = quad(f, a, b, epsabs=0.0, epsrel=1e-13, limit=500)
    return float(v)


def _brentq(f, a, b):
    from scipy.optimize import brentq
    return float(brentq(f, a, b, xtol=1e-300, rtol=8.9e-16, maxiter=500))


def _norm_cdf(x):
    return 0.5 * math.erfc(-x / math.sqrt(2.0))


def _norm_pdf(x):
    return math.exp(-0.5 * x * x) / math.sqrt(2.0 * math.pi)


def _norm_ppf(p):
    """Inverse standard normal: Acklam's rational approximation refined by Halley steps on erfc (full precision)."""
    _need(_isnum(p) and 0 < p < 1, "a probability must lie strictly between 0 and 1")
    a = (-3.969683028665376e+01, 2.209460984245205e+02, -2.759285104469687e+02, 1.383577518672690e+02,
         -3.066479806614716e+01, 2.506628277459239e+00)
    b = (-5.447609879822406e+01, 1.615858368580409e+02, -1.556989798598866e+02, 6.680131188771972e+01,
         -1.328068155288572e+01)
    c = (-7.784894002430293e-03, -3.223964580411365e-01, -2.400758277161838e+00, -2.549732539343734e+00,
         4.374664141464968e+00, 2.938163982698783e+00)
    dd = (7.784695709041462e-03, 3.224671290700398e-01, 2.445134137142996e+00, 3.754408661907416e+00)
    if p < 0.02425:
        s = math.sqrt(-2.0 * math.log(p))
        x = (((((c[0] * s + c[1]) * s + c[2]) * s + c[3]) * s + c[4]) * s + c[5]) / ((((dd[0] * s + dd[1]) * s + dd[2]) * s + dd[3]) * s + 1.0)
    elif p > 1.0 - 0.02425:
        s = math.sqrt(-2.0 * math.log(1.0 - p))
        x = -(((((c[0] * s + c[1]) * s + c[2]) * s + c[3]) * s + c[4]) * s + c[5]) / ((((dd[0] * s + dd[1]) * s + dd[2]) * s + dd[3]) * s + 1.0)
    else:
        s = p - 0.5
        t = s * s
        x = (((((a[0] * t + a[1]) * t + a[2]) * t + a[3]) * t + a[4]) * t + a[5]) * s / (((((b[0] * t + b[1]) * t + b[2]) * t + b[3]) * t + b[4]) * t + 1.0)
    for _ in range(3):
        if p < 0.5:
            e = 0.5 * math.erfc(-x / math.sqrt(2.0)) - p
        else:
            e = (1.0 - p) - 0.5 * math.erfc(x / math.sqrt(2.0))
        u = e * math.sqrt(2.0 * math.pi) * math.exp(x * x / 2.0)
        x = x - u / (1.0 + x * u / 2.0)
    return x


def _import_quiet(name):
    """Import a checking library whose numba dependency imports a module named `coverage`: the factory's tools
    folder, first on sys.path while certifying, holds an unrelated coverage.py, so it is hidden while importing."""
    import importlib
    import os
    import sys
    mod = sys.modules.get("coverage")
    if mod is not None and not hasattr(mod, "types"):
        del sys.modules["coverage"]
    saved = list(sys.path)
    sys.path[:] = [p for p in sys.path if not os.path.exists(os.path.join(p or ".", "coverage.py"))]
    try:
        return importlib.import_module(name)
    finally:
        sys.path[:] = saved


# ================================================================================================ the question reader
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_SEG = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(" + _NUM + r")\s*(.*?)\s*$")
_WORD = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*([A-Za-z][A-Za-z_0-9 \-]*?)\s*$")
_VEC = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(\[[^\[\]]*\])\s*$")
_MAT = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(\[\s*\[.*\]\s*\])\s*$")
_PCT = {"%": 0.01, "percent": 0.01, "per cent": 0.01}


def _split(rest):
    """Split on commas outside brackets."""
    parts, depth, cur = [], 0, ""
    for ch in rest:
        if ch == "[":
            depth += 1
        elif ch == "]":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    parts.append(cur)
    return parts


def _reader(phrases, inputs, aliases=None):
    """The one documented question form, '<phrase>: name = value, ...', for a procedure with these inputs.

    The reading guard is built in: every stated number is bound to a named input (any other text or number refuses
    the whole question); the capability phrase must open the question and be followed by the colon; a unit on a pure
    number is refused; a rate written as a plain number above 1 is refused (write 5% or 0.05)."""
    rx = re.compile(r"\s*(?:" + "|".join(re.escape(p) for p in phrases) + r")\s*:\s*", re.I)
    names = {}
    by = {}
    for i in inputs:
        names[i["symbol"]] = i["symbol"]
        by[i["symbol"]] = i
    for i in inputs:                       # a lower-case spelling only where it names no other input
        names.setdefault(i["symbol"].lower(), i["symbol"])
    for a, s in (aliases or {}).items():
        names[a] = s
        names[a.lower()] = s

    def parse(question):
        if not isinstance(question, str):
            return None
        m = rx.match(question)
        if not m:
            return None
        rest = re.sub(r"[.?!]\s*$", "", question[m.end():].strip()).strip()
        if not rest:
            return {}
        out = {}
        for seg in _split(rest):
            if not seg.strip():
                return None
            mm = _MAT.match(seg)
            if mm:
                key = names.get(mm.group(1), names.get(mm.group(1).lower()))
                if key is None or key in out or by[key].get("type") != "matrix":
                    return None
                out[key] = mm.group(2).strip()
                continue
            mv = _VEC.match(seg)
            if mv:
                key = names.get(mv.group(1), names.get(mv.group(1).lower()))
                if key is None or key in out or by[key].get("type") != "vector":
                    return None
                out[key] = mv.group(2).strip()
                continue
            mw = _WORD.match(seg)
            if mw:
                key = names.get(mw.group(1), names.get(mw.group(1).lower()))
                if key is not None and by[key].get("type") == "choice":
                    if key in out:
                        return None
                    out[key] = mw.group(2).strip().lower().replace(" ", "_").replace("-", "_")
                    continue
                return None
            ms = _SEG.match(seg)
            if not ms:
                return None
            key = names.get(ms.group(1), names.get(ms.group(1).lower()))
            if key is None or key in out:
                return None
            spec = by[key]
            t = spec.get("type", "number")
            num, unit = ms.group(2), ms.group(3).strip()
            if t in ("vector", "matrix", "choice", "text"):
                return None
            if t == "integer":
                if unit:
                    return None
                out[key] = num
                continue
            if unit:
                if spec.get("pct") and unit.lower() in _PCT:
                    out[key] = float(num) * _PCT[unit.lower()]
                    continue
                if unit in spec.get("words", ()):
                    out[key] = float(num)
                    continue
                return None
            if spec.get("pct") and abs(float(num)) > 1:
                return None                       # '5' for a rate: 5 % or 5? refused; write 5% or 0.05
            out[key] = float(num)
        return out
    return parse


def _fmt(x):
    return f"{x:.10g}"


def _written(spec, v):
    """An input value as a user writes it in the documented form."""
    t = spec.get("type", "number")
    if t == "matrix":
        return "[" + ", ".join("[" + ", ".join(_fmt(float(x)) for x in row) + "]" for row in v) + "]"
    if t == "vector":
        return "[" + ", ".join(_fmt(float(x)) for x in v) + "]"
    if t in ("choice", "text"):
        return str(v)
    if t == "integer":
        return str(int(v))
    if spec.get("words"):
        return f"{_fmt(float(v))} {spec['words'][0]}"
    return _fmt(float(v))


# ================================================================================================ input/output specs
def _numin(sym, meaning, required=True, pct=False, words=None, lo=None, hi=None):
    d = {"symbol": sym, "kind": "number", "meaning": meaning, "required": required}
    if pct:
        d["pct"] = True
    if words:
        d["words"] = list(words)
    if lo is not None:
        d["min"] = lo
    if hi is not None:
        d["max"] = hi
    return d


def _ratein(sym="i", meaning="annual effective interest rate (5% or 0.05)"):
    return _numin(sym, meaning, pct=True)


def _intin(sym, meaning, lo=None, hi=None, required=True):
    d = {"symbol": sym, "type": "integer", "meaning": meaning, "required": required}
    if lo is not None:
        d["min"] = lo
    if hi is not None:
        d["max"] = hi
    return d


def _vecin(sym, meaning, lo=1, hi=130, required=True):
    return {"symbol": sym, "type": "vector", "kind": "number", "meaning": meaning, "required": required,
            "min_length": lo, "max_length": hi}


def _matin(sym, meaning, required=True):
    return {"symbol": sym, "type": "matrix", "kind": "number", "meaning": meaning, "required": required}


def _choice(sym, choices, meaning, required=True):
    return {"symbol": sym, "type": "choice", "choices": list(choices), "meaning": meaning, "required": required}


def _o(sym, meaning, kind="number"):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


PROCEDURES: dict = {}
LEVELS: dict = {}
WEAK: dict = {}


def _add(key, *, name, topic, purpose, inputs, outputs, primary, method, latex, run, reference, wrong, rand,
         phrases, examples, level, conditions=(), aliases=None, extra_q=(), verify=None, weak=None):
    """Register a procedure: the documented question form is built from its phrases and inputs; templates and
    questions are written from the worked examples exactly as a user writes them."""
    parse = _reader(phrases, inputs, aliases)
    by = {i["symbol"]: i for i in inputs}

    def ask_text(ex_inputs):
        return f"{phrases[0]}: " + ", ".join(f"{k} = {_written(by[k], v)}" for k, v in ex_inputs.items())
    templates = [ask_text(e["inputs"]) for e in examples[:2]]
    questions = [{"q": ask_text(e["inputs"]), "outputs": e["outputs"]} for e in examples]
    req = [i["symbol"] for i in inputs if i.get("required")]
    if req and examples:
        partial = {k: v for k, v in examples[0]["inputs"].items() if k != req[0]}
        questions.append({"q": ask_text(partial), "outcome": "NEEDS_INPUT"})
        questions.append({"q": ask_text(examples[0]["inputs"]) + ", zz = 1", "outcome": "UNRECOGNIZED_FORMAT"})
    questions += [{"q": q["q"], "outputs": examples[q["like"]]["outputs"]} if "like" in q else q for q in extra_q]
    PROCEDURES[key] = {
        "name": name, "topic": topic, "purpose": purpose, "inputs": inputs, "outputs": outputs, "primary": primary,
        "conditions": list(conditions), "method": method, "latex": latex, "run": run,
        "verify": verify or _recheck(run), "parse": parse, "reference": reference, "wrong": _safe(wrong),
        "random_inputs": rand, "templates": templates, "examples": list(examples), "questions": questions}
    LEVELS[key] = level
    if weak:
        WEAK[key] = weak


def _ex(inputs, outputs=None):
    return {"inputs": inputs, "outputs": outputs or {}}


def _rand_i(rng):
    return _sig(rng.uniform(0.01, 0.12), 3)


# ================================================================================================ INTEREST THEORY
def rate_conversion(i, m):
    _rate(i)
    _whole("the number of conversion periods per year m", m, 1)
    _need(m <= 365, "m must be at most 365")
    v = 1.0 / (1.0 + i)
    i_m = m * ((1.0 + i) ** (1.0 / m) - 1.0)
    d_m = m * (1.0 - v ** (1.0 / m))
    return _out(d=i * v, v=v, delta=math.log1p(i), i_m=i_m, d_m=d_m)


def _ref_rate_conversion(i, m):
    j = _brentq(lambda x: (1.0 + x / m) ** m - (1.0 + i), 0.0, i)
    dm = _brentq(lambda x: (1.0 - x / m) ** (-m) - (1.0 + i), 0.0, min(float(m), 1.0) * 0.999999)
    return {"d": 1.0 - 1.0 / (1.0 + i), "v": 1.0 / (1.0 + i), "delta": math.log(1.0 + i), "i_m": j, "d_m": dm}


_add("act_interest_rate_conversion",
     name="Equivalent rates from an effective annual interest rate", topic="interest theory / rates of interest",
     purpose="The effective discount rate, discount factor, force of interest and the nominal interest and discount rates convertible m times a year equivalent to an annual effective interest rate.",
     inputs=[_ratein(), _intin("m", "conversion periods per year for the nominal rates (12 = monthly)", 1, 365)],
     outputs=[_o("d", "effective annual discount rate"), _o("v", "annual discount factor 1/(1+i)"),
              _o("delta", "force of interest ln(1+i)"), _o("i_m", "nominal interest rate convertible m-thly"),
              _o("d_m", "nominal discount rate convertible m-thly")],
     primary="d", conditions=["0 < i < 1", "1 <= m <= 365"],
     method="d = i/(1+i), v = 1/(1+i), delta = ln(1+i), i(m) = m[(1+i)^(1/m) - 1], d(m) = m[1 - v^(1/m)]",
     latex=r"d = \frac{i}{1+i},\ \delta = \ln(1+i),\ i^{(m)} = m\left[(1+i)^{1/m}-1\right],\ d^{(m)} = m\left[1-v^{1/m}\right]",
     run=rate_conversion, reference=_ref_rate_conversion,
     wrong=lambda i, m: _out(d=i / (1.0 - i), v=1.0 / (1.0 + i), delta=math.log1p(i), i_m=i / m, d_m=(i / (1 + i)) / m),
     rand=lambda rng: {"i": _rand_i(rng), "m": rng.choice([2, 4, 12, 52, 365])},
     phrases=("Interest rate conversion", "Equivalent rates", "Convert an effective interest rate"),
     examples=[_ex({"i": 0.05, "m": 12}), _ex({"i": 0.08, "m": 4})],
     level="beginner",
     extra_q=[{"q": "Equivalent rates: i = 5%, m = 12", "like": 0},
              {"q": "Interest rate conversion: i = 5, m = 12", "outcome": "UNRECOGNIZED_FORMAT"}],
     weak="the nominal rates re-solved from their defining equations by brentq; d, v and delta are definitions")


def nominal_to_effective(rate, m, kind):
    _need(kind in ("interest", "discount"), "kind must be interest or discount")
    _whole("the number of conversion periods per year m", m, 1)
    _need(m <= 365, "m must be at most 365")
    _need(_isnum(rate) and rate > 0, "the nominal rate must be positive")
    if kind == "interest":
        _need(rate < 1, "the nominal rate must lie below 100%")
        i = (1.0 + rate / m) ** m - 1.0
    else:
        _need(rate < min(1.0, m), "the nominal discount rate must lie below 100%")
        i = (1.0 - rate / m) ** (-m) - 1.0
    return _out(i=i, d=i / (1.0 + i), delta=math.log1p(i), v=1.0 / (1.0 + i))


def _ref_nominal_to_effective(rate, m, kind):
    delta = m * math.log1p(rate / m) if kind == "interest" else -m * math.log1p(-rate / m)
    i = math.expm1(delta)
    return {"i": i, "d": -math.expm1(-delta), "delta": delta, "v": math.exp(-delta)}


_add("act_nominal_to_effective_rate",
     name="Effective annual rate from a nominal rate", topic="interest theory / rates of interest",
     purpose="The annual effective interest and discount rates, force of interest and discount factor equivalent to a nominal interest or discount rate convertible m times a year.",
     inputs=[_numin("rate", "nominal annual rate (6% or 0.06)", pct=True),
             _intin("m", "conversion periods per year (12 = monthly)", 1, 365),
             _choice("kind", ("interest", "discount"), "the nominal rate is an interest rate i(m) or a discount rate d(m)")],
     outputs=[_o("i", "annual effective interest rate"), _o("d", "annual effective discount rate"),
              _o("delta", "force of interest"), _o("v", "annual discount factor")],
     primary="i", conditions=["0 < rate < 1", "1 <= m <= 365"],
     method="1 + i = (1 + i(m)/m)^m or (1 - d(m)/m)^(-m); d = i/(1+i); delta = ln(1+i)",
     latex=r"1+i = \left(1+\frac{i^{(m)}}{m}\right)^m = \left(1-\frac{d^{(m)}}{m}\right)^{-m}",
     run=nominal_to_effective, reference=_ref_nominal_to_effective,
     wrong=lambda rate, m, kind: nominal_to_effective(rate, m, "discount" if kind == "interest" else "interest"),
     rand=lambda rng: {"rate": _sig(rng.uniform(0.01, 0.15), 3), "m": rng.choice([2, 4, 12, 365]), "kind": rng.choice(["interest", "discount"])},
     phrases=("Effective rate from a nominal rate", "Nominal to effective rate", "Effective annual rate"),
     examples=[_ex({"rate": 0.06, "m": 12, "kind": "interest"}), _ex({"rate": 0.08, "m": 4, "kind": "discount"})],
     level="beginner",
     extra_q=[{"q": "Nominal to effective rate: rate = 6%, m = 12, kind = interest", "like": 0}],
     weak="the same definition evaluated through the force of interest (log1p/expm1)")


def force_of_interest(delta):
    _need(_isnum(delta) and 0 < delta < 1, "the force of interest must lie between 0 and 1")
    i = math.exp(delta) - 1.0
    return _out(i=i, d=1.0 - math.exp(-delta), v=math.exp(-delta))


def _ref_force_of_interest(delta):
    acc = math.exp(_quad(lambda t: delta, 0.0, 1.0))
    return {"i": acc - 1.0, "d": 1.0 - 1.0 / acc, "v": 1.0 / acc}


_add("act_force_of_interest",
     name="Effective rates from a constant force of interest", topic="interest theory / rates of interest",
     purpose="The annual effective interest rate, discount rate and discount factor for a constant force of interest delta.",
     inputs=[_numin("delta", "constant force of interest per year (0.05 or 5%)", pct=True)],
     outputs=[_o("i", "annual effective interest rate"), _o("d", "annual effective discount rate"), _o("v", "annual discount factor")],
     primary="i", conditions=["0 < delta < 1"],
     method="i = e^delta - 1, d = 1 - e^(-delta), v = e^(-delta)",
     latex=r"i = e^{\delta}-1,\quad d = 1-e^{-\delta},\quad v = e^{-\delta}",
     run=force_of_interest, reference=_ref_force_of_interest,
     wrong=lambda delta: _out(i=delta, d=delta / (1 + delta), v=1 / (1 + delta)),
     rand=lambda rng: {"delta": _sig(rng.uniform(0.01, 0.15), 3)},
     phrases=("Force of interest", "Effective rate from a force of interest"),
     examples=[_ex({"delta": 0.05}), _ex({"delta": 0.1})],
     level="beginner",
     weak="the accumulation factor exp(integral of delta) by scipy quad")


def accumulated_value(P, i, t):
    _fin(P)
    _rate(i)
    _need(_isnum(t) and 0 <= t <= 200, "the time t must lie between 0 and 200 years")
    f = (1.0 + i) ** t
    return _out(AV=P * f, PV=P / f, factor=f)


def _ref_accumulated_value(P, i, t):
    from scipy.integrate import solve_ivp
    dl = math.log(1.0 + i)
    if t == 0:
        f = 1.0
    else:
        s = solve_ivp(lambda s, y: [dl * y[0]], (0.0, t), [1.0], method="DOP853", rtol=1e-13, atol=1e-20)
        f = float(s.y[0, -1])
    return {"AV": P * f, "PV": P / f, "factor": f}


_add("act_accumulated_value",
     name="Accumulated and present value of a single amount", topic="interest theory / compound interest",
     purpose="The value of an amount accumulated for t years at compound interest, the present value of the same amount due in t years, and the accumulation factor (1+i)^t.",
     inputs=[_numin("P", "amount"), _ratein(), _numin("t", "time in years")],
     outputs=[_o("AV", "accumulated value P(1+i)^t"), _o("PV", "present value P v^t"), _o("factor", "accumulation factor (1+i)^t")],
     primary="AV", conditions=["0 < i < 1", "0 <= t <= 200"],
     method="AV = P(1+i)^t; PV = P(1+i)^(-t)",
     latex=r"AV = P(1+i)^t,\quad PV = P\,v^t",
     run=accumulated_value, reference=_ref_accumulated_value,
     wrong=lambda P, i, t: _out(AV=P * (1 + i * t), PV=P / (1 + i * t), factor=1 + i * t),
     rand=lambda rng: {"P": _sig(_lu(rng, 10.0, 1e6), 4), "i": _rand_i(rng), "t": _sig(rng.uniform(0.5, 40.0), 3)},
     phrases=("Accumulated value", "Compound interest", "Present value of a single amount"),
     examples=[_ex({"P": 1000.0, "i": 0.05, "t": 10.0}), _ex({"P": 2500.0, "i": 0.07, "t": 3.5})],
     level="beginner",
     weak="the growth equation dA/dt = delta A integrated by scipy DOP853")


def _annuity_values(n, i):
    v = 1.0 / (1.0 + i)
    an = (1.0 - v ** n) / i
    d = i * v
    return v, an, an * (1.0 + i), ((1.0 + i) ** n - 1.0) / i, ((1.0 + i) ** n - 1.0) / d


def annuity_certain(n, i):
    _whole("the number of payments n", n, 1)
    _need(n <= 1000, "n must be at most 1000")
    _rate(i)
    v, an, adn, sn, sdn = _annuity_values(n, i)
    delta = math.log1p(i)
    return _out(a_n=an, a_due=adn, s_n=sn, s_due=sdn, a_cont=(1.0 - v ** n) / delta)


def _ref_annuity_certain(n, i):
    import numpy as np
    k = np.arange(1, n + 1, dtype=float)
    v = 1.0 / (1.0 + i)
    an = float(np.sum(v ** k))
    adn = float(np.sum(v ** (k - 1)))
    sn = float(np.sum((1.0 + i) ** (n - k)))
    sdn = float(np.sum((1.0 + i) ** (n - k + 1)))
    ac = _quad(lambda t: v ** t, 0.0, float(n))
    return {"a_n": an, "a_due": adn, "s_n": sn, "s_due": sdn, "a_cont": ac}


_add("act_annuity_certain",
     name="Annuity-certain present and accumulated values", topic="interest theory / annuities-certain",
     purpose="The present value of n level payments of 1 at the end (immediate) or start (due) of each year, their accumulated values at time n, and the continuously payable annuity over n years.",
     inputs=[_intin("n", "number of annual payments (term in years)", 1, 1000), _ratein()],
     outputs=[_o("a_n", "present value of an annuity-immediate a_n"), _o("a_due", "present value of an annuity-due"),
              _o("s_n", "accumulated value of an annuity-immediate s_n"), _o("s_due", "accumulated value of an annuity-due"),
              _o("a_cont", "present value of a continuous annuity over n years")],
     primary="a_n", conditions=["n >= 1", "0 < i < 1"],
     method="a_n = (1 - v^n)/i, a-due = (1 - v^n)/d, s_n = ((1+i)^n - 1)/i, s-due = ((1+i)^n - 1)/d, continuous = (1 - v^n)/delta",
     latex=r"a_{\overline{n}|} = \frac{1-v^n}{i},\ \ddot a_{\overline{n}|} = \frac{1-v^n}{d},\ s_{\overline{n}|} = \frac{(1+i)^n-1}{i},\ \bar a_{\overline{n}|} = \frac{1-v^n}{\delta}",
     run=annuity_certain, reference=_ref_annuity_certain,
     wrong=lambda n, i: (lambda v: _out(a_n=(1 - v ** n) / (i * v), a_due=(1 - v ** n) / i, s_n=((1 + i) ** n - 1) / i,
                                        s_due=((1 + i) ** n - 1) / i, a_cont=(1 - v ** n) / i))(1 / (1 + i)),
     rand=lambda rng: {"n": rng.randint(1, 60), "i": _rand_i(rng)},
     phrases=("Annuity-certain", "Annuity certain", "Present value of an annuity-certain"),
     examples=[_ex({"n": 10, "i": 0.05}), _ex({"n": 25, "i": 0.08})],
     level="beginner",
     extra_q=[{"q": "Annuity certain: n = 10, i = 5%", "like": 0},
              {"q": "Annuity-certain: n = 0, i = 5%", "outcome": "INVALID_INPUT"}])


def perpetuity(i):
    _rate(i)
    return _out(a_inf=1.0 / i, a_due_inf=(1.0 + i) / i, a_cont_inf=1.0 / math.log1p(i))


def _ref_perpetuity(i):
    import sympy as sp
    k = sp.symbols("k", integer=True, nonnegative=True)
    v = 1 / (1 + sp.nsimplify(i))
    a = sp.summation(v ** (k + 1), (k, 0, sp.oo))
    ad = sp.summation(v ** k, (k, 0, sp.oo))
    ac = _quad(lambda t: (1.0 + i) ** (-t), 0.0, math.inf)
    return {"a_inf": float(a), "a_due_inf": float(ad), "a_cont_inf": ac}


_add("act_perpetuity",
     name="Present value of a perpetuity", topic="interest theory / annuities-certain",
     purpose="The present value of payments of 1 a year forever, at the end (immediate) or start (due) of each year, and paid continuously.",
     inputs=[_ratein()],
     outputs=[_o("a_inf", "perpetuity-immediate 1/i"), _o("a_due_inf", "perpetuity-due 1/d"), _o("a_cont_inf", "continuous perpetuity 1/delta")],
     primary="a_inf", conditions=["0 < i < 1"],
     method="a_inf = 1/i, a-due_inf = 1/d = (1+i)/i, continuous 1/delta",
     latex=r"a_{\overline{\infty}|} = \frac1i,\quad \ddot a_{\overline{\infty}|} = \frac1d,\quad \bar a_{\overline{\infty}|} = \frac1\delta",
     run=perpetuity, reference=_ref_perpetuity,
     wrong=lambda i: _out(a_inf=1.0 / i, a_due_inf=1.0 / i, a_cont_inf=1.0 / i),
     rand=lambda rng: {"i": _rand_i(rng)},
     phrases=("Perpetuity", "Present value of a perpetuity"),
     examples=[_ex({"i": 0.05}), _ex({"i": 0.04})],
     level="beginner")


def increasing_annuity(n, i):
    _whole("the term n", n, 1)
    _need(n <= 1000, "n must be at most 1000")
    _rate(i)
    v, an, adn, sn, sdn = _annuity_values(n, i)
    ia = (adn - n * v ** n) / i
    da = (n - an) / i
    return _out(Ia=ia, Ia_due=ia * (1.0 + i), Da=da, Da_due=da * (1.0 + i))


def _ref_increasing_annuity(n, i):
    import numpy as np
    k = np.arange(1, n + 1, dtype=float)
    v = 1.0 / (1.0 + i)
    ia = float(np.sum(k * v ** k))
    da = float(np.sum((n + 1 - k) * v ** k))
    return {"Ia": ia, "Ia_due": float(np.sum(k * v ** (k - 1))), "Da": da, "Da_due": float(np.sum((n + 1 - k) * v ** (k - 1)))}


_add("act_increasing_decreasing_annuity",
     name="Increasing and decreasing annuities-certain", topic="interest theory / varying annuities",
     purpose="The present values of an n-year annuity paying 1, 2, ..., n (increasing) and n, n-1, ..., 1 (decreasing), immediate and due.",
     inputs=[_intin("n", "term in years", 1, 1000), _ratein()],
     outputs=[_o("Ia", "increasing annuity-immediate (Ia)_n"), _o("Ia_due", "increasing annuity-due"),
              _o("Da", "decreasing annuity-immediate (Da)_n"), _o("Da_due", "decreasing annuity-due")],
     primary="Ia", conditions=["n >= 1", "0 < i < 1"],
     method="(Ia)_n = (a-due_n - n v^n)/i; (Da)_n = (n - a_n)/i; due values times (1+i)",
     latex=r"(Ia)_{\overline{n}|} = \frac{\ddot a_{\overline{n}|}-nv^n}{i},\quad (Da)_{\overline{n}|} = \frac{n-a_{\overline{n}|}}{i}",
     run=increasing_annuity, reference=_ref_increasing_annuity,
     wrong=lambda n, i: (lambda v, an: _out(Ia=(an - n * v ** n) / i, Ia_due=(an - n * v ** n) / i * (1 + i), Da=(n - an) / i,
                                            Da_due=(n - an) / i))(1 / (1 + i), (1 - (1 / (1 + i)) ** n) / i),
     rand=lambda rng: {"n": rng.randint(2, 50), "i": _rand_i(rng)},
     phrases=("Increasing and decreasing annuity", "Increasing annuity", "Decreasing annuity"),
     examples=[_ex({"n": 10, "i": 0.05}), _ex({"n": 5, "i": 0.1})],
     level="intermediate")


def loan_amortization(L, i, n, t):
    _pos("the loan amount L", L)
    _rate(i)
    _whole("the term n", n, 1)
    _need(n <= 1000, "n must be at most 1000")
    _whole("the payment number t", t, 1)
    _need(t <= n, "the payment number t must lie between 1 and n")
    v = 1.0 / (1.0 + i)
    an = (1.0 - v ** n) / i
    R = L / an
    bal_before = R * (1.0 - v ** (n - t + 1)) / i
    interest = i * bal_before
    principal = R - interest
    bal_after = R * (1.0 - v ** (n - t)) / i
    return _out(R=R, interest_t=interest, principal_t=principal, balance_t=bal_after, total_interest=n * R - L)


def _ref_loan_amortization(L, i, n, t):
    from scipy.optimize import brentq
    def bal(R):
        b = L
        for _ in range(n):
            b = b * (1 + i) - R
        return b
    R = float(brentq(bal, L / n * 0.5, L * (1 + i) * 1.5, xtol=1e-300, rtol=8.9e-16, maxiter=500))
    b = L
    interest = principal = 0.0
    for k in range(1, t + 1):
        interest = b * i
        principal = R - interest
        b = b - principal
    if t == n:
        b = 0.0
    return {"R": R, "interest_t": interest, "principal_t": principal, "balance_t": b, "total_interest": n * R - L}


_add("act_loan_amortization",
     name="Loan amortization: level payment, interest and principal, outstanding balance", topic="interest theory / loans",
     purpose="The level annual payment that repays a loan over n years, the interest and principal parts of payment t and the outstanding balance just after it.",
     inputs=[_numin("L", "loan amount"), _ratein(), _intin("n", "number of annual payments", 1, 1000),
             _intin("t", "payment number of interest (1 to n)", 1, 1000)],
     outputs=[_o("R", "level annual payment"), _o("interest_t", "interest part of payment t"),
              _o("principal_t", "principal repaid in payment t"), _o("balance_t", "outstanding balance just after payment t"),
              _o("total_interest", "total interest paid over the loan")],
     primary="R", conditions=["L > 0", "0 < i < 1", "1 <= t <= n"],
     method="R = L/a_n; balance before payment t = R a_(n-t+1); interest = i x balance; principal = R v^(n-t+1); balance after = R a_(n-t)",
     latex=r"R = \frac{L}{a_{\overline{n}|}},\quad I_t = R\left(1-v^{n-t+1}\right),\quad P_t = R\,v^{n-t+1},\quad B_t = R\,a_{\overline{n-t}|}",
     run=loan_amortization, reference=_ref_loan_amortization,
     wrong=lambda L, i, n, t: (lambda o: dict(o, balance_t=o["balance_t"] + o["principal_t"], principal_t=o["R"] - i * L))(loan_amortization(L, i, n, t)),
     rand=lambda rng: (lambda n: {"L": _sig(_lu(rng, 1e3, 1e6), 4), "i": _rand_i(rng), "n": n, "t": rng.randint(1, n - 1)})(rng.randint(3, 40)),
     phrases=("Loan amortization", "Amortization schedule", "Loan repayment"),
     examples=[_ex({"L": 100000.0, "i": 0.06, "n": 20, "t": 5}), _ex({"L": 5000.0, "i": 0.1, "n": 4, "t": 2})],
     level="intermediate",
     extra_q=[{"q": "Loan amortization: L = 5000, i = 10%, n = 4, t = 5", "outcome": "INVALID_INPUT"}],
     weak="the payment re-solved by brentq on a year-by-year balance roll-forward; the schedule simulated")


def annuity_yield(PV, R, n):
    _pos("the present value and the payment", PV, R)
    _whole("the number of payments n", n, 1)
    _need(n <= 1000, "n must be at most 1000")
    _need(PV < n * R, "the present value must be below the total of the payments (a positive yield)")
    _need(PV > R * (1.0 - 1.99 ** (-n)) / 0.99, "the yield would exceed 99%")

    def f(j):
        return R * (1.0 - (1.0 + j) ** (-n)) / j - PV
    j = _bisect(f, 1e-12, 0.99)
    return _out(i=j, a_n=PV / R)


def _ref_annuity_yield(PV, R, n):
    import numpy as np
    # PV (1+i)^n = R [(1+i)^n - 1]/i  <=> polynomial in v: R(v + ... + v^n) - PV = 0
    coeffs = [R] * n + [-PV]                  # highest power v^n first ... v^1, constant -PV
    rts = np.roots(coeffs)
    v = [float(z.real) for z in rts if abs(z.imag) < 1e-9 and 0 < z.real < 1]
    v = sorted(v)[-1]
    vv = _brentq(lambda x: sum(R * x ** k for k in range(1, n + 1)) - PV, v * (1 - 1e-6), min(v * (1 + 1e-6), 1 - 1e-15))
    return {"i": 1.0 / vv - 1.0, "a_n": PV / R}


def _verify_annuity_yield(inputs, outputs):
    try:
        j = outputs["i"]
        n, R, PV = inputs["n"], inputs["R"], inputs["PV"]
        val = R * (1.0 - (1.0 + j) ** (-n)) / j
        return max(abs(val - PV) / PV, _rel(outputs["a_n"], PV / R))
    except Exception:
        return math.inf


_add("act_annuity_yield",
     name="Yield rate of a level annuity", topic="interest theory / yield rates",
     purpose="The annual effective interest rate at which n level payments R at the ends of the years are worth a given present value.",
     inputs=[_numin("PV", "present value (price) of the annuity"), _numin("R", "level annual payment"),
             _intin("n", "number of annual payments", 1, 1000)],
     outputs=[_o("i", "yield rate (annual effective)"), _o("a_n", "annuity factor PV/R")],
     primary="i", conditions=["0 < PV < n R", "yield below 99%"],
     method="solve R a_n(i) = PV for i by bisection (a_n decreases in i, so the root is unique)",
     latex=r"R\,\frac{1-(1+i)^{-n}}{i} = PV",
     run=annuity_yield, reference=_ref_annuity_yield,
     wrong=lambda PV, R, n: _out(i=(n * R - PV) / (PV * n), a_n=PV / R),
     rand=lambda rng: (lambda n, i, R: {"PV": _sig(R * (1 - (1 + i) ** (-n)) / i, 6), "R": R, "n": n})(rng.randint(2, 40), rng.uniform(0.01, 0.15), _sig(_lu(rng, 100, 1e5), 3)),
     phrases=("Yield of an annuity", "Annuity yield rate", "Yield rate of an annuity"),
     examples=[_ex({"PV": 7721.73, "R": 1000.0, "n": 10}), _ex({"PV": 4000.0, "R": 1000.0, "n": 5})],
     level="intermediate", verify=_verify_annuity_yield,
     extra_q=[{"q": "Yield of an annuity: PV = 12000, R = 1000, n = 10", "outcome": "INVALID_INPUT"}])


def pv_cash_flows(amounts, times, i):
    _rate(i)
    _need(len(amounts) == len(times), "give one time for every amount")
    for t in times:
        _need(_isnum(t) and 0 <= t <= 200, "every time must lie between 0 and 200 years")
    v = 1.0 / (1.0 + i)
    pv = 0.0
    for c, t in zip(amounts, times):
        pv += c * v ** t
    av = pv * (1.0 + i) ** max(times)
    return _out(PV=pv, AV_at_last=av)


def _ref_pv_cash_flows(amounts, times, i):
    import numpy as np
    c, t = np.array(amounts, dtype=float), np.array(times, dtype=float)
    pv = float(np.dot(c, np.exp(-np.log1p(i) * t)))
    T = float(t.max())
    return {"PV": pv, "AV_at_last": float(np.dot(c, np.exp(np.log1p(i) * (T - t))))}


_add("act_present_value_cash_flows",
     name="Present value of a series of cash flows", topic="interest theory / cash flows",
     purpose="The present value at time 0 of amounts paid at stated times (years), and their accumulated value at the last payment time.",
     inputs=[_vecin("amounts", "cash flow amounts (a list)", 1, 200), _vecin("times", "payment times in years (a list)", 1, 200), _ratein()],
     outputs=[_o("PV", "present value at time 0"), _o("AV_at_last", "accumulated value at the last payment time")],
     primary="PV", conditions=["one time per amount", "0 <= t <= 200", "0 < i < 1"],
     method="PV = sum C_k v^(t_k); AV = PV (1+i)^(t_max)",
     latex=r"PV = \sum_k C_k\,v^{t_k}",
     run=pv_cash_flows, reference=_ref_pv_cash_flows,
     wrong=lambda amounts, times, i: _out(PV=sum(c / (1 + i * t) for c, t in zip(amounts, times)), AV_at_last=0.0),
     rand=lambda rng: (lambda k: {"amounts": [_sig(rng.uniform(-500, 2000), 4) for _ in range(k)],
                                  "times": [_sig(rng.uniform(0.5, 30), 3) for _ in range(k)], "i": _rand_i(rng)})(rng.randint(2, 8)),
     phrases=("Present value of cash flows", "Net present value of cash flows"),
     examples=[_ex({"amounts": [100.0, 200.0, 300.0], "times": [1.0, 2.0, 3.0], "i": 0.05}),
               _ex({"amounts": [-1000.0, 400.0, 400.0, 400.0], "times": [0.0, 1.0, 2.0, 3.0], "i": 0.08})],
     level="beginner",
     weak="the same sum by numpy dot products in exponential form")


def duration_convexity(amounts, times, i):
    _rate(i)
    _need(len(amounts) == len(times), "give one time for every amount")
    for c in amounts:
        _need(_isnum(c) and c >= 0, "the cash flows must be zero or positive (liability or bond cash flows)")
    for t in times:
        _need(_isnum(t) and 0 <= t <= 200, "every time must lie between 0 and 200 years")
    v = 1.0 / (1.0 + i)
    p = s1 = s2 = 0.0
    for c, t in zip(amounts, times):
        w = c * v ** t
        p += w
        s1 += t * w
        s2 += t * (t + 1.0) * w
    _need(p > 0, "the present value must be positive")
    mac = s1 / p
    return _out(PV=p, macaulay=mac, modified=mac * v, convexity=s2 * v * v / p)


def _ref_duration_convexity(amounts, times, i):
    import sympy as sp
    x = sp.symbols("x", positive=True)
    P = sum(sp.Float(c, 30) * (1 + x) ** (-sp.Float(t, 30)) for c, t in zip(amounts, times))
    xi = sp.Float(i, 30)
    p = P.subs(x, xi)
    d1 = sp.diff(P, x).subs(x, xi)
    d2 = sp.diff(P, x, 2).subs(x, xi)
    mod = float(-d1 / p)
    return {"PV": float(p), "macaulay": mod * (1 + i), "modified": mod, "convexity": float(d2 / p)}


_add("act_duration_convexity",
     name="Macaulay and modified duration and convexity of cash flows", topic="interest theory / duration and immunization",
     purpose="The present value, Macaulay duration, modified duration and convexity (second derivative of price with respect to i over price) of a set of non-negative cash flows.",
     inputs=[_vecin("amounts", "cash flow amounts (a list, non-negative)", 1, 200), _vecin("times", "payment times in years (a list)", 1, 200), _ratein()],
     outputs=[_o("PV", "present value"), _o("macaulay", "Macaulay duration (years)"),
              _o("modified", "modified duration -P'(i)/P"), _o("convexity", "convexity P''(i)/P")],
     primary="macaulay", conditions=["amounts >= 0 with a positive total", "one time per amount", "0 < i < 1"],
     method="D_mac = sum t C v^t / P; D_mod = D_mac/(1+i); convexity = sum t(t+1) C v^(t+2) / P",
     latex=r"D_{mac} = \frac{\sum t\,C_t v^t}{P},\quad D_{mod} = \frac{D_{mac}}{1+i},\quad C = \frac{\sum t(t+1)C_t v^{t+2}}{P}",
     run=duration_convexity, reference=_ref_duration_convexity,
     wrong=lambda amounts, times, i: (lambda o: dict(o, macaulay=o["modified"], modified=o["macaulay"]))(duration_convexity(amounts, times, i)),
     rand=lambda rng: (lambda k: {"amounts": [_sig(rng.uniform(10, 2000), 4) for _ in range(k)],
                                  "times": [_sig(rng.uniform(0.5, 30), 3) for _ in range(k)], "i": _rand_i(rng)})(rng.randint(1, 8)),
     phrases=("Duration and convexity", "Macaulay duration", "Duration of cash flows"),
     examples=[_ex({"amounts": [5.0, 5.0, 105.0], "times": [1.0, 2.0, 3.0], "i": 0.05}),
               _ex({"amounts": [1000.0, 2000.0], "times": [2.0, 10.0], "i": 0.04})],
     level="advanced")


# ================================================================================================ LIFE TABLES (lx lists)
def _lx_check(lx, closed=False):
    _need(len(lx) >= 2, "a life table needs at least two ages")
    for v in lx:
        _need(_isnum(v) and v >= 0, "survivor numbers lx must be zero or positive")
    for k in range(len(lx) - 1):
        _need(lx[k + 1] <= lx[k], "survivor numbers lx must not increase with age")
        _need(lx[k] > 0, "only the last survivor number may be 0")
    if closed:
        _need(lx[-1] == 0, "this needs a complete table: the last survivor number must be 0")


def _idx(lx, x0, x, *steps):
    _whole("the starting age x0", x0, 0)
    _whole("the age x", x, 0)
    k = x - x0
    _need(0 <= k < len(lx), "the age x must lie inside the table (from x0 to the last age listed)")
    _need(lx[k] > 0, "nobody is alive at age x in the table")
    for s in steps:
        _whole("every number of years", s, 0)
    _need(k + sum(steps) < len(lx), "the ages asked about run past the end of the table")
    return k


def _rand_lx(rng, closed=True):
    x0 = rng.randint(20, 80)
    A, B, c = rng.uniform(5e-4, 2e-3), rng.uniform(2e-6, 4e-5), rng.uniform(1.08, 1.12)
    l, out, age = 100000.0, [], x0
    while True:
        out.append(float(round(l)))
        q = 1.0 - math.exp(-(A + B * c ** age * (c - 1.0) / math.log(c)))
        l = l * (1.0 - q)
        age += 1
        if round(l) <= 0 or age > 120 or len(out) >= 12:
            break
    if closed:
        out.append(0.0)
    return x0, out


def life_table_functions(lx):
    _lx_check(lx)
    dx, qx, px = [], [], []
    for k in range(len(lx) - 1):
        d = lx[k] - lx[k + 1]
        dx.append(d)
        qx.append(d / lx[k])
        px.append(lx[k + 1] / lx[k])
    return _out(dx=dx, qx=qx, px=px)


def _ref_life_table_functions(lx):
    import numpy as np
    l = np.array(lx, dtype=float)
    d = -np.diff(l)
    p = l[1:] / l[:-1]
    return {"dx": [float(v) for v in d], "qx": [float(v) for v in 1.0 - p], "px": [float(v) for v in p]}


_add("act_life_table_functions",
     name="Life table functions dx, qx and px from survivor numbers", topic="life tables / life table functions",
     purpose="The numbers dying, the one-year death probabilities and the one-year survival probabilities at each age of a life table given by its survivor numbers lx.",
     inputs=[_vecin("lx", "survivors at consecutive integer ages (a list)", 2, 130)],
     outputs=[_o("dx", "deaths between ages x and x+1 (a list)"), _o("qx", "one-year death probabilities (a list)"),
              _o("px", "one-year survival probabilities (a list)")],
     primary="qx", conditions=["lx non-increasing", "only the last lx may be 0"],
     method="d_x = l_x - l_(x+1), q_x = d_x/l_x, p_x = l_(x+1)/l_x",
     latex=r"d_x = l_x - l_{x+1},\quad q_x = \frac{d_x}{l_x},\quad p_x = \frac{l_{x+1}}{l_x}",
     run=life_table_functions, reference=_ref_life_table_functions,
     wrong=lambda lx: _out(dx=[lx[k] - lx[k + 1] for k in range(len(lx) - 1)], qx=[(lx[k] - lx[k + 1]) / lx[k + 1] if lx[k + 1] else 1.0 for k in range(len(lx) - 1)],
                           px=[lx[k + 1] / lx[k] for k in range(len(lx) - 1)]),
     rand=lambda rng: {"lx": _rand_lx(rng, rng.random() < 0.5)[1]},
     phrases=("Life table functions", "Life table qx and px", "Death probabilities from a life table"),
     examples=[_ex({"lx": [1000.0, 950.0, 880.0, 780.0]}), _ex({"lx": [100000.0, 99500.0, 98900.0, 98200.0, 97300.0]})],
     level="beginner",
     extra_q=[{"q": "Life table functions: lx = [1000, 1050, 900]", "outcome": "INVALID_INPUT"}],
     weak="the same definitions by numpy differences and ratios")


def survival_probability(lx, x0, x, t):
    _lx_check(lx)
    k = _idx(lx, x0, x, t)
    p = lx[k + t] / lx[k]
    return _out(tpx=p, tqx=(lx[k] - lx[k + t]) / lx[k], deaths=lx[k] - lx[k + t])


def _ref_survival_probability(lx, x0, x, t):
    import numpy as np
    k = x - x0
    p = float(np.prod([lx[j + 1] / lx[j] for j in range(k, k + t)])) if t > 0 else 1.0
    return {"tpx": p, "tqx": 1.0 - p, "deaths": lx[k] - lx[k + t]}


_add("act_survival_probability",
     name="Probability of surviving t years from a life table", topic="life tables / survival probabilities",
     purpose="The probability that a life aged x survives t more years, the probability of dying within t years, and the number of deaths between ages x and x+t, from survivor numbers lx starting at age x0.",
     inputs=[_vecin("lx", "survivors at consecutive integer ages from x0 (a list)", 2, 130), _intin("x0", "age of the first entry of lx", 0, 130),
             _intin("x", "age of the life", 0, 130), _intin("t", "number of years", 0, 130)],
     outputs=[_o("tpx", "probability of surviving t years"), _o("tqx", "probability of dying within t years"),
              _o("deaths", "number dying between ages x and x+t")],
     primary="tpx", conditions=["x0 <= x and x + t inside the table", "l_x > 0"],
     method="t_p_x = l_(x+t)/l_x; t_q_x = 1 - t_p_x",
     latex=r"{}_tp_x = \frac{l_{x+t}}{l_x},\quad {}_tq_x = \frac{l_x - l_{x+t}}{l_x}",
     run=survival_probability, reference=_ref_survival_probability,
     wrong=lambda lx, x0, x, t: (lambda k: _out(tpx=lx[k + t] / lx[0], tqx=1 - lx[k + t] / lx[0], deaths=lx[k] - lx[k + t]))(x - x0),
     rand=lambda rng: (lambda x0, lx: (lambda x: {"lx": lx, "x0": x0, "x": x, "t": rng.randint(1, len(lx) - 1 - (x - x0))})(x0 + rng.randint(1, len(lx) - 3)))(*_rand_lx(rng, False)),
     phrases=("Survival probability from a life table", "Probability of surviving", "t-year survival probability"),
     examples=[_ex({"lx": [1000.0, 950.0, 880.0, 780.0], "x0": 60, "x": 61, "t": 2}),
               _ex({"lx": [100000.0, 99500.0, 98900.0, 98200.0, 97300.0], "x0": 40, "x": 40, "t": 4})],
     level="beginner",
     extra_q=[{"q": "Probability of surviving: lx = [1000, 950, 880, 780], x0 = 60, x = 61, t = 3", "outcome": "INVALID_INPUT"}],
     weak="the product of one-year survival ratios")


def deferred_death_probability(lx, x0, x, u, t):
    _lx_check(lx)
    k = _idx(lx, x0, x, u, t)
    _need(t >= 1, "the death interval t must be at least one year")
    return _out(prob=(lx[k + u] - lx[k + u + t]) / lx[k], upx=lx[k + u] / lx[k])


def _ref_deferred_death_probability(lx, x0, x, u, t):
    k = x - x0
    up = 1.0
    for j in range(k, k + u):
        up *= lx[j + 1] / lx[j]
    tp = 1.0
    for j in range(k + u, k + u + t):
        tp *= lx[j + 1] / lx[j]
    return {"prob": up * (1.0 - tp), "upx": up}


_add("act_deferred_death_probability",
     name="Deferred death probability u|t q x", topic="life tables / survival probabilities",
     purpose="The probability that a life aged x survives u years and then dies within the following t years, from survivor numbers lx starting at age x0.",
     inputs=[_vecin("lx", "survivors at consecutive integer ages from x0 (a list)", 2, 130), _intin("x0", "age of the first entry of lx", 0, 130),
             _intin("x", "age of the life", 0, 130), _intin("u", "years of deferment", 0, 130), _intin("t", "length of the death interval in years", 1, 130)],
     outputs=[_o("prob", "deferred death probability u|t q_x"), _o("upx", "probability of surviving the deferment")],
     primary="prob", conditions=["x + u + t inside the table", "t >= 1"],
     method="u|t q_x = (l_(x+u) - l_(x+u+t))/l_x = u_p_x * t_q_(x+u)",
     latex=r"{}_{u|t}q_x = \frac{l_{x+u}-l_{x+u+t}}{l_x} = {}_up_x\;{}_tq_{x+u}",
     run=deferred_death_probability, reference=_ref_deferred_death_probability,
     wrong=lambda lx, x0, x, u, t: (lambda k: _out(prob=(lx[k + u] - lx[k + u + t]) / lx[k + u], upx=lx[k + u] / lx[k]))(x - x0),
     rand=lambda rng: (lambda x0, lx: (lambda x, u: {"lx": lx, "x0": x0, "x": x, "u": u, "t": rng.randint(1, len(lx) - 1 - (x - x0) - u)})(x0, rng.randint(1, len(lx) - 3)))(*_rand_lx(rng, False)),
     phrases=("Deferred death probability", "Deferred mortality probability"),
     examples=[_ex({"lx": [1000.0, 950.0, 880.0, 780.0, 650.0], "x0": 60, "x": 60, "u": 2, "t": 2}),
               _ex({"lx": [100000.0, 99500.0, 98900.0, 98200.0, 97300.0], "x0": 40, "x": 41, "u": 1, "t": 1})],
     level="intermediate",
     weak="the product of survival ratios over the deferment and the interval")


def _km_rmst(lx, k, horizon):
    """Reference helper: lifelines Kaplan-Meier on the table's deaths (weights d) and its restricted mean."""
    import numpy as np
    lf = _import_quiet("lifelines")
    from lifelines.utils import restricted_mean_survival_time
    dur, wts = [], []
    for j in range(k, len(lx) - 1):
        d = lx[j] - lx[j + 1]
        if d > 0:
            dur.append(float(j - k + 1))
            wts.append(float(d))
    if lx[-1] > 0:
        dur.append(float(len(lx) - k + 5))
        wts.append(float(lx[-1]))
    km = lf.KaplanMeierFitter().fit(np.array(dur), event_observed=np.ones(len(dur)), weights=np.array(wts))
    rm = float(restricted_mean_survival_time(km, t=float(horizon)))
    s = [float(km.survival_function_at_times(float(h)).iloc[0]) for h in range(horizon + 1)]
    return rm, s


def life_expectancy(lx, x0, x):
    _lx_check(lx, closed=True)
    k = _idx(lx, x0, x)
    tot = 0.0
    for j in range(k + 1, len(lx)):
        tot += lx[j]
    e = tot / lx[k]
    return _out(e_curtate=e, e_complete=e + 0.5)


def _ref_life_expectancy(lx, x0, x):
    k = x - x0
    h = len(lx) - 1 - k
    rm, s = _km_rmst(lx, k, h)
    trap = sum((s[j] + s[j + 1]) / 2.0 for j in range(h))
    return {"e_curtate": rm - 1.0, "e_complete": trap}


_add("act_life_expectancy",
     name="Curtate and complete expectation of life", topic="life tables / expectation of life",
     purpose="The curtate expectation of life e_x (whole years lived) and the complete expectation e°_x under a uniform distribution of deaths within each year, for a life aged x in a complete life table.",
     inputs=[_vecin("lx", "survivors at consecutive integer ages from x0, ending with 0 (a list)", 2, 130),
             _intin("x0", "age of the first entry of lx", 0, 130), _intin("x", "age of the life", 0, 130)],
     outputs=[_o("e_curtate", "curtate expectation of life e_x (years)"), _o("e_complete", "complete expectation of life under UDD (years)")],
     primary="e_curtate", conditions=["the table closes (last lx = 0)", "x inside the table"],
     method="e_x = sum_(k>=1) l_(x+k)/l_x; e°_x = e_x + 1/2 under UDD",
     latex=r"e_x = \sum_{k\ge 1}{}_kp_x,\quad \mathring e_x = e_x + \tfrac12",
     run=life_expectancy, reference=_ref_life_expectancy,
     wrong=lambda lx, x0, x: (lambda k: (lambda e: _out(e_curtate=e, e_complete=e + 0.5))(sum(lx[k:]) / lx[k]))(x - x0),
     rand=lambda rng: (lambda x0, lx: {"lx": lx, "x0": x0, "x": x0 + rng.randint(0, len(lx) - 2)})(*_rand_lx(rng, True)),
     phrases=("Expectation of life", "Life expectancy", "Curtate expectation of life"),
     examples=[_ex({"lx": [1000.0, 900.0, 700.0, 400.0, 0.0], "x0": 95, "x": 95}),
               _ex({"lx": [1000.0, 900.0, 700.0, 400.0, 0.0], "x0": 95, "x": 97})],
     level="intermediate",
     extra_q=[{"q": "Life expectancy: lx = [1000, 900, 700], x0 = 95, x = 95", "outcome": "INVALID_INPUT"}])


def temporary_expectation(lx, x0, x, n):
    _lx_check(lx)
    _need(n >= 1, "the term n must be at least one year")
    k = _idx(lx, x0, x, n)
    cur = comp = 0.0
    for j in range(1, n + 1):
        cur += lx[k + j]
    for j in range(n):
        comp += (lx[k + j] + lx[k + j + 1]) / 2.0
    return _out(e_curtate=cur / lx[k], e_complete=comp / lx[k])


def _ref_temporary_expectation(lx, x0, x, n):
    k = x - x0
    rm, s = _km_rmst(lx, k, n)
    trap = sum((s[j] + s[j + 1]) / 2.0 for j in range(n))
    return {"e_curtate": rm - 1.0 + s[n], "e_complete": trap}


_add("act_temporary_expectation",
     name="Temporary (n-year) expectation of life", topic="life tables / expectation of life",
     purpose="The expected number of whole years (curtate) and of years (complete, UDD) lived within the next n years by a life aged x.",
     inputs=[_vecin("lx", "survivors at consecutive integer ages from x0 (a list)", 2, 130), _intin("x0", "age of the first entry of lx", 0, 130),
             _intin("x", "age of the life", 0, 130), _intin("n", "term in years", 1, 130)],
     outputs=[_o("e_curtate", "curtate n-year temporary expectation e_(x:n)"), _o("e_complete", "complete n-year temporary expectation under UDD")],
     primary="e_curtate", conditions=["x + n inside the table", "n >= 1"],
     method="e_(x:n) = sum_(k=1..n) k_p_x; e°_(x:n) = sum_(k=0..n-1) (k_p_x + (k+1)_p_x)/2",
     latex=r"e_{x:\overline{n}|} = \sum_{k=1}^{n}{}_kp_x,\quad \mathring e_{x:\overline{n}|} = \sum_{k=0}^{n-1}\frac{{}_kp_x+{}_{k+1}p_x}{2}",
     run=temporary_expectation, reference=_ref_temporary_expectation,
     wrong=lambda lx, x0, x, n: (lambda k: _out(e_curtate=sum(lx[k:k + n]) / lx[k], e_complete=sum(lx[k + 1:k + n + 1]) / lx[k] + 0.5))(x - x0),
     rand=lambda rng: (lambda x0, lx: (lambda x: {"lx": lx, "x0": x0, "x": x, "n": rng.randint(1, len(lx) - 1 - (x - x0))})(x0 + rng.randint(0, len(lx) - 3)))(*_rand_lx(rng, rng.random() < 0.5)),
     phrases=("Temporary expectation of life", "n-year temporary life expectancy"),
     examples=[_ex({"lx": [1000.0, 900.0, 700.0, 400.0, 0.0], "x0": 95, "x": 95, "n": 2}),
               _ex({"lx": [100000.0, 99500.0, 98900.0, 98200.0, 97300.0], "x0": 40, "x": 40, "n": 4})],
     level="intermediate")


def central_death_rate(m, assumption):
    _need(assumption in ("udd", "constant_force"), "assumption: udd or constant_force")
    _need(_isnum(m) and m > 0, "the central death rate must be positive")
    if assumption == "udd":
        _need(m < 2, "under UDD the central death rate must lie below 2")
        q = 2.0 * m / (2.0 + m)
    else:
        q = 1.0 - math.exp(-m)
    return _out(q=q, p=1.0 - q)


def _ref_central_death_rate(m, assumption):
    def mrate(q):
        if assumption == "udd":
            num = _quad(lambda t: q, 0.0, 1.0)
            den = _quad(lambda t: 1.0 - t * q, 0.0, 1.0)
        else:
            mu = -math.log(1.0 - q)
            num = _quad(lambda t: math.exp(-mu * t) * mu, 0.0, 1.0)
            den = _quad(lambda t: math.exp(-mu * t), 0.0, 1.0)
        return num / den - m
    q = _brentq(mrate, 1e-15, 1.0 - 1e-15)
    return {"q": q, "p": 1.0 - q}


_add("act_central_death_rate",
     name="Death probability from the central death rate", topic="life tables / central death rate",
     purpose="The one-year death probability q_x corresponding to a central death rate m_x (deaths per year lived), under a uniform distribution of deaths or a constant force of mortality within the year.",
     inputs=[_numin("m", "central death rate m_x (deaths per person-year)"),
             _choice("assumption", ("udd", "constant_force"), "fractional-age assumption: udd or constant_force")],
     outputs=[_o("q", "one-year death probability q_x"), _o("p", "one-year survival probability p_x")],
     primary="q", conditions=["m > 0", "m < 2 under UDD"],
     method="UDD: q = 2m/(2+m); constant force: q = 1 - e^(-m)",
     latex=r"\text{UDD: } q_x = \frac{2m_x}{2+m_x},\qquad \text{constant force: } q_x = 1-e^{-m_x}",
     run=central_death_rate, reference=_ref_central_death_rate,
     wrong=lambda m, assumption: _out(q=m / (1 + m), p=1 / (1 + m)),
     rand=lambda rng: {"m": _sig(_lu(rng, 1e-3, 0.8), 4), "assumption": rng.choice(["udd", "constant_force"])},
     phrases=("Death probability from the central death rate", "Central death rate to death probability"),
     examples=[_ex({"m": 0.02, "assumption": "udd"}), _ex({"m": 0.1, "assumption": "constant_force"})],
     level="beginner",
     weak="the defining ratio m = (integral of tp mu)/(integral of tp) by scipy quad, re-solved for q by brentq")


def fractional_age(q, s, assumption):
    _need(assumption in ("udd", "constant_force", "balducci"), "assumption: udd, constant_force or balducci")
    _need(_isnum(q) and 0 < q < 1, "q must lie strictly between 0 and 1")
    _need(_isnum(s) and 0 < s < 1, "the fraction of a year s must lie strictly between 0 and 1")
    if assumption == "udd":
        sp = 1.0 - s * q
        mu = q / (1.0 - s * q)
    elif assumption == "constant_force":
        sp = (1.0 - q) ** s
        mu = -math.log(1.0 - q)
    else:
        sp = (1.0 - q) / (1.0 - (1.0 - s) * q)
        mu = q / (1.0 - (1.0 - s) * q)
    return _out(spx=sp, sqx=1.0 - sp, mu=mu)


def _ref_fractional_age(q, s, assumption):
    import sympy as sp_
    t = sp_.symbols("t", positive=True)
    qq = sp_.nsimplify(q)
    if assumption == "udd":
        S = 1 - t * qq
    elif assumption == "constant_force":
        S = (1 - qq) ** t
    else:
        S = (1 - qq) / (1 - (1 - t) * qq)
    mu = sp_.lambdify(t, sp_.simplify(-sp_.diff(S, t) / S), "math")
    integ = _quad(lambda u: float(mu(u)), 0.0, s)
    p = math.exp(-integ)
    return {"spx": p, "sqx": 1.0 - p, "mu": float(mu(s))}


_add("act_fractional_age",
     name="Survival to a fractional age (UDD, constant force, Balducci)", topic="life tables / fractional ages",
     purpose="The probability of surviving (or dying within) a fraction s of a year from integer age x, and the force of mortality at x+s, under a uniform distribution of deaths, a constant force of mortality or the Balducci assumption.",
     inputs=[_numin("q", "one-year death probability q_x"), _numin("s", "fraction of the year (0 < s < 1)"),
             _choice("assumption", ("udd", "constant_force", "balducci"), "fractional-age assumption")],
     outputs=[_o("spx", "probability of surviving to age x+s"), _o("sqx", "probability of dying before age x+s"),
              _o("mu", "force of mortality at age x+s")],
     primary="spx", conditions=["0 < q < 1", "0 < s < 1"],
     method="UDD: s_p_x = 1 - s q, mu = q/(1 - s q); constant force: (1-q)^s, mu = -ln(1-q); Balducci: (1-q)/(1-(1-s)q), mu = q/(1-(1-s)q)",
     latex=r"\text{UDD: } {}_sp_x = 1-sq_x,\ \mu_{x+s} = \frac{q_x}{1-sq_x};\quad \text{CF: } {}_sp_x = p_x^{\,s};\quad \text{Balducci: } {}_sp_x = \frac{p_x}{1-(1-s)q_x}",
     run=fractional_age, reference=_ref_fractional_age,
     wrong=lambda q, s, assumption: _out(spx=1 - s * q, sqx=s * q, mu=q),
     rand=lambda rng: {"q": _sig(_lu(rng, 1e-3, 0.5), 4), "s": _sig(rng.uniform(0.05, 0.95), 3), "assumption": rng.choice(["constant_force", "balducci", "constant_force", "balducci", "udd"])},
     phrases=("Fractional age survival", "Survival to a fractional age", "Fractional age assumption"),
     examples=[_ex({"q": 0.1, "s": 0.5, "assumption": "udd"}), _ex({"q": 0.1, "s": 0.25, "assumption": "constant_force"})],
     level="intermediate",
     extra_q=[{"q": "Fractional age survival: q = 0.1, s = 0.25, assumption = balducci", "outputs": {"spx": 0.9 / 0.925}},
              {"q": "Fractional age survival: q = 0.1, s = 1.5, assumption = udd", "outcome": "INVALID_INPUT"}],
     weak="the hazard of each assumption (sympy) integrated by scipy quad; the force at x+s is the same derivative")


def commutation_functions(lx, x0, x, i):
    _lx_check(lx, closed=True)
    _rate(i)
    k = _idx(lx, x0, x)
    v = 1.0 / (1.0 + i)
    D = [lx[j] * v ** (x0 + j) for j in range(len(lx))]
    C = [(lx[j] - lx[j + 1]) * v ** (x0 + j + 1) for j in range(len(lx) - 1)]
    N = 0.0
    for j in range(k, len(lx)):
        N += D[j]
    M = 0.0
    for j in range(k, len(lx) - 1):
        M += C[j]
    return _out(D_x=D[k], N_x=N, C_x=C[k] if k < len(C) else 0.0, M_x=M, A_x=M / D[k], a_due_x=N / D[k])


def _ref_commutation_functions(lx, x0, x, i):
    k = x - x0
    v = 1.0 / (1.0 + i)
    A = a = 0.0
    for j in range(len(lx) - 1, k - 1, -1):                 # backward recursions on q and p
        if j == len(lx) - 1:
            A, a = 0.0, (1.0 if lx[j] > 0 else 0.0)
            continue
        qj = (lx[j] - lx[j + 1]) / lx[j]
        A = v * qj + v * (1 - qj) * A
        a = 1.0 + v * (1 - qj) * a
    D = lx[k] * (1.0 + i) ** (-x)
    C = (lx[k] - lx[k + 1]) * (1.0 + i) ** (-(x + 1)) if k + 1 < len(lx) else 0.0
    return {"D_x": D, "N_x": a * D, "C_x": C, "M_x": A * D, "A_x": A, "a_due_x": a}


_add("act_commutation_functions",
     name="Commutation functions D, N, C and M", topic="life tables / commutation functions",
     purpose="The commutation functions D_x = v^x l_x, N_x, C_x = v^(x+1) d_x and M_x at age x of a complete life table, with the whole life insurance A_x = M_x/D_x and annuity-due ä_x = N_x/D_x.",
     inputs=[_vecin("lx", "survivors at consecutive integer ages from x0, ending with 0 (a list)", 2, 130),
             _intin("x0", "age of the first entry of lx", 0, 130), _intin("x", "age at which the functions are wanted", 0, 130), _ratein()],
     outputs=[_o("D_x", "D_x = v^x l_x"), _o("N_x", "N_x = sum of D_y for y >= x"), _o("C_x", "C_x = v^(x+1) d_x"),
              _o("M_x", "M_x = sum of C_y for y >= x"), _o("A_x", "whole life insurance M_x/D_x"), _o("a_due_x", "whole life annuity-due N_x/D_x")],
     primary="A_x", conditions=["the table closes (last lx = 0)", "x inside the table", "0 < i < 1"],
     method="D_y = v^y l_y, C_y = v^(y+1) d_y, N_x = sum D_y, M_x = sum C_y; A_x = M_x/D_x, ä_x = N_x/D_x",
     latex=r"D_x = v^x l_x,\ C_x = v^{x+1}d_x,\ N_x = \sum_{y\ge x}D_y,\ M_x = \sum_{y\ge x}C_y,\ A_x = \frac{M_x}{D_x},\ \ddot a_x = \frac{N_x}{D_x}",
     run=commutation_functions, reference=_ref_commutation_functions,
     wrong=lambda lx, x0, x, i: (lambda o: dict(o, C_x=o["C_x"] * (1 + i), M_x=o["M_x"] * (1 + i), A_x=o["A_x"] * (1 + i)))(commutation_functions(lx, x0, x, i)),
     rand=lambda rng: (lambda x0, lx: {"lx": lx, "x0": x0, "x": x0 + rng.randint(0, len(lx) - 2), "i": _rand_i(rng)})(*_rand_lx(rng, True)),
     phrases=("Commutation functions", "Commutation function values"),
     examples=[_ex({"lx": [1000.0, 900.0, 700.0, 400.0, 0.0], "x0": 95, "x": 95, "i": 0.05}),
               _ex({"lx": [1000.0, 900.0, 700.0, 400.0, 0.0], "x0": 95, "x": 96, "i": 0.06})],
     level="intermediate",
     weak="D and C are definitions; N and M recomputed from backward insurance and annuity recursions")


def pure_endowment_lx(lx, x0, x, n, i):
    _lx_check(lx)
    _rate(i)
    k = _idx(lx, x0, x, n)
    E = (1.0 + i) ** (-n) * lx[k + n] / lx[k]
    _need(E > 0, "nobody survives to age x+n in the table")
    return _out(nEx=E, accumulation=1.0 / E)


def _ref_pure_endowment_lx(lx, x0, x, n, i):
    k = x - x0
    p = 1.0
    for j in range(k, k + n):
        p *= lx[j + 1] / lx[j]
    E = p / (1.0 + i) ** n
    return {"nEx": E, "accumulation": 1.0 / E}


_add("act_pure_endowment",
     name="Pure endowment (actuarial discount factor)", topic="life tables / pure endowment",
     purpose="The actuarial present value of 1 paid at time n if a life aged x is then alive, nEx = v^n n_p_x, and the actuarial accumulation factor 1/nEx.",
     inputs=[_vecin("lx", "survivors at consecutive integer ages from x0 (a list)", 2, 130), _intin("x0", "age of the first entry of lx", 0, 130),
             _intin("x", "age of the life", 0, 130), _intin("n", "years until the payment", 0, 130), _ratein()],
     outputs=[_o("nEx", "pure endowment n_E_x = v^n n_p_x"), _o("accumulation", "actuarial accumulation factor 1/n_E_x")],
     primary="nEx", conditions=["x + n inside the table", "l_(x+n) > 0", "0 < i < 1"],
     method="nEx = v^n l_(x+n)/l_x",
     latex=r"{}_nE_x = v^n\,{}_np_x = v^n\frac{l_{x+n}}{l_x}",
     run=pure_endowment_lx, reference=_ref_pure_endowment_lx,
     wrong=lambda lx, x0, x, n, i: (lambda k: _out(nEx=lx[k + n] / lx[k], accumulation=lx[k] / lx[k + n]))(x - x0),
     rand=lambda rng: (lambda x0, lx: (lambda x: {"lx": lx, "x0": x0, "x": x, "n": rng.randint(1, len(lx) - 1 - (x - x0)), "i": _rand_i(rng)})(x0 + rng.randint(0, len(lx) - 3)))(*_rand_lx(rng, False)),
     phrases=("Pure endowment", "Actuarial discount factor"),
     examples=[_ex({"lx": [100000.0, 99500.0, 98900.0, 98200.0, 97300.0], "x0": 40, "x": 40, "n": 4, "i": 0.05}),
               _ex({"lx": [1000.0, 950.0, 880.0, 780.0], "x0": 60, "x": 61, "n": 2, "i": 0.06})],
     level="beginner",
     weak="the product of one-year survival ratios discounted")


# ================================================================================================ PARAMETRIC LAWS
def de_moivre(omega, x, t):
    _pos("the limiting age omega", omega)
    _need(_isnum(x) and 0 <= x < omega, "the age x must lie between 0 and omega")
    _need(_isnum(t) and 0 <= t < omega - x, "t must lie between 0 and omega - x")
    r = omega - x
    return _out(tpx=(r - t) / r, tqx=t / r, mu=1.0 / (r - t), e_complete=r / 2.0, var_T=r * r / 12.0)


def _ref_de_moivre(omega, x, t):
    from scipy.stats import uniform
    U = uniform(loc=0.0, scale=omega - x)
    return {"tpx": float(U.sf(t)), "tqx": float(U.cdf(t)), "mu": float(U.pdf(t) / U.sf(t)), "e_complete": float(U.mean()), "var_T": float(U.var())}


_add("act_de_moivre_law",
     name="De Moivre's law of mortality", topic="survival models / parametric laws",
     purpose="Survival and death probabilities, the force of mortality, the complete expectation of life and the variance of the future lifetime under de Moivre's law (uniform future lifetime up to the limiting age omega).",
     inputs=[_numin("omega", "limiting age omega (years)"), _numin("x", "age of the life (years)"), _numin("t", "number of years")],
     outputs=[_o("tpx", "probability of surviving t years"), _o("tqx", "probability of dying within t years"),
              _o("mu", "force of mortality at age x+t"), _o("e_complete", "complete expectation of life e°_x"),
              _o("var_T", "variance of the future lifetime")],
     primary="tpx", conditions=["0 <= x < omega", "0 <= t < omega - x"],
     method="t_p_x = (omega - x - t)/(omega - x); mu_(x+t) = 1/(omega - x - t); e°_x = (omega - x)/2; Var T = (omega - x)^2/12",
     latex=r"{}_tp_x = \frac{\omega-x-t}{\omega-x},\ \mu_{x+t} = \frac{1}{\omega-x-t},\ \mathring e_x = \frac{\omega-x}{2}",
     run=de_moivre, reference=_ref_de_moivre,
     wrong=lambda omega, x, t: _out(tpx=(omega - x - t) / omega, tqx=t / omega, mu=1 / (omega - t), e_complete=(omega - x) / 2, var_T=(omega - x) ** 2 / 12),
     rand=lambda rng: (lambda w: (lambda x: {"omega": w, "x": x, "t": _sig(rng.uniform(0, 0.9) * (w - x), 3)})(_sig(rng.uniform(0, 0.8) * w, 3)))(float(rng.randint(90, 120))),
     phrases=("De Moivre's law", "De Moivre law of mortality", "Uniform survival model"),
     examples=[_ex({"omega": 100.0, "x": 40.0, "t": 10.0}), _ex({"omega": 110.0, "x": 65.0, "t": 20.0})],
     level="beginner")


def constant_force(mu, t):
    _need(_isnum(mu) and 0 < mu < 5, "the force of mortality must lie between 0 and 5 per year")
    _need(_isnum(t) and t >= 0, "t must be zero or positive")
    p1 = math.exp(-mu)
    return _out(tpx=math.exp(-mu * t), tqx=-math.expm1(-mu * t), e_complete=1.0 / mu, e_curtate=p1 / (1.0 - p1), var_T=1.0 / (mu * mu))


def _ref_constant_force(mu, t):
    from scipy.stats import expon, geom
    T = expon(scale=1.0 / mu)
    K1 = geom(1.0 - math.exp(-mu))
    return {"tpx": float(T.sf(t)), "tqx": float(T.cdf(t)), "e_complete": float(T.mean()), "e_curtate": float(K1.mean()) - 1.0, "var_T": float(T.var())}


_add("act_constant_force_mortality",
     name="Constant force of mortality (exponential lifetime)", topic="survival models / parametric laws",
     purpose="Survival and death probabilities, the complete and curtate expectations of life and the variance of the future lifetime when the force of mortality mu is constant.",
     inputs=[_numin("mu", "constant force of mortality per year"), _numin("t", "number of years")],
     outputs=[_o("tpx", "probability of surviving t years"), _o("tqx", "probability of dying within t years"),
              _o("e_complete", "complete expectation 1/mu"), _o("e_curtate", "curtate expectation p/(1-p)"), _o("var_T", "variance of the future lifetime 1/mu^2")],
     primary="tpx", conditions=["0 < mu < 5", "t >= 0"],
     method="t_p_x = e^(-mu t); e° = 1/mu; e_x = e^(-mu)/(1 - e^(-mu))",
     latex=r"{}_tp_x = e^{-\mu t},\quad \mathring e_x = \frac1\mu,\quad e_x = \frac{e^{-\mu}}{1-e^{-\mu}}",
     run=constant_force, reference=_ref_constant_force,
     wrong=lambda mu, t: _out(tpx=1 - mu * t if mu * t < 1 else 0.0, tqx=min(mu * t, 1.0), e_complete=1 / mu, e_curtate=1 / mu, var_T=1 / mu),
     rand=lambda rng: {"mu": _sig(_lu(rng, 0.005, 0.3), 3), "t": _sig(rng.uniform(0.5, 30), 3)},
     phrases=("Constant force of mortality", "Exponential survival model"),
     examples=[_ex({"mu": 0.02, "t": 10.0}), _ex({"mu": 0.1, "t": 5.0})],
     level="beginner")


_GLX = (-0.9739065285171717, -0.8650633666889845, -0.6794095682990244, -0.4333953941292472, -0.1488743389816312,
        0.1488743389816312, 0.4333953941292472, 0.6794095682990244, 0.8650633666889845, 0.9739065285171717)
_GLW = (0.0666713443086881, 0.1494513491505806, 0.2190863625159820, 0.2692667193099963, 0.2955242247147529,
        0.2955242247147529, 0.2692667193099963, 0.2190863625159820, 0.1494513491505806, 0.0666713443086881)


def _mk_tp(A, B, c, x, t):
    return math.exp(-A * t - B * c ** x * (c ** t - 1.0) / math.log(c))


def _mk_integral(A, B, c, x, delta):
    """integral over t >= 0 of e^(-delta t) t_p_x under Makeham's law: 10-point Gauss-Legendre on half-year panels
    until the integrand is negligible."""
    tot = 0.0
    a = 0.0
    h = 0.5
    for _ in range(4000):
        part = 0.0
        for xi, wi in zip(_GLX, _GLW):
            u = a + h * (xi + 1.0) / 2.0
            part += wi * math.exp(-delta * u) * _mk_tp(A, B, c, x, u)
        tot += part * h / 2.0
        a += h
        if math.exp(-delta * a) * _mk_tp(A, B, c, x, a) < 1e-18 * tot:
            break
    return tot


def _mk_check(A, B, c, x, t):
    _nonneg("Makeham's A", A)
    _need(_isnum(B) and 0 < B < 0.1, "B must lie between 0 and 0.1")
    _need(_isnum(c) and 1 < c < 1.5, "c must lie between 1 and 1.5")
    _need(_isnum(A) and A < 0.1, "A must lie below 0.1")
    _need(_isnum(x) and 0 <= x <= 120, "the age x must lie between 0 and 120")
    _need(_isnum(t) and 0 <= t <= 100, "t must lie between 0 and 100 years")


def makeham(A, B, c, x, t):
    _mk_check(A, B, c, x, t)
    tp = _mk_tp(A, B, c, x, t)
    return _out(tpx=tp, tqx=1.0 - tp, mu_x=A + B * c ** x, mu_xt=A + B * c ** (x + t), e_complete=_mk_integral(A, B, c, x, 0.0))


def _ref_makeham(A, B, c, x, t):
    mu = lambda s: A + B * c ** (x + s)
    tp = math.exp(-_quad(mu, 0.0, t)) if t > 0 else 1.0
    end = 1.0
    while _mk_tp(A, B, c, x, end) > 1e-30 and end < 2000:
        end *= 1.5
    e = 0.0
    pts = [0.0] + [end * k / 40.0 for k in range(1, 41)]
    for a_, b_ in zip(pts[:-1], pts[1:]):
        e += _quad(lambda s: math.exp(-A * s - B * c ** x * math.expm1(s * math.log(c)) / math.log(c)), a_, b_)
    return {"tpx": tp, "tqx": 1.0 - tp, "mu_x": mu(0.0), "mu_xt": mu(t), "e_complete": e}


_add("act_makeham_law",
     name="Makeham's law of mortality", topic="survival models / parametric laws",
     purpose="Survival and death probabilities, the force of mortality at x and x+t and the complete expectation of life under Makeham's law mu_x = A + B c^x.",
     inputs=[_numin("A", "Makeham's age-independent term A (per year)"), _numin("B", "Makeham's B (per year)"),
             _numin("c", "Makeham's c (> 1)"), _numin("x", "age of the life (years)"), _numin("t", "number of years")],
     outputs=[_o("tpx", "probability of surviving t years"), _o("tqx", "probability of dying within t years"),
              _o("mu_x", "force of mortality at age x"), _o("mu_xt", "force of mortality at age x+t"),
              _o("e_complete", "complete expectation of life e°_x")],
     primary="tpx", conditions=["0 <= A < 0.1", "0 < B < 0.1", "1 < c < 1.5", "0 <= x <= 120", "0 <= t <= 100"],
     method="t_p_x = exp(-A t - B c^x (c^t - 1)/ln c); e°_x = integral of t_p_x (Gauss-Legendre)",
     latex=r"\mu_x = A + Bc^x,\quad {}_tp_x = \exp\!\left(-At - \frac{Bc^x(c^t-1)}{\ln c}\right),\quad \mathring e_x = \int_0^\infty {}_tp_x\,dt",
     run=makeham, reference=_ref_makeham,
     wrong=lambda A, B, c, x, t: (lambda o: dict(o, tpx=math.exp(-A * t - B * c ** x * (c ** t - 1)), tqx=1 - math.exp(-A * t - B * c ** x * (c ** t - 1))))(makeham(A, B, c, x, t)),
     rand=lambda rng: {"A": _sig(rng.uniform(1e-4, 2e-3), 3), "B": _sig(_lu(rng, 2e-6, 5e-5), 3), "c": _sig(rng.uniform(1.07, 1.13), 4),
                       "x": float(rng.randint(20, 90)), "t": _sig(rng.uniform(1, 30), 3)},
     phrases=("Makeham's law", "Makeham law of mortality", "Gompertz-Makeham law"),
     examples=[_ex({"A": 0.00022, "B": 2.7e-06, "c": 1.124, "x": 60.0, "t": 10.0}), _ex({"A": 0.0007, "B": 5e-05, "c": 1.09, "x": 40.0, "t": 20.0})],
     level="intermediate",
     extra_q=[{"q": "Makeham's law: A = 0.00022, B = 2.7e-06, c = 0.9, x = 60, t = 10", "outcome": "INVALID_INPUT"}])


def gompertz(B, c, x, t):
    _mk_check(0.0, B, c, x, t)
    tp = _mk_tp(0.0, B, c, x, t)
    return _out(tpx=tp, tqx=1.0 - tp, mu_x=B * c ** x, mu_xt=B * c ** (x + t), e_complete=_mk_integral(0.0, B, c, x, 0.0))


def _ref_gompertz(B, c, x, t):
    from scipy.stats import gompertz as G
    # scipy's Gompertz: sf(y) = exp(-cc (e^y - 1)); with y = s ln c and cc = B c^x / ln c
    lc = math.log(c)
    dist = G(c=B * c ** x / lc, scale=1.0 / lc)
    return {"tpx": float(dist.sf(t)), "tqx": float(dist.cdf(t)), "mu_x": float(dist.pdf(0.0) / dist.sf(0.0)),
            "mu_xt": float(math.exp(dist.logpdf(t) - dist.logsf(t))), "e_complete": float(dist.mean())}


_add("act_gompertz_law",
     name="Gompertz's law of mortality", topic="survival models / parametric laws",
     purpose="Survival and death probabilities, the force of mortality at x and x+t and the complete expectation of life under Gompertz's law mu_x = B c^x.",
     inputs=[_numin("B", "Gompertz B (per year)"), _numin("c", "Gompertz c (> 1)"), _numin("x", "age of the life (years)"), _numin("t", "number of years")],
     outputs=[_o("tpx", "probability of surviving t years"), _o("tqx", "probability of dying within t years"),
              _o("mu_x", "force of mortality at age x"), _o("mu_xt", "force of mortality at age x+t"),
              _o("e_complete", "complete expectation of life e°_x")],
     primary="tpx", conditions=["0 < B < 0.1", "1 < c < 1.5", "0 <= x <= 120", "0 <= t <= 100"],
     method="t_p_x = exp(-B c^x (c^t - 1)/ln c); e°_x = integral of t_p_x (Gauss-Legendre)",
     latex=r"\mu_x = Bc^x,\quad {}_tp_x = \exp\!\left(-\frac{Bc^x(c^t-1)}{\ln c}\right)",
     run=gompertz, reference=_ref_gompertz,
     wrong=lambda B, c, x, t: (lambda o: dict(o, e_complete=o["e_complete"] - 0.5, tpx=math.exp(-B * c ** (x + t) * t), tqx=1 - math.exp(-B * c ** (x + t) * t)))(gompertz(B, c, x, t)),
     rand=lambda rng: {"B": _sig(_lu(rng, 2e-6, 1e-4), 3), "c": _sig(rng.uniform(1.07, 1.13), 4), "x": float(rng.randint(20, 90)), "t": _sig(rng.uniform(1, 30), 3)},
     phrases=("Gompertz's law", "Gompertz law of mortality"),
     examples=[_ex({"B": 0.0003, "c": 1.07, "x": 50.0, "t": 10.0}), _ex({"B": 2.7e-06, "c": 1.124, "x": 70.0, "t": 5.0})],
     level="intermediate")


# ================================================================================================ INSURANCES AND ANNUITIES (qx lists)
def _q_check(q, whole=False, n=None):
    _need(len(q) >= 1, "give at least one death probability")
    for v in q:
        _need(_isnum(v) and 0 <= v <= 1, "every death probability q must lie between 0 and 1")
    if whole:
        _need(q[-1] == 1, "a whole-life table must close: its last q must be 1")
    if n is not None:
        _whole("the term n", n, 1)
        _need(n <= len(q), "the table must cover the whole term (at least n death probabilities)")


def _kp(q):
    out = [1.0]
    for v in q:
        out.append(out[-1] * (1.0 - v))
    return out


def _ins(q, v, n):
    """term insurance of 1 for n years, benefit at the end of the year of death"""
    kp = _kp(q)
    s = 0.0
    for k in range(n):
        s += v ** (k + 1) * kp[k] * q[k]
    return s


def _ann(q, v, n):
    """temporary annuity-due of 1 for n years"""
    kp = _kp(q)
    s = 0.0
    for k in range(n):
        s += v ** k * kp[k]
    return s


def _pe(q, v, n):
    return v ** n * _kp(q)[n]


def _rand_q(rng, whole=True, lo=3, hi=12):
    x = rng.randint(60, 95) if whole else rng.randint(30, 85)
    A, B, c = rng.uniform(5e-4, 2e-3), rng.uniform(2e-6, 4e-5), rng.uniform(1.08, 1.12)
    out = []
    age = x
    while len(out) < hi:
        q = 1.0 - math.exp(-(A + B * c ** age * (c - 1.0) / math.log(c)))
        out.append(_sig(min(q, 0.999), 4))
        age += 1
        if q > 0.6 and len(out) >= lo:
            break
    if whole:
        out.append(1.0)
    return out


def _ref_back(q, i, n, v2=False):
    """Reference helper: backward recursions A_k = v q_k + v p_k A_(k+1), a_k = 1 + v p_k a_(k+1) (numpy floats)."""
    import numpy as np
    v = np.float64(1.0 / (1.0 + i))
    if v2:
        v = v * v
    A = np.float64(0.0)
    a = np.float64(0.0)
    E = np.float64(1.0)
    for k in range(n - 1, -1, -1):
        A = v * q[k] + v * (1.0 - q[k]) * A
        a = 1.0 + v * (1.0 - q[k]) * a
        E = v * (1.0 - q[k]) * E
    return float(A), float(a), float(E)


def whole_life_insurance(q, i, S):
    _q_check(q, whole=True)
    _rate(i)
    _pos("the sum insured S", S)
    v = 1.0 / (1.0 + i)
    A = _ins(q, v, len(q))
    A2 = _ins(q, v * v, len(q))
    var = max(A2 - A * A, 0.0)
    return _out(A=A, A2=A2, EPV=S * A, sd=S * math.sqrt(var))


def _ref_whole_life_insurance(q, i, S):
    A, _, _ = _ref_back(q, i, len(q))
    A2, _, _ = _ref_back(q, i, len(q), v2=True)
    return {"A": A, "A2": A2, "EPV": S * A, "sd": S * math.sqrt(A2 - A * A)}


_add("act_whole_life_insurance",
     name="Whole life insurance (fully discrete)", topic="life insurance / whole life",
     purpose="The actuarial present value of a whole life insurance paying S at the end of the year of death of a life aged x (A_x), its second moment at double the force of interest, and the standard deviation of the present value of the benefit.",
     inputs=[_vecin("q", "death probabilities q_x, q_(x+1), ... ending with 1 (a list)", 1, 130), _ratein(), _numin("S", "sum insured")],
     outputs=[_o("A", "A_x per unit sum insured"), _o("A2", "second moment 2A_x (at double the force of interest)"),
              _o("EPV", "actuarial present value S A_x"), _o("sd", "standard deviation of the present value S sqrt(2A - A^2)")],
     primary="A", conditions=["the table closes (last q = 1)", "0 < i < 1", "S > 0"],
     method="A_x = sum_k v^(k+1) k_p_x q_(x+k); 2A_x with v^2; Var Z = S^2 (2A - A^2)",
     latex=r"A_x = \sum_{k\ge0} v^{k+1}\,{}_kp_x\,q_{x+k},\quad \mathrm{Var}(Z) = S^2\left({}^2A_x - A_x^2\right)",
     run=whole_life_insurance, reference=_ref_whole_life_insurance,
     wrong=lambda q, i, S: (lambda A: _out(A=A, A2=A * A, EPV=S * A, sd=0.0))(_ins(q, 1 / (1 + i), len(q)) * (1 + i)),
     rand=lambda rng: {"q": _rand_q(rng), "i": _rand_i(rng), "S": _sig(_lu(rng, 1e3, 1e6), 3)},
     phrases=("Whole life insurance", "Whole-life insurance"),
     examples=[_ex({"q": [0.1, 0.2, 0.5, 1.0], "i": 0.05, "S": 1000.0}), _ex({"q": [0.02, 0.03, 0.05, 0.08, 0.15, 0.3, 1.0], "i": 0.04, "S": 100000.0})],
     level="intermediate",
     extra_q=[{"q": "Whole life insurance: q = [0.1, 0.2, 0.5], i = 5%, S = 1000", "outcome": "INVALID_INPUT"},
              {"q": "Whole-life insurance: q = [0.1, 0.2, 0.5, 1], i = 0.05, S = 1000", "like": 0}])


def term_insurance(q, i, n, S):
    _q_check(q, n=n)
    _rate(i)
    _pos("the sum insured S", S)
    v = 1.0 / (1.0 + i)
    A = _ins(q, v, n)
    A2 = _ins(q, v * v, n)
    return _out(A1=A, A1_2=A2, EPV=S * A, sd=S * math.sqrt(max(A2 - A * A, 0.0)))


def _ref_term_insurance(q, i, n, S):
    A, _, _ = _ref_back(q, i, n)
    A2, _, _ = _ref_back(q, i, n, v2=True)
    return {"A1": A, "A1_2": A2, "EPV": S * A, "sd": S * math.sqrt(max(A2 - A * A, 0.0))}


_add("act_term_insurance",
     name="Term insurance (fully discrete)", topic="life insurance / term insurance",
     purpose="The actuarial present value of an n-year term insurance paying S at the end of the year of death if death occurs within n years, its second moment and the standard deviation of the present value.",
     inputs=[_vecin("q", "death probabilities q_x, q_(x+1), ... for at least n years (a list)", 1, 130), _ratein(),
             _intin("n", "term in years", 1, 130), _numin("S", "sum insured")],
     outputs=[_o("A1", "term insurance A^1_(x:n) per unit"), _o("A1_2", "second moment at double the force of interest"),
              _o("EPV", "actuarial present value S A^1_(x:n)"), _o("sd", "standard deviation of the present value")],
     primary="A1", conditions=["at least n death probabilities", "0 < i < 1", "S > 0"],
     method="A^1_(x:n) = sum_(k<n) v^(k+1) k_p_x q_(x+k)",
     latex=r"A^1_{x:\overline{n}|} = \sum_{k=0}^{n-1} v^{k+1}\,{}_kp_x\,q_{x+k}",
     run=term_insurance, reference=_ref_term_insurance,
     wrong=lambda q, i, n, S: (lambda A: _out(A1=A, A1_2=A, EPV=S * A, sd=0.0))(_ins(q, 1 / (1 + i), n) * (1 + i)),
     rand=lambda rng: (lambda q: {"q": q, "i": _rand_i(rng), "n": rng.randint(1, len(q)), "S": _sig(_lu(rng, 1e3, 1e6), 3)})(_rand_q(rng, False)),
     phrases=("Term insurance", "Term life insurance"),
     examples=[_ex({"q": [0.1, 0.2, 0.5, 1.0], "i": 0.05, "n": 2, "S": 1000.0}), _ex({"q": [0.01, 0.012, 0.015, 0.018, 0.022], "i": 0.06, "n": 5, "S": 50000.0})],
     level="intermediate",
     extra_q=[{"q": "Term insurance: q = [0.1, 0.2], i = 5%, n = 3, S = 1000", "outcome": "INVALID_INPUT"}])


def endowment_insurance(q, i, n, S):
    _q_check(q, n=n)
    _rate(i)
    _pos("the sum insured S", S)
    v = 1.0 / (1.0 + i)
    A1 = _ins(q, v, n)
    E = _pe(q, v, n)
    A2 = _ins(q, v * v, n) + _pe(q, v * v, n)
    A = A1 + E
    return _out(A=A, A1=A1, nEx=E, A2=A2, EPV=S * A, sd=S * math.sqrt(max(A2 - A * A, 0.0)))


def _ref_endowment_insurance(q, i, n, S):
    A1, _, E = _ref_back(q, i, n)
    a1, _, e2 = _ref_back(q, i, n, v2=True)
    A = A1 + E
    return {"A": A, "A1": A1, "nEx": E, "A2": a1 + e2, "EPV": S * A, "sd": S * math.sqrt(max(a1 + e2 - A * A, 0.0))}


_add("act_endowment_insurance",
     name="Endowment insurance (fully discrete)", topic="life insurance / endowment insurance",
     purpose="The actuarial present value of an n-year endowment insurance paying S at the end of the year of death within n years or at time n on survival, split into its term insurance and pure endowment parts, with the second moment and standard deviation.",
     inputs=[_vecin("q", "death probabilities q_x, q_(x+1), ... for at least n years (a list)", 1, 130), _ratein(),
             _intin("n", "term in years", 1, 130), _numin("S", "sum insured")],
     outputs=[_o("A", "endowment insurance A_(x:n) per unit"), _o("A1", "term insurance part A^1_(x:n)"),
              _o("nEx", "pure endowment part n_E_x"), _o("A2", "second moment at double the force of interest"),
              _o("EPV", "actuarial present value S A_(x:n)"), _o("sd", "standard deviation of the present value")],
     primary="A", conditions=["at least n death probabilities", "0 < i < 1", "S > 0"],
     method="A_(x:n) = A^1_(x:n) + n_E_x",
     latex=r"A_{x:\overline{n}|} = A^1_{x:\overline{n}|} + {}_nE_x",
     run=endowment_insurance, reference=_ref_endowment_insurance,
     wrong=lambda q, i, n, S: (lambda o: dict(o, A=o["A1"] + _kp(q)[n] * (1 / (1 + i)) ** (n - 1), EPV=S * (o["A1"] + _kp(q)[n] * (1 / (1 + i)) ** (n - 1))))(endowment_insurance(q, i, n, S)),
     rand=lambda rng: (lambda q: {"q": q, "i": _rand_i(rng), "n": rng.randint(1, len(q)), "S": _sig(_lu(rng, 1e3, 1e6), 3)})(_rand_q(rng, False)),
     phrases=("Endowment insurance", "Endowment assurance"),
     examples=[_ex({"q": [0.1, 0.2, 0.5, 1.0], "i": 0.05, "n": 2, "S": 1000.0}), _ex({"q": [0.01, 0.012, 0.015, 0.018, 0.022], "i": 0.06, "n": 5, "S": 50000.0})],
     level="intermediate")


def deferred_insurance(q, i, u, S):
    _q_check(q, whole=True)
    _rate(i)
    _pos("the sum insured S", S)
    _whole("the deferment u", u, 0)
    _need(u < len(q), "the deferment must end before the table closes")
    v = 1.0 / (1.0 + i)
    kp = _kp(q)
    s = 0.0
    for k in range(u, len(q)):
        s += v ** (k + 1) * kp[k] * q[k]
    return _out(uA=s, EPV=S * s, uEx=v ** u * kp[u])


def _ref_deferred_insurance(q, i, u, S):
    _, _, E = _ref_back(q, i, u) if u > 0 else (0.0, 0.0, 1.0)
    Au, _, _ = _ref_back(q[u:], i, len(q) - u)
    return {"uA": E * Au, "EPV": S * E * Au, "uEx": E}


_add("act_deferred_insurance",
     name="Deferred whole life insurance", topic="life insurance / deferred insurance",
     purpose="The actuarial present value of a whole life insurance of S that pays only if death occurs after u years (u|A_x = uE_x A_(x+u)).",
     inputs=[_vecin("q", "death probabilities q_x, q_(x+1), ... ending with 1 (a list)", 1, 130), _ratein(),
             _intin("u", "deferment in years", 0, 130), _numin("S", "sum insured")],
     outputs=[_o("uA", "deferred insurance u|A_x per unit"), _o("EPV", "actuarial present value S u|A_x"), _o("uEx", "pure endowment u_E_x")],
     primary="uA", conditions=["the table closes", "u < length of the table", "0 < i < 1"],
     method="u|A_x = sum_(k>=u) v^(k+1) k_p_x q_(x+k) = u_E_x A_(x+u)",
     latex=r"{}_{u|}A_x = \sum_{k\ge u} v^{k+1}\,{}_kp_x\,q_{x+k} = {}_uE_x\,A_{x+u}",
     run=deferred_insurance, reference=_ref_deferred_insurance,
     wrong=lambda q, i, u, S: (lambda o: dict(o, uA=o["uA"] / o["uEx"] if o["uEx"] else 0.0, EPV=S * (o["uA"] / o["uEx"] if o["uEx"] else 0.0)))(deferred_insurance(q, i, u, S)),
     rand=lambda rng: (lambda q: {"q": q, "i": _rand_i(rng), "u": rng.randint(1, len(q) - 1), "S": _sig(_lu(rng, 1e3, 1e6), 3)})(_rand_q(rng)),
     phrases=("Deferred insurance", "Deferred whole life insurance"),
     examples=[_ex({"q": [0.1, 0.2, 0.5, 1.0], "i": 0.05, "u": 1, "S": 1000.0}), _ex({"q": [0.02, 0.03, 0.05, 0.08, 0.15, 0.3, 1.0], "i": 0.04, "u": 3, "S": 10000.0})],
     level="advanced")


def _ref_enum_annuity(q, i, n):
    """Reference helper: E and Var of the annuity-due paid while alive, up to n payments, by enumerating K (numpy)."""
    import numpy as np
    kp = np.concatenate([[1.0], np.cumprod(1.0 - np.array(q[:n], dtype=float))])
    prob = [kp[k] * q[k] for k in range(n)] + [kp[n]]        # K = 0..n-1, then survival of the term
    v = 1.0 / (1.0 + i)
    pays = [sum(v ** j for j in range(min(k + 1, n))) for k in range(n)] + [sum(v ** j for j in range(n))]
    pr, py = np.array(prob), np.array(pays)
    m = float(np.dot(pr, py))
    return m, float(np.dot(pr, py * py)) - m * m


def whole_life_annuity(q, i, P):
    _q_check(q, whole=True)
    _rate(i)
    _pos("the annual payment P", P)
    v = 1.0 / (1.0 + i)
    a = _ann(q, v, len(q))
    A = _ins(q, v, len(q))
    A2 = _ins(q, v * v, len(q))
    d = i * v
    return _out(a_due=a, a_imm=a - 1.0, EPV=P * a, sd=P * math.sqrt(max(A2 - A * A, 0.0)) / d)


def _ref_whole_life_annuity(q, i, P):
    m, var = _ref_enum_annuity(q, i, len(q))
    return {"a_due": m, "a_imm": m - 1.0, "EPV": P * m, "sd": P * math.sqrt(var)}


_add("act_whole_life_annuity",
     name="Whole life annuity", topic="life annuities / whole life",
     purpose="The actuarial present values of a whole life annuity-due (ä_x) and annuity-immediate (a_x) for a life aged x, the value of an annual payment P, and the standard deviation of the present value of the annuity-due payments.",
     inputs=[_vecin("q", "death probabilities q_x, q_(x+1), ... ending with 1 (a list)", 1, 130), _ratein(), _numin("P", "annual payment")],
     outputs=[_o("a_due", "whole life annuity-due ä_x"), _o("a_imm", "whole life annuity-immediate a_x"),
              _o("EPV", "actuarial present value P ä_x"), _o("sd", "standard deviation of the annuity-due present value")],
     primary="a_due", conditions=["the table closes", "0 < i < 1", "P > 0"],
     method="ä_x = sum_k v^k k_p_x; a_x = ä_x - 1; Var = P^2 (2A_x - A_x^2)/d^2",
     latex=r"\ddot a_x = \sum_{k\ge0} v^k\,{}_kp_x,\quad a_x = \ddot a_x - 1,\quad \mathrm{Var}(Y) = \frac{{}^2A_x - A_x^2}{d^2}",
     run=whole_life_annuity, reference=_ref_whole_life_annuity,
     wrong=lambda q, i, P: (lambda a: _out(a_due=a - 1, a_imm=a - 2, EPV=P * (a - 1), sd=0.0))(_ann(q, 1 / (1 + i), len(q))),
     rand=lambda rng: {"q": _rand_q(rng), "i": _rand_i(rng), "P": _sig(_lu(rng, 100, 1e5), 3)},
     phrases=("Whole life annuity", "Life annuity"),
     examples=[_ex({"q": [0.1, 0.2, 0.5, 1.0], "i": 0.05, "P": 1000.0}), _ex({"q": [0.02, 0.03, 0.05, 0.08, 0.15, 0.3, 1.0], "i": 0.04, "P": 12000.0})],
     level="intermediate")


def temporary_annuity(q, i, n, P):
    _q_check(q, n=n)
    _rate(i)
    _pos("the annual payment P", P)
    v = 1.0 / (1.0 + i)
    a = _ann(q, v, n)
    imm = a - 1.0 + _pe(q, v, n)
    return _out(a_due=a, a_imm=imm, EPV=P * a)


def _ref_temporary_annuity(q, i, n, P):
    m, _ = _ref_enum_annuity(q, i, n)
    _, _, E = _ref_back(q, i, n)
    return {"a_due": m, "a_imm": m - 1.0 + E, "EPV": P * m}


_add("act_temporary_annuity",
     name="Temporary life annuity", topic="life annuities / temporary annuities",
     purpose="The actuarial present values of an n-year temporary life annuity-due (ä_(x:n)) and annuity-immediate (a_(x:n)), and the value of an annual payment P paid in advance.",
     inputs=[_vecin("q", "death probabilities q_x, q_(x+1), ... for at least n years (a list)", 1, 130), _ratein(),
             _intin("n", "number of years", 1, 130), _numin("P", "annual payment")],
     outputs=[_o("a_due", "temporary annuity-due ä_(x:n)"), _o("a_imm", "temporary annuity-immediate a_(x:n)"), _o("EPV", "actuarial present value P ä_(x:n)")],
     primary="a_due", conditions=["at least n death probabilities", "0 < i < 1", "P > 0"],
     method="ä_(x:n) = sum_(k<n) v^k k_p_x; a_(x:n) = ä_(x:n) - 1 + n_E_x",
     latex=r"\ddot a_{x:\overline{n}|} = \sum_{k=0}^{n-1} v^k\,{}_kp_x,\quad a_{x:\overline{n}|} = \ddot a_{x:\overline{n}|} - 1 + {}_nE_x",
     run=temporary_annuity, reference=_ref_temporary_annuity,
     wrong=lambda q, i, n, P: (lambda a: _out(a_due=a, a_imm=a - 1.0, EPV=P * a))(sum((1 / (1 + i)) ** (k + 1) * _kp(q)[k + 1] for k in range(n))),
     rand=lambda rng: (lambda q: {"q": q, "i": _rand_i(rng), "n": rng.randint(1, len(q)), "P": _sig(_lu(rng, 100, 1e5), 3)})(_rand_q(rng, False)),
     phrases=("Temporary life annuity", "Temporary annuity"),
     examples=[_ex({"q": [0.1, 0.2, 0.5, 1.0], "i": 0.05, "n": 3, "P": 1000.0}), _ex({"q": [0.01, 0.012, 0.015, 0.018, 0.022], "i": 0.06, "n": 5, "P": 2000.0})],
     level="intermediate")


def deferred_annuity(q, i, u, P):
    _q_check(q, whole=True)
    _rate(i)
    _pos("the annual payment P", P)
    _whole("the deferment u", u, 0)
    _need(u < len(q), "the deferment must end before the table closes")
    v = 1.0 / (1.0 + i)
    kp = _kp(q)
    s = 0.0
    for k in range(u, len(q)):
        s += v ** k * kp[k]
    return _out(ua_due=s, EPV=P * s)


def _ref_deferred_annuity(q, i, u, P):
    _, _, E = _ref_back(q, i, u) if u > 0 else (0.0, 0.0, 1.0)
    _, a, _ = _ref_back(q[u:], i, len(q) - u)
    return {"ua_due": E * a, "EPV": P * E * a}


_add("act_deferred_annuity",
     name="Deferred whole life annuity-due", topic="life annuities / deferred annuities",
     purpose="The actuarial present value of a whole life annuity-due of P a year starting u years from now if the life is then alive (u|ä_x = uE_x ä_(x+u)).",
     inputs=[_vecin("q", "death probabilities q_x, q_(x+1), ... ending with 1 (a list)", 1, 130), _ratein(),
             _intin("u", "deferment in years", 0, 130), _numin("P", "annual payment")],
     outputs=[_o("ua_due", "deferred annuity-due u|ä_x"), _o("EPV", "actuarial present value P u|ä_x")],
     primary="ua_due", conditions=["the table closes", "u < length of the table", "0 < i < 1"],
     method="u|ä_x = sum_(k>=u) v^k k_p_x = u_E_x ä_(x+u)",
     latex=r"{}_{u|}\ddot a_x = \sum_{k\ge u} v^k\,{}_kp_x = {}_uE_x\,\ddot a_{x+u}",
     run=deferred_annuity, reference=_ref_deferred_annuity,
     wrong=lambda q, i, u, P: (lambda s: _out(ua_due=s, EPV=P * s))(sum((1 / (1 + i)) ** (k + 1) * _kp(q)[k] for k in range(u, len(q)))),
     rand=lambda rng: (lambda q: {"q": q, "i": _rand_i(rng), "u": rng.randint(1, len(q) - 1), "P": _sig(_lu(rng, 100, 1e5), 3)})(_rand_q(rng)),
     phrases=("Deferred annuity", "Deferred life annuity"),
     examples=[_ex({"q": [0.1, 0.2, 0.5, 1.0], "i": 0.05, "u": 2, "P": 1000.0}), _ex({"q": [0.02, 0.03, 0.05, 0.08, 0.15, 0.3, 1.0], "i": 0.04, "u": 3, "P": 5000.0})],
     level="advanced")


def continuous_insurance_udd(q, i):
    _q_check(q, whole=True)
    _rate(i)
    v = 1.0 / (1.0 + i)
    delta = math.log1p(i)
    A = _ins(q, v, len(q))
    Ab = i / delta * A
    return _out(A_bar=Ab, A=A, a_bar=(1.0 - Ab) / delta)


def _ref_continuous_insurance_udd(q, i):
    v = 1.0 / (1.0 + i)
    kp = 1.0
    Ab = ab = 0.0
    for k in range(len(q)):
        qk = q[k]
        Ab += v ** k * kp * _quad(lambda s: v ** s * qk, 0.0, 1.0)
        ab += v ** k * kp * _quad(lambda s: v ** s * (1.0 - s * qk), 0.0, 1.0)
        kp *= 1.0 - qk
    A, _, _ = _ref_back(q, i, len(q))
    return {"A_bar": Ab, "A": A, "a_bar": ab}


_add("act_continuous_insurance_udd",
     name="Continuous whole life insurance and annuity under UDD", topic="life insurance / continuous benefits",
     purpose="The actuarial present value of a whole life insurance paid at the moment of death (Ā_x = (i/delta) A_x) and of a continuous whole life annuity, from an annual life table with deaths uniformly distributed within each year.",
     inputs=[_vecin("q", "death probabilities q_x, q_(x+1), ... ending with 1 (a list)", 1, 130), _ratein()],
     outputs=[_o("A_bar", "insurance payable at the moment of death Ā_x"), _o("A", "insurance payable at the end of the year of death A_x"),
              _o("a_bar", "continuous whole life annuity ā_x")],
     primary="A_bar", conditions=["the table closes", "0 < i < 1", "uniform distribution of deaths within each year"],
     method="Ā_x = (i/delta) A_x; ā_x = (1 - Ā_x)/delta",
     latex=r"\bar A_x = \frac{i}{\delta}A_x,\qquad \bar a_x = \frac{1-\bar A_x}{\delta}",
     run=continuous_insurance_udd, reference=_ref_continuous_insurance_udd,
     wrong=lambda q, i: (lambda A: _out(A_bar=A * (1 + i), A=A, a_bar=(1 - A * (1 + i)) / math.log1p(i)))(_ins(q, 1 / (1 + i), len(q))),
     rand=lambda rng: {"q": _rand_q(rng, True, 3, 10), "i": _rand_i(rng)},
     phrases=("Continuous insurance under UDD", "Insurance payable at the moment of death"),
     examples=[_ex({"q": [0.1, 0.2, 0.5, 1.0], "i": 0.05}), _ex({"q": [0.02, 0.03, 0.05, 0.08, 0.15, 0.3, 1.0], "i": 0.06})],
     level="expert")


def mthly_annuity_udd(q, i, m):
    _q_check(q, whole=True)
    _rate(i)
    _whole("the number of payments per year m", m, 1)
    _need(m <= 365, "m must be at most 365")
    v = 1.0 / (1.0 + i)
    d = i * v
    im = m * ((1.0 + i) ** (1.0 / m) - 1.0)
    dm = m * (1.0 - v ** (1.0 / m))
    alpha = i * d / (im * dm)
    beta = (i - im) / (im * dm)
    a = _ann(q, v, len(q))
    return _out(a_m=alpha * a - beta, alpha=alpha, beta=beta, a_due=a)


def _ref_mthly_annuity_udd(q, i, m):
    import numpy as np
    v = 1.0 / (1.0 + i)
    kp = 1.0
    tot = 0.0
    j = np.arange(m, dtype=float) / m
    for k in range(len(q)):
        tot += float(np.sum(v ** (k + j) * kp * (1.0 - j * q[k]))) / m
        kp *= 1.0 - q[k]
    _, a, _ = _ref_back(q, i, len(q))
    dm = m * (1.0 - v ** (1.0 / m))
    im = m * ((1.0 + i) ** (1.0 / m) - 1.0)
    return {"a_m": tot, "alpha": i * (1 - v) / (im * dm), "beta": (i - im) / (im * dm), "a_due": a}


_add("act_mthly_annuity_udd",
     name="m-thly whole life annuity-due under UDD", topic="life annuities / m-thly payments",
     purpose="The actuarial present value of a whole life annuity-due paying 1/m at the start of every 1/m of a year, from an annual life table under the uniform distribution of deaths (ä^(m)_x = alpha(m) ä_x - beta(m)).",
     inputs=[_vecin("q", "death probabilities q_x, q_(x+1), ... ending with 1 (a list)", 1, 130), _ratein(),
             _intin("m", "payments per year (12 = monthly)", 1, 365)],
     outputs=[_o("a_m", "m-thly annuity-due ä^(m)_x (total 1 a year)"), _o("alpha", "alpha(m) = i d/(i(m) d(m))"),
              _o("beta", "beta(m) = (i - i(m))/(i(m) d(m))"), _o("a_due", "annual annuity-due ä_x")],
     primary="a_m", conditions=["the table closes", "0 < i < 1", "1 <= m <= 365", "UDD within each year"],
     method="ä^(m)_x = alpha(m) ä_x - beta(m)",
     latex=r"\ddot a^{(m)}_x = \alpha(m)\,\ddot a_x - \beta(m),\quad \alpha(m) = \frac{id}{i^{(m)}d^{(m)}},\ \beta(m) = \frac{i-i^{(m)}}{i^{(m)}d^{(m)}}",
     run=mthly_annuity_udd, reference=_ref_mthly_annuity_udd,
     wrong=lambda q, i, m: (lambda o: dict(o, a_m=o["a_due"] - (m + 1) / (2 * m)))(mthly_annuity_udd(q, i, m)),
     rand=lambda rng: {"q": _rand_q(rng), "i": _rand_i(rng), "m": rng.choice([2, 4, 12])},
     phrases=("m-thly annuity under UDD", "Monthly life annuity under UDD", "Mthly annuity under UDD"),
     examples=[_ex({"q": [0.1, 0.2, 0.5, 1.0], "i": 0.05, "m": 12}), _ex({"q": [0.02, 0.03, 0.05, 0.08, 0.15, 0.3, 1.0], "i": 0.04, "m": 4})],
     level="expert")


def woolhouse(a_due, m, i, mu):
    _need(_isnum(a_due) and a_due >= 1, "the annual annuity-due must be at least 1")
    _whole("the number of payments per year m", m, 1)
    _need(m <= 365, "m must be at most 365")
    _rate(i)
    _need(_isnum(mu) and 0 <= mu < 5, "the force of mortality must lie between 0 and 5")
    two = a_due - (m - 1.0) / (2.0 * m)
    three = two - (m * m - 1.0) / (12.0 * m * m) * (math.log1p(i) + mu)
    return _out(a_m3=three, a_m2=two)


def _ref_woolhouse(a_due, m, i, mu):
    import sympy as sp
    mm = sp.Integer(m)
    two = sp.nsimplify(a_due) - (mm - 1) / (2 * mm)
    three = two - (mm ** 2 - 1) / (12 * mm ** 2) * (sp.log(1 + sp.nsimplify(i)) + sp.nsimplify(mu))
    return {"a_m3": float(three), "a_m2": float(two)}


_add("act_woolhouse_approximation",
     name="Woolhouse approximation for an m-thly annuity", topic="life annuities / m-thly payments",
     purpose="Woolhouse's two- and three-term approximations of the m-thly whole life annuity-due ä^(m)_x from the annual annuity-due ä_x, the interest rate and the force of mortality at x.",
     inputs=[_numin("a_due", "annual whole life annuity-due ä_x"), _intin("m", "payments per year (12 = monthly)", 1, 365),
             _ratein(), _numin("mu", "force of mortality at age x (per year)")],
     outputs=[_o("a_m3", "three-term Woolhouse approximation"), _o("a_m2", "two-term approximation ä_x - (m-1)/(2m)")],
     primary="a_m3", conditions=["ä_x >= 1", "1 <= m <= 365", "0 < i < 1", "0 <= mu < 5"],
     method="ä^(m)_x ≈ ä_x - (m-1)/(2m) - (m^2-1)/(12 m^2) (delta + mu_x)",
     latex=r"\ddot a^{(m)}_x \approx \ddot a_x - \frac{m-1}{2m} - \frac{m^2-1}{12m^2}\left(\delta + \mu_x\right)",
     run=woolhouse, reference=_ref_woolhouse,
     wrong=lambda a_due, m, i, mu: _out(a_m3=a_due - (m + 1) / (2 * m) - (m * m - 1) / (12 * m * m) * (math.log1p(i) + mu), a_m2=a_due - (m + 1) / (2 * m)),
     rand=lambda rng: {"a_due": _sig(rng.uniform(3, 20), 4), "m": rng.choice([2, 4, 12]), "i": _rand_i(rng), "mu": _sig(_lu(rng, 1e-3, 0.2), 3)},
     phrases=("Woolhouse approximation", "Woolhouse formula"),
     examples=[_ex({"a_due": 14.8166, "m": 12, "i": 0.05, "mu": 0.00473}), _ex({"a_due": 10.0, "m": 4, "i": 0.06, "mu": 0.02})],
     level="advanced",
     weak="the same published approximation evaluated in exact sympy arithmetic")


# ================================================================================================ PREMIUMS
def net_premium_whole_life(q, i, S):
    _q_check(q, whole=True)
    _rate(i)
    _pos("the sum insured S", S)
    v = 1.0 / (1.0 + i)
    A = _ins(q, v, len(q))
    a = _ann(q, v, len(q))
    return _out(P=S * A / a, A=A, a_due=a)


def _ref_net_premium_whole_life(q, i, S):
    _, a, _ = _ref_back(q, i, len(q))
    d = i / (1.0 + i)
    return {"P": S * (1.0 / a - d), "A": 1.0 - d * a, "a_due": a}


_add("act_net_premium_whole_life",
     name="Net annual premium for whole life insurance", topic="premiums / net premiums",
     purpose="The level annual net premium, payable at the start of each year while alive, for a fully discrete whole life insurance of S, by the equivalence principle.",
     inputs=[_vecin("q", "death probabilities q_x, q_(x+1), ... ending with 1 (a list)", 1, 130), _ratein(), _numin("S", "sum insured")],
     outputs=[_o("P", "annual net premium S A_x / ä_x"), _o("A", "whole life insurance A_x"), _o("a_due", "whole life annuity-due ä_x")],
     primary="P", conditions=["the table closes", "0 < i < 1", "S > 0"],
     method="equivalence principle: P ä_x = S A_x",
     latex=r"P = S\,\frac{A_x}{\ddot a_x} = S\left(\frac{1}{\ddot a_x} - d\right)",
     run=net_premium_whole_life, reference=_ref_net_premium_whole_life,
     wrong=lambda q, i, S: (lambda v: _out(P=S * _ins(q, v, len(q)) / (_ann(q, v, len(q)) - 1), A=_ins(q, v, len(q)), a_due=_ann(q, v, len(q))))(1 / (1 + i)),
     rand=lambda rng: {"q": _rand_q(rng), "i": _rand_i(rng), "S": _sig(_lu(rng, 1e3, 1e6), 3)},
     phrases=("Net premium for whole life insurance", "Whole life net premium", "Net level premium for whole life"),
     examples=[_ex({"q": [0.1, 0.2, 0.5, 1.0], "i": 0.05, "S": 1000.0}), _ex({"q": [0.02, 0.03, 0.05, 0.08, 0.15, 0.3, 1.0], "i": 0.04, "S": 100000.0})],
     level="intermediate")


def net_premium_term(q, i, n, S):
    _q_check(q, n=n)
    _rate(i)
    _pos("the sum insured S", S)
    v = 1.0 / (1.0 + i)
    A = _ins(q, v, n)
    a = _ann(q, v, n)
    return _out(P=S * A / a, A1=A, a_due=a)


def _ref_net_premium_term(q, i, n, S):
    A, a, _ = _ref_back(q, i, n)
    return {"P": S * A / a, "A1": A, "a_due": a}


_add("act_net_premium_term",
     name="Net annual premium for term insurance", topic="premiums / net premiums",
     purpose="The level annual net premium, payable for n years while alive, for a fully discrete n-year term insurance of S, by the equivalence principle.",
     inputs=[_vecin("q", "death probabilities q_x, q_(x+1), ... for at least n years (a list)", 1, 130), _ratein(),
             _intin("n", "term in years", 1, 130), _numin("S", "sum insured")],
     outputs=[_o("P", "annual net premium S A^1_(x:n) / ä_(x:n)"), _o("A1", "term insurance A^1_(x:n)"), _o("a_due", "temporary annuity-due ä_(x:n)")],
     primary="P", conditions=["at least n death probabilities", "0 < i < 1", "S > 0"],
     method="P ä_(x:n) = S A^1_(x:n)",
     latex=r"P = S\,\frac{A^1_{x:\overline{n}|}}{\ddot a_{x:\overline{n}|}}",
     run=net_premium_term, reference=_ref_net_premium_term,
     wrong=lambda q, i, n, S: (lambda v: _out(P=S * _ins(q, v, n) / n, A1=_ins(q, v, n), a_due=_ann(q, v, n)))(1 / (1 + i)),
     rand=lambda rng: (lambda q: {"q": q, "i": _rand_i(rng), "n": rng.randint(2, len(q)), "S": _sig(_lu(rng, 1e3, 1e6), 3)})(_rand_q(rng, False)),
     phrases=("Net premium for term insurance", "Term insurance net premium"),
     examples=[_ex({"q": [0.1, 0.2, 0.5, 1.0], "i": 0.05, "n": 3, "S": 1000.0}), _ex({"q": [0.01, 0.012, 0.015, 0.018, 0.022], "i": 0.06, "n": 5, "S": 50000.0})],
     level="intermediate")


def net_premium_endowment(q, i, n, S):
    _q_check(q, n=n)
    _rate(i)
    _pos("the sum insured S", S)
    v = 1.0 / (1.0 + i)
    A = _ins(q, v, n) + _pe(q, v, n)
    a = _ann(q, v, n)
    return _out(P=S * A / a, A=A, a_due=a)


def _ref_net_premium_endowment(q, i, n, S):
    _, a, _ = _ref_back(q, i, n)
    d = i / (1.0 + i)
    return {"P": S * (1.0 / a - d), "A": 1.0 - d * a, "a_due": a}


_add("act_net_premium_endowment",
     name="Net annual premium for endowment insurance", topic="premiums / net premiums",
     purpose="The level annual net premium, payable for n years while alive, for a fully discrete n-year endowment insurance of S, by the equivalence principle.",
     inputs=[_vecin("q", "death probabilities q_x, q_(x+1), ... for at least n years (a list)", 1, 130), _ratein(),
             _intin("n", "term in years", 1, 130), _numin("S", "sum insured")],
     outputs=[_o("P", "annual net premium S A_(x:n) / ä_(x:n)"), _o("A", "endowment insurance A_(x:n)"), _o("a_due", "temporary annuity-due ä_(x:n)")],
     primary="P", conditions=["at least n death probabilities", "0 < i < 1", "S > 0"],
     method="P ä_(x:n) = S A_(x:n); equivalently P = S (1/ä_(x:n) - d)",
     latex=r"P = S\,\frac{A_{x:\overline{n}|}}{\ddot a_{x:\overline{n}|}} = S\left(\frac{1}{\ddot a_{x:\overline{n}|}} - d\right)",
     run=net_premium_endowment, reference=_ref_net_premium_endowment,
     wrong=lambda q, i, n, S: (lambda v: _out(P=S * (_ins(q, v, n) + _pe(q, v, n)) / (_ann(q, v, n) + _pe(q, v, n)), A=_ins(q, v, n) + _pe(q, v, n), a_due=_ann(q, v, n)))(1 / (1 + i)),
     rand=lambda rng: (lambda q: {"q": q, "i": _rand_i(rng), "n": rng.randint(2, len(q)), "S": _sig(_lu(rng, 1e3, 1e6), 3)})(_rand_q(rng, False)),
     phrases=("Net premium for endowment insurance", "Endowment net premium"),
     examples=[_ex({"q": [0.1, 0.2, 0.5, 1.0], "i": 0.05, "n": 3, "S": 1000.0}), _ex({"q": [0.01, 0.012, 0.015, 0.018, 0.022], "i": 0.06, "n": 5, "S": 50000.0})],
     level="intermediate")


def net_premium_limited_pay(q, i, h, S):
    _q_check(q, whole=True)
    _rate(i)
    _pos("the sum insured S", S)
    _whole("the premium paying period h", h, 1)
    _need(h <= len(q), "the premium paying period cannot outlast the table")
    v = 1.0 / (1.0 + i)
    A = _ins(q, v, len(q))
    a = _ann(q, v, h)
    return _out(P=S * A / a, A=A, a_due_h=a)


def _ref_net_premium_limited_pay(q, i, h, S):
    A, _, _ = _ref_back(q, i, len(q))
    _, a, _ = _ref_back(q, i, h)
    return {"P": S * A / a, "A": A, "a_due_h": a}


_add("act_net_premium_limited_pay",
     name="Net annual premium for limited-payment whole life insurance", topic="premiums / net premiums",
     purpose="The level annual net premium for a fully discrete whole life insurance of S with premiums payable for at most h years.",
     inputs=[_vecin("q", "death probabilities q_x, q_(x+1), ... ending with 1 (a list)", 1, 130), _ratein(),
             _intin("h", "premium paying period in years", 1, 130), _numin("S", "sum insured")],
     outputs=[_o("P", "annual net premium S A_x / ä_(x:h)"), _o("A", "whole life insurance A_x"), _o("a_due_h", "h-year temporary annuity-due")],
     primary="P", conditions=["the table closes", "1 <= h <= length of the table", "0 < i < 1"],
     method="P ä_(x:h) = S A_x",
     latex=r"{}_hP_x = S\,\frac{A_x}{\ddot a_{x:\overline{h}|}}",
     run=net_premium_limited_pay, reference=_ref_net_premium_limited_pay,
     wrong=lambda q, i, h, S: (lambda v: _out(P=S * _ins(q, v, h) / _ann(q, v, h), A=_ins(q, v, len(q)), a_due_h=_ann(q, v, h)))(1 / (1 + i)),
     rand=lambda rng: (lambda q: {"q": q, "i": _rand_i(rng), "h": rng.randint(1, len(q) - 1), "S": _sig(_lu(rng, 1e3, 1e6), 3)})(_rand_q(rng)),
     phrases=("Limited payment whole life premium", "Limited-pay net premium"),
     examples=[_ex({"q": [0.1, 0.2, 0.5, 1.0], "i": 0.05, "h": 2, "S": 1000.0}), _ex({"q": [0.02, 0.03, 0.05, 0.08, 0.15, 0.3, 1.0], "i": 0.04, "h": 3, "S": 100000.0})],
     level="advanced")


def _gross_G(q, i, S, e0, r, c, E):
    v = 1.0 / (1.0 + i)
    A = _ins(q, v, len(q))
    a = _ann(q, v, len(q))
    den = a * (1.0 - r) - (e0 - r)
    _need(den > 0, "the expense loadings take the whole premium")
    return ((S + E) * A + c * a) / den, A, a


def _exp_check(S, e0, r, c, E):
    _pos("the sum insured S", S)
    _frac("the initial expense fraction e0", e0, hi_open=True)
    _frac("the renewal expense fraction r", r, hi_open=True)
    _nonneg("the per-policy expense c and the settlement expense E", c, E)


def gross_premium(q, i, S, e0, r, c, E):
    _q_check(q, whole=True)
    _rate(i)
    _exp_check(S, e0, r, c, E)
    G, A, a = _gross_G(q, i, S, e0, r, c, E)
    P = S * A / a
    return _out(G=G, P=P, loading=G - P)


def _ref_gross_premium(q, i, S, e0, r, c, E):
    import numpy as np
    n = len(q)
    kp = np.concatenate([[1.0], np.cumprod(1.0 - np.array(q, dtype=float))])
    v = 1.0 / (1.0 + i)

    def loss(G):                                     # expected present value of the gross loss, enumerating K
        tot = 0.0
        for k in range(n):
            pk = kp[k] * q[k]
            pv_ben = (S + E) * v ** (k + 1)
            pv_prem = sum(G * v ** j for j in range(k + 1))
            pv_exp = e0 * G + sum(r * G * v ** j for j in range(1, k + 1)) + sum(c * v ** j for j in range(k + 1))
            tot += pk * (pv_ben + pv_exp - pv_prem)
        return tot
    G = _brentq(loss, 0.0, 10.0 * (S + E + c))
    Pn = _brentq(lambda P: sum(kp[k] * q[k] * (S * v ** (k + 1) - sum(P * v ** j for j in range(k + 1))) for k in range(n)), 0.0, 10.0 * S)
    return {"G": G, "P": Pn, "loading": G - Pn}


_add("act_gross_premium",
     name="Gross annual premium for whole life insurance with expenses", topic="premiums / gross premiums",
     purpose="The level annual gross premium for a fully discrete whole life insurance of S by the equivalence principle with expenses: an initial fraction e0 of the first premium, a renewal fraction r of later premiums, a per-policy expense c every year (first year included) and a settlement expense E per claim; with the net premium and the loading.",
     inputs=[_vecin("q", "death probabilities q_x, q_(x+1), ... ending with 1 (a list)", 1, 130), _ratein(), _numin("S", "sum insured"),
             _numin("e0", "initial expense as a fraction of the first gross premium (e.g. 40%)", pct=True),
             _numin("r", "renewal expense as a fraction of each later gross premium (e.g. 5%)", pct=True),
             _numin("c", "per-policy expense at the start of every year"), _numin("E", "settlement expense per claim")],
     outputs=[_o("G", "annual gross premium"), _o("P", "annual net premium"), _o("loading", "expense loading G - P")],
     primary="G", conditions=["the table closes", "0 < i < 1", "0 <= e0, r < 1", "c, E >= 0"],
     method="G ä_x = (S + E) A_x + e0 G + r G (ä_x - 1) + c ä_x",
     latex=r"G = \frac{(S+E)A_x + c\,\ddot a_x}{(1-r)\ddot a_x - (e_0 - r)}",
     run=gross_premium, reference=_ref_gross_premium,
     wrong=lambda q, i, S, e0, r, c, E: (lambda G, A, a: _out(G=((S + E) * A + c * a) / (a * (1 - r) - e0), P=S * A / a, loading=0.0))(*_gross_G(q, i, S, e0, r, c, E)),
     rand=lambda rng: {"q": _rand_q(rng, True, 3, 10), "i": _rand_i(rng), "S": _sig(_lu(rng, 1e4, 1e6), 3), "e0": _sig(rng.uniform(0.1, 0.9), 2),
                       "r": _sig(rng.uniform(0.01, 0.1), 2), "c": _sig(rng.uniform(0, 100), 3), "E": _sig(rng.uniform(0, 500), 3)},
     phrases=("Gross premium", "Gross premium with expenses", "Expense-loaded premium"),
     examples=[_ex({"q": [0.1, 0.2, 0.5, 1.0], "i": 0.05, "S": 1000.0, "e0": 0.4, "r": 0.05, "c": 10.0, "E": 20.0}),
               _ex({"q": [0.02, 0.03, 0.05, 0.08, 0.15, 0.3, 1.0], "i": 0.04, "S": 100000.0, "e0": 0.6, "r": 0.03, "c": 50.0, "E": 200.0})],
     level="advanced",
     extra_q=[{"q": "Gross premium: q = [0.1, 0.2, 0.5, 1], i = 5%, S = 1000, e0 = 40%, r = 5%, c = 10, E = 20", "like": 0}])


def loss_at_issue(q, i, S, P):
    _q_check(q, whole=True)
    _rate(i)
    _pos("the sum insured S and the premium P", S, P)
    v = 1.0 / (1.0 + i)
    d = i * v
    A = _ins(q, v, len(q))
    A2 = _ins(q, v * v, len(q))
    a = _ann(q, v, len(q))
    kp = _kp(q)
    pos = 0.0
    for k in range(len(q)):
        if S * v ** (k + 1) - P * (1.0 - v ** (k + 1)) / d > 0:
            pos += kp[k] * q[k]
    return _out(mean=S * A - P * a, sd=(S + P / d) * math.sqrt(max(A2 - A * A, 0.0)), prob_loss=pos)


def _ref_loss_at_issue(q, i, S, P):
    import numpy as np
    n = len(q)
    kp = np.concatenate([[1.0], np.cumprod(1.0 - np.array(q, dtype=float))])
    pk = np.array([kp[k] * q[k] for k in range(n)])
    v = 1.0 / (1.0 + i)
    L = np.array([S * v ** (k + 1) - P * sum(v ** j for j in range(k + 1)) for k in range(n)])
    m = float(np.dot(pk, L))
    return {"mean": m, "sd": math.sqrt(float(np.dot(pk, (L - m) ** 2))), "prob_loss": float(np.sum(pk[L > 0]))}


_add("act_loss_at_issue",
     name="Loss at issue of whole life insurance: mean, standard deviation, probability of loss", topic="premiums / loss random variable",
     purpose="The mean and standard deviation of the insurer's loss at issue L0 = S v^(K+1) - P ä_(K+1) for a fully discrete whole life insurance of S with annual premium P, and the probability that the loss is positive.",
     inputs=[_vecin("q", "death probabilities q_x, q_(x+1), ... ending with 1 (a list)", 1, 130), _ratein(), _numin("S", "sum insured"), _numin("P", "annual premium")],
     outputs=[_o("mean", "expected loss at issue S A_x - P ä_x"), _o("sd", "standard deviation (S + P/d) sqrt(2A_x - A_x^2)"),
              _o("prob_loss", "probability that the loss at issue is positive")],
     primary="sd", conditions=["the table closes", "0 < i < 1", "S, P > 0"],
     method="L0 = (S + P/d) v^(K+1) - P/d; E = S A - P ä; Var = (S + P/d)^2 (2A - A^2)",
     latex=r"L_0 = \left(S+\frac Pd\right)v^{K+1} - \frac Pd,\quad \mathrm{Var}(L_0) = \left(S+\frac Pd\right)^2\left({}^2A_x - A_x^2\right)",
     run=loss_at_issue, reference=_ref_loss_at_issue,
     wrong=lambda q, i, S, P: (lambda o: dict(o, sd=o["sd"] * S / (S + P * (1 + i) / i)))(loss_at_issue(q, i, S, P)),
     rand=lambda rng: (lambda q, i, S: {"q": q, "i": i, "S": S, "P": _sig(S * _ins(q, 1 / (1 + i), len(q)) / _ann(q, 1 / (1 + i), len(q)) * rng.uniform(0.7, 1.4), 4)})(_rand_q(rng), _rand_i(rng), _sig(_lu(rng, 1e3, 1e6), 3)),
     phrases=("Loss at issue", "Loss random variable at issue"),
     examples=[_ex({"q": [0.1, 0.2, 0.5, 1.0], "i": 0.05, "S": 1000.0, "P": 450.0}), _ex({"q": [0.02, 0.03, 0.05, 0.08, 0.15, 0.3, 1.0], "i": 0.04, "S": 100000.0, "P": 15000.0})],
     level="expert")


# ================================================================================================ RESERVES
def _fwd_reserve(q, i, S, P, n, t, endow=False):
    """Reference helper: the policy value recursion (tV + P)(1+i) = q S + p (t+1)V from 0V = 0."""
    V = 0.0
    for k in range(t):
        V = ((V + P) * (1.0 + i) - q[k] * S) / (1.0 - q[k]) if q[k] < 1 else 0.0
    return V


def reserve_whole_life(q, i, S, t):
    _q_check(q, whole=True)
    _rate(i)
    _pos("the sum insured S", S)
    _whole("the policy duration t", t, 0)
    _need(t < len(q), "the duration t must lie inside the table")
    v = 1.0 / (1.0 + i)
    a0 = _ann(q, v, len(q))
    P = S * _ins(q, v, len(q)) / a0
    qt = q[t:]
    At = _ins(qt, v, len(qt))
    at = _ann(qt, v, len(qt))
    return _out(V=S * At - P * at, P=P, A_t=At, a_due_t=at)


def _ref_reserve_whole_life(q, i, S, t):
    A, a, _ = _ref_back(q, i, len(q))
    P = S * A / a
    At, at, _ = _ref_back(q[t:], i, len(q) - t)
    return {"V": _fwd_reserve(q, i, S, P, len(q), t), "P": P, "A_t": At, "a_due_t": at}


_add("act_reserve_whole_life",
     name="Net premium reserve for whole life insurance", topic="reserves / prospective reserves",
     purpose="The net premium reserve (policy value) at integer duration t of a fully discrete whole life insurance of S with the equivalence-principle net premium, by the prospective method.",
     inputs=[_vecin("q", "death probabilities q_x, q_(x+1), ... ending with 1 (a list)", 1, 130), _ratein(), _numin("S", "sum insured"),
             _intin("t", "policy duration in years", 0, 130)],
     outputs=[_o("V", "net premium reserve tV"), _o("P", "annual net premium"), _o("A_t", "A_(x+t)"), _o("a_due_t", "ä_(x+t)")],
     primary="V", conditions=["the table closes", "0 <= t < length of the table", "0 < i < 1"],
     method="tV = S A_(x+t) - P ä_(x+t) = S (1 - ä_(x+t)/ä_x)",
     latex=r"{}_tV = S\,A_{x+t} - P\,\ddot a_{x+t} = S\left(1 - \frac{\ddot a_{x+t}}{\ddot a_x}\right)",
     run=reserve_whole_life, reference=_ref_reserve_whole_life,
     wrong=lambda q, i, S, t: (lambda o: dict(o, V=S * o["A_t"] - o["P"] * (o["a_due_t"] - 1)))(reserve_whole_life(q, i, S, t)),
     rand=lambda rng: (lambda q: {"q": q, "i": _rand_i(rng), "S": _sig(_lu(rng, 1e3, 1e6), 3), "t": rng.randint(1, len(q) - 1)})(_rand_q(rng)),
     phrases=("Whole life reserve", "Net premium reserve for whole life", "Policy value for whole life"),
     examples=[_ex({"q": [0.1, 0.2, 0.5, 1.0], "i": 0.05, "S": 1000.0, "t": 1}), _ex({"q": [0.02, 0.03, 0.05, 0.08, 0.15, 0.3, 1.0], "i": 0.04, "S": 100000.0, "t": 3})],
     level="advanced")


def reserve_endowment(q, i, n, S, t):
    _q_check(q, n=n)
    _rate(i)
    _pos("the sum insured S", S)
    _whole("the policy duration t", t, 0)
    _need(t <= n, "the duration t must lie between 0 and n")
    if t == n:
        return _out(V=S, P=S * (_ins(q, 1 / (1 + i), n) + _pe(q, 1 / (1 + i), n)) / _ann(q, 1 / (1 + i), n))
    v = 1.0 / (1.0 + i)
    P = S * (_ins(q, v, n) + _pe(q, v, n)) / _ann(q, v, n)
    qt = q[t:n]
    m = n - t
    return _out(V=S * (_ins(qt, v, m) + _pe(qt, v, m)) - P * _ann(qt, v, m), P=P)


def _ref_reserve_endowment(q, i, n, S, t):
    A, a, E = _ref_back(q, i, n)
    P = S * (A + E) / a
    return {"V": _fwd_reserve(q, i, S, P, n, t) if t < n else S, "P": P}


_add("act_reserve_endowment",
     name="Net premium reserve for endowment insurance", topic="reserves / prospective reserves",
     purpose="The net premium reserve at integer duration t of a fully discrete n-year endowment insurance of S with level net premiums payable for n years, by the prospective method (S at maturity).",
     inputs=[_vecin("q", "death probabilities q_x, q_(x+1), ... for at least n years (a list)", 1, 130), _ratein(),
             _intin("n", "term in years", 1, 130), _numin("S", "sum insured"), _intin("t", "policy duration in years", 0, 130)],
     outputs=[_o("V", "net premium reserve tV"), _o("P", "annual net premium")],
     primary="V", conditions=["at least n death probabilities", "0 <= t <= n", "0 < i < 1"],
     method="tV = S A_(x+t:n-t) - P ä_(x+t:n-t)",
     latex=r"{}_tV = S\,A_{x+t:\overline{n-t}|} - P\,\ddot a_{x+t:\overline{n-t}|}",
     run=reserve_endowment, reference=_ref_reserve_endowment,
     wrong=lambda q, i, n, S, t: (lambda o: dict(o, V=o["V"] * (1 + i)))(reserve_endowment(q, i, n, S, t)),
     rand=lambda rng: (lambda q: (lambda n: {"q": q, "i": _rand_i(rng), "n": n, "S": _sig(_lu(rng, 1e3, 1e6), 3), "t": rng.randint(1, n - 1)})(rng.randint(2, len(q))))(_rand_q(rng, False)),
     phrases=("Endowment reserve", "Net premium reserve for endowment insurance"),
     examples=[_ex({"q": [0.1, 0.2, 0.5, 1.0], "i": 0.05, "n": 3, "S": 1000.0, "t": 1}), _ex({"q": [0.01, 0.012, 0.015, 0.018, 0.022], "i": 0.06, "n": 5, "S": 50000.0, "t": 3})],
     level="advanced")


def reserve_term(q, i, n, S, t):
    _q_check(q, n=n)
    _rate(i)
    _pos("the sum insured S", S)
    _whole("the policy duration t", t, 0)
    _need(t <= n, "the duration t must lie between 0 and n")
    v = 1.0 / (1.0 + i)
    P = S * _ins(q, v, n) / _ann(q, v, n)
    if t == n:
        return _out(V=0.0, P=P)
    qt = q[t:n]
    m = n - t
    return _out(V=S * _ins(qt, v, m) - P * _ann(qt, v, m), P=P)


def _ref_reserve_term(q, i, n, S, t):
    A, a, _ = _ref_back(q, i, n)
    P = S * A / a
    return {"V": _fwd_reserve(q, i, S, P, n, t) if t < n else 0.0, "P": P}


_add("act_reserve_term",
     name="Net premium reserve for term insurance", topic="reserves / prospective reserves",
     purpose="The net premium reserve at integer duration t of a fully discrete n-year term insurance of S with level net premiums payable for n years, by the prospective method.",
     inputs=[_vecin("q", "death probabilities q_x, q_(x+1), ... for at least n years (a list)", 1, 130), _ratein(),
             _intin("n", "term in years", 1, 130), _numin("S", "sum insured"), _intin("t", "policy duration in years", 0, 130)],
     outputs=[_o("V", "net premium reserve tV"), _o("P", "annual net premium")],
     primary="V", conditions=["at least n death probabilities", "0 <= t <= n", "0 < i < 1"],
     method="tV = S A^1_(x+t:n-t) - P ä_(x+t:n-t)",
     latex=r"{}_tV = S\,A^1_{x+t:\overline{n-t}|} - P\,\ddot a_{x+t:\overline{n-t}|}",
     run=reserve_term, reference=_ref_reserve_term,
     wrong=lambda q, i, n, S, t: (lambda o: dict(o, V=o["V"] + o["P"]))(reserve_term(q, i, n, S, t)),
     rand=lambda rng: (lambda q: (lambda n: {"q": q, "i": _rand_i(rng), "n": n, "S": _sig(_lu(rng, 1e3, 1e6), 3), "t": rng.randint(1, n - 1)})(rng.randint(3, len(q))))(_rand_q(rng, False, 4)),
     phrases=("Term insurance reserve", "Net premium reserve for term insurance"),
     examples=[_ex({"q": [0.01, 0.02, 0.04, 0.08], "i": 0.05, "n": 4, "S": 1000.0, "t": 2}), _ex({"q": [0.01, 0.012, 0.015, 0.018, 0.022], "i": 0.06, "n": 5, "S": 50000.0, "t": 3})],
     level="advanced")


def retrospective_reserve(q, i, S, P, t):
    _q_check(q)
    _rate(i)
    _pos("the sum insured S and the premium P", S, P)
    _whole("the policy duration t", t, 1)
    _need(t <= len(q), "the table must cover the first t years")
    v = 1.0 / (1.0 + i)
    E = _pe(q, v, t)
    _need(E > 0, "nobody survives t years in the table")
    return _out(V=(P * _ann(q, v, t) - S * _ins(q, v, t)) / E, tEx=E)


def _ref_retrospective_reserve(q, i, S, P, t):
    _, _, E = _ref_back(q, i, t)
    return {"V": _fwd_reserve(q, i, S, P, t, t), "tEx": E}


_add("act_retrospective_reserve",
     name="Retrospective reserve (accumulated fund per survivor)", topic="reserves / retrospective reserves",
     purpose="The retrospective reserve at integer duration t of a fully discrete life insurance of S with annual premium P: the actuarial accumulated value of the premiums received less the cost of insurance, per survivor.",
     inputs=[_vecin("q", "death probabilities q_x, q_(x+1), ... for at least t years (a list)", 1, 130), _ratein(), _numin("S", "sum insured"),
             _numin("P", "annual premium"), _intin("t", "policy duration in years", 1, 130)],
     outputs=[_o("V", "retrospective reserve tV"), _o("tEx", "pure endowment t_E_x")],
     primary="V", conditions=["at least t death probabilities", "t_p_x > 0", "0 < i < 1"],
     method="tV = (P ä_(x:t) - S A^1_(x:t)) / t_E_x",
     latex=r"{}_tV = \frac{P\,\ddot a_{x:\overline{t}|} - S\,A^1_{x:\overline{t}|}}{{}_tE_x}",
     run=retrospective_reserve, reference=_ref_retrospective_reserve,
     wrong=lambda q, i, S, P, t: (lambda v: _out(V=(P * _ann(q, v, t) - S * _ins(q, v, t)) * (1 + i) ** t, tEx=_pe(q, v, t)))(1 / (1 + i)),
     rand=lambda rng: (lambda q: {"q": q, "i": _rand_i(rng), "S": _sig(_lu(rng, 1e3, 1e6), 3), "P": _sig(_lu(rng, 10, 5e4), 3), "t": rng.randint(1, len(q))})(_rand_q(rng, False)),
     phrases=("Retrospective reserve", "Retrospective policy value"),
     examples=[_ex({"q": [0.1, 0.2, 0.5, 1.0], "i": 0.05, "S": 1000.0, "P": 450.0, "t": 2}),
               _ex({"q": [0.01, 0.012, 0.015, 0.018, 0.022], "i": 0.06, "S": 50000.0, "P": 8000.0, "t": 3})],
     level="advanced")


def policy_value_recursion(V, P, e, i, q, S, E):
    _fin(V, P, e)
    _rate(i)
    _need(_isnum(q) and 0 <= q < 1, "q must lie between 0 and 1 (below 1)")
    _pos("the sum insured S", S)
    _nonneg("the settlement expense E", E)
    return _out(V_next=((V + P - e) * (1.0 + i) - q * (S + E)) / (1.0 - q))


def _ref_policy_value_recursion(V, P, e, i, q, S, E):
    import sympy as sp
    W = sp.symbols("W")
    R = lambda x: sp.Rational(repr(float(x)))
    expr = (R(V) + R(P) - R(e)) * (1 + R(i)) - R(q) * (R(S) + R(E)) - (1 - R(q)) * W
    return {"V_next": float(sp.solve_linear(expr, symbols=[W])[1])}


_add("act_policy_value_recursion",
     name="Policy value recursion (one year)", topic="reserves / recursion",
     purpose="The policy value at the end of a policy year from the value at its start, the premium and expense at the start of the year, the interest rate, the death probability, the sum insured and the settlement expense: (tV + P - e)(1+i) = q (S + E) + p (t+1)V.",
     inputs=[_numin("V", "policy value at the start of the year tV"), _numin("P", "premium at the start of the year"),
             _numin("e", "expense at the start of the year"), _ratein(), _numin("q", "death probability for the year q_(x+t)"),
             _numin("S", "death benefit"), _numin("E", "settlement expense per claim")],
     outputs=[_o("V_next", "policy value at the end of the year (t+1)V")],
     primary="V_next", conditions=["0 <= q < 1", "0 < i < 1"],
     method="(t+1)V = [(tV + P - e)(1 + i) - q (S + E)] / (1 - q)",
     latex=r"\left({}_tV + P - e\right)(1+i) = q_{x+t}\,(S+E) + p_{x+t}\;{}_{t+1}V",
     run=policy_value_recursion, reference=_ref_policy_value_recursion,
     wrong=lambda V, P, e, i, q, S, E: _out(V_next=(V + P - e) * (1 + i) - q * (S + E)),
     rand=lambda rng: {"V": _sig(rng.uniform(0, 5000), 4), "P": _sig(rng.uniform(10, 1000), 4), "e": _sig(rng.uniform(0, 50), 3), "i": _rand_i(rng),
                       "q": _sig(_lu(rng, 1e-3, 0.3), 3), "S": _sig(rng.uniform(1e3, 1e5), 3), "E": _sig(rng.uniform(0, 100), 3)},
     phrases=("Policy value recursion", "Reserve recursion"),
     examples=[_ex({"V": 100.0, "P": 50.0, "e": 5.0, "i": 0.05, "q": 0.01, "S": 1000.0, "E": 0.0}),
               _ex({"V": 2500.0, "P": 400.0, "e": 20.0, "i": 0.04, "q": 0.03, "S": 10000.0, "E": 100.0})],
     level="intermediate",
     weak="the same recursion equation solved for the end-of-year value by sympy")


def gross_premium_reserve(q, i, S, e0, r, c, E, t):
    _q_check(q, whole=True)
    _rate(i)
    _exp_check(S, e0, r, c, E)
    _whole("the policy duration t", t, 0)
    _need(t < len(q), "the duration t must lie inside the table")
    G, _, _ = _gross_G(q, i, S, e0, r, c, E)
    if t == 0:
        return _out(V=0.0, G=G)
    v = 1.0 / (1.0 + i)
    qt = q[t:]
    A = _ins(qt, v, len(qt))
    a = _ann(qt, v, len(qt))
    return _out(V=(S + E) * A + c * a - G * (1.0 - r) * a, G=G)


def _ref_gross_premium_reserve(q, i, S, e0, r, c, E, t):
    G = _ref_gross_premium(q, i, S, e0, r, c, E)["G"]
    V = 0.0
    for k in range(t):
        exp = e0 * G + c if k == 0 else r * G + c
        V = ((V + G - exp) * (1.0 + i) - q[k] * (S + E)) / (1.0 - q[k])
    return {"V": V if t > 0 else 0.0, "G": G}


_add("act_gross_premium_reserve",
     name="Gross premium reserve for whole life insurance with expenses", topic="reserves / gross premium reserves",
     purpose="The gross premium policy value at integer duration t of a fully discrete whole life insurance of S, with the gross premium set by the equivalence principle with initial, renewal, per-policy and settlement expenses.",
     inputs=[_vecin("q", "death probabilities q_x, q_(x+1), ... ending with 1 (a list)", 1, 130), _ratein(), _numin("S", "sum insured"),
             _numin("e0", "initial expense as a fraction of the first gross premium", pct=True),
             _numin("r", "renewal expense as a fraction of each later gross premium", pct=True),
             _numin("c", "per-policy expense at the start of every year"), _numin("E", "settlement expense per claim"),
             _intin("t", "policy duration in years", 0, 130)],
     outputs=[_o("V", "gross premium reserve tV"), _o("G", "annual gross premium")],
     primary="V", conditions=["the table closes", "0 <= t < length of the table", "0 <= e0, r < 1"],
     method="tV = (S + E) A_(x+t) + c ä_(x+t) - G (1 - r) ä_(x+t) for t >= 1 (0 at issue by the equivalence principle)",
     latex=r"{}_tV^g = (S+E)A_{x+t} + c\,\ddot a_{x+t} - (1-r)G\,\ddot a_{x+t}",
     run=gross_premium_reserve, reference=_ref_gross_premium_reserve,
     wrong=lambda q, i, S, e0, r, c, E, t: (lambda o: dict(o, V=o["V"] - r * o["G"] * _ann(q[t:], 1 / (1 + i), len(q) - t)))(gross_premium_reserve(q, i, S, e0, r, c, E, t)),
     rand=lambda rng: (lambda q: {"q": q, "i": _rand_i(rng), "S": _sig(_lu(rng, 1e4, 1e6), 3), "e0": _sig(rng.uniform(0.1, 0.9), 2),
                                  "r": _sig(rng.uniform(0.02, 0.1), 2), "c": _sig(rng.uniform(0, 100), 3), "E": _sig(rng.uniform(0, 500), 3),
                                  "t": rng.randint(1, len(q) - 1)})(_rand_q(rng, True, 3, 10)),
     phrases=("Gross premium reserve", "Gross premium policy value"),
     examples=[_ex({"q": [0.1, 0.2, 0.5, 1.0], "i": 0.05, "S": 1000.0, "e0": 0.4, "r": 0.05, "c": 10.0, "E": 20.0, "t": 1}),
               _ex({"q": [0.02, 0.03, 0.05, 0.08, 0.15, 0.3, 1.0], "i": 0.04, "S": 100000.0, "e0": 0.6, "r": 0.03, "c": 50.0, "E": 200.0, "t": 2})],
     level="expert")


def thiele_endowment(mu, delta, n, S, t):
    _need(_isnum(mu) and 0 < mu < 1, "the force of mortality must lie between 0 and 1")
    _need(_isnum(delta) and 0 < delta < 1, "the force of interest must lie between 0 and 1")
    _need(_isnum(n) and 0 < n <= 100, "the term n must lie between 0 and 100 years")
    _pos("the sum insured S", S)
    _need(_isnum(t) and 0 <= t <= n, "t must lie between 0 and n")
    k = mu + delta

    def vals(m):
        e = math.exp(-k * m)
        abar = (1.0 - e) / k
        return mu * abar + e, abar
    A, a = vals(n)
    P = S * A / a
    if t == n:
        return _out(P=P, A_bar=A, a_bar=a, V=S)
    At, at = vals(n - t)
    return _out(P=P, A_bar=A, a_bar=a, V=S * At - P * at)


def _ref_thiele_endowment(mu, delta, n, S, t):
    from scipy.integrate import solve_ivp
    a = _quad(lambda s: math.exp(-(mu + delta) * s), 0.0, n)
    A = _quad(lambda s: math.exp(-(mu + delta) * s) * mu, 0.0, n) + math.exp(-(mu + delta) * n)
    P = S * A / a
    if t == n:
        return {"P": P, "A_bar": A, "a_bar": a, "V": S}
    sol = solve_ivp(lambda s, y: [delta * y[0] + P - mu * (S - y[0])], (n, t), [S], method="DOP853", rtol=1e-13, atol=1e-12 * S)
    return {"P": P, "A_bar": A, "a_bar": a, "V": float(sol.y[0, -1])}


_add("act_thiele_endowment",
     name="Thiele's equation: continuous endowment insurance reserve (constant forces)", topic="reserves / Thiele's differential equation",
     purpose="The continuous net premium rate and the reserve at time t of an n-year endowment insurance of S paid at the moment of death or at maturity, with a constant force of mortality mu and a constant force of interest delta (the solution of Thiele's equation dV/dt = delta V + P - mu (S - V), V(n) = S).",
     inputs=[_numin("mu", "constant force of mortality per year"), _numin("delta", "constant force of interest per year"),
             _numin("n", "term in years"), _numin("S", "sum insured"), _numin("t", "time since issue in years")],
     outputs=[_o("P", "continuous annual net premium rate"), _o("A_bar", "endowment insurance Ā_(x:n)"),
              _o("a_bar", "continuous temporary annuity ā_(x:n)"), _o("V", "reserve at time t")],
     primary="V", conditions=["0 < mu, delta < 1", "0 < n <= 100", "0 <= t <= n"],
     method="Ā = mu/(mu+delta)(1 - e^(-(mu+delta)n)) + e^(-(mu+delta)n); ā = (1 - e^(-(mu+delta)n))/(mu+delta); P = S Ā/ā; tV = S Ā_(n-t) - P ā_(n-t)",
     latex=r"\frac{d\,{}_tV}{dt} = \delta\,{}_tV + \bar P - \mu\left(S - {}_tV\right),\quad {}_nV = S",
     run=thiele_endowment, reference=_ref_thiele_endowment,
     wrong=lambda mu, delta, n, S, t: (lambda o: dict(o, V=o["V"] * math.exp(-mu * (n - t))))(thiele_endowment(mu, delta, n, S, t)),
     rand=lambda rng: (lambda n: {"mu": _sig(_lu(rng, 0.002, 0.1), 3), "delta": _sig(rng.uniform(0.01, 0.1), 3), "n": n, "S": _sig(_lu(rng, 1e3, 1e6), 3),
                                  "t": _sig(rng.uniform(0.05, 0.95) * n, 3)})(float(rng.randint(5, 40))),
     phrases=("Thiele's equation for endowment insurance", "Continuous endowment reserve"),
     examples=[_ex({"mu": 0.02, "delta": 0.05, "n": 20.0, "S": 1000.0, "t": 10.0}), _ex({"mu": 0.05, "delta": 0.04, "n": 10.0, "S": 50000.0, "t": 3.0})],
     level="expert")


def makeham_continuous(A, B, c, x, i, S, t):
    _mk_check(A, B, c, x, t)
    _rate(i)
    _pos("the sum insured S", S)
    delta = math.log1p(i)
    a0 = _mk_integral(A, B, c, x, delta)
    at = _mk_integral(A, B, c, x + t, delta)
    Abar = 1.0 - delta * a0
    P = S * Abar / a0
    return _out(a_bar=a0, A_bar=Abar, P=P, V=S * (1.0 - at / a0))


def _ref_makeham_continuous(A, B, c, x, i, S, t):
    delta = math.log(1.0 + i)
    lc = math.log(c)

    def both(age):                                    # (annuity, insurance) by scipy quad on 40 panels
        end = 1.0
        while _mk_tp(A, B, c, age, end) > 1e-30 and end < 2000:
            end *= 1.5
        pts = [end * k / 40.0 for k in range(41)]
        surv = lambda s: math.exp(-delta * s - A * s - B * c ** age * math.expm1(s * lc) / lc)
        a = sum(_quad(surv, u, w) for u, w in zip(pts[:-1], pts[1:]))
        ins = sum(_quad(lambda s: surv(s) * (A + B * c ** (age + s)), u, w) for u, w in zip(pts[:-1], pts[1:]))
        return a, ins
    a0, Ab = both(x)
    P = S * Ab / a0
    at, At = both(x + t)
    return {"a_bar": a0, "A_bar": Ab, "P": P, "V": S * At - P * at if t > 0 else 0.0}


_add("act_makeham_continuous_whole_life",
     name="Continuous whole life insurance, premium and reserve under Makeham's law", topic="reserves / Thiele's differential equation",
     purpose="The continuous whole life annuity, the insurance paid at the moment of death, the continuous net premium rate and the reserve at time t for a life aged x following Makeham's law, at a constant interest rate.",
     inputs=[_numin("A", "Makeham's A (per year)"), _numin("B", "Makeham's B (per year)"), _numin("c", "Makeham's c (> 1)"),
             _numin("x", "age at issue (years)"), _ratein(), _numin("S", "sum insured"), _numin("t", "time since issue in years")],
     outputs=[_o("a_bar", "continuous whole life annuity ā_x"), _o("A_bar", "insurance payable at death Ā_x"),
              _o("P", "continuous annual net premium rate"), _o("V", "reserve at time t")],
     primary="V", conditions=["Makeham parameters 0 <= A < 0.1, 0 < B < 0.1, 1 < c < 1.5", "0 <= x <= 120", "0 <= t <= 100", "0 < i < 1"],
     method="ā_x = integral of e^(-delta s) s_p_x (Gauss-Legendre); Ā_x = 1 - delta ā_x; P = S Ā_x/ā_x; tV = S (1 - ā_(x+t)/ā_x)",
     latex=r"\bar a_x = \int_0^\infty e^{-\delta s}{}_sp_x\,ds,\quad \bar A_x = 1-\delta\bar a_x,\quad {}_t\bar V = S\left(1-\frac{\bar a_{x+t}}{\bar a_x}\right)",
     run=makeham_continuous, reference=_ref_makeham_continuous,
     wrong=lambda A, B, c, x, i, S, t: (lambda o: dict(o, A_bar=1 - i * o["a_bar"], P=S * (1 - i * o["a_bar"]) / o["a_bar"], V=o["V"] * (1 + i)))(makeham_continuous(A, B, c, x, i, S, t)),
     rand=lambda rng: {"A": _sig(rng.uniform(1e-4, 2e-3), 3), "B": _sig(_lu(rng, 2e-6, 5e-5), 3), "c": _sig(rng.uniform(1.07, 1.13), 4),
                       "x": float(rng.randint(20, 80)), "i": _rand_i(rng), "S": _sig(_lu(rng, 1e3, 1e6), 3), "t": _sig(rng.uniform(1, 30), 3)},
     phrases=("Continuous whole life under Makeham", "Makeham continuous whole life insurance"),
     examples=[_ex({"A": 0.00022, "B": 2.7e-06, "c": 1.124, "x": 50.0, "i": 0.05, "S": 100000.0, "t": 10.0}),
               _ex({"A": 0.0007, "B": 5e-05, "c": 1.09, "x": 40.0, "i": 0.04, "S": 1000.0, "t": 5.0})],
     level="expert")


# ================================================================================================ SEVERAL LIVES AND DECREMENTS
def joint_life(qx, qy, i):
    _q_check(qx, whole=True)
    _q_check(qy, whole=True)
    _rate(i)
    v = 1.0 / (1.0 + i)
    d = i * v
    px, py = _kp(qx), _kp(qy)
    axy = ax = ay = 0.0
    for k in range(max(len(qx), len(qy))):
        sx = px[k] if k < len(px) else 0.0
        sy = py[k] if k < len(py) else 0.0
        axy += v ** k * sx * sy
        ax += v ** k * sx
        ay += v ** k * sy
    last = ax + ay - axy
    return _out(a_joint=axy, a_last=last, A_joint=1.0 - d * axy, A_last=1.0 - d * last)


def _ref_joint_life(qx, qy, i):
    import numpy as np
    def dist(q):
        kp = np.concatenate([[1.0], np.cumprod(1.0 - np.array(q, dtype=float))])
        return np.array([kp[k] * q[k] for k in range(len(q))])
    fx, fy = dist(qx), dist(qy)
    v = 1.0 / (1.0 + i)
    J = np.outer(fx, fy)                                     # P(Kx = j, Ky = k), independent lives
    mn = np.minimum.outer(np.arange(len(qx)), np.arange(len(qy)))
    mx = np.maximum.outer(np.arange(len(qx)), np.arange(len(qy)))
    ad = lambda K: (1.0 - v ** (K + 1)) / (1.0 - v)
    return {"a_joint": float(np.sum(J * ad(mn))), "a_last": float(np.sum(J * ad(mx))),
            "A_joint": float(np.sum(J * v ** (mn + 1))), "A_last": float(np.sum(J * v ** (mx + 1)))}


_add("act_joint_life_last_survivor",
     name="Joint life and last survivor annuities and insurances", topic="multiple lives / joint life and last survivor",
     purpose="The joint life and last survivor annuities-due and whole life insurances (end of year of the first or second death) for two independent lives aged x and y with their own life tables.",
     inputs=[_vecin("qx", "death probabilities of the first life from age x, ending with 1 (a list)", 1, 130),
             _vecin("qy", "death probabilities of the second life from age y, ending with 1 (a list)", 1, 130), _ratein()],
     outputs=[_o("a_joint", "joint life annuity-due ä_xy"), _o("a_last", "last survivor annuity-due"),
              _o("A_joint", "joint life insurance A_xy"), _o("A_last", "last survivor insurance")],
     primary="a_joint", conditions=["both tables close", "independent lives", "0 < i < 1"],
     method="ä_xy = sum v^k k_p_x k_p_y; last survivor ä = ä_x + ä_y - ä_xy; A = 1 - d ä",
     latex=r"\ddot a_{xy} = \sum_{k\ge0} v^k\,{}_kp_x\,{}_kp_y,\quad \ddot a_{\overline{xy}} = \ddot a_x + \ddot a_y - \ddot a_{xy},\quad A = 1 - d\,\ddot a",
     run=joint_life, reference=_ref_joint_life,
     wrong=lambda qx, qy, i: (lambda o: dict(o, a_last=o["a_last"] - o["a_joint"], A_last=1 - i / (1 + i) * (o["a_last"] - o["a_joint"])))(joint_life(qx, qy, i)),
     rand=lambda rng: {"qx": _rand_q(rng, True, 3, 10), "qy": _rand_q(rng, True, 3, 10), "i": _rand_i(rng)},
     phrases=("Joint life and last survivor", "Joint life annuity", "Last survivor annuity"),
     examples=[_ex({"qx": [0.1, 0.2, 0.5, 1.0], "qy": [0.05, 0.1, 0.3, 0.6, 1.0], "i": 0.05}),
               _ex({"qx": [0.02, 0.03, 0.05, 0.08, 0.15, 0.3, 1.0], "qy": [0.01, 0.02, 0.04, 0.07, 0.12, 0.25, 0.5, 1.0], "i": 0.04})],
     level="expert")


def reversionary_annuity(qx, qy, i, P):
    _q_check(qx, whole=True)
    _q_check(qy, whole=True)
    _rate(i)
    _pos("the annual payment P", P)
    v = 1.0 / (1.0 + i)
    px, py = _kp(qx), _kp(qy)
    ay = axy = 0.0
    for k in range(1, len(py)):
        sx = px[k] if k < len(px) else 0.0
        ay += v ** k * py[k]
        axy += v ** k * sx * py[k]
    return _out(a_rev=ay - axy, EPV=P * (ay - axy))


def _ref_reversionary_annuity(qx, qy, i, P):
    import numpy as np
    v = 1.0 / (1.0 + i)
    kx = np.concatenate([[1.0], np.cumprod(1.0 - np.array(qx, dtype=float))])
    ky = np.concatenate([[1.0], np.cumprod(1.0 - np.array(qy, dtype=float))])
    tot = 0.0
    for k in range(1, len(ky)):                          # paid at k if (y) alive and (x) dead
        dead_x = 1.0 - (kx[k] if k < len(kx) else 0.0)
        tot += v ** k * ky[k] * dead_x
    return {"a_rev": tot, "EPV": P * tot}


_add("act_reversionary_annuity",
     name="Reversionary annuity", topic="multiple lives / reversionary annuities",
     purpose="The actuarial present value of an annuity-immediate of P a year paid to a life aged y after the death of a life aged x (a_(x|y) = a_y - a_xy), independent lives.",
     inputs=[_vecin("qx", "death probabilities of the life whose death starts the annuity, ending with 1 (a list)", 1, 130),
             _vecin("qy", "death probabilities of the annuitant from age y, ending with 1 (a list)", 1, 130), _ratein(), _numin("P", "annual payment")],
     outputs=[_o("a_rev", "reversionary annuity a_(x|y) per unit"), _o("EPV", "actuarial present value P a_(x|y)")],
     primary="a_rev", conditions=["both tables close", "independent lives", "0 < i < 1"],
     method="a_(x|y) = a_y - a_xy = sum_(k>=1) v^k k_p_y (1 - k_p_x)",
     latex=r"a_{x|y} = a_y - a_{xy} = \sum_{k\ge1} v^k\,{}_kp_y\left(1-{}_kp_x\right)",
     run=reversionary_annuity, reference=_ref_reversionary_annuity,
     wrong=lambda qx, qy, i, P: (lambda o: dict(o, a_rev=o["a_rev"] * (1 + i), EPV=o["EPV"] * (1 + i)))(reversionary_annuity(qx, qy, i, P)),
     rand=lambda rng: {"qx": _rand_q(rng, True, 3, 10), "qy": _rand_q(rng, True, 3, 10), "i": _rand_i(rng), "P": _sig(_lu(rng, 100, 1e5), 3)},
     phrases=("Reversionary annuity", "Survivor's reversionary annuity"),
     examples=[_ex({"qx": [0.1, 0.2, 0.5, 1.0], "qy": [0.05, 0.1, 0.3, 0.6, 1.0], "i": 0.05, "P": 1000.0}),
               _ex({"qx": [0.02, 0.03, 0.05, 0.08, 0.15, 0.3, 1.0], "qy": [0.01, 0.02, 0.04, 0.07, 0.12, 0.25, 0.5, 1.0], "i": 0.04, "P": 5000.0})],
     level="advanced")


def multiple_to_single(q1, q2):
    _need(_isnum(q1) and _isnum(q2) and q1 > 0 and q2 > 0, "both decrement probabilities must be positive")
    qt = q1 + q2
    _need(qt < 1, "the total decrement probability must lie below 1")
    pt = 1.0 - qt
    return _out(q1_single=1.0 - pt ** (q1 / qt), q2_single=1.0 - pt ** (q2 / qt), q_total=qt)


def _ref_multiple_to_single(q1, q2):
    qt = q1 + q2
    i1 = _quad(lambda t: q1 / (1.0 - t * qt), 0.0, 1.0)
    i2 = _quad(lambda t: q2 / (1.0 - t * qt), 0.0, 1.0)
    return {"q1_single": -math.expm1(-i1), "q2_single": -math.expm1(-i2), "q_total": qt}


_add("act_multiple_decrement_to_single",
     name="Associated single decrement rates from a multiple decrement table (UDD)", topic="multiple decrements / associated single decrement",
     purpose="The associated single decrement probabilities q'(1) and q'(2) from the two decrement probabilities of a double decrement table, assuming each decrement is uniformly distributed within the year in the multiple decrement table.",
     inputs=[_numin("q1", "probability of leaving by decrement 1 within the year q(1)"), _numin("q2", "probability of leaving by decrement 2 within the year q(2)")],
     outputs=[_o("q1_single", "associated single decrement probability q'(1)"), _o("q2_single", "associated single decrement probability q'(2)"),
              _o("q_total", "total decrement probability q(tau)")],
     primary="q1_single", conditions=["q1, q2 > 0", "q1 + q2 < 1", "UDD in the multiple decrement table"],
     method="q'(j) = 1 - (p(tau))^(q(j)/q(tau))",
     latex=r"q'^{(j)}_x = 1 - \left(p^{(\tau)}_x\right)^{q^{(j)}_x/q^{(\tau)}_x}",
     run=multiple_to_single, reference=_ref_multiple_to_single,
     wrong=lambda q1, q2: _out(q1_single=q1 * (1 + q2), q2_single=q2 * (1 + q1), q_total=q1 + q2),
     rand=lambda rng: {"q1": _sig(_lu(rng, 0.005, 0.4), 3), "q2": _sig(_lu(rng, 0.005, 0.4), 3)},
     phrases=("Associated single decrement rates", "Multiple decrement to single decrement"),
     examples=[_ex({"q1": 0.1, "q2": 0.2}), _ex({"q1": 0.02, "q2": 0.15})],
     level="advanced")


def single_to_multiple(q1s, q2s):
    _frac("q'(1)", q1s, lo_open=True, hi_open=True)
    _frac("q'(2)", q2s, lo_open=True, hi_open=True)
    a = q1s * (1.0 - q2s / 2.0)
    b = q2s * (1.0 - q1s / 2.0)
    return _out(q1=a, q2=b, q_total=a + b)


def _ref_single_to_multiple(q1s, q2s):
    a = _quad(lambda t: (1.0 - t * q2s) * q1s, 0.0, 1.0)
    b = _quad(lambda t: (1.0 - t * q1s) * q2s, 0.0, 1.0)
    return {"q1": a, "q2": b, "q_total": 1.0 - (1.0 - q1s) * (1.0 - q2s)}


_add("act_single_to_multiple_decrement",
     name="Multiple decrement probabilities from single decrement rates (UDD)", topic="multiple decrements / multiple decrement tables",
     purpose="The double decrement probabilities q(1) and q(2) from the associated single decrement probabilities q'(1) and q'(2), each uniformly distributed within the year in its single decrement table.",
     inputs=[_numin("q1s", "associated single decrement probability q'(1)"), _numin("q2s", "associated single decrement probability q'(2)")],
     outputs=[_o("q1", "probability of leaving by decrement 1, q(1)"), _o("q2", "probability of leaving by decrement 2, q(2)"),
              _o("q_total", "total decrement probability q(tau)")],
     primary="q1", conditions=["0 < q'(1), q'(2) < 1", "UDD in each single decrement table"],
     method="q(1) = q'(1) (1 - q'(2)/2), q(2) = q'(2) (1 - q'(1)/2)",
     latex=r"q^{(1)}_x = q'^{(1)}_x\left(1-\tfrac12 q'^{(2)}_x\right),\quad q^{(2)}_x = q'^{(2)}_x\left(1-\tfrac12 q'^{(1)}_x\right)",
     run=single_to_multiple, reference=_ref_single_to_multiple,
     wrong=lambda q1s, q2s: _out(q1=q1s * (1 - q2s), q2=q2s * (1 - q1s), q_total=q1s + q2s - 2 * q1s * q2s),
     rand=lambda rng: {"q1s": _sig(_lu(rng, 0.01, 0.6), 3), "q2s": _sig(_lu(rng, 0.01, 0.6), 3)},
     phrases=("Multiple decrement probabilities", "Single decrement to multiple decrement"),
     examples=[_ex({"q1s": 0.1, "q2s": 0.2}), _ex({"q1s": 0.05, "q2s": 0.3})],
     level="advanced")


# ================================================================================================ LOSS MODELS: frequency
def poisson_frequency(lam, k):
    _need(_isnum(lam) and 0 < lam <= 500, "the Poisson mean must lie between 0 and 500")
    _whole("the number of claims k", k, 0)
    _need(k <= 2000, "k must be at most 2000")
    pmf = math.exp(-lam + k * math.log(lam) - math.lgamma(k + 1.0))
    cdf = 0.0
    for j in range(k + 1):
        cdf += math.exp(-lam + j * math.log(lam) - math.lgamma(j + 1.0))
    return _out(pmf=pmf, cdf=min(cdf, 1.0), mean=lam, var=lam)


def _ref_poisson_frequency(lam, k):
    from scipy.stats import poisson
    return {"pmf": float(poisson.pmf(k, lam)), "cdf": float(poisson.cdf(k, lam)), "mean": float(poisson.mean(lam)), "var": float(poisson.var(lam))}


_add("act_poisson_frequency",
     name="Poisson claim frequency probabilities", topic="loss models / frequency distributions",
     purpose="The probability of exactly k claims and of at most k claims when the number of claims is Poisson with mean lambda.",
     inputs=[_numin("lam", "Poisson mean number of claims lambda"), _intin("k", "number of claims", 0, 2000)],
     outputs=[_o("pmf", "P(N = k)"), _o("cdf", "P(N <= k)"), _o("mean", "mean number of claims"), _o("var", "variance of the number of claims")],
     primary="pmf", conditions=["0 < lambda <= 500", "k >= 0"],
     method="P(N = k) = e^(-lambda) lambda^k / k!",
     latex=r"P(N=k) = \frac{e^{-\lambda}\lambda^k}{k!}",
     run=poisson_frequency, reference=_ref_poisson_frequency,
     wrong=lambda lam, k: _out(pmf=math.exp(-lam) * lam ** k / math.factorial(max(k - 1, 0)), cdf=0.0, mean=lam, var=lam),
     rand=lambda rng: (lambda lam: {"lam": lam, "k": rng.randint(0, int(lam * 2 + 3))})(_sig(_lu(rng, 0.1, 40), 3)),
     phrases=("Poisson claim frequency", "Poisson frequency probabilities", "Poisson claim count"),
     aliases={"lambda": "lam"},
     examples=[_ex({"lam": 2.0, "k": 3}), _ex({"lam": 0.5, "k": 0})],
     level="beginner",
     extra_q=[{"q": "Poisson claim count: lambda = 2, k = 3", "like": 0}])


def negative_binomial_frequency(r, beta, k):
    _need(_isnum(r) and 0 < r <= 1000, "r must lie between 0 and 1000")
    _need(_isnum(beta) and 0 < beta <= 1000, "beta must lie between 0 and 1000")
    _whole("the number of claims k", k, 0)
    _need(k <= 2000, "k must be at most 2000")

    def pk(j):
        return math.exp(math.lgamma(r + j) - math.lgamma(r) - math.lgamma(j + 1.0) - r * math.log1p(beta) + j * (math.log(beta) - math.log1p(beta)))
    cdf = 0.0
    for j in range(k + 1):
        cdf += pk(j)
    return _out(pmf=pk(k), cdf=min(cdf, 1.0), mean=r * beta, var=r * beta * (1.0 + beta))


def _ref_negative_binomial_frequency(r, beta, k):
    from scipy.stats import nbinom
    p = 1.0 / (1.0 + beta)
    return {"pmf": float(nbinom.pmf(k, r, p)), "cdf": float(nbinom.cdf(k, r, p)), "mean": float(nbinom.mean(r, p)), "var": float(nbinom.var(r, p))}


_add("act_negative_binomial_frequency",
     name="Negative binomial claim frequency probabilities", topic="loss models / frequency distributions",
     purpose="The probability of exactly k claims and of at most k claims, with the mean and variance, for a negative binomial claim count with parameters r and beta (mean r beta).",
     inputs=[_numin("r", "negative binomial r"), _numin("beta", "negative binomial beta"), _intin("k", "number of claims", 0, 2000)],
     outputs=[_o("pmf", "P(N = k)"), _o("cdf", "P(N <= k)"), _o("mean", "mean r beta"), _o("var", "variance r beta (1 + beta)")],
     primary="pmf", conditions=["r, beta > 0", "k >= 0"],
     method="P(N = k) = Gamma(r+k)/(Gamma(r) k!) (1/(1+beta))^r (beta/(1+beta))^k",
     latex=r"P(N=k) = \binom{r+k-1}{k}\left(\frac{1}{1+\beta}\right)^{r}\left(\frac{\beta}{1+\beta}\right)^{k}",
     run=negative_binomial_frequency, reference=_ref_negative_binomial_frequency,
     wrong=lambda r, beta, k: _out(pmf=math.exp(math.lgamma(r + k) - math.lgamma(r) - math.lgamma(k + 1.0)) * (beta / (1 + beta)) ** r * (1 / (1 + beta)) ** k, cdf=0.0, mean=r * beta, var=r * beta),
     rand=lambda rng: {"r": _sig(_lu(rng, 0.3, 20), 3), "beta": _sig(_lu(rng, 0.1, 5), 3), "k": rng.randint(0, 15)},
     phrases=("Negative binomial claim frequency", "Negative binomial frequency probabilities"),
     examples=[_ex({"r": 2.0, "beta": 1.5, "k": 3}), _ex({"r": 0.5, "beta": 4.0, "k": 0})],
     level="intermediate")


def compound_moments(EN, VarN, EX, VarX):
    _nonneg("the moments", EN, VarN, VarX)
    _fin(EX)
    return _out(ES=EN * EX, VarS=EN * VarX + VarN * EX * EX)


def _ref_compound_moments(EN, VarN, EX, VarX):
    ES2 = EN * (VarX + EX * EX) + (VarN + EN * EN - EN) * EX * EX       # E[S^2] = E[N]E[X^2] + E[N(N-1)]E[X]^2
    return {"ES": EN * EX, "VarS": ES2 - (EN * EX) ** 2}


_add("act_compound_distribution_moments",
     name="Mean and variance of aggregate losses (compound distribution)", topic="loss models / aggregate losses",
     purpose="The mean and variance of aggregate losses S = X_1 + ... + X_N from the mean and variance of the claim count N and of the claim severity X (independent, identically distributed severities).",
     inputs=[_numin("EN", "mean number of claims E[N]"), _numin("VarN", "variance of the number of claims"),
             _numin("EX", "mean claim severity E[X]"), _numin("VarX", "variance of the claim severity")],
     outputs=[_o("ES", "mean aggregate loss E[N] E[X]"), _o("VarS", "variance E[N] Var X + Var N E[X]^2")],
     primary="VarS", conditions=["moments non-negative", "N and the X's independent"],
     method="E[S] = E[N] E[X]; Var S = E[N] Var X + Var N (E[X])^2",
     latex=r"E[S] = E[N]\,E[X],\quad \mathrm{Var}(S) = E[N]\,\mathrm{Var}(X) + \mathrm{Var}(N)\,E[X]^2",
     run=compound_moments, reference=_ref_compound_moments,
     wrong=lambda EN, VarN, EX, VarX: _out(ES=EN * EX, VarS=EN * VarX + VarN * VarX),
     rand=lambda rng: {"EN": _sig(_lu(rng, 0.5, 100), 3), "VarN": _sig(_lu(rng, 0.5, 200), 3), "EX": _sig(_lu(rng, 100, 1e5), 3), "VarX": _sig(_lu(rng, 1e3, 1e9), 3)},
     phrases=("Aggregate loss mean and variance", "Compound distribution moments"),
     examples=[_ex({"EN": 3.0, "VarN": 3.0, "EX": 1000.0, "VarX": 250000.0}), _ex({"EN": 10.0, "VarN": 25.0, "EX": 500.0, "VarX": 40000.0})],
     level="beginner",
     weak="the variance through E[S^2] = E[N]E[X^2] + E[N(N-1)]E[X]^2")


def compound_poisson_moments(lam, m1, m2, m3):
    _need(_isnum(lam) and lam > 0, "lambda must be positive")
    _need(_isnum(m1) and _isnum(m2) and _isnum(m3) and m1 > 0 and m2 > 0 and m3 > 0, "the severity moments must be positive")
    _need(m2 >= m1 * m1 * (1 - 1e-12), "the second moment cannot be below the squared mean")
    var = lam * m2
    return _out(mean=lam * m1, var=var, skew=lam * m3 / var ** 1.5)


def _ref_compound_poisson_moments(lam, m1, m2, m3):
    import sympy as sp
    t = sp.symbols("t")
    L = sp.nsimplify(lam)
    M = 1 + sp.nsimplify(m1) * t + sp.nsimplify(m2) * t ** 2 / 2 + sp.nsimplify(m3) * t ** 3 / 6
    K = L * (M - 1)                                 # cumulant generating function of a compound Poisson sum
    k1, k2, k3 = [sp.diff(K, t, n).subs(t, 0) for n in (1, 2, 3)]
    return {"mean": float(k1), "var": float(k2), "skew": float(k3 / k2 ** sp.Rational(3, 2))}


_add("act_compound_poisson_moments",
     name="Compound Poisson mean, variance and skewness", topic="loss models / aggregate losses",
     purpose="The mean, variance and coefficient of skewness of aggregate losses when the claim count is Poisson(lambda), from the first three raw moments of the claim severity.",
     inputs=[_numin("lam", "Poisson mean number of claims lambda"), _numin("m1", "E[X]"), _numin("m2", "E[X^2]"), _numin("m3", "E[X^3]")],
     outputs=[_o("mean", "lambda E[X]"), _o("var", "lambda E[X^2]"), _o("skew", "lambda E[X^3] / (lambda E[X^2])^(3/2)")],
     primary="var", conditions=["lambda > 0", "moments positive", "E[X^2] >= E[X]^2"],
     method="cumulants of a compound Poisson sum: kappa_n = lambda E[X^n]",
     latex=r"\kappa_n(S) = \lambda\,E[X^n],\quad \gamma_S = \frac{\lambda E[X^3]}{(\lambda E[X^2])^{3/2}}",
     run=compound_poisson_moments, reference=_ref_compound_poisson_moments,
     wrong=lambda lam, m1, m2, m3: _out(mean=lam * m1, var=lam * (m2 - m1 * m1), skew=lam * m3 / (lam * (m2 - m1 * m1)) ** 1.5 if m2 > m1 * m1 else 0.0),
     rand=lambda rng: (lambda m1, cv: {"lam": _sig(_lu(rng, 0.5, 100), 3), "m1": m1, "m2": _sig(m1 * m1 * (1 + cv * cv), 4), "m3": _sig(m1 ** 3 * (1 + 3 * cv * cv) * rng.uniform(1.2, 3), 4)})(_sig(_lu(rng, 1, 1000), 3), rng.uniform(0.3, 2)),
     phrases=("Compound Poisson moments", "Compound Poisson mean and variance"),
     aliases={"lambda": "lam"},
     examples=[_ex({"lam": 3.0, "m1": 2.0, "m2": 6.0, "m3": 24.0}), _ex({"lam": 10.0, "m1": 100.0, "m2": 20000.0, "m3": 6000000.0})],
     level="intermediate")


def individual_risk_model(q, b):
    _need(len(q) == len(b), "give one benefit for every probability")
    _need(len(q) <= 200, "at most 200 policies")
    m = v = 0.0
    p0 = 1.0
    for qi, bi in zip(q, b):
        _need(_isnum(qi) and 0 <= qi <= 1, "every claim probability must lie between 0 and 1")
        _need(_isnum(bi) and bi > 0, "every benefit must be positive")
        m += qi * bi
        v += qi * (1.0 - qi) * bi * bi
        p0 *= 1.0 - qi
    return _out(mean=m, var=v, prob_no_claim=p0)


def _ref_individual_risk_model(q, b):
    dist = {0.0: 1.0}
    for qi, bi in zip(q, b):                         # exact distribution by convolution
        new = {}
        for s, p in dist.items():
            new[s] = new.get(s, 0.0) + p * (1.0 - qi)
            new[s + bi] = new.get(s + bi, 0.0) + p * qi
        dist = new
    mean = sum(s * p for s, p in dist.items())
    var = sum((s - mean) ** 2 * p for s, p in dist.items())
    return {"mean": mean, "var": var, "prob_no_claim": dist.get(0.0, 0.0)}


_add("act_individual_risk_model",
     name="Individual risk model: aggregate mean and variance", topic="loss models / individual risk model",
     purpose="The mean and variance of total claims, and the probability of no claim, for a portfolio of independent policies each paying a fixed benefit b_i with probability q_i.",
     inputs=[_vecin("q", "claim probabilities of the policies (a list)", 1, 200), _vecin("b", "benefit amounts of the policies (a list)", 1, 200)],
     outputs=[_o("mean", "expected total claims sum q b"), _o("var", "variance sum q (1-q) b^2"), _o("prob_no_claim", "probability of no claim at all")],
     primary="var", conditions=["one benefit per probability", "0 <= q <= 1", "b > 0"],
     method="E[S] = sum q_i b_i; Var S = sum q_i (1 - q_i) b_i^2; P(S = 0) = prod (1 - q_i)",
     latex=r"E[S] = \sum_i q_i b_i,\quad \mathrm{Var}(S) = \sum_i q_i(1-q_i)\,b_i^2",
     run=individual_risk_model, reference=_ref_individual_risk_model,
     wrong=lambda q, b: (lambda o: dict(o, var=sum(qi * bi * bi for qi, bi in zip(q, b))))(individual_risk_model(q, b)),
     rand=lambda rng: (lambda n: {"q": [_sig(_lu(rng, 0.001, 0.3), 3) for _ in range(n)], "b": [float(rng.choice([1000, 2000, 5000, 10000, 2500])) for _ in range(n)]})(rng.randint(1, 12)),
     phrases=("Individual risk model", "Aggregate claims of individual policies"),
     examples=[_ex({"q": [0.01, 0.02, 0.05], "b": [10000.0, 5000.0, 2000.0]}), _ex({"q": [0.1, 0.1, 0.2, 0.05], "b": [1000.0, 1000.0, 3000.0, 5000.0]})],
     level="intermediate")


def normal_approx_aggregate(mean, var, s):
    _fin(mean, s)
    _pos("the variance", var)
    z = (s - mean) / math.sqrt(var)
    return _out(prob_exceed=0.5 * math.erfc(z / math.sqrt(2.0)), z=z)


def _ref_normal_approx_aggregate(mean, var, s):
    from scipy.stats import norm
    return {"prob_exceed": float(norm.sf(s, loc=mean, scale=math.sqrt(var))), "z": float((s - mean) / norm(loc=0, scale=math.sqrt(var)).std())}


_add("act_normal_approximation_aggregate",
     name="Normal approximation to aggregate losses", topic="loss models / aggregate losses",
     purpose="The probability that aggregate losses exceed an amount s under the normal approximation with the given mean and variance.",
     inputs=[_numin("mean", "mean of aggregate losses"), _numin("var", "variance of aggregate losses"), _numin("s", "threshold amount")],
     outputs=[_o("prob_exceed", "P(S > s) by the normal approximation"), _o("z", "standardized threshold (s - mean)/sd")],
     primary="prob_exceed", conditions=["variance > 0"],
     method="P(S > s) = 1 - Phi((s - E[S])/sd(S))",
     latex=r"P(S>s) \approx 1-\Phi\!\left(\frac{s-E[S]}{\sqrt{\mathrm{Var}(S)}}\right)",
     run=normal_approx_aggregate, reference=_ref_normal_approx_aggregate,
     wrong=lambda mean, var, s: _out(prob_exceed=0.5 * math.erfc((s - mean) / var / math.sqrt(2.0)), z=(s - mean) / var),
     rand=lambda rng: (lambda m, sd: {"mean": m, "var": _sig(sd * sd, 4), "s": _sig(m + sd * rng.uniform(-2.5, 2.5), 4)})(_sig(_lu(rng, 100, 1e6), 4), _lu(rng, 10, 1e5)),
     phrases=("Normal approximation to aggregate losses", "Normal approximation of aggregate claims"),
     examples=[_ex({"mean": 3000.0, "var": 1000000.0, "s": 5000.0}), _ex({"mean": 100000.0, "var": 400000000.0, "s": 120000.0})],
     level="beginner")


def _sev_check(f):
    _need(len(f) >= 2, "give the severity probabilities for amounts 0, 1, 2, ...")
    tot = 0.0
    for p in f:
        _need(_isnum(p) and 0 <= p <= 1, "every severity probability must lie between 0 and 1")
        tot += p
    _need(abs(tot - 1.0) <= 1e-9, "the severity probabilities must add up to 1")


def _panjer(a, b, g0, f, smax):
    g = [g0]
    c = 1.0 - a * f[0]
    for s in range(1, smax + 1):
        acc = 0.0
        for j in range(1, min(s, len(f) - 1) + 1):
            acc += (a + b * j / s) * f[j] * g[s - j]
        g.append(acc / c)
    return g


def panjer_poisson(lam, f, smax):
    _need(_isnum(lam) and 0 < lam <= 200, "lambda must lie between 0 and 200")
    _sev_check(f)
    _whole("the largest amount smax", smax, 0)
    _need(smax <= 300, "smax must be at most 300")
    g = _panjer(0.0, lam, math.exp(lam * (f[0] - 1.0)), f, smax)
    tot = 0.0
    for v in g:
        tot += v
    return _out(g=g, F_smax=min(tot, 1.0))


def _ref_convolution(pN, f, smax):
    """Reference helper: g = sum_n P(N = n) f^(*n), truncated to amounts 0..smax (numpy convolution)."""
    import numpy as np
    f = np.array(f, dtype=float)
    g = np.zeros(smax + 1)
    conv = np.zeros(smax + 1)
    conv[0] = 1.0
    for n, pn in enumerate(pN):
        if n > 0:
            conv = np.convolve(conv, f)[:smax + 1]
        g += pn * conv
    return g


def _ref_panjer_poisson(lam, f, smax):
    from scipy.stats import poisson
    nmax = int(lam + 40 * math.sqrt(lam) + 60)
    g = _ref_convolution([float(poisson.pmf(n, lam)) for n in range(nmax + 1)], f, smax)
    return {"g": [float(x) for x in g], "F_smax": float(min(g.sum(), 1.0))}


_add("act_panjer_poisson",
     name="Panjer recursion for a compound Poisson distribution", topic="loss models / Panjer recursion",
     purpose="The probabilities of aggregate losses 0, 1, ..., smax (in monetary units) for a Poisson(lambda) number of claims with claim severities on 0, 1, 2, ... units, by Panjer's recursion, and P(S <= smax).",
     inputs=[_numin("lam", "Poisson mean number of claims lambda"), _vecin("f", "severity probabilities of 0, 1, 2, ... units (a list adding to 1)", 2, 200),
             _intin("smax", "largest aggregate amount wanted", 0, 300)],
     outputs=[_o("g", "P(S = s) for s = 0..smax (a list)"), _o("F_smax", "P(S <= smax)")],
     primary="F_smax", conditions=["0 < lambda <= 200", "severity probabilities add to 1", "smax <= 300"],
     method="g_0 = e^(-lambda(1 - f_0)); g_s = (1/s) sum_(j=1..s) lambda j f_j g_(s-j)",
     latex=r"g_0 = e^{-\lambda(1-f_0)},\quad g_s = \frac{\lambda}{s}\sum_{j=1}^{s} j\,f_j\,g_{s-j}",
     run=panjer_poisson, reference=_ref_panjer_poisson,
     wrong=lambda lam, f, smax: (lambda g: _out(g=g, F_smax=min(sum(g), 1.0)))(_panjer(0.0, lam, math.exp(-lam), f, smax)),
     rand=lambda rng: (lambda w: {"lam": _sig(_lu(rng, 0.2, 8), 3), "f": [x / sum(w) for x in w], "smax": rng.randint(3, 25)})([float(rng.randint(0, 6)) for _ in range(rng.randint(2, 6))] + [1.0]),
     phrases=("Panjer recursion for compound Poisson", "Compound Poisson Panjer recursion"),
     aliases={"lambda": "lam"},
     examples=[_ex({"lam": 2.0, "f": [0.0, 0.5, 0.3, 0.2], "smax": 5}), _ex({"lam": 1.0, "f": [0.2, 0.4, 0.4], "smax": 4})],
     level="advanced",
     extra_q=[{"q": "Panjer recursion for compound Poisson: lam = 2, f = [0, 0.5, 0.3, 0.3], smax = 5", "outcome": "INVALID_INPUT"}])


def panjer_negative_binomial(r, beta, f, smax):
    _need(_isnum(r) and 0 < r <= 200, "r must lie between 0 and 200")
    _need(_isnum(beta) and 0 < beta <= 50, "beta must lie between 0 and 50")
    _sev_check(f)
    _whole("the largest amount smax", smax, 0)
    _need(smax <= 300, "smax must be at most 300")
    a = beta / (1.0 + beta)
    g = _panjer(a, (r - 1.0) * a, (1.0 + beta * (1.0 - f[0])) ** (-r), f, smax)
    tot = 0.0
    for v in g:
        tot += v
    return _out(g=g, F_smax=min(tot, 1.0))


def _ref_panjer_negative_binomial(r, beta, f, smax):
    from scipy.stats import nbinom
    p = 1.0 / (1.0 + beta)
    nmax = int(r * beta + 60.0 * math.sqrt(r * beta * (1.0 + beta)) + 100)
    g = _ref_convolution([float(nbinom.pmf(n, r, p)) for n in range(nmax + 1)], f, smax)
    return {"g": [float(x) for x in g], "F_smax": float(min(g.sum(), 1.0))}


_add("act_panjer_negative_binomial",
     name="Panjer recursion for a compound negative binomial distribution", topic="loss models / Panjer recursion",
     purpose="The probabilities of aggregate losses 0, 1, ..., smax units for a negative binomial (r, beta) number of claims with claim severities on 0, 1, 2, ... units, by Panjer's recursion, and P(S <= smax).",
     inputs=[_numin("r", "negative binomial r"), _numin("beta", "negative binomial beta"),
             _vecin("f", "severity probabilities of 0, 1, 2, ... units (a list adding to 1)", 2, 200), _intin("smax", "largest aggregate amount wanted", 0, 300)],
     outputs=[_o("g", "P(S = s) for s = 0..smax (a list)"), _o("F_smax", "P(S <= smax)")],
     primary="F_smax", conditions=["r, beta > 0", "severity probabilities add to 1", "smax <= 300"],
     method="(a, b, 0) class: a = beta/(1+beta), b = (r-1) a; g_0 = (1 + beta(1 - f_0))^(-r); g_s = [sum (a + b j/s) f_j g_(s-j)]/(1 - a f_0)",
     latex=r"g_s = \frac{1}{1-af_0}\sum_{j=1}^{s}\left(a+\frac{bj}{s}\right)f_j\,g_{s-j},\quad a = \frac{\beta}{1+\beta},\ b = (r-1)a",
     run=panjer_negative_binomial, reference=_ref_panjer_negative_binomial,
     wrong=lambda r, beta, f, smax: (lambda a: (lambda g: _out(g=g, F_smax=min(sum(g), 1.0)))(_panjer(a, r * a, (1 + beta * (1 - f[0])) ** (-r), f, smax)))(beta / (1 + beta)),
     rand=lambda rng: (lambda w: {"r": _sig(_lu(rng, 0.5, 6), 3), "beta": _sig(_lu(rng, 0.2, 3), 3), "f": [x / sum(w) for x in w], "smax": rng.randint(3, 25)})([float(rng.randint(0, 6)) for _ in range(rng.randint(2, 6))] + [1.0]),
     phrases=("Panjer recursion for compound negative binomial", "Compound negative binomial Panjer recursion"),
     examples=[_ex({"r": 2.0, "beta": 1.0, "f": [0.0, 0.5, 0.3, 0.2], "smax": 5}), _ex({"r": 0.5, "beta": 2.0, "f": [0.2, 0.4, 0.4], "smax": 4})],
     level="expert")


def stop_loss_compound_poisson(lam, f, d):
    _need(_isnum(lam) and 0 < lam <= 200, "lambda must lie between 0 and 200")
    _sev_check(f)
    _whole("the retention d", d, 0)
    _need(d <= 300, "the retention must be at most 300 units")
    ES = 0.0
    for j in range(len(f)):
        ES += j * f[j]
    ES *= lam
    if d == 0:
        return _out(premium=ES, ES=ES, limited=0.0)
    g = _panjer(0.0, lam, math.exp(lam * (f[0] - 1.0)), f, d - 1)
    F = 0.0
    lim = 0.0
    for s in range(d):
        F += g[s]
        lim += 1.0 - F
    return _out(premium=ES - lim, ES=ES, limited=lim)


def _ref_stop_loss_compound_poisson(lam, f, d):
    import numpy as np
    from scipy.stats import poisson
    mean_sev = sum(j * p for j, p in enumerate(f))
    var_s = lam * sum(j * j * p for j, p in enumerate(f))
    M = int(lam * mean_sev + 60 * math.sqrt(var_s) + 80 + d)
    nmax = int(lam + 40 * math.sqrt(lam) + 60)
    g = _ref_convolution([float(poisson.pmf(n, lam)) for n in range(nmax + 1)], f, M)
    s = np.arange(M + 1, dtype=float)
    return {"premium": float(np.sum(np.maximum(s - d, 0.0) * g)), "ES": float(np.sum(s * g)), "limited": float(np.sum(np.minimum(s, d) * g) + d * max(0.0, 1.0 - g.sum()))}


_add("act_stop_loss_premium",
     name="Stop-loss premium for a compound Poisson distribution", topic="loss models / stop-loss insurance",
     purpose="The net stop-loss premium E[(S - d)+] for a retention of d units, with E[S] and the limited expectation E[S ∧ d], when S is compound Poisson with claim severities on 0, 1, 2, ... units.",
     inputs=[_numin("lam", "Poisson mean number of claims lambda"), _vecin("f", "severity probabilities of 0, 1, 2, ... units (a list adding to 1)", 2, 200),
             _intin("d", "retention (whole number of units)", 0, 300)],
     outputs=[_o("premium", "net stop-loss premium E[(S - d)+]"), _o("ES", "expected aggregate loss E[S]"), _o("limited", "limited expectation E[S ∧ d]")],
     primary="premium", conditions=["0 < lambda <= 200", "severity probabilities add to 1", "whole-number retention"],
     method="E[(S - d)+] = E[S] - sum_(s=0..d-1) P(S > s), with P(S = s) from Panjer's recursion",
     latex=r"E[(S-d)_+] = E[S] - \sum_{s=0}^{d-1}\left(1 - F_S(s)\right)",
     run=stop_loss_compound_poisson, reference=_ref_stop_loss_compound_poisson,
     wrong=lambda lam, f, d: (lambda o: dict(o, premium=o["ES"] - d))(stop_loss_compound_poisson(lam, f, d)),
     rand=lambda rng: (lambda w, lam: {"lam": lam, "f": [x / sum(w) for x in w], "d": rng.randint(1, max(2, int(lam * len(w)) + 2))})([float(rng.randint(0, 6)) for _ in range(rng.randint(2, 6))] + [1.0], _sig(_lu(rng, 0.2, 6), 3)),
     phrases=("Stop-loss premium", "Net stop-loss premium"),
     aliases={"lambda": "lam"},
     examples=[_ex({"lam": 2.0, "f": [0.0, 0.5, 0.3, 0.2], "d": 3}), _ex({"lam": 1.0, "f": [0.2, 0.4, 0.4], "d": 2})],
     level="advanced")


# ================================================================================================ LOSS MODELS: severity and coverage
def _lev(dist, p1, p2, u):
    """limited expected value E[X ^ u] (u may be inf -> E[X])"""
    if dist == "exponential":
        return p1 if u == math.inf else p1 * (1.0 - math.exp(-u / p1))
    if dist == "pareto":
        a, th = p1, p2
        if u == math.inf:
            return th / (a - 1.0)
        return th / (a - 1.0) * (1.0 - (th / (th + u)) ** (a - 1.0))
    mu, sg = p1, p2
    m = math.exp(mu + sg * sg / 2.0)
    if u == math.inf:
        return m
    if u <= 0:
        return 0.0
    lu = math.log(u)
    return m * _norm_cdf((lu - mu - sg * sg) / sg) + u * (1.0 - _norm_cdf((lu - mu) / sg))


def _sf(dist, p1, p2, x):
    if x <= 0:
        return 1.0
    if dist == "exponential":
        return math.exp(-x / p1)
    if dist == "pareto":
        return (p2 / (p2 + x)) ** p1
    return 0.5 * math.erfc((math.log(x) - p1) / p2 / math.sqrt(2.0))


def _deductible(dist, p1, p2, d):
    _nonneg("the deductible d", d)
    mean = _lev(dist, p1, p2, math.inf)
    lev = _lev(dist, p1, p2, d)
    sfd = _sf(dist, p1, p2, d)
    _need(sfd > 1e-300, "the deductible is beyond the range of the losses")
    per_loss = mean - lev
    return _out(mean=mean, lev_d=lev, per_loss=per_loss, per_payment=per_loss / sfd, LER=lev / mean)


def _scipy_dist(dist, p1, p2):
    from scipy import stats
    if dist == "exponential":
        return stats.expon(scale=p1)
    if dist == "pareto":
        return stats.lomax(p1, scale=p2)
    return stats.lognorm(p2, scale=math.exp(p1))


def _sf_integral(D, a, b=math.inf):
    """Reference helper: integral of a scipy distribution's survival function from a to b (b may be infinite), by
    scipy quad on panels whose width doubles from the distribution's scale, so heavy tails and large scales keep
    full relative accuracy."""
    h = float(D.std()) if math.isfinite(float(D.std())) else float(D.median())
    h = max(h, float(D.median()), 1e-12)
    tot, lo = 0.0, a
    for _ in range(2000):
        hi = lo + h
        if hi >= b:
            tot += _quad(lambda x: float(D.sf(x)), lo, b)
            return tot
        part = _quad(lambda x: float(D.sf(x)), lo, hi)
        tot += part
        if b == math.inf and part < 1e-18 * tot:
            return tot
        lo, h = hi, h * 1.5
    return tot


def _ref_deductible(dist, p1, p2, d):
    D = _scipy_dist(dist, p1, p2)
    mean = float(D.mean())
    lev = _sf_integral(D, 0.0, d) if d > 0 else 0.0
    per_loss = _sf_integral(D, d)
    return {"mean": mean, "lev_d": lev, "per_loss": per_loss, "per_payment": per_loss / float(D.sf(d)), "LER": lev / mean}


def exponential_deductible(theta, d):
    _pos("the mean theta", theta)
    return _deductible("exponential", theta, 0.0, d)


def pareto_deductible(alpha, theta, d):
    _need(_isnum(alpha) and 1 < alpha <= 100, "alpha must lie between 1 and 100 (a finite mean)")
    _pos("theta", theta)
    return _deductible("pareto", alpha, theta, d)


def lognormal_deductible(mu, sigma, d):
    _need(_isnum(mu) and -20 < mu < 30, "mu must lie between -20 and 30")
    _need(_isnum(sigma) and 0 < sigma <= 5, "sigma must lie between 0 and 5")
    return _deductible("lognormal", mu, sigma, d)


_DED_OUT = [_o("mean", "mean loss E[X]"), _o("lev_d", "limited expected value E[X ∧ d]"),
            _o("per_loss", "expected payment per loss E[(X - d)+]"), _o("per_payment", "expected payment per payment (mean excess loss e(d))"),
            _o("LER", "loss elimination ratio E[X ∧ d]/E[X]")]
_DED_METHOD = "per loss = E[X] - E[X ∧ d]; per payment = per loss / S(d); LER = E[X ∧ d]/E[X]"

_add("act_exponential_deductible",
     name="Ordinary deductible with exponential losses", topic="loss models / deductibles",
     purpose="The expected payment per loss and per payment, the limited expected value and the loss elimination ratio of an ordinary deductible d when losses are exponential with mean theta.",
     inputs=[_numin("theta", "mean loss theta"), _numin("d", "ordinary deductible")],
     outputs=_DED_OUT, primary="per_loss", conditions=["theta > 0", "d >= 0"], method=_DED_METHOD + "; E[X ∧ d] = theta (1 - e^(-d/theta))",
     latex=r"E[X\wedge d] = \theta\left(1-e^{-d/\theta}\right),\quad E[(X-d)_+] = \theta e^{-d/\theta},\quad e(d) = \theta",
     run=exponential_deductible, reference=lambda theta, d: _ref_deductible("exponential", theta, 0.0, d),
     wrong=lambda theta, d: (lambda o: dict(o, per_payment=o["per_loss"], LER=o["lev_d"] / (theta + d)))(exponential_deductible(theta, d)),
     rand=lambda rng: (lambda th: {"theta": th, "d": _sig(th * rng.uniform(0.05, 2.5), 3)})(_sig(_lu(rng, 100, 1e5), 3)),
     phrases=("Exponential losses with a deductible", "Deductible with exponential losses"),
     examples=[_ex({"theta": 1000.0, "d": 500.0}), _ex({"theta": 2500.0, "d": 250.0})],
     level="intermediate")

_add("act_pareto_deductible",
     name="Ordinary deductible with Pareto losses", topic="loss models / deductibles",
     purpose="The expected payment per loss and per payment, the limited expected value and the loss elimination ratio of an ordinary deductible d when losses follow a two-parameter Pareto distribution (alpha, theta).",
     inputs=[_numin("alpha", "Pareto shape alpha (> 1)"), _numin("theta", "Pareto scale theta"), _numin("d", "ordinary deductible")],
     outputs=_DED_OUT, primary="per_loss", conditions=["alpha > 1", "theta > 0", "d >= 0"],
     method=_DED_METHOD + "; E[X ∧ d] = theta/(alpha-1) [1 - (theta/(theta+d))^(alpha-1)]",
     latex=r"E[X\wedge d] = \frac{\theta}{\alpha-1}\left[1-\left(\frac{\theta}{\theta+d}\right)^{\alpha-1}\right],\quad e(d) = \frac{\theta+d}{\alpha-1}",
     run=pareto_deductible, reference=lambda alpha, theta, d: _ref_deductible("pareto", alpha, theta, d),
     wrong=lambda alpha, theta, d: (lambda o: dict(o, per_payment=(theta + d) / alpha, per_loss=o["per_loss"] * (alpha - 1) / alpha))(pareto_deductible(alpha, theta, d)),
     rand=lambda rng: (lambda th: {"alpha": _sig(rng.uniform(1.5, 6), 3), "theta": th, "d": _sig(th * rng.uniform(0.05, 2), 3)})(_sig(_lu(rng, 100, 1e5), 3)),
     phrases=("Pareto losses with a deductible", "Deductible with Pareto losses"),
     examples=[_ex({"alpha": 3.0, "theta": 2000.0, "d": 500.0}), _ex({"alpha": 2.5, "theta": 1000.0, "d": 1000.0})],
     level="intermediate")

_add("act_lognormal_deductible",
     name="Ordinary deductible with lognormal losses", topic="loss models / deductibles",
     purpose="The expected payment per loss and per payment, the limited expected value and the loss elimination ratio of an ordinary deductible d when losses are lognormal (mu, sigma of the log of the loss).",
     inputs=[_numin("mu", "mean of the log of the loss"), _numin("sigma", "standard deviation of the log of the loss"), _numin("d", "ordinary deductible")],
     outputs=_DED_OUT, primary="per_loss", conditions=["sigma > 0", "d >= 0"],
     method=_DED_METHOD + "; E[X ∧ d] = e^(mu + sigma^2/2) Phi((ln d - mu - sigma^2)/sigma) + d [1 - Phi((ln d - mu)/sigma)]",
     latex=r"E[X\wedge d] = e^{\mu+\sigma^2/2}\,\Phi\!\left(\frac{\ln d-\mu-\sigma^2}{\sigma}\right) + d\left[1-\Phi\!\left(\frac{\ln d-\mu}{\sigma}\right)\right]",
     run=lognormal_deductible, reference=lambda mu, sigma, d: _ref_deductible("lognormal", mu, sigma, d),
     wrong=lambda mu, sigma, d: (lambda o: dict(o, mean=math.exp(mu), per_loss=math.exp(mu) - o["lev_d"]))(lognormal_deductible(mu, sigma, d)),
     rand=lambda rng: (lambda mu, sg: {"mu": mu, "sigma": sg, "d": _sig(math.exp(mu + sg * rng.uniform(-1.5, 1.5)), 3)})(_sig(rng.uniform(4, 10), 3), _sig(rng.uniform(0.3, 1.8), 3)),
     phrases=("Lognormal losses with a deductible", "Deductible with lognormal losses"),
     examples=[_ex({"mu": 7.0, "sigma": 1.0, "d": 1000.0}), _ex({"mu": 5.0, "sigma": 0.5, "d": 100.0})],
     level="intermediate")


def _coverage(dist, p1, p2, d, u, c, r):
    _nonneg("the deductible d", d)
    _need(_isnum(u) and u > d, "the policy limit u must exceed the deductible d")
    _need(_isnum(c) and 0 < c <= 1, "the coinsurance c must lie between 0 and 100%")
    _need(_isnum(r) and 0 <= r < 1, "the inflation r must lie between 0 and 100%")
    f = 1.0 + r
    per_loss = c * f * (_lev(dist, p1, p2, u / f) - _lev(dist, p1, p2, d / f))
    sfd = _sf(dist, p1, p2, d / f)
    _need(sfd > 1e-300, "the deductible is beyond the range of the losses")
    return _out(per_loss=per_loss, per_payment=per_loss / sfd, max_payment=c * (u - d))


def _ref_coverage(dist, p1, p2, d, u, c, r):
    D = _scipy_dist(dist, p1, p2)
    f = 1.0 + r
    sy = lambda y: float(D.sf(y / f))                 # survival function of the inflated loss (1+r)X
    per_loss = c * f * _sf_integral(D, d / f, u / f)
    return {"per_loss": per_loss, "per_payment": per_loss / sy(d), "max_payment": c * (u - d)}


def exponential_coverage(theta, d, u, c, r):
    _pos("the mean theta", theta)
    return _coverage("exponential", theta, 0.0, d, u, c, r)


def pareto_coverage(alpha, theta, d, u, c, r):
    _need(_isnum(alpha) and 1 < alpha <= 100, "alpha must lie between 1 and 100")
    _pos("theta", theta)
    return _coverage("pareto", alpha, theta, d, u, c, r)


_COV_IN = [_numin("d", "ordinary deductible"), _numin("u", "policy limit (maximum covered loss)"),
           _numin("c", "coinsurance fraction paid by the insurer (80% or 0.8)", pct=True), _numin("r", "uniform loss inflation (0 for none)", pct=True)]
_COV_OUT = [_o("per_loss", "expected payment per loss"), _o("per_payment", "expected payment per payment"), _o("max_payment", "largest possible payment c (u - d)")]
_COV_METHOD = "per loss = c (1 + r) [E[X ∧ u/(1+r)] - E[X ∧ d/(1+r)]]; per payment = per loss / S(d/(1+r))"
_COV_LATEX = r"E[Y^L] = c\,(1+r)\left[E\!\left[X\wedge\tfrac{u}{1+r}\right] - E\!\left[X\wedge\tfrac{d}{1+r}\right]\right],\quad E[Y^P] = \frac{E[Y^L]}{S_X\!\left(\frac{d}{1+r}\right)}"

_add("act_exponential_coverage_modifications",
     name="Deductible, policy limit, coinsurance and inflation with exponential losses", topic="loss models / coverage modifications",
     purpose="The expected insurer payment per loss and per payment for exponential losses (mean theta) under an ordinary deductible, a policy limit (maximum covered loss), coinsurance and uniform inflation.",
     inputs=[_numin("theta", "mean loss theta before inflation")] + _COV_IN, outputs=_COV_OUT, primary="per_loss",
     conditions=["theta > 0", "0 <= d < u", "0 < c <= 1", "0 <= r < 1"], method=_COV_METHOD, latex=_COV_LATEX,
     run=exponential_coverage, reference=lambda theta, d, u, c, r: _ref_coverage("exponential", theta, 0.0, d, u, c, r),
     wrong=lambda theta, d, u, c, r: (lambda o: dict(o, per_loss=c * (_lev("exponential", theta, 0, u) - _lev("exponential", theta, 0, d)) * (1 + r)))(exponential_coverage(theta, d, u, c, r)) if r > 0.005 else (lambda o: dict(o, per_payment=o["per_loss"]))(exponential_coverage(theta, d, u, c, r)),
     rand=lambda rng: (lambda th: (lambda d: {"theta": th, "d": d, "u": _sig(d + th * rng.uniform(0.3, 4), 3), "c": rng.choice([1.0, 0.9, 0.8, 0.75]), "r": rng.choice([0.0, 0.05, 0.1, 0.03])})(_sig(th * rng.uniform(0.05, 1.5), 3)))(_sig(_lu(rng, 100, 1e5), 3)),
     phrases=("Exponential losses with deductible, limit and coinsurance", "Coverage modifications with exponential losses"),
     examples=[_ex({"theta": 1000.0, "d": 200.0, "u": 3000.0, "c": 0.8, "r": 0.1}), _ex({"theta": 5000.0, "d": 500.0, "u": 10000.0, "c": 1.0, "r": 0.0})],
     level="advanced")

_add("act_pareto_coverage_modifications",
     name="Deductible, policy limit, coinsurance and inflation with Pareto losses", topic="loss models / coverage modifications",
     purpose="The expected insurer payment per loss and per payment for two-parameter Pareto losses under an ordinary deductible, a policy limit (maximum covered loss), coinsurance and uniform inflation.",
     inputs=[_numin("alpha", "Pareto shape alpha (> 1)"), _numin("theta", "Pareto scale theta before inflation")] + _COV_IN, outputs=_COV_OUT, primary="per_loss",
     conditions=["alpha > 1", "theta > 0", "0 <= d < u", "0 < c <= 1", "0 <= r < 1"], method=_COV_METHOD, latex=_COV_LATEX,
     run=pareto_coverage, reference=lambda alpha, theta, d, u, c, r: _ref_coverage("pareto", alpha, theta, d, u, c, r),
     wrong=lambda alpha, theta, d, u, c, r: (lambda o: dict(o, per_payment=o["per_loss"], per_loss=o["per_loss"] / c))(pareto_coverage(alpha, theta, d, u, c, r)),
     rand=lambda rng: (lambda th: (lambda d: {"alpha": _sig(rng.uniform(1.5, 6), 3), "theta": th, "d": d, "u": _sig(d + th * rng.uniform(0.3, 4), 3), "c": rng.choice([0.9, 0.8, 0.75, 0.5]), "r": rng.choice([0.0, 0.05, 0.1, 0.03])})(_sig(th * rng.uniform(0.05, 1.5), 3)))(_sig(_lu(rng, 100, 1e5), 3)),
     phrases=("Pareto losses with deductible, limit and coinsurance", "Coverage modifications with Pareto losses"),
     examples=[_ex({"alpha": 3.0, "theta": 2000.0, "d": 500.0, "u": 5000.0, "c": 0.8, "r": 0.05}), _ex({"alpha": 2.0, "theta": 1000.0, "d": 100.0, "u": 2000.0, "c": 0.9, "r": 0.0})],
     level="advanced")


def single_loss_payment(x, d, u, c):
    _nonneg("the loss x and the deductible d", x, d)
    _need(_isnum(u) and u > d, "the policy limit u must exceed the deductible d")
    _need(_isnum(c) and 0 < c <= 1, "the coinsurance c must lie between 0 and 100%")
    covered = min(x, u)
    pay = c * max(covered - d, 0.0)
    return _out(payment=pay, retained=x - pay)


def _ref_single_loss_payment(x, d, u, c):
    import numpy as np
    pay = float(c * np.clip(x, d, u) - c * d)
    return {"payment": pay, "retained": x - pay}


_add("act_single_loss_payment",
     name="Insurer payment for one loss (deductible, limit, coinsurance)", topic="loss models / coverage modifications",
     purpose="The amount the insurer pays and the amount the insured retains for a single loss x under an ordinary deductible d, a policy limit u (maximum covered loss) and coinsurance c.",
     inputs=[_numin("x", "loss amount"), _numin("d", "ordinary deductible"), _numin("u", "policy limit (maximum covered loss)"),
             _numin("c", "coinsurance fraction paid by the insurer (80% or 0.8)", pct=True)],
     outputs=[_o("payment", "insurer payment c (min(x, u) - d)+"), _o("retained", "amount retained by the insured")],
     primary="payment", conditions=["x, d >= 0", "u > d", "0 < c <= 1"],
     method="payment = c x (min(x, u) - d)+",
     latex=r"Y = c\left(\min(X,u) - d\right)_+",
     run=single_loss_payment, reference=_ref_single_loss_payment,
     wrong=lambda x, d, u, c: (lambda p: _out(payment=p, retained=x - p))(max(c * min(x, u) - d, 0.0)),
     rand=lambda rng: (lambda d: {"x": _sig(d * rng.uniform(0.5, 30), 3), "d": d, "u": _sig(d + _lu(rng, 100, 5e4), 3), "c": rng.choice([0.8, 0.9, 0.7, 0.5])})(_sig(_lu(rng, 50, 5000), 3)),
     phrases=("Insurer payment for a loss", "Payment for a single loss"),
     examples=[_ex({"x": 5000.0, "d": 500.0, "u": 4000.0, "c": 0.8}), _ex({"x": 1200.0, "d": 250.0, "u": 10000.0, "c": 0.9})],
     level="beginner",
     weak="the same definition by numpy clipping")


# ================================================================================================ RISK MEASURES
def _p_check(p):
    _need(_isnum(p) and 0 < p < 1, "the security level p must lie strictly between 0 and 1")


def _ref_tail(D, p):
    var = float(D.ppf(p))
    ex = _sf_integral(D, var)
    return {"VaR": var, "TVaR": var + ex / (1.0 - p)}


def var_tvar_normal(mu, sigma, p):
    _fin(mu)
    _pos("sigma", sigma)
    _p_check(p)
    z = _norm_ppf(p)
    return _out(VaR=mu + sigma * z, TVaR=mu + sigma * _norm_pdf(z) / (1.0 - p))


def var_tvar_exponential(theta, p):
    _pos("the mean theta", theta)
    _p_check(p)
    v = -theta * math.log1p(-p)
    return _out(VaR=v, TVaR=v + theta)


def var_tvar_pareto(alpha, theta, p):
    _need(_isnum(alpha) and 1 < alpha <= 100, "alpha must lie between 1 and 100 (a finite mean)")
    _pos("theta", theta)
    _p_check(p)
    v = theta * ((1.0 - p) ** (-1.0 / alpha) - 1.0)
    return _out(VaR=v, TVaR=v + (theta + v) / (alpha - 1.0))


def var_tvar_lognormal(mu, sigma, p):
    _need(_isnum(mu) and -20 < mu < 30, "mu must lie between -20 and 30")
    _need(_isnum(sigma) and 0 < sigma <= 5, "sigma must lie between 0 and 5")
    _p_check(p)
    z = _norm_ppf(p)
    return _out(VaR=math.exp(mu + sigma * z), TVaR=math.exp(mu + sigma * sigma / 2.0) * _norm_cdf(sigma - z) / (1.0 - p))


_VAR_OUT = [_o("VaR", "value at risk at level p (the p-quantile)"), _o("TVaR", "tail value at risk E[X | X > VaR]")]

_add("act_var_tvar_normal",
     name="VaR and TVaR of a normal loss", topic="risk measures / VaR and TVaR",
     purpose="The value at risk and tail value at risk at security level p of a normally distributed loss.",
     inputs=[_numin("mu", "mean loss"), _numin("sigma", "standard deviation of the loss"), _numin("p", "security level (95% or 0.95)", pct=True)],
     outputs=_VAR_OUT, primary="TVaR", conditions=["sigma > 0", "0 < p < 1"],
     method="VaR = mu + sigma z_p; TVaR = mu + sigma phi(z_p)/(1 - p)",
     latex=r"\mathrm{VaR}_p = \mu+\sigma z_p,\quad \mathrm{TVaR}_p = \mu + \sigma\frac{\phi(z_p)}{1-p}",
     run=var_tvar_normal, reference=lambda mu, sigma, p: _ref_tail(__import__("scipy.stats", fromlist=["norm"]).norm(loc=mu, scale=sigma), p),
     wrong=lambda mu, sigma, p: (lambda z: _out(VaR=mu + sigma * z, TVaR=mu + sigma * _norm_pdf(z) / p))(_norm_ppf(p)),
     rand=lambda rng: {"mu": _sig(rng.uniform(-1000, 1e5), 4), "sigma": _sig(_lu(rng, 10, 1e4), 3), "p": rng.choice([0.9, 0.95, 0.99, 0.995, 0.75, 0.8])},
     phrases=("VaR and TVaR of a normal loss", "Normal value at risk"),
     examples=[_ex({"mu": 1000.0, "sigma": 200.0, "p": 0.95}), _ex({"mu": 0.0, "sigma": 1.0, "p": 0.99})],
     level="intermediate")

_add("act_var_tvar_exponential",
     name="VaR and TVaR of an exponential loss", topic="risk measures / VaR and TVaR",
     purpose="The value at risk and tail value at risk at security level p of an exponentially distributed loss with mean theta.",
     inputs=[_numin("theta", "mean loss theta"), _numin("p", "security level (95% or 0.95)", pct=True)],
     outputs=_VAR_OUT, primary="TVaR", conditions=["theta > 0", "0 < p < 1"],
     method="VaR = -theta ln(1 - p); TVaR = VaR + theta",
     latex=r"\mathrm{VaR}_p = -\theta\ln(1-p),\quad \mathrm{TVaR}_p = \mathrm{VaR}_p + \theta",
     run=var_tvar_exponential, reference=lambda theta, p: _ref_tail(__import__("scipy.stats", fromlist=["expon"]).expon(scale=theta), p),
     wrong=lambda theta, p: (lambda v: _out(VaR=v, TVaR=v + theta))(-theta * math.log(p)),
     rand=lambda rng: {"theta": _sig(_lu(rng, 10, 1e5), 3), "p": rng.choice([0.9, 0.95, 0.99, 0.995, 0.75, 0.8])},
     phrases=("VaR and TVaR of an exponential loss", "Exponential value at risk"),
     examples=[_ex({"theta": 1000.0, "p": 0.95}), _ex({"theta": 500.0, "p": 0.99})],
     level="intermediate")

_add("act_var_tvar_pareto",
     name="VaR and TVaR of a Pareto loss", topic="risk measures / VaR and TVaR",
     purpose="The value at risk and tail value at risk at security level p of a two-parameter Pareto loss (alpha, theta).",
     inputs=[_numin("alpha", "Pareto shape alpha (> 1)"), _numin("theta", "Pareto scale theta"), _numin("p", "security level (95% or 0.95)", pct=True)],
     outputs=_VAR_OUT, primary="TVaR", conditions=["alpha > 1", "theta > 0", "0 < p < 1"],
     method="VaR = theta [(1-p)^(-1/alpha) - 1]; TVaR = VaR + (theta + VaR)/(alpha - 1)",
     latex=r"\mathrm{VaR}_p = \theta\left[(1-p)^{-1/\alpha}-1\right],\quad \mathrm{TVaR}_p = \mathrm{VaR}_p + \frac{\theta+\mathrm{VaR}_p}{\alpha-1}",
     run=var_tvar_pareto, reference=lambda alpha, theta, p: _ref_tail(__import__("scipy.stats", fromlist=["lomax"]).lomax(alpha, scale=theta), p),
     wrong=lambda alpha, theta, p: (lambda v: _out(VaR=v, TVaR=v + (theta + v) / alpha))(theta * ((1 - p) ** (-1 / alpha) - 1)),
     rand=lambda rng: {"alpha": _sig(rng.uniform(1.5, 8), 3), "theta": _sig(_lu(rng, 10, 1e5), 3), "p": rng.choice([0.9, 0.95, 0.99, 0.75, 0.8])},
     phrases=("VaR and TVaR of a Pareto loss", "Pareto value at risk"),
     examples=[_ex({"alpha": 3.0, "theta": 2000.0, "p": 0.95}), _ex({"alpha": 2.5, "theta": 1000.0, "p": 0.99})],
     level="advanced")

_add("act_var_tvar_lognormal",
     name="VaR and TVaR of a lognormal loss", topic="risk measures / VaR and TVaR",
     purpose="The value at risk and tail value at risk at security level p of a lognormal loss (mu, sigma of the log of the loss).",
     inputs=[_numin("mu", "mean of the log of the loss"), _numin("sigma", "standard deviation of the log of the loss"), _numin("p", "security level (95% or 0.95)", pct=True)],
     outputs=_VAR_OUT, primary="TVaR", conditions=["sigma > 0", "0 < p < 1"],
     method="VaR = exp(mu + sigma z_p); TVaR = e^(mu + sigma^2/2) Phi(sigma - z_p)/(1 - p)",
     latex=r"\mathrm{VaR}_p = e^{\mu+\sigma z_p},\quad \mathrm{TVaR}_p = e^{\mu+\sigma^2/2}\,\frac{\Phi(\sigma - z_p)}{1-p}",
     run=var_tvar_lognormal, reference=lambda mu, sigma, p: _ref_tail(__import__("scipy.stats", fromlist=["lognorm"]).lognorm(sigma, scale=math.exp(mu)), p),
     wrong=lambda mu, sigma, p: (lambda z: _out(VaR=math.exp(mu + sigma * z), TVaR=math.exp(mu) * _norm_cdf(sigma - z) / (1 - p)))(_norm_ppf(p)),
     rand=lambda rng: {"mu": _sig(rng.uniform(2, 10), 3), "sigma": _sig(rng.uniform(0.2, 1.5), 3), "p": rng.choice([0.9, 0.95, 0.99, 0.75, 0.8])},
     phrases=("VaR and TVaR of a lognormal loss", "Lognormal value at risk"),
     examples=[_ex({"mu": 7.0, "sigma": 1.0, "p": 0.95}), _ex({"mu": 5.0, "sigma": 0.5, "p": 0.99})],
     level="advanced")


def premium_principle(mean, var, theta, principle):
    _need(principle in ("expected_value", "variance", "standard_deviation"), "principle: expected_value, variance or standard_deviation")
    _nonneg("the mean, variance and loading", mean, var, theta)
    if principle == "expected_value":
        return _out(premium=(1.0 + theta) * mean, loading=theta * mean)
    if principle == "variance":
        return _out(premium=mean + theta * var, loading=theta * var)
    return _out(premium=mean + theta * math.sqrt(var), loading=theta * math.sqrt(var))


def _ref_premium_principle(mean, var, theta, principle):
    import sympy as sp
    M, V, T = sp.nsimplify(mean), sp.nsimplify(var), sp.nsimplify(theta)
    load = {"expected_value": T * M, "variance": T * V, "standard_deviation": T * sp.sqrt(V)}[principle]
    return {"premium": float(M + load), "loading": float(load)}


_add("act_premium_principles",
     name="Premium principles: expected value, variance, standard deviation", topic="loss models / premium principles",
     purpose="The premium for a risk with a given mean and variance of loss by the expected value principle (1 + theta) E[X], the variance principle E[X] + theta Var X or the standard deviation principle E[X] + theta sd(X).",
     inputs=[_numin("mean", "expected loss"), _numin("var", "variance of the loss"), _numin("theta", "loading factor"),
             _choice("principle", ("expected_value", "variance", "standard_deviation"), "premium principle")],
     outputs=[_o("premium", "premium"), _o("loading", "risk loading added to the expected loss")],
     primary="premium", conditions=["mean, variance, loading >= 0"],
     method="expected value: (1 + theta) E[X]; variance: E[X] + theta Var X; standard deviation: E[X] + theta sd(X)",
     latex=r"\pi = (1+\theta)E[X],\quad \pi = E[X] + \theta\,\mathrm{Var}(X),\quad \pi = E[X] + \theta\sqrt{\mathrm{Var}(X)}",
     run=premium_principle, reference=_ref_premium_principle,
     wrong=lambda mean, var, theta, principle: _out(premium=mean + theta * (var if principle == "standard_deviation" else math.sqrt(var)), loading=0.0),
     rand=lambda rng: {"mean": _sig(_lu(rng, 100, 1e5), 3), "var": _sig(_lu(rng, 1e4, 1e9), 3), "theta": _sig(rng.uniform(0.01, 0.5), 2),
                       "principle": rng.choice(["expected_value", "variance", "standard_deviation"])},
     phrases=("Premium principle", "Premium by a premium principle"),
     examples=[_ex({"mean": 1000.0, "var": 250000.0, "theta": 0.2, "principle": "expected_value"}),
               _ex({"mean": 1000.0, "var": 250000.0, "theta": 0.1, "principle": "standard_deviation"})],
     level="beginner",
     weak="the same definitions in exact sympy arithmetic")


# ================================================================================================ CREDIBILITY
def lf_full_credibility(p, k):
    _p_check(p)
    _need(_isnum(k) and 0 < k < 1, "the tolerance k must lie strictly between 0 and 1")
    z = _norm_ppf((1.0 + p) / 2.0)
    return _out(n_full=(z / k) ** 2, z=z)


def _ref_lf_full_credibility(p, k):
    from scipy.stats import norm
    z = float(norm.isf((1.0 - p) / 2.0))
    return {"n_full": (z / k) ** 2, "z": z}


_add("act_limited_fluctuation_full_credibility",
     name="Limited fluctuation full credibility standard for claim counts", topic="credibility / limited fluctuation",
     purpose="The expected number of claims needed for full credibility of the claim frequency so that the observed number is within a fraction k of its mean with probability p (Poisson claim counts).",
     inputs=[_numin("p", "probability level (90% or 0.9)", pct=True), _numin("k", "tolerance as a fraction of the mean (5% or 0.05)", pct=True)],
     outputs=[_o("n_full", "expected claims needed for full credibility (z/k)^2"), _o("z", "standard normal quantile z_((1+p)/2)")],
     primary="n_full", conditions=["0 < p < 1", "0 < k < 1", "Poisson claim counts"],
     method="n_F = (z_((1+p)/2) / k)^2",
     latex=r"n_F = \left(\frac{z_{(1+p)/2}}{k}\right)^2",
     run=lf_full_credibility, reference=_ref_lf_full_credibility,
     wrong=lambda p, k: (lambda z: _out(n_full=(z / k) ** 2, z=z))(_norm_ppf(p)),
     rand=lambda rng: {"p": rng.choice([0.9, 0.95, 0.99, 0.8]), "k": rng.choice([0.05, 0.1, 0.025, 0.075])},
     phrases=("Full credibility standard", "Limited fluctuation full credibility"),
     examples=[_ex({"p": 0.9, "k": 0.05}), _ex({"p": 0.95, "k": 0.05})],
     level="beginner")


def lf_full_credibility_severity(p, k, mean_x, sd_x):
    _p_check(p)
    _need(_isnum(k) and 0 < k < 1, "the tolerance k must lie strictly between 0 and 1")
    _pos("the severity mean and standard deviation", mean_x, sd_x)
    n0 = (_norm_ppf((1.0 + p) / 2.0) / k) ** 2
    cv2 = (sd_x / mean_x) ** 2
    return _out(claims_aggregate=n0 * (1.0 + cv2), claims_severity=n0 * cv2, claims_frequency=n0)


def _ref_lf_full_credibility_severity(p, k, mean_x, sd_x):
    from scipy.stats import norm
    n0 = (float(norm.ppf(0.5 + p / 2.0)) / k) ** 2
    ex2 = sd_x * sd_x + mean_x * mean_x
    return {"claims_aggregate": n0 * ex2 / mean_x ** 2, "claims_severity": n0 * sd_x ** 2 / mean_x ** 2, "claims_frequency": n0}


_add("act_limited_fluctuation_severity_aggregate",
     name="Limited fluctuation full credibility for severity and aggregate losses", topic="credibility / limited fluctuation",
     purpose="The expected number of claims needed for full credibility of the claim severity, of aggregate losses (pure premium) and of the claim frequency, for Poisson claim counts and a severity with the given mean and standard deviation.",
     inputs=[_numin("p", "probability level (90% or 0.9)", pct=True), _numin("k", "tolerance as a fraction of the mean (5% or 0.05)", pct=True),
             _numin("mean_x", "mean claim severity"), _numin("sd_x", "standard deviation of the claim severity")],
     outputs=[_o("claims_aggregate", "claims for full credibility of aggregate losses n0 (1 + CV^2)"),
              _o("claims_severity", "claims for full credibility of severity n0 CV^2"), _o("claims_frequency", "claims for full credibility of frequency n0")],
     primary="claims_aggregate", conditions=["0 < p < 1", "0 < k < 1", "Poisson claim counts"],
     method="n0 = (z/k)^2; aggregate: n0 (1 + (sd/mean)^2); severity: n0 (sd/mean)^2",
     latex=r"n_S = n_0\left(1 + \frac{\sigma_X^2}{\mu_X^2}\right),\quad n_0 = \left(\frac{z_{(1+p)/2}}{k}\right)^2",
     run=lf_full_credibility_severity, reference=_ref_lf_full_credibility_severity,
     wrong=lambda p, k, mean_x, sd_x: (lambda n0: _out(claims_aggregate=n0 * (1 + sd_x / mean_x), claims_severity=n0 * sd_x / mean_x, claims_frequency=n0))((_norm_ppf((1 + p) / 2) / k) ** 2),
     rand=lambda rng: (lambda m: {"p": rng.choice([0.9, 0.95, 0.99]), "k": rng.choice([0.05, 0.1, 0.025]), "mean_x": m, "sd_x": _sig(m * rng.uniform(0.3, 3), 3)})(_sig(_lu(rng, 100, 1e5), 3)),
     phrases=("Full credibility for aggregate losses", "Full credibility standard for severity and aggregate losses"),
     examples=[_ex({"p": 0.9, "k": 0.05, "mean_x": 1000.0, "sd_x": 2000.0}), _ex({"p": 0.95, "k": 0.1, "mean_x": 500.0, "sd_x": 500.0})],
     level="intermediate")


def partial_credibility(n, n_full, xbar, M):
    _nonneg("the observed claims n", n)
    _pos("the full credibility standard", n_full)
    _fin(xbar, M)
    Z = min(1.0, math.sqrt(n / n_full))
    return _out(Z=Z, premium=Z * xbar + (1.0 - Z) * M)


def _ref_partial_credibility(n, n_full, xbar, M):
    import numpy as np
    Z = float(np.clip(np.sqrt(np.float64(n) / n_full), 0.0, 1.0))
    return {"Z": Z, "premium": M + Z * (xbar - M)}


_add("act_partial_credibility",
     name="Partial credibility (square root rule)", topic="credibility / limited fluctuation",
     purpose="The credibility factor Z = min(1, sqrt(n/n_F)) of an experience with n claims against a full credibility standard n_F, and the credibility-weighted estimate Z xbar + (1 - Z) M.",
     inputs=[_numin("n", "observed (or expected) number of claims"), _numin("n_full", "full credibility standard in claims"),
             _numin("xbar", "observed experience (mean or rate)"), _numin("M", "complement of credibility (manual rate)")],
     outputs=[_o("Z", "credibility factor"), _o("premium", "credibility-weighted estimate")],
     primary="Z", conditions=["n >= 0", "n_F > 0"],
     method="Z = min(1, sqrt(n / n_F)); estimate = Z xbar + (1 - Z) M",
     latex=r"Z = \min\!\left(1, \sqrt{n/n_F}\right),\quad P_C = Z\bar X + (1-Z)M",
     run=partial_credibility, reference=_ref_partial_credibility,
     wrong=lambda n, n_full, xbar, M: (lambda Z: _out(Z=Z, premium=Z * xbar + (1 - Z) * M))(min(1.0, n / n_full)),
     rand=lambda rng: (lambda nf: {"n": _sig(nf * rng.uniform(0.05, 0.9), 3), "n_full": nf, "xbar": _sig(_lu(rng, 50, 5000), 4), "M": _sig(_lu(rng, 50, 5000), 4)})(_sig(rng.uniform(500, 3000), 4)),
     phrases=("Partial credibility", "Square root rule credibility"),
     examples=[_ex({"n": 300.0, "n_full": 1082.0, "xbar": 800.0, "M": 1000.0}), _ex({"n": 2000.0, "n_full": 1082.0, "xbar": 800.0, "M": 1000.0})],
     level="beginner",
     weak="the same rule by numpy clipping, the estimate written as M + Z (xbar - M)")


def buhlmann_premium(mu, EPV, VHM, n, xbar):
    _fin(mu, xbar)
    _pos("the expected process variance and the variance of hypothetical means", EPV, VHM)
    _need(_isnum(n) and n > 0, "the number of observations n must be positive")
    k = EPV / VHM
    Z = n / (n + k)
    return _out(k=k, Z=Z, premium=Z * xbar + (1.0 - Z) * mu)


def _ref_buhlmann_premium(mu, EPV, VHM, n, xbar):
    import numpy as np
    m = int(round(n))
    if abs(m - n) > 1e-12 or m > 60:                     # a non-integer n: the limit of the same normal equations
        Z = 1.0 / (1.0 + EPV / (VHM * n))
        return {"k": EPV / VHM, "Z": Z, "premium": mu + Z * (xbar - mu)}
    C = VHM * np.ones((m, m)) + EPV * np.eye(m)          # Cov(X_i, X_j); Cov(X_j, mu(Theta)) = VHM
    a = np.linalg.solve(C, VHM * np.ones(m))
    Z = float(a.sum())
    return {"k": EPV / VHM, "Z": Z, "premium": (1.0 - Z) * mu + Z * xbar}


_add("act_buhlmann_credibility",
     name="Buhlmann credibility premium (known structural parameters)", topic="credibility / Buhlmann",
     purpose="The Buhlmann credibility factor Z = n/(n + k), k = EPV/VHM, and the credibility premium Z xbar + (1 - Z) mu from the overall mean, the expected process variance, the variance of the hypothetical means and n observations with mean xbar.",
     inputs=[_numin("mu", "overall (collective) mean"), _numin("EPV", "expected process variance"), _numin("VHM", "variance of the hypothetical means"),
             _numin("n", "number of observations"), _numin("xbar", "mean of the observations")],
     outputs=[_o("k", "Buhlmann k = EPV/VHM"), _o("Z", "credibility factor n/(n + k)"), _o("premium", "Buhlmann credibility premium")],
     primary="premium", conditions=["EPV, VHM > 0", "n > 0"],
     method="k = EPV/VHM; Z = n/(n + k); P = Z xbar + (1 - Z) mu",
     latex=r"k = \frac{\mathrm{EPV}}{\mathrm{VHM}},\quad Z = \frac{n}{n+k},\quad P_C = Z\bar X + (1-Z)\mu",
     run=buhlmann_premium, reference=_ref_buhlmann_premium,
     wrong=lambda mu, EPV, VHM, n, xbar: (lambda k: (lambda Z: _out(k=k, Z=Z, premium=Z * xbar + (1 - Z) * mu))(n / (n + k)))(VHM / EPV),
     rand=lambda rng: {"mu": _sig(_lu(rng, 10, 1e4), 4), "EPV": _sig(_lu(rng, 10, 1e6), 3), "VHM": _sig(_lu(rng, 1, 1e5), 3), "n": float(rng.randint(1, 30)), "xbar": _sig(_lu(rng, 10, 1e4), 4)},
     phrases=("Buhlmann credibility premium", "Bühlmann credibility premium", "Buhlmann credibility"),
     examples=[_ex({"mu": 1000.0, "EPV": 50000.0, "VHM": 10000.0, "n": 5.0, "xbar": 1200.0}), _ex({"mu": 0.2, "EPV": 0.2, "VHM": 0.04, "n": 3.0, "xbar": 0.5})],
     level="intermediate")


def _mat_check(X, name):
    _need(len(X) >= 2 and len(X[0]) >= 2, f"{name} needs at least two risks (rows) and two periods (columns)")
    for row in X:
        for v in row:
            _need(_isnum(v), f"every entry of {name} must be a finite number")


def buhlmann_empirical(X):
    _mat_check(X, "the data")
    r, n = len(X), len(X[0])
    means = []
    for row in X:
        s = 0.0
        for v in row:
            s += v
        means.append(s / n)
    grand = 0.0
    for m_ in means:
        grand += m_
    grand /= r
    epv = 0.0
    for i in range(r):
        for v in X[i]:
            epv += (v - means[i]) ** 2
    epv /= r * (n - 1)
    vb = 0.0
    for m_ in means:
        vb += (m_ - grand) ** 2
    vhm = vb / (r - 1) - epv / n
    _need(vhm > 0, "the estimated variance of the hypothetical means is not positive: no credibility (Z = 0)")
    _need(epv > 0, "the expected process variance estimate is zero")
    k = epv / vhm
    Z = n / (n + k)
    return _out(EPV=epv, VHM=vhm, k=k, Z=Z, premiums=[Z * m_ + (1.0 - Z) * grand for m_ in means], mean=grand)


def _ref_buhlmann_empirical(X):
    import numpy as np
    from scipy.stats import f_oneway
    A = np.array(X, dtype=float)
    r, n = A.shape
    msw = float(np.mean(np.var(A, axis=1, ddof=1)))
    F = float(f_oneway(*[row for row in A]).statistic)      # F = MSB/MSW of the one-way analysis of variance
    vhm = (F * msw - msw) / n
    k = msw / vhm
    Z = n / (n + k)
    means = A.mean(axis=1)
    g = float(A.mean())
    return {"EPV": msw, "VHM": vhm, "k": k, "Z": Z, "premiums": [float(Z * m + (1 - Z) * g) for m in means], "mean": g}


_add("act_buhlmann_empirical",
     name="Empirical Buhlmann credibility from a data table", topic="credibility / empirical Bayes",
     purpose="The nonparametric estimates of the expected process variance and of the variance of the hypothetical means from r risks observed for n periods each, the Buhlmann k and Z, and each risk's credibility premium.",
     inputs=[_matin("X", "observations: one row per risk, one column per period (rows of equal length)")],
     outputs=[_o("EPV", "estimated expected process variance"), _o("VHM", "estimated variance of the hypothetical means"),
              _o("k", "Buhlmann k = EPV/VHM"), _o("Z", "credibility factor n/(n + k)"), _o("premiums", "credibility premium of each risk (a list)"),
              _o("mean", "overall mean")],
     primary="Z", conditions=["at least 2 risks and 2 periods", "estimated VHM > 0"],
     method="EPV = mean of the within-risk sample variances; VHM = sample variance of the risk means - EPV/n; Z = n/(n + EPV/VHM)",
     latex=r"\hat v = \frac{1}{r(n-1)}\sum_{i,j}\left(X_{ij}-\bar X_i\right)^2,\quad \hat a = \frac{1}{r-1}\sum_i\left(\bar X_i-\bar X\right)^2 - \frac{\hat v}{n}",
     run=buhlmann_empirical, reference=_ref_buhlmann_empirical,
     wrong=lambda X: (lambda o: dict(o, Z=len(X[0]) / (len(X[0]) + o["VHM"] / o["EPV"])))(buhlmann_empirical(X)),
     rand=lambda rng: (lambda r, n: (lambda mus: {"X": [[_sig(max(0.0, rng.gauss(mus[i], 40)), 3) for _ in range(n)] for i in range(r)]})([rng.uniform(100, 400) for _ in range(r)]))(rng.randint(2, 6), rng.randint(2, 6)),
     phrases=("Empirical Buhlmann credibility", "Empirical Bühlmann credibility", "Nonparametric Buhlmann"),
     examples=[_ex({"X": [[3.0, 5.0, 7.0], [6.0, 12.0, 9.0]]}), _ex({"X": [[100.0, 120.0, 110.0, 130.0], [200.0, 180.0, 210.0, 190.0], [150.0, 160.0, 140.0, 170.0]]})],
     level="expert",
     extra_q=[{"q": "Empirical Buhlmann credibility: X = [[1, 2], [1, 2]]", "outcome": "INVALID_INPUT"}])


def buhlmann_straub(X, m):
    _mat_check(X, "the losses per exposure")
    _mat_check(m, "the exposures")
    _need(len(m) == len(X) and len(m[0]) == len(X[0]), "the exposures must have the same shape as the losses")
    r, n = len(X), len(X[0])
    mi, xi = [], []
    for i in range(r):
        s = w = 0.0
        for j in range(n):
            _need(m[i][j] > 0, "every exposure must be positive")
            s += m[i][j] * X[i][j]
            w += m[i][j]
        mi.append(w)
        xi.append(s / w)
    mt = 0.0
    xs = 0.0
    for i in range(r):
        mt += mi[i]
        xs += mi[i] * xi[i]
    xbar = xs / mt
    v = 0.0
    for i in range(r):
        for j in range(n):
            v += m[i][j] * (X[i][j] - xi[i]) ** 2
    v /= r * (n - 1)
    num = 0.0
    sq = 0.0
    for i in range(r):
        num += mi[i] * (xi[i] - xbar) ** 2
        sq += mi[i] * mi[i]
    a = (num - v * (r - 1)) / (mt - sq / mt)
    _need(a > 0, "the estimated variance of the hypothetical means is not positive: no credibility (Z = 0)")
    _need(v > 0, "the expected process variance estimate is zero")
    k = v / a
    Z = [w / (w + k) for w in mi]
    return _out(EPV=v, VHM=a, k=k, Z=Z, premiums=[Z[i] * xi[i] + (1.0 - Z[i]) * xbar for i in range(r)], mean=xbar)


def _ref_buhlmann_straub(X, m):
    import numpy as np
    A, W = np.array(X, dtype=float), np.array(m, dtype=float)
    r, n = A.shape
    wi = W.sum(axis=1)
    xi = np.average(A, axis=1, weights=W)
    wt = wi.sum()
    xb = float(np.average(xi, weights=wi))
    within = float(np.sum(W * A * A) - np.sum(wi * xi * xi))             # sum m (X - Xi)^2 expanded
    v = within / (r * (n - 1))
    between = float(np.sum(wi * xi * xi) - wt * xb * xb)
    a = (between - v * (r - 1)) / (wt - float(np.sum(wi ** 2)) / wt)
    Z = wi / (wi + v / a)
    return {"EPV": v, "VHM": a, "k": v / a, "Z": [float(z) for z in Z], "premiums": [float(z * x + (1 - z) * xb) for z, x in zip(Z, xi)], "mean": xb}


_add("act_buhlmann_straub",
     name="Buhlmann-Straub credibility with exposures", topic="credibility / empirical Bayes",
     purpose="The nonparametric Buhlmann-Straub estimates of the expected process variance and of the variance of the hypothetical means from losses per unit of exposure and the exposures of r risks over n periods, each risk's credibility factor and its credibility premium (complement: the exposure-weighted overall mean).",
     inputs=[_matin("X", "losses per unit of exposure: one row per risk, one column per period"),
             _matin("m", "exposures, the same shape as X")],
     outputs=[_o("EPV", "estimated expected process variance"), _o("VHM", "estimated variance of the hypothetical means"),
              _o("k", "k = EPV/VHM"), _o("Z", "credibility factor of each risk m_i/(m_i + k) (a list)"),
              _o("premiums", "credibility premium per unit of exposure of each risk (a list)"), _o("mean", "exposure-weighted overall mean")],
     primary="k", conditions=["at least 2 risks and 2 periods", "positive exposures", "estimated VHM > 0"],
     method="v = sum m_ij (X_ij - Xbar_i)^2 / (r(n-1)); a = [sum m_i (Xbar_i - Xbar)^2 - v (r-1)] / (m - sum m_i^2/m); Z_i = m_i/(m_i + v/a)",
     latex=r"\hat v = \frac{\sum_{i,j} m_{ij}(X_{ij}-\bar X_i)^2}{r(n-1)},\quad \hat a = \frac{\sum_i m_i(\bar X_i-\bar X)^2 - \hat v(r-1)}{m - \sum_i m_i^2/m},\quad Z_i = \frac{m_i}{m_i + \hat v/\hat a}",
     run=buhlmann_straub, reference=_ref_buhlmann_straub,
     wrong=lambda X, m: (lambda o: dict(o, k=o["EPV"] / o["VHM"] * len(X[0]), Z=[z * 0.5 for z in o["Z"]]))(buhlmann_straub(X, m)),
     rand=lambda rng: (lambda r, n: (lambda mus: {"X": [[_sig(max(0.0, rng.gauss(mus[i], 30)), 3) for _ in range(n)] for i in range(r)],
                                                  "m": [[float(rng.randint(5, 100)) for _ in range(n)] for _ in range(r)]})([rng.uniform(100, 400) for _ in range(r)]))(rng.randint(2, 5), rng.randint(2, 5)),
     phrases=("Buhlmann-Straub credibility", "Bühlmann-Straub credibility"),
     examples=[_ex({"X": [[100.0, 120.0, 110.0], [200.0, 180.0, 230.0]], "m": [[10.0, 20.0, 15.0], [30.0, 25.0, 20.0]]}),
               _ex({"X": [[3.0, 5.0], [8.0, 6.0], [4.0, 4.5]], "m": [[50.0, 60.0], [40.0, 45.0], [80.0, 70.0]]})],
     level="expert",
     weak="the same estimators from expanded sums of squares and numpy weighted averages")


def poisson_gamma(alpha, theta, counts):
    _pos("the gamma prior parameters", alpha, theta)
    n = len(counts)
    s = 0.0
    for c in counts:
        _need(_isnum(c) and c >= 0 and c == int(c), "the claim counts must be whole numbers")
        s += c
    Z = n * theta / (1.0 + n * theta)
    return _out(post_mean=(alpha + s) * theta / (1.0 + n * theta), Z=Z, prior_mean=alpha * theta,
                post_alpha=alpha + s, post_theta=theta / (1.0 + n * theta))


def _ref_poisson_gamma(alpha, theta, counts):
    from scipy.stats import gamma, poisson
    s = sum(counts)
    n = len(counts)
    mode = max((alpha + s - 1.0) / (n + 1.0 / theta), 1e-12)
    const = sum(float(poisson.logpmf(int(c), 1.0)) for c in counts)     # log-likelihood at rate 1 (scipy)

    def logpost(l):                                   # gamma prior density (scipy) times the Poisson likelihood
        return float(gamma.logpdf(l, alpha, scale=theta)) + const + s * math.log(l) - n * (l - 1.0)
    c0 = logpost(mode)
    hi = (alpha + s) * theta / (1 + n * theta) * 50 + 50 * theta + 10
    Z0 = _quad(lambda l: math.exp(logpost(l) - c0), 0.0, hi)
    Z1 = _quad(lambda l: l * math.exp(logpost(l) - c0), 0.0, hi)
    m = Z1 / Z0
    Zc = (m - alpha * theta) / (s / n - alpha * theta) if abs(s / n - alpha * theta) > 1e-9 else n * theta / (1 + n * theta)
    return {"post_mean": m, "Z": Zc, "prior_mean": alpha * theta, "post_alpha": alpha + s, "post_theta": m / (alpha + s)}


_add("act_poisson_gamma_credibility",
     name="Poisson-gamma Bayesian credibility", topic="credibility / Bayesian credibility",
     purpose="The posterior (Bayesian) mean claim frequency of a risk whose Poisson claim rate has a gamma prior (alpha, theta: mean alpha theta), after observing claim counts in n periods, with the equivalent credibility factor and the posterior gamma parameters.",
     inputs=[_numin("alpha", "gamma prior shape alpha"), _numin("theta", "gamma prior scale theta"), _vecin("counts", "observed claim counts, one per period (a list)", 1, 200)],
     outputs=[_o("post_mean", "posterior mean claim rate (alpha + sum x) theta/(1 + n theta)"), _o("Z", "credibility factor n theta/(1 + n theta)"),
              _o("prior_mean", "prior mean alpha theta"), _o("post_alpha", "posterior shape alpha + sum x"), _o("post_theta", "posterior scale theta/(1 + n theta)")],
     primary="post_mean", conditions=["alpha, theta > 0", "whole-number counts"],
     method="posterior Gamma(alpha + sum x, theta/(1 + n theta)); its mean = Z xbar + (1 - Z) alpha theta with Z = n theta/(1 + n theta)",
     latex=r"E[\Lambda\mid x] = \frac{(\alpha + \sum x_j)\,\theta}{1+n\theta} = Z\bar x + (1-Z)\alpha\theta,\quad Z = \frac{n\theta}{1+n\theta}",
     run=poisson_gamma, reference=_ref_poisson_gamma,
     wrong=lambda alpha, theta, counts: (lambda o: dict(o, post_mean=(alpha + sum(counts)) * theta / (len(counts) * theta)))(poisson_gamma(alpha, theta, counts)),
     rand=lambda rng: (lambda a, th: {"alpha": a, "theta": th, "counts": [float(rng.randint(0, max(1, int(a * th * 2 + 1)))) for _ in range(rng.randint(1, 8))]})(_sig(rng.uniform(0.5, 10), 3), _sig(_lu(rng, 0.05, 1), 3)),
     phrases=("Poisson-gamma credibility", "Poisson gamma Bayesian credibility"),
     examples=[_ex({"alpha": 3.0, "theta": 0.5, "counts": [2.0, 1.0, 3.0]}), _ex({"alpha": 1.0, "theta": 0.2, "counts": [0.0, 0.0, 1.0, 0.0]})],
     level="advanced")


# ================================================================================================ LOSS RESERVING
def _tri(T):
    N = len(T)
    _need(N >= 2 and N <= 30, "the triangle needs between 2 and 30 origin years")
    for i in range(N):
        _need(len(T[i]) == N, "the triangle must be square (unknown cells written as 0)")
        for j in range(N):
            v = T[i][j]
            _need(_isnum(v), "every cell must be a number")
            if j < N - i:
                _need(v > 0, "every known cumulative amount must be positive")
            else:
                _need(v == 0, "the cells below the latest diagonal must be written as 0")
    return N


def _cl_factors(T, simple=False):
    N = len(T)
    f = []
    for j in range(N - 1):
        if simple:
            s = 0.0
            for i in range(N - 1 - j):
                s += T[i][j + 1] / T[i][j]
            f.append(s / (N - 1 - j))
        else:
            a = b = 0.0
            for i in range(N - 1 - j):
                a += T[i][j + 1]
                b += T[i][j]
            f.append(a / b)
    cdf = [1.0] * N
    for j in range(N - 2, -1, -1):
        cdf[j] = f[j] * cdf[j + 1]
    return f, cdf


def _cl_projection(T, cdf):
    N = len(T)
    latest = [T[i][N - 1 - i] for i in range(N)]
    ult = [latest[i] * cdf[N - 1 - i] for i in range(N)]
    return latest, ult


def chain_ladder_factors(T):
    _tri(T)
    f, cdf = _cl_factors(T)
    return _out(ldf=f, cdf=cdf)


def _triangle(T):
    """Reference helper: a chainladder Triangle from the square matrix (annual origins and ages)."""
    import pandas as pd
    cl = _import_quiet("chainladder")
    rows = []
    N = len(T)
    for i in range(N):
        for j in range(N - i):
            rows.append({"origin": str(2000 + i), "dev": str(2000 + i + j), "v": float(T[i][j])})
    return cl, cl.Triangle(pd.DataFrame(rows), origin="origin", development="dev", columns="v", cumulative=True)


def _ref_chain_ladder_factors(T):
    cl, tri = _triangle(T)
    dev = cl.Development(average="volume").fit(tri)
    f = [float(x) for x in dev.ldf_.values.ravel()]
    c = [float(x) for x in dev.cdf_.values.ravel()]
    return {"ldf": f, "cdf": c + [1.0]}


_TRI_IN = _matin("T", "cumulative loss triangle: origin years in rows, development ages in columns, unknown cells written as 0")

_add("act_chain_ladder_factors",
     name="Chain-ladder age-to-age and cumulative development factors", topic="loss reserving / chain-ladder",
     purpose="The volume-weighted age-to-age development factors of a cumulative loss triangle and the cumulative factors to ultimate from each development age (no tail factor).",
     inputs=[_TRI_IN],
     outputs=[_o("ldf", "age-to-age factors f_1 ... f_(N-1) (a list)"), _o("cdf", "cumulative development factors to ultimate from each age, last = 1 (a list)")],
     primary="ldf", conditions=["square triangle with N >= 2", "known cells positive, unknown cells 0", "no tail factor"],
     method="f_j = sum_i C_(i,j+1) / sum_i C_(i,j) over origins with both ages known; CDF_j = product of f from age j on",
     latex=r"\hat f_j = \frac{\sum_{i} C_{i,j+1}}{\sum_{i} C_{i,j}},\qquad F_j = \prod_{k\ge j}\hat f_k",
     run=chain_ladder_factors, reference=_ref_chain_ladder_factors,
     wrong=lambda T: (lambda f: _out(ldf=f[0], cdf=f[1]))(_cl_factors(T, simple=True)) if len(T) > 2 else _out(ldf=[T[0][1] / T[0][0] * 1.01], cdf=[1.0, 1.0]),
     rand=lambda rng: {"T": _rand_tri(rng)},
     phrases=("Chain-ladder development factors", "Chain ladder development factors", "Age-to-age factors"),
     examples=[_ex({"T": [[100.0, 150.0, 175.0], [110.0, 168.0, 0.0], [120.0, 0.0, 0.0]]}),
               _ex({"T": [[1000.0, 1800.0, 2100.0, 2200.0], [1100.0, 2000.0, 2300.0, 0.0], [1200.0, 2100.0, 0.0, 0.0], [1300.0, 0.0, 0.0, 0.0]]})],
     level="intermediate",
     extra_q=[{"q": "Age-to-age factors: T = [[100, 150, 175], [110, 168, 5], [120, 0, 0]]", "outcome": "INVALID_INPUT"}])


def _rand_tri(rng, N=None):
    N = N or rng.randint(3, 6)
    f = [rng.uniform(1.02, 2.0) for _ in range(N - 1)]
    T = []
    for i in range(N):
        c = rng.uniform(500, 5000)
        row = [float(round(c))]
        for j in range(N - 1 - i):
            c = c * f[j] * rng.uniform(0.85, 1.15)
            row.append(float(round(c)))
        T.append(row + [0.0] * (N - len(row)))
    return T


def chain_ladder_reserve(T):
    _tri(T)
    f, cdf = _cl_factors(T)
    latest, ult = _cl_projection(T, cdf)
    res = [u - l for u, l in zip(ult, latest)]
    tot = 0.0
    for v in res:
        tot += v
    return _out(ultimate=ult, reserve=res, total_reserve=tot)


def _ref_chain_ladder_reserve(T):
    cl, tri = _triangle(T)
    m = cl.Chainladder().fit(cl.Development(average="volume").fit_transform(tri))
    ult = [float(x) for x in m.ultimate_.values.ravel()]
    latest = [float(x) for x in tri.latest_diagonal.values.ravel()]
    res = [u - l for u, l in zip(ult, latest)]
    res[0] = 0.0
    return {"ultimate": ult, "reserve": res, "total_reserve": float(sum(res))}


_add("act_chain_ladder_reserve",
     name="Chain-ladder ultimate losses and reserves", topic="loss reserving / chain-ladder",
     purpose="The ultimate losses, the reserve (ultimate less reported to date) of each origin year and the total reserve of a cumulative loss triangle by the volume-weighted chain-ladder method (no tail factor).",
     inputs=[_TRI_IN],
     outputs=[_o("ultimate", "ultimate losses of each origin year (a list)"), _o("reserve", "reserve of each origin year (a list)"), _o("total_reserve", "total reserve")],
     primary="total_reserve", conditions=["square triangle with N >= 2", "known cells positive, unknown cells 0", "no tail factor"],
     method="ultimate_i = latest_i x CDF at its age; reserve_i = ultimate_i - latest_i",
     latex=r"\hat C_{i,N} = C_{i,N-i}\prod_{k\ge N-i}\hat f_k,\qquad R_i = \hat C_{i,N} - C_{i,N-i}",
     run=chain_ladder_reserve, reference=_ref_chain_ladder_reserve,
     wrong=lambda T: (lambda f, cdf: (lambda latest, ult: (lambda res: _out(ultimate=ult, reserve=res, total_reserve=sum(res)))([u - l for u, l in zip(ult, latest)]))(*_cl_projection(T, [cdf[min(j + 1, len(T) - 1)] for j in range(len(T))])))(*_cl_factors(T)),
     rand=lambda rng: {"T": _rand_tri(rng)},
     phrases=("Chain-ladder reserve", "Chain ladder reserve", "Chain-ladder ultimate losses"),
     examples=[_ex({"T": [[100.0, 150.0, 175.0], [110.0, 168.0, 0.0], [120.0, 0.0, 0.0]]}),
               _ex({"T": [[1000.0, 1800.0, 2100.0, 2200.0], [1100.0, 2000.0, 2300.0, 0.0], [1200.0, 2100.0, 0.0, 0.0], [1300.0, 0.0, 0.0, 0.0]]})],
     level="intermediate")


def chain_ladder_simple(T):
    _tri(T)
    f, cdf = _cl_factors(T, simple=True)
    latest, ult = _cl_projection(T, cdf)
    tot = 0.0
    for u, l in zip(ult, latest):
        tot += u - l
    return _out(ldf=f, ultimate=ult, total_reserve=tot)


def _ref_chain_ladder_simple(T):
    cl, tri = _triangle(T)
    dev = cl.Development(average="simple").fit(tri)
    m = cl.Chainladder().fit(dev.transform(tri))
    ult = [float(x) for x in m.ultimate_.values.ravel()]
    latest = [float(x) for x in tri.latest_diagonal.values.ravel()]
    return {"ldf": [float(x) for x in dev.ldf_.values.ravel()], "ultimate": ult, "total_reserve": float(sum(u - l for u, l in zip(ult[1:], latest[1:])))}


_add("act_chain_ladder_simple_average",
     name="Chain-ladder with simple-average development factors", topic="loss reserving / chain-ladder",
     purpose="The simple-average age-to-age factors (mean of the individual link ratios), the ultimate losses of each origin year and the total reserve of a cumulative loss triangle (no tail factor).",
     inputs=[_TRI_IN],
     outputs=[_o("ldf", "simple-average age-to-age factors (a list)"), _o("ultimate", "ultimate losses of each origin year (a list)"), _o("total_reserve", "total reserve")],
     primary="total_reserve", conditions=["square triangle with N >= 2", "known cells positive, unknown cells 0", "no tail factor"],
     method="f_j = mean over origins of C_(i,j+1)/C_(i,j); ultimate_i = latest_i x product of f",
     latex=r"\hat f_j = \frac{1}{n_j}\sum_i \frac{C_{i,j+1}}{C_{i,j}}",
     run=chain_ladder_simple, reference=_ref_chain_ladder_simple,
     wrong=lambda T: (lambda f, cdf: (lambda latest, ult: _out(ldf=f, ultimate=ult, total_reserve=sum(u - l for u, l in zip(ult, latest))))(*_cl_projection(T, cdf)))(*_cl_factors(T)),
     rand=lambda rng: {"T": _rand_tri(rng)},
     phrases=("Simple average chain-ladder", "Chain-ladder with simple average factors"),
     examples=[_ex({"T": [[100.0, 150.0, 175.0], [110.0, 168.0, 0.0], [120.0, 0.0, 0.0]]}),
               _ex({"T": [[1000.0, 1800.0, 2100.0, 2200.0], [1100.0, 2000.0, 2300.0, 0.0], [1200.0, 2100.0, 0.0, 0.0], [1300.0, 0.0, 0.0, 0.0]]})],
     level="advanced")


def _prem_check(P, N):
    _need(len(P) == N, "give one premium (exposure) for every origin year")
    for v in P:
        _need(_isnum(v) and v > 0, "every premium must be positive")


def bornhuetter_ferguson(T, P, elr):
    N = _tri(T)
    _prem_check(P, N)
    _need(_isnum(elr) and 0 < elr <= 5, "the expected loss ratio must lie between 0 and 500%")
    f, cdf = _cl_factors(T)
    latest = [T[i][N - 1 - i] for i in range(N)]
    res = [P[i] * elr * (1.0 - 1.0 / cdf[N - 1 - i]) for i in range(N)]
    tot = 0.0
    for v in res:
        tot += v
    return _out(ultimate=[latest[i] + res[i] for i in range(N)], reserve=res, total_reserve=tot)


def _ref_bornhuetter_ferguson(T, P, elr):
    import pandas as pd
    cl, tri = _triangle(T)
    N = len(T)
    ep = cl.Triangle(pd.DataFrame({"origin": [str(2000 + i) for i in range(N)], "dev": [str(2000 + N - 1)] * N, "p": [float(x) for x in P]}),
                     origin="origin", development="dev", columns="p", cumulative=True)
    m = cl.BornhuetterFerguson(apriori=elr).fit(tri, sample_weight=ep.latest_diagonal)
    ult = [float(x) for x in m.ultimate_.values.ravel()]
    latest = [float(x) for x in tri.latest_diagonal.values.ravel()]
    res = [u - l for u, l in zip(ult, latest)]
    return {"ultimate": ult, "reserve": res, "total_reserve": float(sum(res))}


_add("act_bornhuetter_ferguson",
     name="Bornhuetter-Ferguson reserve", topic="loss reserving / Bornhuetter-Ferguson",
     purpose="The Bornhuetter-Ferguson reserve and ultimate losses of each origin year from a cumulative loss triangle, the earned premiums and an expected loss ratio: the unreported part (1 - 1/CDF) of the expected losses, with chain-ladder volume-weighted factors.",
     inputs=[_TRI_IN, _vecin("P", "earned premium of each origin year (a list)", 2, 30), _numin("elr", "expected (a priori) loss ratio (70% or 0.7)", pct=True)],
     outputs=[_o("ultimate", "ultimate losses of each origin year (a list)"), _o("reserve", "reserve of each origin year (a list)"), _o("total_reserve", "total reserve")],
     primary="total_reserve", conditions=["square triangle", "one positive premium per origin year", "0 < ELR <= 500%"],
     method="reserve_i = P_i ELR (1 - 1/CDF_i); ultimate_i = latest_i + reserve_i",
     latex=r"R_i^{BF} = P_i\cdot\mathrm{ELR}\left(1-\frac{1}{F_{N-i}}\right),\qquad \hat U_i = C_{i,N-i} + R_i^{BF}",
     run=bornhuetter_ferguson, reference=_ref_bornhuetter_ferguson,
     wrong=lambda T, P, elr: (lambda N, cdf: (lambda res: _out(ultimate=[T[i][N - 1 - i] + res[i] for i in range(N)], reserve=res, total_reserve=sum(res)))([P[i] * elr * (cdf[N - 1 - i] - 1) for i in range(N)]))(len(T), _cl_factors(T)[1]),
     rand=lambda rng: (lambda T: {"T": T, "P": [float(round(T[i][0] * rng.uniform(2.0, 4.0))) for i in range(len(T))], "elr": _sig(rng.uniform(0.5, 0.9), 2)})(_rand_tri(rng)),
     phrases=("Bornhuetter-Ferguson reserve", "Bornhuetter-Ferguson method"),
     examples=[_ex({"T": [[100.0, 150.0, 175.0], [110.0, 168.0, 0.0], [120.0, 0.0, 0.0]], "P": [200.0, 210.0, 220.0], "elr": 0.8}),
               _ex({"T": [[1000.0, 1800.0, 2100.0, 2200.0], [1100.0, 2000.0, 2300.0, 0.0], [1200.0, 2100.0, 0.0, 0.0], [1300.0, 0.0, 0.0, 0.0]], "P": [3000.0, 3200.0, 3400.0, 3600.0], "elr": 0.7})],
     level="advanced")


def cape_cod(T, P):
    N = _tri(T)
    _prem_check(P, N)
    f, cdf = _cl_factors(T)
    latest = [T[i][N - 1 - i] for i in range(N)]
    a = b = 0.0
    for i in range(N):
        a += latest[i]
        b += P[i] / cdf[N - 1 - i]
    elr = a / b
    res = [P[i] * elr * (1.0 - 1.0 / cdf[N - 1 - i]) for i in range(N)]
    tot = 0.0
    for v in res:
        tot += v
    return _out(elr=elr, ultimate=[latest[i] + res[i] for i in range(N)], reserve=res, total_reserve=tot)


def _ref_cape_cod(T, P):
    import pandas as pd
    cl, tri = _triangle(T)
    N = len(T)
    ep = cl.Triangle(pd.DataFrame({"origin": [str(2000 + i) for i in range(N)], "dev": [str(2000 + N - 1)] * N, "p": [float(x) for x in P]}),
                     origin="origin", development="dev", columns="p", cumulative=True)
    m = cl.CapeCod(trend=0.0, decay=1.0).fit(tri, sample_weight=ep.latest_diagonal)
    ult = [float(x) for x in m.ultimate_.values.ravel()]
    latest = [float(x) for x in tri.latest_diagonal.values.ravel()]
    res = [u - l for u, l in zip(ult, latest)]
    return {"elr": float(m.apriori_.values.ravel()[0]), "ultimate": ult, "reserve": res, "total_reserve": float(sum(res))}


_add("act_cape_cod",
     name="Cape Cod (Stanard-Buhlmann) reserve", topic="loss reserving / Cape Cod",
     purpose="The Cape Cod expected loss ratio (reported losses over used-up premium), and the resulting reserves and ultimate losses of each origin year, from a cumulative loss triangle and the earned premiums.",
     inputs=[_TRI_IN, _vecin("P", "earned premium of each origin year (a list)", 2, 30)],
     outputs=[_o("elr", "Cape Cod expected loss ratio"), _o("ultimate", "ultimate losses of each origin year (a list)"),
              _o("reserve", "reserve of each origin year (a list)"), _o("total_reserve", "total reserve")],
     primary="total_reserve", conditions=["square triangle", "one positive premium per origin year", "no trend, no decay"],
     method="ELR = sum latest_i / sum (P_i / CDF_i); reserve_i = P_i ELR (1 - 1/CDF_i)",
     latex=r"\mathrm{ELR}_{CC} = \frac{\sum_i C_{i,N-i}}{\sum_i P_i/F_{N-i}},\qquad R_i = P_i\,\mathrm{ELR}_{CC}\left(1-\frac{1}{F_{N-i}}\right)",
     run=cape_cod, reference=_ref_cape_cod,
     wrong=lambda T, P: (lambda o: dict(o, elr=sum(T[i][len(T) - 1 - i] for i in range(len(T))) / sum(P), total_reserve=o["total_reserve"] * sum(T[i][len(T) - 1 - i] for i in range(len(T))) / sum(P) / o["elr"]))(cape_cod(T, P)),
     rand=lambda rng: (lambda T: {"T": T, "P": [float(round(T[i][0] * rng.uniform(2.0, 4.0))) for i in range(len(T))]})(_rand_tri(rng)),
     phrases=("Cape Cod reserve", "Cape Cod method", "Stanard-Buhlmann reserve"),
     examples=[_ex({"T": [[100.0, 150.0, 175.0], [110.0, 168.0, 0.0], [120.0, 0.0, 0.0]], "P": [200.0, 210.0, 220.0]}),
               _ex({"T": [[1000.0, 1800.0, 2100.0, 2200.0], [1100.0, 2000.0, 2300.0, 0.0], [1200.0, 2100.0, 0.0, 0.0], [1300.0, 0.0, 0.0, 0.0]], "P": [3000.0, 3200.0, 3400.0, 3600.0]})],
     level="advanced")


def mack_standard_error(T):
    N = _tri(T)
    _need(N >= 4, "Mack's method needs at least 4 origin years")
    f, cdf = _cl_factors(T)
    s2 = []
    for j in range(N - 1):
        nj = N - 1 - j
        if nj >= 2:
            acc = 0.0
            for i in range(nj):
                acc += T[i][j] * (T[i][j + 1] / T[i][j] - f[j]) ** 2
            s2.append(acc / (nj - 1))
        else:
            a, b = s2[-1], s2[-2]
            s2.append(min(a * a / b, a, b) if b > 0 else 0.0)
    S = []
    for k in range(N - 1):
        acc = 0.0
        for i in range(N - 1 - k):
            acc += T[i][k]
        S.append(acc)
    full = []                                            # completed triangle
    for i in range(N):
        row = [T[i][j] for j in range(N - i)]
        for j in range(N - i, N):
            row.append(row[-1] * f[j - 1])
        full.append(row)
    mse = []
    for i in range(N):
        acc = 0.0
        for k in range(N - 1 - i, N - 1):
            acc += s2[k] / (f[k] * f[k]) * (1.0 / full[i][k] + 1.0 / S[k])
        mse.append(full[i][N - 1] ** 2 * acc)
    tot = 0.0
    for i in range(N):
        later = 0.0
        for j in range(i + 1, N):
            later += full[j][N - 1]
        acc = 0.0
        for k in range(N - 1 - i, N - 1):
            acc += 2.0 * s2[k] / (f[k] * f[k]) / S[k]
        tot += mse[i] + full[i][N - 1] * later * acc
    return _out(se=[math.sqrt(m_) for m_ in mse], total_se=math.sqrt(tot), sigma2=s2)


def _ref_mack_standard_error(T):
    cl, tri = _triangle(T)
    dev = cl.Development(average="volume", sigma_interpolation="mack").fit_transform(tri)
    m = cl.MackChainladder().fit(dev)
    # the last development age (column N-1): chainladder's extra 'ultimate' column re-scales the error when the last
    # factor is below 1, although no tail factor is applied
    se = [float(x) for x in m.mack_std_err_.values[0, 0, :, len(T) - 1]]
    se = [0.0 if (x != x) else x for x in se]
    tot = float(m.total_mack_std_err_.values.ravel()[-1])
    s2 = [float(x) ** 2 for x in dev.sigma_.values.ravel()]
    return {"se": se, "total_se": tot, "sigma2": s2}


_add("act_mack_standard_error",
     name="Mack's standard error of chain-ladder reserves", topic="loss reserving / Mack chain-ladder",
     purpose="The standard errors (root mean squared errors of prediction) of the chain-ladder reserve of each origin year and of the total reserve by Mack's distribution-free model, with the variance parameters sigma^2 of each development age.",
     inputs=[_TRI_IN],
     outputs=[_o("se", "standard error of each origin year's reserve (a list)"), _o("total_se", "standard error of the total reserve"),
              _o("sigma2", "Mack variance parameters sigma^2 of each age (a list)")],
     primary="total_se", conditions=["square triangle with N >= 4", "known cells positive, unknown cells 0", "last sigma^2 by Mack's extrapolation"],
     method="sigma_k^2 = sum C_ik (C_(i,k+1)/C_ik - f_k)^2/(n_k - 1); mse(R_i) = C_iN^2 sum_k sigma_k^2/f_k^2 (1/C_ik + 1/S_k); total adds the covariance terms",
     latex=r"\widehat{\mathrm{mse}}(\hat R_i) = \hat C_{iN}^2\sum_{k=N-i}^{N-1}\frac{\hat\sigma_k^2}{\hat f_k^2}\left(\frac{1}{\hat C_{ik}} + \frac{1}{\sum_{j} C_{jk}}\right)",
     run=mack_standard_error, reference=_ref_mack_standard_error,
     wrong=lambda T: (lambda o: dict(o, total_se=math.sqrt(sum(s * s for s in o["se"]))))(mack_standard_error(T)),
     rand=lambda rng: {"T": _rand_tri(rng, rng.randint(4, 6))},
     phrases=("Mack standard error", "Mack chain-ladder standard error"),
     examples=[_ex({"T": [[1000.0, 1800.0, 2100.0, 2200.0], [1100.0, 2000.0, 2300.0, 0.0], [1200.0, 2100.0, 0.0, 0.0], [1300.0, 0.0, 0.0, 0.0]]}),
               _ex({"T": [[500.0, 900.0, 1100.0, 1150.0, 1160.0], [550.0, 1000.0, 1250.0, 1300.0, 0.0], [600.0, 1050.0, 1350.0, 0.0, 0.0], [620.0, 1200.0, 0.0, 0.0, 0.0], [700.0, 0.0, 0.0, 0.0, 0.0]]})],
     level="expert")


def expected_claims(P, elr, paid):
    _need(len(P) == len(paid) and len(P) >= 1, "give one premium and one paid amount for every origin year")
    _need(_isnum(elr) and 0 < elr <= 5, "the expected loss ratio must lie between 0 and 500%")
    ult, res = [], []
    tot = 0.0
    for p, c in zip(P, paid):
        _need(_isnum(p) and p > 0, "every premium must be positive")
        _need(_isnum(c) and c >= 0, "every paid amount must be zero or positive")
        u = p * elr
        ult.append(u)
        res.append(u - c)
        tot += u - c
    return _out(ultimate=ult, reserve=res, total_reserve=tot)


def _ref_expected_claims(P, elr, paid):
    import numpy as np
    u = np.array(P, dtype=float) * elr
    r = u - np.array(paid, dtype=float)
    return {"ultimate": [float(x) for x in u], "reserve": [float(x) for x in r], "total_reserve": float(r.sum())}


_add("act_expected_claims_method",
     name="Expected claims reserving method", topic="loss reserving / expected claims",
     purpose="The ultimate losses (premium x expected loss ratio), the reserve (ultimate less paid or reported to date) of each origin year and the total reserve by the expected claims method.",
     inputs=[_vecin("P", "earned premium of each origin year (a list)", 1, 30), _numin("elr", "expected loss ratio (65% or 0.65)", pct=True),
             _vecin("paid", "losses paid (or reported) to date of each origin year (a list)", 1, 30)],
     outputs=[_o("ultimate", "expected ultimate losses (a list)"), _o("reserve", "reserve of each origin year (a list)"), _o("total_reserve", "total reserve")],
     primary="total_reserve", conditions=["one paid amount per premium", "0 < ELR <= 500%"],
     method="ultimate_i = P_i x ELR; reserve_i = ultimate_i - paid_i",
     latex=r"\hat U_i = P_i\cdot\mathrm{ELR},\qquad R_i = \hat U_i - C_i",
     run=expected_claims, reference=_ref_expected_claims,
     wrong=lambda P, elr, paid: (lambda u: _out(ultimate=u, reserve=[x - c for x, c in zip(u, paid)], total_reserve=sum(x - c for x, c in zip(u, paid))))([p / elr for p in P]),
     rand=lambda rng: (lambda n: (lambda P: {"P": P, "elr": _sig(rng.uniform(0.5, 0.9), 2), "paid": [float(round(p * rng.uniform(0.1, 0.6))) for p in P]})([float(round(rng.uniform(1e3, 1e5))) for _ in range(n)]))(rng.randint(1, 8)),
     phrases=("Expected claims method", "Expected claims reserve"),
     examples=[_ex({"P": [1000.0, 1100.0, 1200.0], "elr": 0.7, "paid": [650.0, 500.0, 300.0]}), _ex({"P": [5000.0], "elr": 0.65, "paid": [1000.0]})],
     level="beginner",
     weak="the same definitions by numpy vector arithmetic")


def loss_ratio_rate_change(L, EP, F, V, Q):
    _nonneg("the losses", L)
    _pos("the earned premium at current rates", EP)
    _frac("the fixed expense ratio F", F, hi_open=True)
    _frac("the variable expense ratio V", V, hi_open=True)
    _need(_isnum(Q) and -0.5 < Q < 1, "the profit provision Q must lie between -50% and 100%")
    perm = 1.0 - V - Q
    _need(perm > 0, "the variable expenses and profit leave nothing for losses")
    lr = L / EP
    return _out(change=(lr + F) / perm - 1.0, loss_ratio=lr, permissible=perm)


def _ref_loss_ratio_rate_change(L, EP, F, V, Q):
    from fractions import Fraction as Fr
    # pure premium method with one unit of exposure: indicated rate / current average rate - 1
    pp, fixed = Fr(L), Fr(F) * Fr(EP)
    rate = (pp + fixed) / (1 - Fr(V) - Fr(Q))
    return {"change": float(rate / Fr(EP) - 1), "loss_ratio": float(Fr(L) / Fr(EP)), "permissible": float(1 - Fr(V) - Fr(Q))}


_add("act_loss_ratio_rate_change",
     name="Indicated rate change by the loss ratio method", topic="ratemaking / loss ratio method",
     purpose="The indicated rate change from projected losses (with loss adjustment expenses), earned premium at current rates, the fixed expense ratio, the variable expense ratio and the profit provision.",
     inputs=[_numin("L", "projected ultimate losses and LAE"), _numin("EP", "earned premium at current rates"),
             _numin("F", "fixed expense ratio (to premium)", pct=True), _numin("V", "variable expense ratio", pct=True), _numin("Q", "profit and contingencies provision", pct=True)],
     outputs=[_o("change", "indicated rate change (0.05 = +5%)"), _o("loss_ratio", "projected loss ratio L/EP"), _o("permissible", "permissible loss ratio 1 - V - Q")],
     primary="change", conditions=["EP > 0", "0 <= F, V < 1", "V + Q < 1"],
     method="indicated change = (L/EP + F)/(1 - V - Q) - 1",
     latex=r"\Delta = \frac{L/EP + F}{1 - V - Q} - 1",
     run=loss_ratio_rate_change, reference=_ref_loss_ratio_rate_change,
     wrong=lambda L, EP, F, V, Q: _out(change=(L / EP) / (1 - V - Q - F) - 1, loss_ratio=L / EP, permissible=1 - V - Q),
     rand=lambda rng: (lambda EP: {"L": _sig(EP * rng.uniform(0.4, 0.9), 4), "EP": EP, "F": _sig(rng.uniform(0.03, 0.15), 2), "V": _sig(rng.uniform(0.1, 0.25), 2), "Q": _sig(rng.uniform(0.0, 0.08), 2)})(_sig(_lu(rng, 1e4, 1e8), 4)),
     phrases=("Loss ratio rate change", "Indicated rate change by the loss ratio method"),
     examples=[_ex({"L": 650000.0, "EP": 1000000.0, "F": 0.1, "V": 0.15, "Q": 0.05}), _ex({"L": 75000.0, "EP": 100000.0, "F": 0.05, "V": 0.2, "Q": 0.03})],
     level="beginner",
     weak="the pure premium form of the same indication in exact fractions")


def pure_premium_rate(L, X, Fe, V, Q):
    _nonneg("the losses and the fixed expense per exposure", L, Fe)
    _pos("the exposures", X)
    _frac("the variable expense ratio V", V, hi_open=True)
    _need(_isnum(Q) and -0.5 < Q < 1, "the profit provision Q must lie between -50% and 100%")
    perm = 1.0 - V - Q
    _need(perm > 0, "the variable expenses and profit leave nothing for losses")
    pp = L / X
    return _out(pure_premium=pp, rate=(pp + Fe) / perm)


def _ref_pure_premium_rate(L, X, Fe, V, Q):
    from fractions import Fraction as Fr
    pp = Fr(L) / Fr(X)
    return {"pure_premium": float(pp), "rate": float((pp + Fr(Fe)) / (1 - Fr(V) - Fr(Q)))}


_add("act_pure_premium_rate",
     name="Indicated rate by the pure premium method", topic="ratemaking / pure premium method",
     purpose="The pure premium (losses per exposure) and the indicated rate per exposure from projected losses, exposures, the fixed expense per exposure, the variable expense ratio and the profit provision.",
     inputs=[_numin("L", "projected ultimate losses and LAE"), _numin("X", "earned exposures"), _numin("Fe", "fixed expense per exposure"),
             _numin("V", "variable expense ratio", pct=True), _numin("Q", "profit and contingencies provision", pct=True)],
     outputs=[_o("pure_premium", "pure premium L/X"), _o("rate", "indicated rate per exposure (PP + F)/(1 - V - Q)")],
     primary="rate", conditions=["X > 0", "0 <= V < 1", "V + Q < 1"],
     method="rate = (L/X + F)/(1 - V - Q)",
     latex=r"R = \frac{L/X + F}{1 - V - Q}",
     run=pure_premium_rate, reference=_ref_pure_premium_rate,
     wrong=lambda L, X, Fe, V, Q: _out(pure_premium=L / X, rate=L / X / (1 - V - Q) + Fe * (1 - V - Q)),
     rand=lambda rng: {"L": _sig(_lu(rng, 1e4, 1e8), 4), "X": float(rng.randint(100, 100000)), "Fe": _sig(rng.uniform(5, 100), 3), "V": _sig(rng.uniform(0.1, 0.25), 2), "Q": _sig(rng.uniform(0.0, 0.08), 2)},
     phrases=("Pure premium rate", "Indicated rate by the pure premium method"),
     examples=[_ex({"L": 500000.0, "X": 2000.0, "Fe": 30.0, "V": 0.15, "Q": 0.05}), _ex({"L": 90000.0, "X": 600.0, "Fe": 20.0, "V": 0.2, "Q": 0.0})],
     level="beginner",
     weak="the same definitions in exact fractions")


# ================================================================================================ SURVIVAL ESTIMATION
def _surv_check(times, events):
    _need(len(times) == len(events) and len(times) >= 1, "give one event indicator for every time")
    for t in times:
        _need(_isnum(t) and t > 0, "every time must be positive")
    for e in events:
        _need(e == 0 or e == 1, "every event indicator must be 1 (death or failure) or 0 (censored)")


def _risk_table(times, events, t):
    """distinct event times up to t with deaths d_j and numbers at risk n_j (censored at t_j still at risk)"""
    ts = sorted(set([times[k] for k in range(len(times)) if events[k] == 1 and times[k] <= t]))
    out = []
    for tj in ts:
        d = 0
        n = 0
        for k in range(len(times)):
            if times[k] >= tj:
                n += 1
            if times[k] == tj and events[k] == 1:
                d += 1
        out.append((tj, d, n))
    return out


def kaplan_meier(times, events, t):
    _surv_check(times, events)
    _need(_isnum(t) and t >= 0, "the time t must be zero or positive")
    S = 1.0
    deaths = 0
    for tj, d, n in _risk_table(times, events, t):
        S *= 1.0 - d / n
        deaths += d
    return _out(S=S, F=1.0 - S, deaths=deaths)


def _ref_kaplan_meier(times, events, t):
    lf = _import_quiet("lifelines")
    km = lf.KaplanMeierFitter().fit(times, event_observed=events)
    S = float(km.survival_function_at_times(t).iloc[0])
    return {"S": S, "F": 1.0 - S, "deaths": int(sum(1 for a, b in zip(times, events) if b == 1 and a <= t))}


_add("act_kaplan_meier",
     name="Kaplan-Meier product-limit survival estimate", topic="survival estimation / Kaplan-Meier",
     purpose="The Kaplan-Meier estimate of the survival function at time t from observed times with death (1) or censoring (0) indicators; a life censored at a death time is counted at risk then.",
     inputs=[_vecin("times", "observed times (a list)", 1, 500), _vecin("events", "1 for a death or failure, 0 for censoring (a list)", 1, 500), _numin("t", "time at which the survival is wanted")],
     outputs=[_o("S", "Kaplan-Meier survival estimate S(t)"), _o("F", "estimated probability of death by t"), _o("deaths", "number of deaths observed up to t")],
     primary="S", conditions=["one indicator per time", "positive times", "indicators 0 or 1"],
     method="S(t) = product over death times t_j <= t of (1 - d_j/n_j)",
     latex=r"\hat S(t) = \prod_{t_j\le t}\left(1-\frac{d_j}{n_j}\right)",
     run=kaplan_meier, reference=_ref_kaplan_meier,
     wrong=lambda times, events, t: (lambda o: dict(o, S=1.0 - o["deaths"] / len(times), F=o["deaths"] / len(times)))(kaplan_meier(times, events, t)),
     rand=lambda rng: (lambda n: (lambda ts: {"times": ts, "events": [float(rng.random() < 0.7) for _ in range(n)], "t": _sig(sorted(ts)[rng.randint(n // 3, n - 1)], 4)})([float(rng.randint(1, 30)) for _ in range(n)]))(rng.randint(6, 14)),
     phrases=("Kaplan-Meier estimate", "Kaplan-Meier survival", "Product-limit estimate"),
     examples=[_ex({"times": [1.0, 2.0, 2.0, 3.0, 5.0], "events": [1.0, 1.0, 0.0, 1.0, 0.0], "t": 3.0}),
               _ex({"times": [2.0, 3.0, 3.0, 5.0, 6.0, 8.0, 9.0], "events": [1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0], "t": 6.0})],
     level="intermediate",
     extra_q=[{"q": "Kaplan-Meier estimate: times = [1, 2, 3], events = [1, 2, 0], t = 2", "outcome": "INVALID_INPUT"}])


def nelson_aalen(times, events, t):
    _surv_check(times, events)
    _need(_isnum(t) and t >= 0, "the time t must be zero or positive")
    H = 0.0
    for tj, d, n in _risk_table(times, events, t):
        H += d / n
    return _out(H=H, S=math.exp(-H))


def _ref_nelson_aalen(times, events, t):
    lf = _import_quiet("lifelines")
    na = lf.NelsonAalenFitter(nelson_aalen_smoothing=False).fit(times, event_observed=events)
    H = float(na.cumulative_hazard_at_times(t).iloc[0])
    return {"H": H, "S": math.exp(-H)}


_add("act_nelson_aalen",
     name="Nelson-Aalen cumulative hazard estimate", topic="survival estimation / Nelson-Aalen",
     purpose="The Nelson-Aalen estimate of the cumulative hazard at time t from observed times with death (1) or censoring (0) indicators, and the survival estimate exp(-H(t)).",
     inputs=[_vecin("times", "observed times (a list)", 1, 500), _vecin("events", "1 for a death or failure, 0 for censoring (a list)", 1, 500), _numin("t", "time at which the estimate is wanted")],
     outputs=[_o("H", "Nelson-Aalen cumulative hazard H(t)"), _o("S", "survival estimate exp(-H(t))")],
     primary="H", conditions=["one indicator per time", "positive times", "indicators 0 or 1"],
     method="H(t) = sum over death times t_j <= t of d_j/n_j",
     latex=r"\hat H(t) = \sum_{t_j\le t}\frac{d_j}{n_j},\quad \hat S(t) = e^{-\hat H(t)}",
     run=nelson_aalen, reference=_ref_nelson_aalen,
     wrong=lambda times, events, t: (lambda H: _out(H=H, S=math.exp(-H)))(sum(d / (n + 1) for _, d, n in _risk_table(times, events, t))),
     rand=lambda rng: (lambda n: (lambda ts: {"times": ts, "events": [float(rng.random() < 0.7) for _ in range(n)], "t": _sig(sorted(ts)[rng.randint(n // 3, n - 1)], 4)})([float(rng.randint(1, 30)) for _ in range(n)]))(rng.randint(6, 14)),
     phrases=("Nelson-Aalen estimate", "Nelson-Aalen cumulative hazard"),
     examples=[_ex({"times": [1.0, 2.0, 2.0, 3.0, 5.0], "events": [1.0, 1.0, 0.0, 1.0, 0.0], "t": 3.0}),
               _ex({"times": [2.0, 3.0, 3.0, 5.0, 6.0, 8.0, 9.0], "events": [1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0], "t": 6.0})],
     level="intermediate")


def exponential_mle(times, events, t):
    _surv_check(times, events)
    _need(_isnum(t) and t >= 0, "the time t must be zero or positive")
    d = 0.0
    tot = 0.0
    for a, b in zip(times, events):
        d += b
        tot += a
    _need(d > 0, "at least one death is needed to estimate the hazard")
    lam = d / tot
    return _out(hazard=lam, mean=1.0 / lam, S=math.exp(-lam * t))


def _ref_exponential_mle(times, events, t):
    lf = _import_quiet("lifelines")
    ef = lf.ExponentialFitter().fit(times, event_observed=events)
    m_lib = float(ef.lambda_)                 # lifelines' lambda_ is the scale (mean), fitted by a numerical optimizer
    # full precision: the maximum of the censored log-likelihood by brentq on its derivative (numerical difference
    # free: the score is written from the log-likelihood with sympy)
    import sympy as sp
    L = sp.symbols("L", positive=True)
    score = sp.lambdify(L, sp.diff(sum(sp.Float(e) * sp.log(L) for e in events) - L * sp.Float(sum(times)), L), "math")
    lam = _brentq(score, 1e-12, 1e6)
    if abs(1.0 / lam - m_lib) > 1e-4 * m_lib:
        raise RuntimeError("lifelines and the likelihood maximum disagree")
    return {"hazard": lam, "mean": 1.0 / lam, "S": math.exp(-lam * t)}


_add("act_exponential_mle",
     name="Exponential maximum-likelihood fit with censoring", topic="survival estimation / parametric fitting",
     purpose="The maximum-likelihood constant hazard (deaths divided by total time observed), the mean lifetime and the fitted survival probability at time t from times with death (1) or censoring (0) indicators.",
     inputs=[_vecin("times", "observed times (a list)", 1, 500), _vecin("events", "1 for a death or failure, 0 for censoring (a list)", 1, 500), _numin("t", "time for the fitted survival probability")],
     outputs=[_o("hazard", "maximum-likelihood hazard rate (deaths / total exposure)"), _o("mean", "fitted mean lifetime"), _o("S", "fitted survival probability exp(-lambda t)")],
     primary="hazard", conditions=["one indicator per time", "positive times", "at least one death"],
     method="lambda = sum of death indicators / sum of times; S(t) = e^(-lambda t)",
     latex=r"\hat\lambda = \frac{\sum_i \delta_i}{\sum_i t_i},\quad \hat S(t) = e^{-\hat\lambda t}",
     run=exponential_mle, reference=_ref_exponential_mle,
     wrong=lambda times, events, t: (lambda lam: _out(hazard=lam, mean=1 / lam, S=math.exp(-lam * t)))(len(times) / sum(times)) if sum(events) < len(events) else (lambda lam: _out(hazard=lam, mean=1 / lam, S=math.exp(-lam * t)))(sum(events) / sum(times) * 1.1),
     rand=lambda rng: (lambda n: {"times": [_sig(rng.expovariate(0.1) + 0.01, 3) for _ in range(n)], "events": [1.0] + [float(rng.random() < 0.7) for _ in range(n - 1)], "t": _sig(rng.uniform(1, 20), 3)})(rng.randint(3, 14)),
     phrases=("Exponential maximum likelihood fit", "Exponential survival fit"),
     examples=[_ex({"times": [1.0, 2.0, 2.0, 3.0, 5.0], "events": [1.0, 1.0, 0.0, 1.0, 0.0], "t": 3.0}),
               _ex({"times": [10.0, 4.0, 7.5, 12.0], "events": [1.0, 1.0, 1.0, 1.0], "t": 5.0})],
     level="intermediate",
     weak="lifelines' ExponentialFitter agrees only to its optimizer's tolerance (checked to 1e-4); the compared value is the root of the likelihood score by brentq")

# ================================================================================================ worked example values
# every documented example's outputs, each checked against its reference computation on the box (scipy, lifelines,
# chainladder) before it was written here
_EXV = {
    'act_interest_rate_conversion': [{'d': 0.047619047619, 'v': 0.952380952381, 'delta': 0.0487901641694, 'i_m': 0.0488894854038, 'd_m': 0.0486911117872}, {'d': 0.0740740740741, 'v': 0.925925925926, 'delta': 0.0769610411361, 'i_m': 0.0777061876331, 'd_m': 0.0762253914897}],
    'act_nominal_to_effective_rate': [{'i': 0.0616778118645, 'd': 0.0580946603341, 'delta': 0.0598504981325, 'v': 0.941905339666}, {'i': 0.0841657847339, 'd': 0.07763184, 'delta': 0.0808108292701, 'v': 0.92236816}],
    'act_force_of_interest': [{'i': 0.051271096376, 'd': 0.0487705754993, 'v': 0.951229424501}, {'i': 0.105170918076, 'd': 0.095162581964, 'v': 0.904837418036}],
    'act_accumulated_value': [{'AV': 1628.89462678, 'PV': 613.913253541, 'factor': 1.62889462678}, {'AV': 3167.98583141, 'PV': 1972.8623588, 'factor': 1.26719433256}],
    'act_annuity_certain': [{'a_n': 7.72173492918, 'a_due': 8.10782167564, 's_n': 12.5778925355, 's_due': 13.2067871623, 'a_cont': 7.91320859505}, {'a_n': 10.6747761886, 'a_due': 11.5287582837, 's_n': 73.1059399527, 's_due': 78.954415149, 'a_cont': 11.0962908308}],
    'act_perpetuity': [{'a_inf': 20.0, 'a_due_inf': 21.0, 'a_cont_inf': 20.4959343143}, {'a_inf': 25.0, 'a_due_inf': 26.0, 'a_cont_inf': 25.496731691}],
    'act_increasing_decreasing_annuity': [{'Ia': 39.3737828047, 'Ia_due': 41.342471945, 'Da': 45.5653014163, 'Da_due': 47.8435664871}, {'Ia': 10.6525883105, 'Ia_due': 11.7178471416, 'Da': 12.0921323059, 'Da_due': 13.3013455365}],
    'act_loan_amortization': [{'R': 8718.45569769, 'interest_t': 5286.46801258, 'principal_t': 3431.98768511, 'balance_t': 84675.8125245, 'total_interest': 74369.1139537}, {'R': 1577.35401853, 'interest_t': 392.264598147, 'principal_t': 1185.08942038, 'balance_t': 2737.55656109, 'total_interest': 1309.41607412}],
    'act_annuity_yield': [{'i': 0.0500001314491, 'a_n': 7.72173}, {'i': 0.0793082611605, 'a_n': 4.0}],
    'act_present_value_cash_flows': [{'PV': 535.795270489, 'AV_at_last': 620.25}, {'PV': 30.8387948992, 'AV_at_last': 38.848}],
    'act_duration_convexity': [{'PV': 100.0, 'macaulay': 2.85941043084, 'modified': 2.72324802937, 'convexity': 10.2056241998}, {'PV': 2275.68455067, 'macaulay': 6.74979130918, 'modified': 6.49018395114, 'convexity': 62.6361751289}],
    'act_life_table_functions': [{'dx': [50.0, 70.0, 100.0], 'qx': [0.05, 0.0736842105263, 0.113636363636], 'px': [0.95, 0.926315789474, 0.886363636364]}, {'dx': [500.0, 600.0, 700.0, 900.0], 'qx': [0.005, 0.00603015075377, 0.00707785642063, 0.0091649694501], 'px': [0.995, 0.993969849246, 0.992922143579, 0.99083503055]}],
    'act_survival_probability': [{'tpx': 0.821052631579, 'tqx': 0.178947368421, 'deaths': 170.0}, {'tpx': 0.973, 'tqx': 0.027, 'deaths': 2700.0}],
    'act_deferred_death_probability': [{'prob': 0.23, 'upx': 0.88}, {'prob': 0.0070351758794, 'upx': 0.993969849246}],
    'act_life_expectancy': [{'e_curtate': 2.0, 'e_complete': 2.5}, {'e_curtate': 0.571428571429, 'e_complete': 1.07142857143}],
    'act_temporary_expectation': [{'e_curtate': 1.6, 'e_complete': 1.75}, {'e_curtate': 3.939, 'e_complete': 3.9525}],
    'act_central_death_rate': [{'q': 0.019801980198, 'p': 0.980198019802}, {'q': 0.095162581964, 'p': 0.904837418036}],
    'act_fractional_age': [{'spx': 0.95, 'sqx': 0.05, 'mu': 0.105263157895}, {'spx': 0.974003746425, 'sqx': 0.0259962535747, 'mu': 0.105360515658}],
    'act_commutation_functions': [{'D_x': 9.7054703765, 'N_x': 27.5402284876, 'C_x': 0.924330512048, 'M_x': 8.39403092471, 'A_x': 0.864876260406, 'a_due_x': 2.83759853148}, {'D_x': 3.34872468872, 'N_x': 7.1304615608, 'C_x': 0.70203871881, 'M_x': 2.94511365698, 'A_x': 0.879473211667, 'a_due_x': 2.12930659388}],
    'act_pure_endowment': [{'nEx': 0.800489507973, 'accumulation': 1.24923561151}, {'nEx': 0.73073391917, 'accumulation': 1.36848717949}],
    'act_de_moivre_law': [{'tpx': 0.833333333333, 'tqx': 0.166666666667, 'mu': 0.02, 'e_complete': 30.0, 'var_T': 300.0}, {'tpx': 0.555555555556, 'tqx': 0.444444444444, 'mu': 0.04, 'e_complete': 22.5, 'var_T': 168.75}],
    'act_constant_force_mortality': [{'tpx': 0.818730753078, 'tqx': 0.181269246922, 'e_complete': 50.0, 'e_curtate': 49.5016666556, 'var_T': 2500.0}, {'tpx': 0.606530659713, 'tqx': 0.393469340287, 'e_complete': 10.0, 'e_curtate': 9.50833194478, 'var_T': 100.0}],
    'act_makeham_law': [{'tpx': 0.942549207986, 'tqx': 0.0574507920136, 'mu_x': 0.00322152827009, 'mu_xt': 0.00988063225618, 'e_complete': 27.2096866558}, {'tpx': 0.906731314902, 'tqx': 0.0932686850976, 'mu_x': 0.0022704710027, 'mu_xt': 0.00950156459801, 'e_complete': 40.0880114591}],
    'act_gompertz_law': [{'tpx': 0.881330429727, 'tqx': 0.118669570273, 'mu_x': 0.00883710751892, 'mu_xt': 0.0173839280504, 'e_complete': 26.6911436088}, {'tpx': 0.936483954641, 'tqx': 0.0635160453585, 'mu_x': 0.00966063225618, 'mu_xt': 0.0173315381172, 'e_complete': 18.5553790553}],
    'act_whole_life_insurance': [{'A': 0.865657827757, 'A2': 0.751089106428, 'EPV': 865.657827757, 'sd': 41.5407230434}, {'A': 0.795948501069, 'A2': 0.635963049602, 'EPV': 79594.8501069, 'sd': 4928.52234244}],
    'act_term_insurance': [{'A1': 0.258503401361, 'A1_2': 0.238789393308, 'EPV': 258.503401361, 'sd': 414.687092629}, {'A1': 0.0616160698981, 'A1_2': 0.0511563208338, 'EPV': 3080.8034949, 'sd': 10881.1512217}],
    'act_endowment_insurance': [{'A': 0.91156462585, 'A1': 0.258503401361, 'nEx': 0.65306122449, 'A2': 0.831135175158, 'EPV': 911.56462585, 'sd': 13.6054421769}, {'A': 0.753047927856, 'A1': 0.0616160698981, 'nEx': 0.691431857958, 'A2': 0.567834427673, 'EPV': 37652.3963928, 'sd': 1372.26639574}],
    'act_deferred_insurance': [{'uA': 0.770419732519, 'EPV': 770.419732519, 'uEx': 0.857142857143}, {'uA': 0.707281782248, 'EPV': 7072.81782248, 'uEx': 0.802825941625}],
    'act_whole_life_annuity': [{'a_due': 2.8211856171, 'a_imm': 1.8211856171, 'EPV': 2821.1856171, 'sd': 872.355183912}, {'a_due': 5.30533897221, 'a_imm': 4.30533897221, 'EPV': 63664.0676665, 'sd': 15376.9897084}],
    'act_temporary_annuity': [{'a_due': 2.51020408163, 'a_imm': 1.8211856171, 'EPV': 2510.20408163}, {'a_due': 4.36281994121, 'a_imm': 4.05425179916, 'EPV': 8725.63988241}],
    'act_deferred_annuity': [{'ua_due': 0.964042759961, 'EPV': 964.042759961}, {'ua_due': 2.48414814381, 'EPV': 12420.740719}],
    'act_continuous_insurance_udd': [{'A_bar': 0.887123298818, 'A': 0.865657827757, 'a_bar': 2.31351345304}, {'A_bar': 0.734604493985, 'A': 0.713410029458, 'a_bar': 4.55466756731}],
    'act_mthly_annuity_udd': [{'a_m': 2.3552334027, 'alpha': 1.00019701122, 'beta': 0.466508019623, 'a_due': 2.8211856171}, {'a_m': 4.92478780165, 'alpha': 1.00012018249, 'beta': 0.381188779391, 'a_due': 5.30533897221}],
    'act_woolhouse_approximation': [{'a_m3': 14.3538376253, 'a_m2': 14.3582666667}, {'a_m3': 9.61888524155, 'a_m2': 9.625}],
    'act_net_premium_whole_life': [{'P': 306.841854896, 'A': 0.865657827757, 'a_due': 2.8211856171}, {'P': 15002.7831443, 'A': 0.795948501069, 'a_due': 5.30533897221}],
    'act_net_premium_term': [{'P': 226.867982966, 'A1': 0.569484936832, 'a_due': 2.51020408163}, {'P': 706.149585915, 'A1': 0.0616160698981, 'a_due': 4.36281994121}],
    'act_net_premium_endowment': [{'P': 350.754936121, 'A': 0.880466472303, 'a_due': 2.51020408163}, {'P': 8630.28887285, 'A': 0.753047927856, 'a_due': 4.36281994121}],
    'act_net_premium_limited_pay': [{'P': 466.123445715, 'A': 0.865657827757, 'a_due_h': 1.85714285714}, {'P': 28213.2102889, 'A': 0.795948501069, 'a_due_h': 2.8211908284}],
    'act_gross_premium': [{'G': 391.044393734, 'P': 306.841854896, 'loading': 84.2025388389}, {'G': 17486.0533645, 'P': 15002.7831443, 'loading': 2483.27022028}],
    'act_loss_at_issue': [{'mean': -403.87569994, 'sd': 434.100555804, 'prob_loss': 0.28}, {'mean': 14.7655237545, 'sd': 24149.759478, 'prob_loss': 0.29379926}],
    'act_reserve_whole_life': [{'V': 246.871052934, 'P': 306.841854896, 'A_t': 0.898823021272, 'a_due_t': 2.12471655329}, {'V': 41676.5834164, 'P': 15002.7831443, 'A_t': 0.880990194233, 'a_due_t': 3.09425494993}],
    'act_reserve_endowment': [{'V': 298.10298103, 'P': 350.754936121}, {'V': 27922.3630553, 'P': 8630.28887285}],
    'act_reserve_term': [{'V': 43.2103528242, 'P': 33.7174063072}, {'V': 450.093537109, 'P': 706.149585915}],
    'act_retrospective_reserve': [{'V': 883.854166667, 'tEx': 0.65306122449}, {'V': 25737.2089127, 'tEx': 0.808929686923}],
    'act_policy_value_recursion': [{'V_next': 143.686868687}, {'V_next': 2775.46391753}],
    'act_gross_premium_reserve': [{'V': 148.731075614, 'G': 391.044393734}, {'V': 20741.8663466, 'G': 17486.0533645}],
    'act_thiele_endowment': [{'P': 42.9117572531, 'A_bar': 0.461854974244, 'a_bar': 10.7629005151, 'V': 331.812227832}, {'P': 5583.02987682, 'A_bar': 0.736253182107, 'a_bar': 6.59367044733, 'V': 10618.1073596}],
    'act_makeham_continuous_whole_life': [{'a_bar': 16.5203732076, 'A_bar': 0.193968279062, 'P': 1174.11560033, 'V': 12836.4717411}, {'a_bar': 19.309508675, 'A_bar': 0.242667299128, 'P': 12.5672435903, 'V': 55.1885530381}],
    'act_joint_life_last_survivor': [{'a_joint': 2.5587755102, 'a_last': 3.65664399093, 'A_joint': 0.878153547133, 'A_last': 0.82587409567}, {'a_joint': 4.81428240145, 'a_last': 6.20396161347, 'A_joint': 0.814835292252, 'A_last': 0.76138609179}],
    'act_reversionary_annuity': [{'a_rev': 0.835458373826, 'EPV': 835.458373826}, {'a_rev': 0.898622641259, 'EPV': 4493.11320629}],
    'act_multiple_decrement_to_single': [{'q1_single': 0.112095998257, 'q2_single': 0.211626483689, 'q_total': 0.3}, {'q1_single': 0.0216826050133, 'q2_single': 0.151604577151, 'q_total': 0.17}],
    'act_single_to_multiple_decrement': [{'q1': 0.09, 'q2': 0.19, 'q_total': 0.28}, {'q1': 0.0425, 'q2': 0.2925, 'q_total': 0.335}],
    'act_poisson_frequency': [{'pmf': 0.180447044315, 'cdf': 0.857123460499, 'mean': 2.0, 'var': 2.0}, {'pmf': 0.606530659713, 'cdf': 0.606530659713, 'mean': 0.5, 'var': 0.5}],
    'act_negative_binomial_frequency': [{'pmf': 0.13824, 'cdf': 0.66304, 'mean': 3.0, 'var': 7.5}, {'pmf': 0.4472135955, 'cdf': 0.4472135955, 'mean': 2.0, 'var': 10.0}],
    'act_compound_distribution_moments': [{'ES': 3000.0, 'VarS': 3750000.0}, {'ES': 5000.0, 'VarS': 6650000.0}],
    'act_compound_poisson_moments': [{'mean': 6.0, 'var': 18.0, 'skew': 0.942809041582}, {'mean': 1000.0, 'var': 200000.0, 'skew': 0.67082039325}],
    'act_individual_risk_model': [{'mean': 300.0, 'var': 1670000.0, 'prob_no_claim': 0.92169}, {'mean': 1050.0, 'var': 2807500.0, 'prob_no_claim': 0.6156}],
    'act_normal_approximation_aggregate': [{'prob_exceed': 0.0227501319482, 'z': 2.0}, {'prob_exceed': 0.158655253931, 'z': 1.0}],
    'act_panjer_poisson': [{'g': [0.135335283237, 0.135335283237, 0.14886881156, 0.157891163776, 0.124734019383, 0.0985691979573], 'F_smax': 0.80073375915}, {'g': [0.449328964117, 0.179731585647, 0.215677902776, 0.0766854765427, 0.0508041282095], 'F_smax': 0.972228057293}],
    'act_panjer_negative_binomial': [{'g': [0.25, 0.125, 0.121875, 0.121875, 0.0873828125, 0.07130859375], 'F_smax': 0.77744140625}, {'g': [0.620173672946, 0.0954113342994, 0.117429334522, 0.0496816415287, 0.0404749037629], 'F_smax': 0.923170887059}],
    'act_stop_loss_premium': [{'premium': 1.22554522774, 'ES': 3.4, 'limited': 2.17445477226}, {'premium': 0.278389513881, 'ES': 1.2, 'limited': 0.921610486119}],
    'act_exponential_deductible': [{'mean': 1000.0, 'lev_d': 393.469340287, 'per_loss': 606.530659713, 'per_payment': 1000.0, 'LER': 0.393469340287}, {'mean': 2500.0, 'lev_d': 237.90645491, 'per_loss': 2262.09354509, 'per_payment': 2500.0, 'LER': 0.095162581964}],
    'act_pareto_deductible': [{'mean': 1000.0, 'lev_d': 360.0, 'per_loss': 640.0, 'per_payment': 1250.0, 'LER': 0.36}, {'mean': 666.666666667, 'lev_d': 430.964406271, 'per_loss': 235.702260396, 'per_payment': 1333.33333333, 'LER': 0.646446609407}],
    'act_lognormal_deductible': [{'mean': 1808.04241446, 'lev_d': 785.105920222, 'per_loss': 1022.93649423, 'per_payment': 1905.80331922, 'LER': 0.434229813385}, {'mean': 168.174141652, 'lev_d': 95.0930233818, 'per_loss': 73.0811182701, 'per_payment': 93.0807553639, 'LER': 0.565443786112}],
    'act_exponential_coverage_modifications': [{'per_loss': 676.152853064, 'per_payment': 810.975096345, 'max_payment': 2240.0}, {'per_loss': 3847.510674, 'per_payment': 4252.15690389, 'max_payment': 9500.0}],
    'act_pareto_coverage_modifications': [{'per_loss': 474.502746121, 'per_payment': 900.535608014, 'max_payment': 3600.0}, {'per_loss': 518.181818182, 'per_payment': 627.0, 'max_payment': 1710.0}],
    'act_single_loss_payment': [{'payment': 2800.0, 'retained': 2200.0}, {'payment': 855.0, 'retained': 345.0}],
    'act_var_tvar_normal': [{'VaR': 1328.97072539, 'TVaR': 1412.5425615}, {'VaR': 2.32634787404, 'TVaR': 2.66521422035}],
    'act_var_tvar_exponential': [{'VaR': 2995.73227355, 'TVaR': 3995.73227355}, {'VaR': 2302.58509299, 'TVaR': 2802.58509299}],
    'act_var_tvar_pareto': [{'VaR': 3428.83523319, 'TVaR': 6143.25284978}, {'VaR': 5309.5734448, 'TVaR': 9515.95574134}],
    'act_var_tvar_lognormal': [{'VaR': 5680.83567601, 'TVaR': 9384.13872632}, {'VaR': 474.933092881, 'TVaR': 570.092498989}],
    'act_premium_principles': [{'premium': 1200.0, 'loading': 200.0}, {'premium': 1050.0, 'loading': 50.0}],
    'act_limited_fluctuation_full_credibility': [{'n_full': 1082.21738164, 'z': 1.64485362695}, {'n_full': 1536.58352828, 'z': 1.95996398454}],
    'act_limited_fluctuation_severity_aggregate': [{'claims_aggregate': 5411.08690819, 'claims_severity': 4328.86952655, 'claims_frequency': 1082.21738164}, {'claims_aggregate': 768.291764139, 'claims_severity': 384.145882069, 'claims_frequency': 384.145882069}],
    'act_partial_credibility': [{'Z': 0.526558947625, 'premium': 894.688210475}, {'Z': 1.0, 'premium': 800.0}],
    'act_buhlmann_credibility': [{'k': 5.0, 'Z': 0.5, 'premium': 1100.0}, {'k': 5.0, 'Z': 0.375, 'premium': 0.3125}],
    'act_buhlmann_empirical': [{'EPV': 6.5, 'VHM': 5.83333333333, 'k': 1.11428571429, 'Z': 0.729166666667, 'premiums': [5.54166666667, 8.45833333333], 'mean': 7.0}, {'EPV': 166.666666667, 'VHM': 1558.33333333, 'k': 0.106951871658, 'Z': 0.973958333333, 'premiums': [116.041666667, 193.958333333, 155.0], 'mean': 155.0}],
    'act_buhlmann_straub': [{'EPV': 7661.11111111, 'VHM': 3834.19753086, 'k': 1.99810026725, 'Z': [0.957485509927, 0.974050005645], 'premiums': [114.590043128, 200.466171022], 'mean': 167.916666667}, {'EPV': 67.7100415924, 'VHM': 1.59407614318, 'k': 42.4760397314, 'Z': [0.721424823164, 0.666791972665, 0.779317780069], 'premiums': [4.30378603763, 6.24606986966, 4.37054010486], 'mean': 4.85507246377}],
    'act_poisson_gamma_credibility': [{'post_mean': 1.8, 'Z': 0.6, 'prior_mean': 1.5, 'post_alpha': 9.0, 'post_theta': 0.2}, {'post_mean': 0.222222222222, 'Z': 0.444444444444, 'prior_mean': 0.2, 'post_alpha': 2.0, 'post_theta': 0.111111111111}],
    'act_chain_ladder_factors': [{'ldf': [1.51428571429, 1.16666666667], 'cdf': [1.76666666667, 1.16666666667, 1.0]}, {'ldf': [1.78787878788, 1.15789473684, 1.04761904762], 'cdf': [2.16875522139, 1.21303258145, 1.04761904762, 1.0]}],
    'act_chain_ladder_reserve': [{'ultimate': [175.0, 196.0, 212.0], 'reserve': [0.0, 28.0, 92.0], 'total_reserve': 120.0}, {'ultimate': [2200.0, 2409.52380952, 2547.36842105, 2819.3817878], 'reserve': [0.0, 109.523809524, 447.368421053, 1519.3817878], 'total_reserve': 2076.27401838}],
    'act_chain_ladder_simple_average': [{'ldf': [1.51363636364, 1.16666666667], 'ultimate': [175.0, 196.0, 211.909090909], 'total_reserve': 119.909090909}, {'ldf': [1.78939393939, 1.15833333333, 1.04761904762], 'ultimate': [2200.0, 2409.52380952, 2548.33333333, 2822.83994709], 'total_reserve': 2080.69708995}],
    'act_bornhuetter_ferguson': [{'ultimate': [175.0, 192.0, 196.377358491], 'reserve': [0.0, 24.0, 76.3773584906], 'total_reserve': 100.377358491}, {'ultimate': [2200.0, 2401.81818182, 2517.97520661, 2658.0431433], 'reserve': [0.0, 101.818181818, 417.975206612, 1358.0431433], 'total_reserve': 1877.83653173}],
    'act_cape_cod': [{'elr': 0.917688855647, 'ultimate': [175.0, 195.530665669, 207.613313388], 'reserve': [0.0, 27.5306656694, 87.6133133882], 'total_reserve': 115.143979058}, {'elr': 0.751137899047, 'ultimate': [2200.0, 2409.25642168, 2548.5100265, 2757.25381925], 'reserve': [0.0, 109.25642168, 448.510026497, 1457.25381925], 'total_reserve': 2015.02026742}],
    'act_mack_standard_error': [{'se': [0.0, 15.2267485994, 34.5563923039, 72.2173151114], 'total_se': 90.6808325843, 'sigma2': [1.43939393939, 0.263157894737, 0.0481119696749]}, {'se': [0.0, 0.920569091089, 6.21333668409, 43.5180914566, 89.6552167478], 'total_se': 105.917008511, 'sigma2': [3.75268601892, 0.990382028518, 0.0174081237911, 0.000305985736009]}],
    'act_expected_claims_method': [{'ultimate': [700.0, 770.0, 840.0], 'reserve': [50.0, 270.0, 540.0], 'total_reserve': 860.0}, {'ultimate': [3250.0], 'reserve': [2250.0], 'total_reserve': 2250.0}],
    'act_loss_ratio_rate_change': [{'change': -0.0625, 'loss_ratio': 0.65, 'permissible': 0.8}, {'change': 0.038961038961, 'loss_ratio': 0.75, 'permissible': 0.77}],
    'act_pure_premium_rate': [{'pure_premium': 250.0, 'rate': 350.0}, {'pure_premium': 150.0, 'rate': 212.5}],
    'act_kaplan_meier': [{'S': 0.3, 'F': 0.7, 'deaths': 3}, {'S': 0.380952380952, 'F': 0.619047619048, 'deaths': 4}],
    'act_nelson_aalen': [{'H': 0.95, 'S': 0.386741023455}, {'H': 0.809523809524, 'S': 0.445069953843}],
    'act_exponential_mle': [{'hazard': 0.230769230769, 'mean': 4.33333333333, 'S': 0.500419920361}, {'hazard': 0.119402985075, 'mean': 8.375, 'S': 0.550452327358}],
}

for _k, _vals in _EXV.items():
    if _k in PROCEDURES:
        for _e, _v in zip(PROCEDURES[_k]["examples"], _vals):
            _e["outputs"].update(_v)


# ================================================================================================ RATEMAKING: ON-LEVEL AND TREND
# Conventions: dates are decimal years (2022.5 = 1 July 2022, 2022.25 = 1 April 2022); policies have a term T of at
# most one year (1 = annual, 0.5 = six months) and are written evenly through time; a rate change applies to policies
# WRITTEN on or after its effective date. Calendar year Y runs from Y to Y + 1. The rate level before the first change
# is 1; after change k it is the product of (1 + c_j) for j <= k; the current level is the level after the last change.
def _earned_share(d, T):
    """Share of one calendar year's earned exposure that comes from policies written before time d (d measured from
    the start of the year): the area of the parallelogram diagram to the left of the line written at d."""
    if d <= -T:
        return 0.0
    if d <= 0.0:
        return (d + T) * (d + T) / (2.0 * T)
    if d <= 1.0 - T:
        return T / 2.0 + d
    if d <= 1.0:
        return 1.0 - (1.0 - d) * (1.0 - d) / (2.0 * T)
    return 1.0


def _on_level_check(dates, changes, years, EP, T):
    _need(len(dates) == len(changes) and len(dates) >= 1, "give one rate change for every effective date")
    _need(len(years) == len(EP) and len(years) >= 1, "give one earned premium for every calendar year")
    _need(_isnum(T) and 0 < T <= 1, "the policy term T must lie between 0 and 1 year (0.5 = six months)")
    for k in range(len(dates)):
        _need(_isnum(dates[k]) and _isnum(changes[k]), "every date and rate change must be a finite number")
        _need(-1 < changes[k] < 1, "each rate change is a decimal between -100% and +100% (0.05 = +5%)")
        if k > 0:
            _need(dates[k] > dates[k - 1], "the effective dates must increase")
    for k in range(len(years)):
        _need(_isnum(years[k]) and years[k] == math.floor(years[k]), "each calendar year must be a whole number")
        _need(_isnum(EP[k]) and EP[k] >= 0, "earned premiums must be zero or positive")
        if k > 0:
            _need(years[k] > years[k - 1], "the calendar years must increase")


def _written_share(d, T):
    # the wrong variant's share: the calendar year's WRITINGS before d (the earning lag ignored)
    return min(max(d, 0.0), 1.0)


def _on_level(dates, changes, years, EP, T, by_writing):
    _on_level_check(dates, changes, years, EP, T)
    levels = [1.0]
    for c in changes:
        levels.append(levels[-1] * (1.0 + c))
    current = levels[-1]
    avg = []
    factors = []
    olep = []
    total = 0.0
    for k in range(len(years)):
        a = 0.0
        prev = 0.0
        for j in range(len(dates)):
            if by_writing:
                s = _written_share(dates[j] - years[k], T)
            else:
                s = _earned_share(dates[j] - years[k], T)
            a += (s - prev) * levels[j]
            prev = s
        a += (1.0 - prev) * current
        f = current / a
        avg.append(a)
        factors.append(f)
        olep.append(EP[k] * f)
        total += EP[k] * f
    return _out(average_level=avg, factors=factors, on_level_EP=olep, total_on_level_EP=total, current_level=current)


def on_level_premium(dates, changes, years, EP, T):
    return _on_level(dates, changes, years, EP, T, False)


def _ref_on_level_premium(dates, changes, years, EP, T):
    from fractions import Fraction as Fr
    # exact decimals; each level's share of a calendar year is the integral, over its writing window, of the part of
    # a policy's term that falls inside the year (divided by T): piecewise linear, so the trapezoid rule on its
    # breakpoints is exact
    q = lambda v: Fr(repr(float(v)))
    Tq = q(T)
    lv = [Fr(1)]
    for c in changes:
        lv.append(lv[-1] * (1 + q(c)))
    avg, fac, ole = [], [], []
    for y, ep in zip(years, EP):
        Y = q(y)
        lo, hi = Y - Tq, Y + 1
        def ov(s):
            return max(Fr(0), min(s + Tq, Y + 1) - max(s, Y)) / Tq
        cuts = [lo] + [min(max(q(d), lo), hi) for d in dates] + [hi]
        a = Fr(0)
        for j in range(len(cuts) - 1):
            u, w = cuts[j], cuts[j + 1]
            pts = sorted(set([u, w] + [p for p in (Y - Tq, Y, Y + 1 - Tq, Y + 1) if u < p < w]))
            area = sum(((pts[i + 1] - pts[i]) * (ov(pts[i]) + ov(pts[i + 1])) / 2 for i in range(len(pts) - 1)), Fr(0))
            a += area * lv[j]
        f = lv[-1] / a
        avg.append(float(a))
        fac.append(float(f))
        ole.append(float(q(ep) * f))
    return {"average_level": avg, "factors": fac, "on_level_EP": ole, "total_on_level_EP": float(sum(Fr(x) for x in ole)),
            "current_level": float(lv[-1])}


def _rand_on_level(rng):
    T = rng.choice([1.0, 1.0, 0.5])
    y0 = float(rng.randint(2012, 2022))
    n = rng.randint(1, 3)
    dates = sorted(y0 - 1.0 + 0.25 * k for k in rng.sample(range(13), n))
    changes = []
    for _ in range(n):
        c = _sig(rng.uniform(-0.1, 0.15), 2)
        changes.append(c if abs(c) >= 0.01 else 0.05)
    return {"dates": dates, "changes": changes, "years": [y0, y0 + 1.0, y0 + 2.0],
            "EP": [_sig(_lu(rng, 1e4, 1e7), 4) for _ in range(3)], "T": T}


_add("act_on_level_premium",
     name="On-level earned premium by the parallelogram method", topic="ratemaking / premium adjustments",
     purpose="The average rate level of each calendar year's earned premium under past rate changes (policies written evenly, earned evenly over their term), the on-level factor that brings it to the current rate level, and the earned premium at current rates.",
     inputs=[_vecin("dates", "effective dates of the rate changes as decimal years, increasing (2022.5 = 1 July 2022)", 1, 40),
             _vecin("changes", "rate change at each date as a decimal (0.05 = +5%, -0.03 = -3%)", 1, 40),
             _vecin("years", "calendar years of the earned premium (whole numbers, increasing)", 1, 40),
             _vecin("EP", "earned premium of each calendar year (at the rates then charged)", 1, 40),
             _numin("T", "policy term in years, at most 1 (1 = annual, 0.5 = six months)")],
     outputs=[_o("average_level", "average rate level of each calendar year's earned premium (a list)"),
              _o("factors", "on-level factor of each year: current level / average level (a list)"),
              _o("on_level_EP", "earned premium at current rate level of each year (a list)"),
              _o("total_on_level_EP", "total earned premium at current rate level"),
              _o("current_level", "current cumulative rate level (1 before the first change)")],
     primary="factors",
     conditions=["one change per date, dates increasing", "-100% < change < 100%", "whole calendar years, increasing",
                 "earned premiums >= 0", "0 < T <= 1"],
     method="share of CY earned exposure written before d: (d+T)^2/(2T), T/2 + d, 1 - (1-d)^2/(2T) on the three parts of the parallelogram; average level = sum of shares x levels; factor = current level / average level",
     latex=r"\bar L_Y = \sum_j \big(A(t_{j+1} - Y) - A(t_j - Y)\big) L_j,\qquad \text{factor}_Y = \frac{L_{\mathrm{current}}}{\bar L_Y}",
     run=on_level_premium, reference=_ref_on_level_premium,
     wrong=lambda dates, changes, years, EP, T: _on_level(dates, changes, years, EP, T, True),
     rand=_rand_on_level,
     phrases=("On-level earned premium", "On-level premium", "Parallelogram method"),
     examples=[_ex({"dates": [2022.5], "changes": [0.1], "years": [2022.0, 2023.0, 2024.0], "EP": [1000.0, 1100.0, 1200.0], "T": 1.0},
                   {"average_level": [1.0125, 1.0875, 1.1], "factors": [1.08641975309, 1.01149425287, 1.0],
                    "on_level_EP": [1086.41975309, 1112.64367816, 1200.0], "total_on_level_EP": 3399.06343125,
                    "current_level": 1.1}),
               _ex({"dates": [2021.25, 2022.0], "changes": [0.05, -0.02], "years": [2021.0, 2022.0], "EP": [500.0, 520.0], "T": 0.5},
                   {"average_level": [1.025, 1.03425], "factors": [1.00390243902, 0.994923857868],
                    "on_level_EP": [501.951219512, 517.360406091], "total_on_level_EP": 1019.3116256,
                    "current_level": 1.029})],
     level="intermediate",
     extra_q=({"q": "On-level earned premium: dates = [2022.5], changes = [0.1], years = [2022, 2023, 2024], EP = [1000, 1100, 1200], T = 2", "outcome": "INVALID_INPUT"},
              {"q": "On-level earned premium: dates = [2022.5, 2022], changes = [0.1, 0.05], years = [2022], EP = [1000], T = 1", "outcome": "INVALID_INPUT"},
              {"q": "Parallelogram method: dates = [2022.5], changes = [0.1], years = [2022, 2023, 2024], EP = [1000, 1100, 1200], T = 1", "like": 0}),
     weak="the same parallelogram areas integrated exactly from the policy-overlap definition (trapezoids on the breakpoints, exact fractions) instead of the closed-form areas")


# Trend: an experience year's amounts (accident-year losses, or calendar-year earned premium) are centred at the middle
# of the year, y + 0.5; the trend period runs from there to the average date of the future period (a decimal year:
# for annual policies written evenly for one year from an effective date E, the average accident date is E + 1).
def trend_projection(amounts, years, trend, to_date):
    _need(len(amounts) == len(years) and len(years) >= 1, "give one experience year for every amount")
    _need(_isnum(trend) and -1 < trend < 1, "the annual trend must lie between -100% and 100% (write 3% or 0.03)")
    _fin(to_date)
    periods = []
    factors = []
    trended = []
    total = 0.0
    for k in range(len(years)):
        y = years[k]
        x = amounts[k]
        _need(_isnum(y) and y == math.floor(y), "each experience year must be a whole number")
        _need(_isnum(x) and x >= 0, "the amounts must be zero or positive")
        t = to_date - (y + 0.5)
        _need(t >= 0, "the projection date must not lie before the middle of an experience year")
        f = (1.0 + trend) ** t
        periods.append(t)
        factors.append(f)
        trended.append(x * f)
        total += x * f
    return _out(periods=periods, factors=factors, trended=trended, total_trended=total)


def _ref_trend_projection(amounts, years, trend, to_date):
    import mpmath
    # 40-digit arithmetic: the factor as exp(t log(1 + trend)) from the written decimals
    periods, factors, trended = [], [], []
    with mpmath.workdps(40):
        g = mpmath.log(1 + mpmath.mpf(repr(float(trend))))
        tot = mpmath.mpf(0)
        for x, y in zip(amounts, years):
            t = mpmath.mpf(repr(float(to_date))) - mpmath.mpf(repr(float(y))) - mpmath.mpf("0.5")
            f = mpmath.exp(t * g)
            v = mpmath.mpf(repr(float(x))) * f
            tot += v
            periods.append(float(t))
            factors.append(float(f))
            trended.append(float(v))
        total = float(tot)
    return {"periods": periods, "factors": factors, "trended": trended, "total_trended": total}


def _rand_trend(rng):
    n = rng.randint(1, 5)
    y0 = float(rng.randint(2010, 2022))
    tr = _sig(rng.uniform(0.01, 0.09), 2) * rng.choice([1, 1, -1])
    return {"amounts": [_sig(_lu(rng, 1e4, 1e7), 4) for _ in range(n)], "years": [y0 + k for k in range(n)],
            "trend": tr, "to_date": y0 + n + rng.choice([1.0, 1.5, 2.0, 2.25, 3.0])}


_add("act_trend_projection",
     name="Loss and premium trend factors to a future period", topic="ratemaking / loss and premium adjustments",
     purpose="The trend period of each experience year (from its midpoint to the average date of the future period), the compound trend factor at an annual trend rate, and the trended losses or premiums.",
     inputs=[_vecin("amounts", "losses (or premiums) of each experience year (a list)", 1, 40),
             _vecin("years", "experience years (whole numbers, a list)", 1, 40),
             _numin("trend", "annual trend rate (3% or 0.03; negative for a falling trend)", pct=True),
             _numin("to_date", "average date of the future period as a decimal year (2025.5 = 1 July 2025)")],
     outputs=[_o("periods", "trend period of each year in years: to_date - (year + 0.5) (a list)"),
              _o("factors", "trend factor (1 + trend)^period of each year (a list)"),
              _o("trended", "trended amount of each year (a list)"),
              _o("total_trended", "total of the trended amounts")],
     primary="factors",
     conditions=["one year per amount", "whole years", "amounts >= 0", "-100% < trend < 100%", "to_date >= year + 0.5"],
     method="period = to_date - (year + 0.5); factor = (1 + trend)^period; trended = amount x factor",
     latex=r"t_y = D - (y + \tfrac12),\qquad \text{trended}_y = X_y\,(1 + g)^{t_y}",
     run=trend_projection, reference=_ref_trend_projection,
     wrong=lambda amounts, years, trend, to_date: (lambda ts: _out(periods=ts, factors=[1 + trend * t for t in ts], trended=[x * (1 + trend * t) for x, t in zip(amounts, ts)], total_trended=sum(x * (1 + trend * t) for x, t in zip(amounts, ts))))([to_date - y - 0.5 for y in years]),
     rand=_rand_trend,
     phrases=("Trend factors", "Loss trend", "Premium trend"),
     examples=[_ex({"amounts": [100000.0, 120000.0], "years": [2022.0, 2023.0], "trend": 0.05, "to_date": 2025.5},
                   {"periods": [3.0, 2.0], "factors": [1.157625, 1.1025], "trended": [115762.5, 132300.0],
                    "total_trended": 248062.5}),
               _ex({"amounts": [50000.0], "years": [2021.0], "trend": -0.02, "to_date": 2024.0},
                   {"periods": [2.5], "factors": [0.950747493712], "trended": [47537.3746856],
                    "total_trended": 47537.3746856})],
     level="beginner",
     extra_q=({"q": "Loss trend: amounts = [100000, 120000], years = [2022, 2023], trend = 5%, to_date = 2025.5", "like": 0},
              {"q": "Trend factors: amounts = [100000, 120000], years = [2022, 2023], trend = 5%, to_date = 2022", "outcome": "INVALID_INPUT"},
              {"q": "Trend factors: amounts = [100000, 120000], years = [2022, 2023], trend = 5, to_date = 2025.5", "outcome": "UNRECOGNIZED_FORMAT"}),
     weak="the same compound factor evaluated as exp(t ln(1 + g)) in 40-digit arithmetic")
