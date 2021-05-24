# Personal Project Demos
Personal projects in a workplace or academic settings. 
Dabbles in application to financial engineering, data analysis, machine learning, cloud computing.

All source codes presented in the repo is owned by me personally, or with a sizable contribution or effort.

#### gd.py
**[Python]** a performant gradient descent algorithm that solve matrix types questions (maximize B in Ax = B for some input x, transformation A), with respect to different input, objective, output constraints.
Applicable to transformation based optimization problems, such as portfolio construction involving factor modeling, PCA.

#### macro_factor_trading (undergraduate independent project)
**[R]** an implementation of a macro factor trend following model based on historical analysis of a 4-phase economy (expansion - speeding up, expansion - slowing down, contraction - speeding up, contraction - slowing down) and corresponding market return.

Includes simple implementation of basket construction, use of closest semipositive-definite matrix inversion of portfolio fitting, backtesting results and live trading results.
Experiments also included modelling the 4-phase economy as a hidden Markov model to smoothen the portfolio change between economy state change. (NOT included in src code)

#### option_pricer (postgraduate coursework)
**[Python]** an implementation of European, Asian options pricing using Black Scholes formula, and by extension, Monte Carlo simulation with spot-asset model. American option pricing is implemented using a grid based approach, and a recursion approach.

Some numerical techniques including quasi-monte carlo, control variates, dynamic programming with subtree collapses are explored, but not included in the code.

#### tweeter_sentiment (postgraduate coursework - no code provided, demo video available)
**[Python][Apache][Linux]** an infrastructure built upon standard online APIs such as Tweeter on Apache-loaded AWS platform, includes a complete pipeline of data extraction from Tweeter, realtime feeding into a sentiment analysis engine using Apache Spark and Kafka, and outputting the time series of internet sentiment.

Project also involves extra effort such as classification of emojis, slangs, creation of word cloud as visualization.

#### natural_language_processing (undergraduate coursework)
**[C++]** using training data of english vocabs, the script creates N-ary word tokens (e.g. 2-nary tokens for "rice" include 'ri', 'ic', 'ce'), and assign score to such tokens. Using Djikstra's algorithm, we solve for, any supplied word, the top 5 scored non-replacing anagrams, which should be considered most 'natural'. No heuristics/pruning are applied in the script in the repo, so the script cannot handle words with 13+ characters due to the complexity.
