# demo repo
Personal projects in a workplace or academic settings. 
Dabbles in application to financial engineering, data analysis, machine learning, cloud computing.

#### gd.py
**[Python]** a performant gradient descent algorithm that solve matrix types questions, with respect to different input, objective, output constraints.
Applicable to transformation based optimization problems, such as portfolio construction involving factor modeling, PCA.

#### macro_factor_trading (undergraduate independent project)
**[R]** an implementation of a macro factor trend following model based on historical analysis of a 4-phase economy (expansion - speeding up, expansion - slowing down, contraction - speeding up, contraction - slowing down) and corresponding market return.
Includes simple implementation of basket construction, use of closest semipositive-definite matrix inversion of portfolio fitting, backtesting results and live trading results.
Experiments also included modelling the 4-phase economy as a hidden Markov model to smoothen the portfolio change between economy state change. (NOT included in src code)

#### natural_language_processing (undergraduate course work)
**[C++]** using training data of english vocabs, create N-ary word tokens (e.g. 2-nary tokens for "rice" include 'ri', 'ic', 'ce'), and assign score to such tokens. Using Djikstra's algorithm, we solve for, any supplied word, the top 5 scored non-replacing anagrams, which should be considered most 'natural'. No heuristics/pruning are applied in the script in the repo, so the script cannot handle words with 13+ characters due to the complexity.
