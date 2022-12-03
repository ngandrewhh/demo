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

import pandas as pd
import numpy as np
import cvxpy as cp

class Optimizer:
    def __init__(
        self, 
        constituent_panel_price, 
        benchmark_price, 
        constituent_weight, 
        start_date: str,
    ):
        self.n = len(constituent_weight)
        self.wc = constituent_weight
        assert np.isclose(self.wc.sum(), 1, 4), "weight not equal to 1.0, check data or rounding error?"

        self._const_ret = constituent_panel_price.pct_change()
        self._bmk_ret = benchmark_price.pct_change()

        self.const_ret = self._const_ret.loc[start_date:]
        self.bmk_ret = self._bmk_ret.loc[start_date:]
        self.const_cov = self._const_ret.loc[start_date:].cov(ddof=1)

        assert np.all(self.wc.index == self._const_ret.columns), "misalignment in data labels"

        self.start_date = start_date

    def _cov_is_psd(self):
        return np.all(np.linalg.eigvals(self.const_cov) > 0)


    def _cov_shrink(self, x0=0.001, dx=0.00001, **kwargs):
        '''
            Perform covariance shrinkage - this is required due to incompleteness of data on 
            all constituents (lacking 5 years of data) - some constituents have as short as
            half year data only

            Pass kwarg x0 and dx in Optimizer.optimize to shrink if required.
        '''
        self._const_cov = self.const_cov

        x = x0
        while not np.all(np.linalg.eigvals((1-x) * self._const_cov + x * np.identity(len(self._const_cov))) > 0):
            x += dx
        
        self.const_cov = (1-x) * self._const_cov + x * np.identity(len(self.const_cov))
        assert np.all(np.linalg.eigvals(self.const_cov) > 0)


    def optimize_mvp(
        self, 
        risk_aversion: float = 1, 
        turnover_constr: float = 0.15,
        deviation_constr: float = 0.03,
        long_only_constr: bool = True,
        w0: np.ndarray = None, 
        shrink_cov: bool = False, 
        rfr: float = 0.02 / 260, 
        **kwargs
    ):
        """After optimize completed, you can print(Optimizer) to get the metrics.

        Args:
            risk_aversion (float, optional): coefficient for risk aversion for mean-variance. Defaults to 1.
            turnover_constr (float, optional): Defaults to 0.15.
            deviation_constr (float, optional): Defaults to 0.03.
            long_only_constr (bool, optional): Defaults to True.
            w0 (np.ndarray, optional): Initial weight, set to equal if not passed. Defaults to None.
            shrink_cov (bool, optional): Shrink cov on demand, otherwise only shrink if cov is not psd. Defaults to False.
            rfr (float, optional): Daily risk free rate. Defaults to 2% / 260.

        Returns:
            A tuple of (score, weight_optimal, metrics).
        """

        # portfolio construction
        if not w0:
            w0 = np.repeat(1/self.n, self.n)
        
        if not self._cov_is_psd() or shrink_cov:
            self._cov_shrink(**kwargs)

        mu = self.const_ret.mean(axis=0).values
        cov = self.const_cov

        x = cp.Variable(self.n)
        constr = [
            x <= 1,                                                 # no excess leverage
            x @ np.ones(self.n) == 1,                               # weight sum to 1 (req)
            cp.abs(x - w0) @ np.ones(self.n) <= turnover_constr,    # turnover <= 0.15 (req)
            cp.abs(x - self.wc) <= deviation_constr            # deviation <= 0.03 (req)
        ]
        
        if long_only_constr:
            constr.append(x >= 0)
        else:
            constr.append(x >= -1)

        # construct cvxpy problem
        prob = cp.Problem(cp.Maximize(x.T @ mu - risk_aversion * cp.quad_form(x, self.const_cov)), constr)

        # portfolio optimization
        result = prob.solve()
        weight_optimal_dict = dict(zip(self.const_ret.columns, np.round(x.value, 6)))
        weight_optimal = pd.Series(weight_optimal_dict, name='weight_optimal')
        
        assert np.all(np.equal(weight_optimal.index, constituent_weight.index)), "data misalignment"

        # metrics
        exp_ret = weight_optimal @ mu
        exp_var = weight_optimal @ cov @ weight_optimal
        exp_std = np.sqrt(exp_var)
        exp_shp = np.sqrt(260) * (exp_ret - rfr) / exp_std
        metrics = dict(exp_ret=exp_ret, exp_var=exp_var, exp_std=exp_std, exp_shp=exp_shp)
        self.__repr__ = f"[Mean Variance]\nExpected daily return: {exp_ret:.4%},\nExpected daily st dev: {exp_std:.4%},\nExpected annu sharpe: {exp_shp:.4f}\n"

        return result, weight_optimal, metrics

    def optimize_sharpe(
        self, 
        turnover_constr: float = 0.15,
        deviation_constr: float = 0.03,
        w0: np.ndarray = None, 
        shrink_cov: bool = False, 
        rfr: float = 0.02 / 260, 
        **kwargs
    ):
        """After optimize completed, you can print(Optimizer) to get the metrics.

        Args:
            risk_aversion (float, optional): coefficient for risk aversion for mean-variance. Defaults to 1.
            turnover_constr (float, optional): Defaults to 0.15.
            deviation_constr (float, optional): Defaults to 0.03.
            w0 (np.ndarray, optional): Initial weight, set to equal if not passed. Defaults to None.
            shrink_cov (bool, optional): Shrink cov on demand, otherwise only shrink if cov is not psd. Defaults to False.
            rfr (float, optional): Daily risk free rate. Defaults to 2% / 260.

        Returns:
            A tuple of (score, weight_optimal, metrics).
        """

        # portfolio construction
        if not w0:
            w0 = np.repeat(1/self.n, self.n)

        if not self._cov_is_psd() or shrink_cov:
            self._cov_shrink(**kwargs)

        mu = self.const_ret.mean(axis=0).values
        cov = self.const_cov
        
        x = cp.Variable(self.n)
        
        prob = cp.Problem(
            cp.Maximize(x @ (mu - rfr)),
            [cp.quad_form(x, cov) <= 1, 
            cp.sum(x) >= 0, 
            x >= 0,
            cp.sum(cp.abs(x - cp.sum(x) * w0)) <= turnover_constr * cp.sum(x),
            cp.abs(x - cp.sum(x) * self.wc) <= deviation_constr * cp.sum(x)]
        )

        # portfolio optimization
        result = prob.solve()
        weight_optimal_dict = dict(zip(self.const_ret.columns, np.round(x.value, 6) / x.value.sum()))
        weight_optimal = pd.Series(weight_optimal_dict, name='weight_optimal')
        
        # visualization
        assert np.all(np.equal(weight_optimal.index, constituent_weight.index)), "data misalignment"

        # metrics
        exp_ret = weight_optimal @ mu
        exp_var = weight_optimal @ cov @ weight_optimal
        exp_std = np.sqrt(exp_var)
        exp_shp = np.sqrt(260) * (exp_ret - rfr) / exp_std
        metrics = dict(exp_ret=exp_ret, exp_var=exp_var, exp_std=exp_std, exp_shp=exp_shp)
        self.__repr__ = f"[Max Sharpe]\nExpected daily return: {exp_ret:.4%},\nExpected daily st dev: {exp_std:.4%},\nExpected annu sharpe: {exp_shp:.4f}\n"

        return result, weight_optimal, metrics


    def get_ew_baseline(self, w=None, rfr = 0.02 / 260):
        if not w:
            w = np.repeat(1/self.n, self.n)

        # portfolio construction
        if not self._cov_is_psd():
            self._cov_shrink()

        mu = self.const_ret.mean(axis=0).values
        cov = self.const_cov

        # metrics
        exp_ret = w @ mu
        exp_var = w @ cov @ w
        exp_std = np.sqrt(exp_var)
        exp_shp = np.sqrt(260) * (exp_ret - rfr) / exp_std
        print(f"[Baseline]\nExpected daily return: {exp_ret:.4%},\nExpected daily st dev: {exp_std:.4%},\nExpected annu sharpe: {exp_shp:.4f}\n")
        return


    def __repr__(self):
        return self.__repr__            

        
