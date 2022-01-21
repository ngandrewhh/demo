import matplotlib.pyplot as plt
import numpy as np
from collections import defaultdict
from fractions import Fraction
from statistics import NormalDist

# [UTILITY] Time of execution 
def time(f):
    def wrap(*args):
        import time
        print(f"\n{time.strftime('[%Y-%m-%dT%H:%M:%SZ]')}  Running: {f}")
        res = f(*args)
        print(f"{time.strftime('[%Y-%m-%dT%H:%M:%SZ]')}  Result: {res}")
        return res
    return wrap

# Discrete Random Variable Practice
class Expector:
    def __init__(self, sample_space, prob_space=None, payoff=None):
        self.sample_space   = sample_space
        self.payoff         = payoff if payoff else lambda x: x

        pspace = prob_space if prob_space else [1/len(sample_space)] * len(sample_space)
        assert(len(pspace) == len(sample_space))
        
        self.prob_space = pspace
        self.ev         = self.exp_val()
        
        print(f"sample space is {self.sample_space}\nprobability is {[round(i, 4) for i in self.prob_space]}\nexpected value is {round(self.ev, 6)}\n")

    def exp_val(self):
        return sum([self.payoff(s)*p for s, p in zip(self.sample_space, self.prob_space)])
        

    def exp_n(self, n_trials):
        print("\nn rerolls test")
        self.evm= self.ev
        n  = 1

        np.random.seed()
        s = np.random.choice(self.sample_space)
        while n_trials:
            print(f"roll outcome: {s}, # of trials: {n}, expected value for termination: {self.evm:6f}\n")
            if s > self.evm:
                return s
            else:
                self.evm= sum([(s if s > self.evm else self.evm)*p for s, p in zip(self.sample_space, self.prob_space)])
            n_trials -= 1
            n += 1
            
            np.random.seed()
            s = np.random.choice(self.sample_space)
        return s

    def _create_pdf(self):
        if "prob_ctr" not in dir(self):
            dd = defaultdict(float)
            for s, p in zip(self.sample_space, self.prob_space):
                dd[self.payoff(s)] += p
            self.prob_ctr = dd
            
    def pdf(self, x=None):
        self._create_pdf()

        if not x:
            print("-------- Payoff Counter --------")
            for k, v in self.prob_ctr.items(): print(f"{k}: {Fraction(str(v)).limit_denominator()}")
            print()
            return self.prob_ctr
        else:
            return self.prob_ctr[x]



# d2 = [(i + 1, j + 1) for i in range(6) for j in range(6)]
# t = Expector(d2, payoff=lambda x: 500 if x[0] + x[1] == 7 else -100)
# u = Expector(d2, payoff=lambda x: max(x))
# v = Expector(d2, payoff=lambda x: sum(x))
# w = Expector(d2, payoff=lambda x: sum(x) <= 4)

# t.pdf()
# u.pdf()
# u.pdf(8)
# v.pdf()
# w.pdf()
# pw = w.pdf()

