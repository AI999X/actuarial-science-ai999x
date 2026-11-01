"""actuarial-science-ai999x — offline actuarial science and insurance answers with their formula, steps and verification.

    import actuarial_science_ai999x as m
    m.ask("Net premium for whole life insurance: q = [0.1, 0.2, 0.5, 1], i = 5%, S = 1000").show()
    m.ask("Chain-ladder reserve: T = [[100, 150, 175], [110, 168, 0], [120, 0, 0]]").show()
    m.solve("act_annuity_certain", n=10, i=0.05)
    m.capabilities()                   # every certified capability

91 certified capabilities: interest theory (rate conversions, annuities-certain, loans, yields, duration and
convexity); life tables and survival models (qx, px, expectations of life, fractional ages, commutation functions,
de Moivre, Gompertz and Makeham laws); life insurances and annuities (whole life, term, endowment, deferred,
continuous and m-thly under UDD, Woolhouse); net and gross premiums, the loss at issue, prospective, retrospective and
gross premium reserves, Thiele's equation; joint lives and multiple decrements; frequency and severity models,
compound distributions, Panjer's recursion, stop-loss premiums, deductibles, limits and coinsurance, VaR and TVaR;
limited fluctuation, Buhlmann, Buhlmann-Straub and Poisson-gamma credibility; chain-ladder, Bornhuetter-Ferguson,
Cape Cod and Mack loss reserving; Kaplan-Meier and Nelson-Aalen estimation. Life tables are written as lists of death
probabilities from the insured's age (a whole-life list ends with 1); rates as 5% or 0.05; loss triangles as square
matrices of cumulative amounts with the unknown cells written as 0. Built by the AI999X model machine from the same
engine as every domain; a question that is not determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.0"
_P = Product("actuarial_science", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
