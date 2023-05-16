## Project demo (with code)
Personal projects for practice, self learn, or in coursework settings. Spans Python, Rust and C++.
All source codes presented in the repo is owned by me personally, or with a sizable contribution or effort.

#### Rust <practice_rust>
> Refer to [Advent of Code 2022 Website](https://adventofcode.com/2022) for more information.
> 
> Also included are some memory safe traversal practices, such as LinkedList implementation. Inspiration obtain from [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/index.html).

#### Python <convex_optimizer> (CSI 300 convex optimizer with Pareto frontier construction)
> A traditional convex optimizer that supports minimum variance, Pareto optimization (risk-aversion budget), maximum sharpe.
> 
> Includes some method of naive shrinkage or covariance estimation.
> Incorporates constraints of long-only/short-allowed, max tracking error etc.

#### Python <yield_curve_sim> (Naive yield curve simulation based on multivariate normal simulation)
> A script to use parameters to simulate the curve movement in a rate hike environment based on forward curve, provide the best path way of fixed income investing based on buy-and-hold strategy.

#### Python <rl_trading> (Multiple reinforcement pair trading agents)
> Multiple reinforcement pair trading agents implemented with test data, that allows test runs.
> Best performing agent, as expected, is the SOTA PPO2 agent implemented with stable-baselines3, achieving 1.14x return (roughly stable 6% p.a.).

#### Python <stats.py> (Methods to find exp value, pdf, cdf; demonstration of law of large number and central limit theorem)
> A naively implemented package which includes: 
> 1. a discrete random variable class Expector, with methods to find expected value, expected value on N replays, pdf; 
> 2. a continuous r.v. class ContinuousRandomVariable, which includes pdf, cdf, exp_val (approximated with integration); 
> 3. some tools and methods to demonstrate Law of Large Number with visualization, Central Limit Theorem, and also some empirical way to simulate correlated variable. 

#### Python <gd.py> (Gradient descent to solve matrix optimization)
> A gradient descent script that solve matrix types questions (maximize B in `Ax = B` for some input x, transformation A), with respect to different input, objective, output constraints.
> 
> Applicable to transformation based optimization problems, such as portfolio construction involving factor modeling, PCA.
  
#### Python <option_pricer> (European, American, Asian options closed form pricing and monte carlo, postgraduate coursework)
> An implementation of European, Asian options pricing using Black Scholes formula, and by extension, Monte Carlo simulation with spot-asset model. American option pricing is implemented using a grid based approach, and a recursion approach.
> Some numerical techniques including quasi-monte carlo, control variates, dynamic programming with subtree collapses are explored, but not included in the code.

#### R <macro_factor_trading> (Economy indicators based macro asset allocation, undergraduate independent project)
<details>
  An implementation of a macro factor trend following model based on historical analysis of a 4-phase economy (expansion - speeding up, expansion - slowing down, contraction - speeding up, contraction - slowing down) and corresponding market return.

  Includes simple implementation of basket construction, use of closest semipositive-definite matrix inversion of portfolio fitting, backtesting results and live trading results.

  Experiments also included modelling the 4-phase economy as a hidden Markov model to smoothen the portfolio change between economy state change. (NOT included in src code)
</details>

#### C++ <n-ary_vocab_cpp> (A markov model based word generation and word verification model, undergraduate coursework)
<details>
  Using training data of english vocabs, the script creates N-ary word tokens (e.g. 2-nary tokens for "rice" include 'ri', 'ic', 'ce'), and assign score to such tokens. 
  
  With Djikstra's algorithm, we solve for, any supplied word, the top 5 scored non-replacing anagrams, which should be considered most 'natural'. 
  
  No heuristics/pruning are applied in the script in the repo, so the script cannot handle words with 13+ characters due to the complexity.
</details>

## Project demo (without code)
#### (Python + Apache) <twitter_sentiment> (A cloud pipeline to analyze Tweeter sentiment, postgraduate coursework)
<details>
  An infrastructure built upon standard online APIs such as Twitter on Apache-loaded AWS platform, includes a complete pipeline of data extraction from Twitter, realtime feeding into a sentiment analysis engine using Apache Spark and Kafka, and outputting the time series of internet sentiment.

  Project also involves extra effort such as classification of emojis, slangs, creation of word cloud as visualization.
</details>