# Continuous Random Variable Practice        
class ContinuousRandomVariable:
    def __init__(self, pdf):
        if not callable(pdf): raise TypeError(f"The argument <{pdf}> is not a function or a lambda.")
        self._pdf = pdf
        self._cdf_step_size = 10

    def pdf(self, a, b, n_interval=10000, precision=4, verbose=False):
        dx = max(10 ** -precision, (b - a) // n_interval)
        _x = np.arange(a, b+dx, dx)
        fx = np.round(self._pdf(_x), precision)
        if sum(fx) == 0: return 0
        Ax = sum((fx[:-1] + fx[1:])/2) * dx
        # Ax = sum(((fx[i]+fx[i+1])/2 for i in range(len(_x)-1))) * dx
        if verbose: print(f"[pdf] P({a:6f} <= X <= {b:6f}) = {Ax:6f}")
        return Ax

    def cdf(self, x, lower_bound=0, verbose=False):
        _v = verbose
        cp = 0
        cc = 0
        sz = self._cdf_step_size
            
        for i in np.arange(x, lower_bound-sz, -sz):
            seg = self.pdf(i-sz, i, verbose=_v)
            cp += seg
            if i < lower_bound and seg == 0: cc += 1
            if cc == 100: break
        print(f"[cdf] P(X <= {x:4f}) = {cp:6f}")
        return cp

    def exp_val(self, a, b, n_interval=10000, precision=4):
        dx = max(10 ** -precision, (b - a) // n_interval)
        _x = np.arange(a, b+dx, dx)
        fx = np.round(self._pdf(_x), precision)
        Ax = sum((fx[:-1] + fx[1:]) * (_x[:-1] + _x[1:])/4) * dx
        # Ax = sum(((fx[i]+fx[i+1])*(_x[i]+_x[i+1])/4 for i in range(len(_x)-1))) * dx
        print(f"[exp] E[{a} <= X <= {b}] = {Ax:6f}")
        return Ax
            
crv = ContinuousRandomVariable

print("\n... x ...")
# x = ContinuousRandomVariable(lambda x: 3 if 1/3>=x>=0 else 0)
x = ContinuousRandomVariable(np.vectorize(lambda x: 3 if 1/3>=x>=0 else 0))
x.pdf(.1, .2)
x.cdf(1/6)

print("\n... y ...")        
y = ContinuousRandomVariable(np.vectorize(lambda x: 3 * x ** 2 if 1>=x>=0 else 0))
y.cdf(0.5)

print("\n... z ...")        
z = ContinuousRandomVariable(np.vectorize(lambda x: 1 if 1>=x>=0 else 0))
z.cdf(1)
z.exp_val(0, 1)

print("\n... A ...")
A = ContinuousRandomVariable(np.vectorize(lambda x: 3/8 * x**2 if 2>=x>=0 else 0))
A.cdf(2)
A.exp_val(0, 2)

print("\n... Z (Standard Normal) ...")
Z = ContinuousRandomVariable(np.vectorize(lambda x: (2 * 3.14159) ** (-0.5) * (2.71828) ** (-x**2/2)  if 10>=x>=-10 else 0, otypes=[float]))
Z.cdf(10)
Z.exp_val(-10, 10)

# Showcase - Law of Large Numbers
@time
def lln(n):
    bins = np.arange(-4, 4.1, 0.1)
    x = [NormalDist(0, 1).inv_cdf(np.random.uniform()) for _ in range(n)]
    plt.hist(x, bins)
    plt.show()

# Showcase - Central Limit Theorem
def clt_uniform(n_sample, n_iter=5000, plot=True):
    bins = np.arange(-4, 4.1, 0.1)
    _s = [np.average((np.random.rand(n_sample) - 0.5) / (1 / np.sqrt(n_sample) / np.sqrt(12))) for _ in range(n_iter)]
    _n = np.vectorize(lambda x: (2 * 3.14159) ** (-0.5) * (2.71828) ** (-x**2/2)  if 10>=x>=-10 else 0)
    _y = _n(bins) * n_iter / 10

    if plot:
        plt.hist(_s, bins, label = "sample")
        plt.plot(bins, _y, label = "true normal")
        plt.show()
    return _s, _y

bins = np.arange(-4, 4.1, 0.1)
fig, axs = plt.subplots(2, 4)
_x = [1, 2, 4, 8, 12, 20, 40, 200]
_n = np.vectorize(lambda x: (2 * 3.14159) ** (-0.5) * (2.71828) ** (-x**2/2)  if 10>=x>=-10 else 0)
n_iter = 5000

for i in range(len(_x)):
    _s, _y = clt_uniform(_x[i], n_iter, False)
    axs[i // 4, i % 4].hist(_s, bins)
    axs[i // 4, i % 4].plot(bins, _y)
    axs[i // 4, i % 4].set_title(f"n={_x[i]}, # of iter={n_iter}")

for ax in axs.flat: ax.label_outer()
# plt.show()
plt.close()
    
@time
def reduce(func, iterable):
    if not iterable: return None
    if len(iterable) == 1: return iterable[0]
    else:
        res = 0
        x, *y = iterable
        while (y):
            res = func(res, x)
            x, *y = y
        return func(res, x)

# ocw.mit.edu 18.05 class 7 - empirical correlation from simulation
@time
def bivariate(n, x, n_iter=200000):
    i = 0
    _x = np.round(np.random.rand(n_iter), 2)
    _y = np.round(np.random.rand(n_iter), 2)
    _p = [[], []]
    while i < n_iter - n:
        if len([jj for jj in _y[i:i+n] if jj in _x[i:i+n]]) >= x:
            _p[0] += [sum(_x[i:i+n]),]
            _p[1] += [sum(_y[i:i+n]),]
        i += 1

    print(f"true corr: {x/n}, sample corr: {np.corrcoef(_p[0], _p[1])[0, 1]}")
    _, ax = plt.subplots()
    ax.scatter(_p[0], _p[1])
    plt.show()
    return _p

Q = bivariate(4, 1)
    
# x = reduce(lambda x, y: x + y, list(np.random.rand(1000)))
# y = reduce(lambda x, y: x if x > y else y, list(np.random.rand(1000)))
