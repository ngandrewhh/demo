''' 
    @ Andrew Ng @

Files:
    * 000300closeweight - weight file downloaded from CSIC as of 8/31
    * 000300_10y_price - 10y price of CURRENT constituents of CSI 300
    * 000300_bmk_10y_price - 10y price of CSI 300

Assumptions:
    * Covariance is not well formed due to most series only having partial (not completely 5y data)
    * I do not have Barra covariance data availble.
        * Solution: apply minimum shrinkage [(1+a)*cov + (a)I], where a is the minimum to yield
        *           a positive PSD cov matrix
    * Turnover is treated as total deviation from current (equal weight) portfolio.
    * There may be minimal floating point precision problem.
        * Solution: I tried to restrain the precision to 6 digits.

Implementation:
    * An Optimizer class.
        * Two optimization methods: Mean Variance, Max Sharpe.
    * Demo is shown below.

'''