if __name__ == '__main__':
    # assumptions
    initial_cash = 10_000_000

    # data preprocess
    constituent_px = pd.read_csv('./000300_10y_price.csv', index_col=0)
    benchmark_px = pd.read_csv('./000300_bmk_10y_price.csv', index_col=0)
    constituent_info = pd.read_csv('./000300_closeweight.csv')

    exch_map = {'Shenzhen  Exchange': 'SZ', 'Shanghai  Exchange': 'SH'}
    constituent_info.loc[:, 'code'] = constituent_info.apply(lambda x: f"{x['成分券代码Constituent Code']:06d}.{exch_map[x['交易所英文名称Exchange(Eng)']]}", axis=1)
    constituent_weight = constituent_info.set_index('code')['权重(%)weight'].rename('weight_bmk') / 100
    
    metrics_full = {}
    opt = Optimizer(constituent_px, benchmark_px, constituent_weight, "2017-08-31")
    opt.get_ew_baseline()

    # Mean Variance Problem (MVP) demo
    score, opt_weight, metrics = opt.optimize_mvp()
    print(opt)
    
    # Frontier / pareto curve can be constructed by looping over risk_aversion
    # For default risk_aversion = 1, we have:
    # Expected return: 0.1287%, expected st dev: 1.3994%, expected sharpe: 0.0920
    for L in np.arange(0, 3, 0.05):
        print(f"Risk aversion level: {L:.3f}")
        opt.optimize_mvp(risk_aversion=L)
        metrics_full.update({f"mvp_ra_{L:.3f}": metrics})
        print(opt)

    # If we remove the long only constr, beats all portfolio that is long only
    print("MVP short allowed")
    opt.optimize_mvp(long_only_constr=False)
    print(opt)
    metrics_full.update({f"mvp_short_allowed": metrics})

    # Max Sharpe (Convex Reformulation) demo
    score, opt_weight, metrics = opt.optimize_sharpe()
    print(opt)
    metrics_full.update({f"max_sharpe": metrics})

    print('Full metrics')
    df_metrics = pd.DataFrame(metrics_full).T
    df_metrics.to_csv('metrics.csv')

    print('Min max of metrics')
    print(df_metrics.agg(['idxmin', 'idxmax']))

    # Summary
    df_test = pd.concat([opt_weight, constituent_weight, pd.Series(np.repeat(1/300, 300), index=opt_weight.index, name='weight_equal')], axis=1)
    print('\nturnover: ', (df_test.weight_optimal - df_test.weight_equal).abs().sum())
    print('deviation from bmk: ', 300 - ((df_test.weight_optimal - df_test.weight_bmk).abs() < 0.03).sum())

    # Output
    opt_weight.to_csv('000300_optimal_weight.csv')
