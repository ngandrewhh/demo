import csv
import math
import time
import numpy as np
import pandas as pd
np.set_printoptions(precision = 6, suppress = True, linewidth = 200)

def pdf(x):
    return 1 / math.sqrt(2*math.pi) * math.exp(-0.5 * (x ** 2))
    
def cdf(x):
    return 0.5 * (1 + math.erf(x/math.sqrt(2)))

class BlackScholesOption:
    def __init__(self, spot, strike, t, T, iv, rate, cost = 0, option_type = 'call'):
        self.spot = spot
        self.strike = strike
        self.t = t
        self.T = T
        self.iv = iv
        self.rate = rate
        self.cost = cost
        self.option_type = option_type
        
        self.d1 = (math.log(self.spot/self.strike) + (self.rate - self.cost) * (self.T - self.t)) / (self.iv * math.sqrt(self.T - self.t)) + 0.5 * self.iv * math.sqrt(self.T - self.t) if self.T - self.t > 0 else 0
        self.d2 = self.d1 - self.iv * math.sqrt(self.T - self.t) if self.T - self.t > 0 else 0

    def __vega(self):
        return self.spot * math.exp(-self.cost * (self.T - self.t)) * math.sqrt((self.T - self.t)) * pdf(self.d1)

    def __call(self):
        return self.spot * math.exp(-self.cost * (self.T - self.t)) * cdf(self.d1) - self.strike * math.exp(-self.rate * (self.T - self.t)) * cdf(self.d2)

    def __put(self):
        return -self.spot * math.exp(-self.cost * (self.T - self.t)) * cdf(-self.d1) + self.strike * math.exp(-self.rate * (self.T - self.t)) * cdf(-self.d2)

    def price(self):
        if self.option_type == 'call':
            return self.__call()
        if self.option_type == 'put':
            return self.__put()
        return None        
    
    def vega(self):
        return self.__vega()

    def __str__(self):
        return "S = {0}, K = {1}, t = {2}, T = {3}, iv = {4}, rate = {5}, cost = {6}, option type = {7}".format(self.spot, self.strike, self.t, self.T, self.iv, self.rate, self.cost, self.option_type)

class BinomialPricer(BlackScholesOption):
    def __init__(self, spot, strike, T, iv, rate, N, option_type = 'call', option_class = 'american'):
        super().__init__(spot, strike, 0, T, iv, rate, option_type = option_type)
        self.N = N
        self.u = math.exp(self.iv * math.sqrt(self.T / self.N)) if self.N > 0 else 0
        self.d = 1 / self.u if self.N > 0 else 0
        self.p = (math.exp(self.rate * self.T / self.N) - self.d) / (self.u - self.d) if self.N > 0 else 0
        self.x = math.exp(-self.rate * self.T / self.N) if self.N > 0 else 0
        self.option_class = option_class
        #print(self)

    def __str__(self):
        return "[S = {0:6f}, K = {1}, iv = {2}, N = {3}, u = {4:6f}, d = {5:6f}, p = {6:6f}, x = {7:6f}]".format(self.spot, self.strike, self.iv, self.N, self.u, self.d, self.p, self.x)

    def price(self, verbose = False):
        spot_grid = np.zeros((self.N + 1, self.N + 1))
        for r in range(self.N + 1):
            for c in range(self.N + 1):
                if c < r:
                    continue
                #print(r, c)
                if r == 0:
                    spot_grid[r, c] = self.spot * self.u ** c
                else:
                    if r == c:
                        spot_grid[r, c] = self.spot * self.d ** c
                    else:
                        spot_grid[r, c] = spot_grid[r - 1, c - 2]

        price_grid = np.zeros((self.N + 1, self.N + 1))
        for c in reversed(range(self.N + 1)):
            for r in range(self.N + 1):
                if c < r:
                    continue
                #print(r, c)
                if c == self.N:
                    if self.option_type == 'call':
                        price_grid[r, c] = max( [ spot_grid[r, c] - self.strike, 0 ] )
                    if self.option_type == 'put':
                        price_grid[r, c] = max( [ self.strike - spot_grid[r, c], 0 ] )
                else:
                    price_grid[r, c] = max( [ self.x * (self.p * price_grid[r, c + 1] + (1 - self.p) * price_grid[r + 1, c + 1]), 0 ] )
                    if self.option_class == 'american':
                        if self.option_type == 'call':
                            price_grid[r, c] = max( [ price_grid[r, c], spot_grid[r, c] - self.strike ] )
                        else:
                            price_grid[r, c] = max( [ price_grid[r, c], self.strike - spot_grid[r, c] ] )
                        

        if verbose:
            np.set_printoptions(precision = 6, suppress = True)
            print("\nSpot Price Grid")
            print(spot_grid)

            print("\nOption Price Grid")
            print(price_grid)
        
        return price_grid[0, 0]

    def price_recursion(self):
        if self.option_type == 'call':
            if self.N == 0:
                price = max( [self.spot - self.strike, 0 ])
                U = self.spot
                D = self.spot

            else: 
                U = max( [BinomialPricer(self.spot * self.u, self.strike, self.T - self.T / self.N, self.iv, self.rate, self.N - 1, self.option_type, self.option_class).price() , 0] )
                D = max( [BinomialPricer(self.spot * self.d, self.strike, self.T - self.T / self.N, self.iv, self.rate, self.N - 1, self.option_type, self.option_class).price() , 0] )
                price = self.x * ( self.p * U + (1 - self.p) * D )
                if self.option_class == 'american':
                    price = max( [ price , self.spot - self.strike ] )
        else:
            if self.N == 0:
                price =  max( [self.strike - self.spot, 0 ])
                U = self.spot
                D = self.spot

            else:
                U = max( [ BinomialPricer(self.spot * self.u, self.strike, self.T - self.T / self.N, self.iv, self.rate, self.N - 1, self.option_type, self.option_class).price(), 0] )
                D = max( [ BinomialPricer(self.spot * self.d, self.strike, self.T - self.T / self.N, self.iv, self.rate, self.N - 1, self.option_type, self.option_class).price(), 0] )
                price = self.x * ( self.p * U + (1 - self.p) * D )
                if self.option_class == 'american':
                    price = max( [ price , self.strike - self.spot ] )
        
        #print("[#N: {3}, price = {0:6f}, U = {1:6f}, D = {2:6f}]".format(price, U, D, self.N))
        return price

class AsianOption(BlackScholesOption):
    def __init__(self, spot, strike, T, iv, rate, steps, option_type = 'call', option_class = 'geometric'):
        super().__init__(spot, strike, 0, T, iv, rate, option_type = option_type)
        self.option_class   = option_class
        self.steps          = steps
        self.iv_geo         = self.iv * math.sqrt( (self.steps + 1) * (2 * self.steps + 1) / 6 / self.steps ** 2 )
        self.mu_geo         = (self.rate - 0.5 * self.iv ** 2) * (self.steps + 1) / (2 * self.steps) + 0.5 * self.iv_geo ** 2
        self.d1_geo         = (math.log( self.spot / self.strike ) + ( self.mu_geo + 0.5 * self.iv_geo ** 2 ) * self.T) / ( self.iv_geo * math.sqrt(self.T) )
        self.d2_geo         = self.d1_geo - self.iv_geo * math.sqrt( self.T )

    def price(self):
        if self.option_type == 'call':
            price = math.exp( -self.rate * self.T ) * ( self.spot * math.exp( self.mu_geo * self.T ) * cdf( self.d1_geo ) - self.strike * cdf( self.d2_geo ) ) 
        if self.option_type == 'put':            
            price = math.exp( -self.rate * self.T ) * ( -self.spot * math.exp( self.mu_geo * self.T ) * cdf( -self.d1_geo ) + self.strike * cdf( -self.d2_geo ) ) 
        return price

    def price_monte_carlo(self, iterations, control = True):
        drift = math.exp( (self.rate - 0.5 * self.iv ** 2) * self.T / self.steps )
        listAP = np.zeros(iterations)
        listGP = np.zeros(iterations)

        for i in range(iterations):
            np.random.seed(i)
            arrZ = np.random.normal(0, 1, self.steps)
            arrR = np.exp(self.iv * math.sqrt( self.T / self.steps ) * arrZ) * drift 
            arrP = self.spot * np.cumproduct( arrR )

            arithmeticMean      = np.mean( arrP )
            if self.option_type == 'call':
                arithmeticPayoff    = math.exp( -self.rate * self.T ) * max ( [ arithmeticMean - self.strike, 0 ] )
            else:
                arithmeticPayoff    = math.exp( -self.rate * self.T ) * max ( [ self.strike - arithmeticMean, 0 ] )
            listAP[i] = arithmeticPayoff

            geometricMean       = np.exp( 1 / self.steps * sum ( np.log( arrP ) ) )
            if self.option_type == 'call':
                geometricPayoff    = math.exp( -self.rate * self.T ) * max ( [ geometricMean - self.strike, 0 ] )
            else:
                geometricPayoff    = math.exp( -self.rate * self.T ) * max ( [ self.strike - geometricMean, 0 ] )
            listGP[i] = geometricPayoff

        meanGP  = np.mean(listGP)
        meanAP  = np.mean(listAP)
        sdAP    = np.std (listAP, ddof = 1)
        pxClose = self.price()

        if self.option_class == 'geometric':
            print("[Geometric Asian from Standard MC: {0:6f}, Closed-form price: {1:6f}]".format(meanGP, pxClose))
            return (meanGP, pxClose)
        
        if self.option_class == 'arithmetic':
            if control:
                covPP   = np.mean(listGP * listAP) - np.mean(listGP) * np.mean(listAP)
                theta   = covPP / np.var(listGP, ddof = 1)

                listCV  = listAP + theta * (pxClose - listGP)
                meanCV  = np.mean(listCV)
                sdCV    = np.std(listCV, ddof = 1)
                ciCV    = (meanCV - 1.96 * sdCV / math.sqrt(iterations), meanCV + 1.96 * sdCV / math.sqrt(iterations), meanCV, pxClose)
                print("Control Variate: [Confidence interval: ({0:6f}, {1:6f}), Mean Price: {2:6f}, Closed-form geometric price: {3:6f}]".format(*ciCV))
                return ciCV
            else:
                ciAP    = (meanAP - 1.96 * sdAP / math.sqrt(iterations), meanAP + 1.96 * sdAP / math.sqrt(iterations), meanAP, pxClose)
                print("No Control:      [Confidence interval: ({0:6f}, {1:6f}), Mean Price: {2:6f}, Closed-form geometric price: {3:6f}]".format(*ciAP))
                return ciAP

class BasketOption():
    def __init__(self, S1, S2, v1, v2, rate, T, strike, corr, option_type = 'call', option_class = 'geometric'):
        self.a1             = BlackScholesOption(S1, strike, 0, T, v1, rate, option_type = option_type)
        self.a2             = BlackScholesOption(S2, strike, 0, T, v2, rate, option_type = option_type)
        self.option_type    = option_type
        self.rate           = rate
        self.T              = T
        self.strike         = strike
        self.corr           = corr
        self.option_class   = option_class
        self.iv             = math.sqrt(v1 ** 2 + v2 ** 2 + 2 * v1 * v2 * corr) / 2
        self.mu             = rate - 0.5 * (v1 ** 2 + v2 ** 2) / 2 + 0.5 * self.iv ** 2
        self.bg             = (S1 * S2) ** ( 1 / 2 )
        self.d1             = ( math.log( self.bg / strike ) + ( self.mu + 0.5 * self.iv ** 2 ) * T ) / ( self.iv * math.sqrt(T) )
        self.d2             = self.d1 - self.iv * math.sqrt( T )

    def price(self):
        if self.option_type == 'call':
            price = math.exp( -self.rate * self.T ) * ( self.bg * math.exp( self.mu * self.T ) * cdf( self.d1 ) - self.strike * cdf( self.d2 ) ) 
        if self.option_type == 'put':            
            price = math.exp( -self.rate * self.T ) * (-self.bg * math.exp( self.mu * self.T ) * cdf(-self.d1 ) + self.strike * cdf(-self.d2 ) ) 
        return price

    def price_monte_carlo(self, iterations, control = True):
        drift1 = math.exp( ( self.rate - 0.5 * self.a1.iv ** 2 ) * self.T )
        drift2 = math.exp( ( self.rate - 0.5 * self.a2.iv ** 2 ) * self.T )

        np.random.seed(888)
        arrA1 = np.random.normal(0, 1, iterations)
        np.random.seed(444)
        arrA3 = np.random.normal(0, 1, iterations)
        arrA2 = self.corr * arrA1 + np.sqrt(1 - self.corr ** 2) * arrA3
        #print("corr(Z1, Z2) = ",sample_correlation(arrA1, arrA2))

        arrR1 = np.exp(self.a1.iv * math.sqrt( self.T ) * arrA1) * drift1
        arrR2 = np.exp(self.a2.iv * math.sqrt( self.T ) * arrA2) * drift2
        arrPA = (self.a1.spot * arrR1 + self.a2.spot * arrR2) / 2
        arrPG = (self.a1.spot * arrR1 * self.a2.spot * arrR2) ** 0.5

        if self.option_type == 'call':
            arithmeticPayoff    = math.exp( -self.rate * self.T ) * np.array( [max ( [ i - self.strike, 0 ] ) for i in arrPA ] )
            geometricPayoff     = math.exp( -self.rate * self.T ) * np.array( [max ( [ i - self.strike, 0 ] ) for i in arrPG ] )
        else:
            arithmeticPayoff    = math.exp( -self.rate * self.T ) * np.array( [max ( [ self.strike - i, 0 ] ) for i in arrPA ] )
            geometricPayoff     = math.exp( -self.rate * self.T ) * np.array( [max ( [ self.strike - i, 0 ] ) for i in arrPG ] )

        meanAP  = np.mean(arithmeticPayoff)
        meanGP  = np.mean(geometricPayoff)
        sdAP    = np.std (arithmeticPayoff, ddof = 1)
        pxClose = self.price()

        if self.option_class == 'geometric':
            print("[Geometric Basket from Standard MC: {0:6f}, Closed-form price: {1:6f}]".format(meanGP, pxClose))
            return (meanGP, pxClose)
        
        if self.option_class == 'arithmetic':
            if control:
                covPP   = np.mean(geometricPayoff * arithmeticPayoff) - np.mean(geometricPayoff) * np.mean(arithmeticPayoff)
                theta   = covPP / np.var(geometricPayoff, ddof = 1)

                listCV  = arithmeticPayoff + theta * (pxClose - geometricPayoff)
                meanCV  = np.mean(listCV)
                sdCV    = np.std (listCV, ddof = 1)
                ciCV    = (meanCV - 1.96 * sdCV / math.sqrt(iterations), meanCV + 1.96 * sdCV / math.sqrt(iterations), meanCV, pxClose)
                print("Control Variate: [Confidence interval: ({0:6f}, {1:6f}), Mean Price: {2:6f}, Closed-form geometric price: {3:6f}]".format(*ciCV))
                return ciCV

            else:
                ciAP    = (meanAP - 1.96 * sdAP / math.sqrt(iterations), meanAP + 1.96 * sdAP / math.sqrt(iterations), meanAP, pxClose)
                print("No Control:      [Confidence interval: ({0:6f}, {1:6f}), Mean Price: {2:6f}, Closed-form geometric price: {3:6f}]".format(*ciAP))
                return ciAP


def sample_correlation(x, y):
    mean_x = np.mean(x)
    mean_y = np.mean(y)
    n = len(x)

    return (sum(x * y) - n * mean_x * mean_y)/math.sqrt(sum(x * x) - n * mean_x**2)/math.sqrt(sum(y * y) - n * mean_y**2)

def iv_newton_method(spot, strike, t, T, r, q, mkt_price, option_type, guess = None):
    if guess is None:
        guess = math.sqrt(2 * abs( (math.log(spot/strike) + (r - q)*(T - t))/(T - t)))

    prev_guess = 0
    i = 0
    target_price = mkt_price

    while i < 100 and abs(guess - prev_guess) > 1e-8 :
        target = BlackScholesOption(spot,strike,t,T,guess,r,q)
        target_vega = target.vega()
        prev_guess = guess
        guess = guess - (target.price() - target_price)/target_vega
        print("next guess: ","{:.6f}".format(guess),"; last guess: ","{:.6f}".format(prev_guess),"; vega: ","{:.6f}".format(target_vega))
        i += 1

    print("implied volatility: {0:6f}".format(guess))
    return guess

# Assignment 2
# Question 1
if False:
    o = BlackScholesOption
    o1 = o(50,50,0,0.5,0.2,0.01, option_type = 'call')
    o2 = o(50,60,0,0.5,0.2,0.01, option_type = 'call')
    o3 = o(50,50,0,1.0,0.2,0.01, option_type = 'call')
    o4 = o(50,50,0,0.5,0.3,0.01, option_type = 'call')
    o5 = o(50,50,0,0.5,0.2,0.02, option_type = 'call')
    options = [o1,o2,o3,o4,o5]

    i = 1   
    for option in options:
        print("(1.",i,")",sep = "")
        print(option)
        print("call value :",option.price(),"; put value :",option.price())
        print()
        i += 1

#Question 2
if False:
    x_rand = np.random.normal(0, 1, 10000)
    y_rand = np.random.normal(0, 1, 10000)
    rho = 0.5
    z_rand = np.array([rho * x + math.sqrt(1 - rho**2) * y for x, y in zip(x_rand, y_rand)])

    print("(Q2)")
    print("correlation:",sample_correlation(x_rand, z_rand))

#Question 3 Data Processing
if False:
    with open('C:\\Users\\hohin\\Downloads\\instruments.csv') as f:
        dfInstrument = pd.DataFrame(csv.reader(f))

    dfInstrument.columns = dfInstrument.iloc[0]
    dfInstrument = dfInstrument[1:]

    with open('C:\\Users\\hohin\\Downloads\\marketdata.csv') as f:
        dfMktData = pd.DataFrame(csv.reader(f))

    dfMktData.columns = dfMktData.iloc[0]
    dfMktData = dfMktData[1:]
    cutoff = 31

    for row in dfMktData.itertuples():
        
        if time.strptime(row.LocalTime, '%Y-%b-%d %H:%M:%S.%f').tm_min < cutoff:
            pass
        else:
            cutoff = time.strptime(row.LocalTime, '%Y-%b-%d %H:%M:%S.%f').tm_min + 1

        dfInstrument.loc[dfInstrument.Symbol == row.Symbol,str(cutoff) + 'b'] = row.Bid1
        dfInstrument.loc[dfInstrument.Symbol == row.Symbol,str(cutoff) + 'a'] = row.Ask1

    dfInstrument.to_csv('out_snapshots.csv', index=False)

if False:
    with open('C:\\Users\\hohin\\Downloads\\out.csv') as f:
        dfInstrument = pd.DataFrame(csv.reader(f))
        dfInstrument.columns = dfInstrument.iloc[0]
        dfInstrument = dfInstrument[1:]

if False:
    for row in dfInstrument.itertuples():
        
        spot_array = dfInstrument.loc[dfInstrument.Symbol == '510050', ['31a','31b']].values[0]
        spot_array = spot_array.astype(np.float64)
        spot = np.mean(spot_array)

        if row.Type == 'Option':
            pass
            # print("pricing",_OTYP[row.OptionType],"option, strike =","{:.2f}".format(float(row.Strike)))
            # dfInstrument.loc[dfInstrument.Symbol == row.Symbol,'31biv'] = iv_newton_method(spot, float(row.Strike), _OTYP[row.OptionType], float(row._6))
            # dfInstrument.loc[dfInstrument.Symbol == row.Symbol,'31aiv'] = iv_newton_method(spot, float(row.Strike), _OTYP[row.OptionType], float(row._7))
            # dfInstrument.loc[dfInstrument.Symbol == row.Symbol,'32biv'] = iv_newton_method(spot, float(row.Strike), _OTYP[row.OptionType], float(row._8))
            # dfInstrument.loc[dfInstrument.Symbol == row.Symbol,'32aiv'] = iv_newton_method(spot, float(row.Strike), _OTYP[row.OptionType], float(row._9))
            # dfInstrument.loc[dfInstrument.Symbol == row.Symbol,'33biv'] = iv_newton_method(spot, float(row.Strike), _OTYP[row.OptionType], float(row._10))
            # dfInstrument.loc[dfInstrument.Symbol == row.Symbol,'33aiv'] = iv_newton_method(spot, float(row.Strike), _OTYP[row.OptionType], float(row._11))

    dfInstrument.to_csv('out_iv.csv', index=False)

# Assignment 3
# From Lecture Notes
# print(BinomialPricer(50, 50, 0.05, 0.3, 0.05, 5, 'call', option_class = 'american').price())
# print(BinomialPricer(50, 52, 2.0, 0.223144, 0.05, 2, 'put', option_class='american').price(verbose=True))
# print(BinomialPricer(50, 52, 2.0, 0.223144, 0.05, 2, 'put', option_class='european').price(verbose=True))
'''
Spot Price Grid
[[50.       62.500028 78.12507 ]
 [ 0.       39.999982 50.      ]
 [ 0.        0.       31.999971]]

Option Price Grid
[[ 5.487243  0.840164  0.      ]
 [ 0.       12.000018  2.      ]
 [ 0.        0.       20.000029]]
5.487243237467179

Spot Price Grid
[[50.       62.500028 78.12507 ]
 [ 0.       39.999982 50.      ]
 [ 0.        0.       31.999971]]

Option Price Grid
[[ 4.421886  0.840164  0.      ]
 [ 0.        9.463948  2.      ]
 [ 0.        0.       20.000029]]
4.421885754517021
'''

# Convergence Test
# cf = BlackScholesOption(50, 50, 0, 0.25, 0.3, 0.05, option_type='call').price()
# for steps in range(10, 101, 10):
#     print(steps, ":", BinomialPricer(50, 50, 0.25, 0.3, 0.05, steps, 'call', 'european').price() - cf)

# cf = BlackScholesOption(50, 50, 0, 0.25, 0.3, 0.05, option_type='call').price()
# print(BinomialPricer(50, 50, 0.25, 0.3, 0.05, 500, 'call', 'european').price() - cf)
'''
-0.0014900545338205617
'''

ao = AsianOption
# From Forum
# g1 = ao(100, 100, 3, 0.3, 0.05, 50, 'call', 'arithmetic')
# g1.price()
# g1.price_monte_carlo(100000, True)
# g1.price_monte_carlo(100000, False)

# g2 = ao(100, 100, 3, 0.3, 0.05, 50, 'put', 'arithmetic')
# g2.price()
# g2.price_monte_carlo(100000, True)
# g2.price_monte_carlo(100000, False)

# g3 = ao(100, 100, 3, 0.3, 0.05, 50, 'call', 'geometric')
# g3.price()
# g3.price_monte_carlo(100000, True)
# g3.price_monte_carlo(100000, False)

# Assignment Test Cases
ao = AsianOption
# t1 = [[0.3,100,50,'put'],[0.3,100,100,'put'],[0.4,100,50,'put'],[0.3,100,50,'call'],[0.3,100,100,'call'],[0.4,100,50,'call']]
# kmc = 100000

# for t in t1:
#     print("\nTest Case: Vol = {0}, K = {1}, n obs = {2}, option type = {3}".format(*t))
#     ag = ao(100, t[1], 3, t[0], 0.05, t[2], t[3], 'geometric')
#     aa = ao(100, t[1], 3, t[0], 0.05, t[2], t[3], 'arithmetic')

#     ag.price()
#     ag.price_monte_carlo(kmc)
#     aa.price_monte_carlo(kmc, control = True)
#     aa.price_monte_carlo(kmc, control = False)

'''
Test Case: Vol = 0.3, K = 100, n obs = 50, option type = put
[Geometric Asian from Standard MC: 8.557693, Closed-form price: 8.482705]
Control Variate: [Confidence interval: (7.797752, 7.806702), Mean Price: 7.802227, Closed-form geometric price: 8.482705]
No Control:      [Confidence interval: (7.803454, 7.942034), Mean Price: 7.872744, Closed-form geometric price: 8.482705]

Test Case: Vol = 0.3, K = 100, n obs = 100, option type = put
[Geometric Asian from Standard MC: 8.518146, Closed-form price: 8.431080]
Control Variate: [Confidence interval: (7.747192, 7.756022), Mean Price: 7.751607, Closed-form geometric price: 8.431080]
No Control:      [Confidence interval: (7.764651, 7.902188), Mean Price: 7.833419, Closed-form geometric price: 8.431080]

Test Case: Vol = 0.4, K = 100, n obs = 50, option type = put
[Geometric Asian from Standard MC: 12.660593, Closed-form price: 12.558769]
Control Variate: [Confidence interval: (11.276688, 11.292345), Mean Price: 11.284516, Closed-form geometric price: 12.558769]
No Control:      [Confidence interval: (11.288642, 11.468903), Mean Price: 11.378772, Closed-form geometric price: 12.558769]

Test Case: Vol = 0.3, K = 100, n obs = 50, option type = call
[Geometric Asian from Standard MC: 13.162008, Closed-form price: 13.259126]
Control Variate: [Confidence interval: (14.717723, 14.739238), Mean Price: 14.728481, Closed-form geometric price: 13.259126]
No Control:      [Confidence interval: (14.477516, 14.764529), Mean Price: 14.621022, Closed-form geometric price: 13.259126]

Test Case: Vol = 0.3, K = 100, n obs = 100, option type = call
[Geometric Asian from Standard MC: 13.004248, Closed-form price: 13.138779]
Control Variate: [Confidence interval: (14.596916, 14.618474), Mean Price: 14.607695, Closed-form geometric price: 13.138779]
No Control:      [Confidence interval: (14.315975, 14.601295), Mean Price: 14.458635, Closed-form geometric price: 13.138779]

Test Case: Vol = 0.4, K = 100, n obs = 50, option type = call
[Geometric Asian from Standard MC: 15.641245, Closed-form price: 15.759820]
Control Variate: [Confidence interval: (18.187189, 18.228187), Mean Price: 18.207688, Closed-form geometric price: 15.759820]
No Control:      [Confidence interval: (17.869973, 18.272265), Mean Price: 18.071119, Closed-form geometric price: 15.759820]
'''

bo = BasketOption
# From Forum
# b1 = bo(100, 100, 0.3, 0.3, 0.05, 3, 100, 0.5, option_type = 'call', option_class='arithmetic')
# b1.price()
# b1.price_monte_carlo(100000, True)
# b1.price_monte_carlo(100000, False)

# b2 = bo(100, 100, 0.3, 0.3, 0.05, 3, 100, 0.5, option_type = 'put', option_class='arithmetic')
# b2.price()
# b2.price_monte_carlo(100000, True)
# b2.price_monte_carlo(100000, False)

# b3 = bo(100, 100, 0.3, 0.3, 0.05, 3, 100, 0.5, option_type = 'call', option_class='geometric')
# b3.price()
# b3.price_monte_carlo(100000, True)

# Assignment Test Cases
bo = BasketOption
# t2 = [[100,100,100,0.3,0.3,0.5,'put'],[100,100,100,0.3,0.3,0.9,'put'],[100,100,100,0.1,0.3,0.5,'put'],[100,100,80,0.3,0.3,0.5,'put'],[100,100,120,0.3,0.3,0.5,'put'],[100,100,100,0.5,0.5,0.5,'put'],[100,100,100,0.3,0.3,0.5,'call'],[100,100,100,0.3,0.3,0.9,'call'],[100,100,100,0.1,0.3,0.5,'call'],[100,100,80,0.3,0.3,0.5,'call'],[100,100,120,0.3,0.3,0.5,'call'],[100,100,100,0.5,0.5,0.5,'call']]
# kmc = 100000

# for t in t2:
#     print("\nTest Case: S0_1 = {0}, S0_2 = {1}, K = {2}, v_1 = {3}, v_2 = {4}, corr = {5}, option type = {6}".format(*t))
#     bg = bo(t[0], t[1], t[3], t[4], 0.05, 3, t[2], t[5], t[6], 'geometric')
#     ba = bo(t[0], t[1], t[3], t[4], 0.05, 3, t[2], t[5], t[6], 'arithmetic')

#     bg.price()
#     bg.price_monte_carlo(kmc)
#     ba.price_monte_carlo(kmc, control = True)
#     ba.price_monte_carlo(kmc, control = False)

'''
Test Case: S0_1 = 100, S0_2 = 100, K = 100, v_1 = 0.3, v_2 = 0.3, corr = 0.5, option type = put
[Geometric Basket from Standard MC: 11.463490, Closed-form price: 11.491573]
Control Variate: [Confidence interval: (10.559941, 10.584037), Mean Price: 10.571989, Closed-form geometric price: 11.491573]
No Control:      [Confidence interval: (10.450727, 10.639323), Mean Price: 10.545025, Closed-form geometric price: 11.491573]

Test Case: S0_1 = 100, S0_2 = 100, K = 100, v_1 = 0.3, v_2 = 0.3, corr = 0.9, option type = put
[Geometric Basket from Standard MC: 12.589602, Closed-form price: 12.622350]
Control Variate: [Confidence interval: (12.422683, 12.428238), Mean Price: 12.425461, Closed-form geometric price: 12.622350]
No Control:      [Confidence interval: (12.288009, 12.497839), Mean Price: 12.392924, Closed-form geometric price: 12.622350]

Test Case: S0_1 = 100, S0_2 = 100, K = 100, v_1 = 0.1, v_2 = 0.3, corr = 0.5, option type = put
[Geometric Basket from Standard MC: 6.580378, Closed-form price: 6.586381]
Control Variate: [Confidence interval: (5.505177, 5.522200), Mean Price: 5.513689, Closed-form geometric price: 6.586381]
No Control:      [Confidence interval: (5.451399, 5.565769), Mean Price: 5.508584, Closed-form geometric price: 6.586381]

Test Case: S0_1 = 100, S0_2 = 100, K = 80, v_1 = 0.3, v_2 = 0.3, corr = 0.5, option type = put
[Geometric Basket from Standard MC: 4.703026, Closed-form price: 4.711577]
Control Variate: [Confidence interval: (4.241074, 4.256443), Mean Price: 4.248759, Closed-form geometric price: 4.711577]
No Control:      [Confidence interval: (4.185506, 4.295878), Mean Price: 4.240692, Closed-form geometric price: 4.711577]

Test Case: S0_1 = 100, S0_2 = 100, K = 120, v_1 = 0.3, v_2 = 0.3, corr = 0.5, option type = put
[Geometric Basket from Standard MC: 21.249924, Closed-form price: 21.289105]
Control Variate: [Confidence interval: (19.861920, 19.894414), Mean Price: 19.878167, Closed-form geometric price: 21.289105]
No Control:      [Confidence interval: (19.706461, 19.973551), Mean Price: 19.840006, Closed-form geometric price: 21.289105]

Test Case: S0_1 = 100, S0_2 = 100, K = 100, v_1 = 0.5, v_2 = 0.5, corr = 0.5, option type = put
[Geometric Basket from Standard MC: 23.425374, Closed-form price: 23.469148]
Control Variate: [Confidence interval: (21.049975, 21.105846), Mean Price: 21.077910, Closed-form geometric price: 23.469148]
No Control:      [Confidence interval: (20.889942, 21.182162), Mean Price: 21.036052, Closed-form geometric price: 23.469148]

Test Case: S0_1 = 100, S0_2 = 100, K = 100, v_1 = 0.3, v_2 = 0.3, corr = 0.5, option type = call
[Geometric Basket from Standard MC: 22.244795, Closed-form price: 22.102093]
Control Variate: [Confidence interval: (24.478994, 24.541658), Mean Price: 24.510326, Closed-form geometric price: 22.102093]
No Control:      [Confidence interval: (24.418021, 24.903844), Mean Price: 24.660933, Closed-form geometric price: 22.102093]

Test Case: S0_1 = 100, S0_2 = 100, K = 100, v_1 = 0.3, v_2 = 0.3, corr = 0.9, option type = call
[Geometric Basket from Standard MC: 26.075112, Closed-form price: 25.878826]
Control Variate: [Confidence interval: (26.348664, 26.361405), Mean Price: 26.355034, Closed-form geometric price: 25.878826]
No Control:      [Confidence interval: (26.275649, 26.831006), Mean Price: 26.553328, Closed-form geometric price: 25.878826]

Test Case: S0_1 = 100, S0_2 = 100, K = 100, v_1 = 0.1, v_2 = 0.3, corr = 0.5, option type = call
[Geometric Basket from Standard MC: 18.006384, Closed-form price: 17.924737]
Control Variate: [Confidence interval: (19.431171, 19.469232), Mean Price: 19.450202, Closed-form geometric price: 17.924737]
No Control:      [Confidence interval: (19.367512, 19.714796), Mean Price: 19.541154, Closed-form geometric price: 17.924737]

Test Case: S0_1 = 100, S0_2 = 100, K = 80, v_1 = 0.3, v_2 = 0.3, corr = 0.5, option type = call
[Geometric Basket from Standard MC: 32.698491, Closed-form price: 32.536256]
Control Variate: [Confidence interval: (35.368752, 35.433413), Mean Price: 35.401082, Closed-form geometric price: 32.536256]
No Control:      [Confidence interval: (35.300125, 35.841394), Mean Price: 35.570759, Closed-form geometric price: 32.536256]

Test Case: S0_1 = 100, S0_2 = 100, K = 120, v_1 = 0.3, v_2 = 0.3, corr = 0.5, option type = call
[Geometric Basket from Standard MC: 14.817070, Closed-form price: 14.685466]
Control Variate: [Confidence interval: (16.571880, 16.631247), Mean Price: 16.601563, Closed-form geometric price: 14.685466]
No Control:      [Confidence interval: (16.530748, 16.952760), Mean Price: 16.741754, Closed-form geometric price: 14.685466]

Test Case: S0_1 = 100, S0_2 = 100, K = 100, v_1 = 0.5, v_2 = 0.5, corr = 0.5, option type = call
[Geometric Basket from Standard MC: 28.707434, Closed-form price: 28.449387]
Control Variate: [Confidence interval: (34.917604, 35.131649), Mean Price: 35.024627, Closed-form geometric price: 28.449387]
No Control:      [Confidence interval: (34.840361, 35.790173), Mean Price: 35.315267, Closed-form geometric price: 28.449387]
'''

def main():
    running = True

    while running:
        s_toplevel = "\nMini Option Pricer\n-------------------------------------\n1. European Option\n2. Implied Volatility Calculator\n3. American Option\n4. Geometric Asian Option\n5. Arithmetic Asian Option\n6. Geometric Basket Option\n7. Arithmetic Basket Option\n9. Exit"
        print(s_toplevel)
        user_input = input("(Enter an option) ")

        try:
            if user_input == '1':
                print("(Enter spot price, volatility, risk-free rate, repo rate, time to maturity (years), strike in decimals)")
                user_input  = input("(Separated by a space) ")
                spot, iv, r, q, T, K = user_input.split()
                user_input2 = input("(Enter option type [1. call / 2. put]) ")
                option_type = 'call' if user_input2 == '1' else 'put'
                user_input3 = input("(Enter pricing method [1. closed-form / 2. binomial tree]) ")

                if user_input3 == '1':
                    eo = BlackScholesOption(float(spot), float(K), 0, float(T), float(iv), float(r), float(q), option_type)
                    print(eo)
                    print("Option Price: {0:6f}".format(eo.price()))
                if user_input3 == '2':
                    steps = int(input("(Enter steps) "))
                    bt = BinomialPricer(float(spot), float(K), float(T), float(iv), float(r), steps, option_type, 'european')
                    bt.price(True if steps <= 10 else False)

                input("(Press [Enter] to continue...) ")

            if user_input == '2':
                print("(Enter spot price, risk-free rate, repo rate, time to maturity (years), strike, option premium in decimals)")
                user_input  = input("(Separated by a space) ")
                spot, r, q, T, K, px = user_input.split()
                user_input2 = input("(Enter option type [1. call / 2. put]) ")
                option_type = 'call' if user_input2 == '1' else 'put'

                iv_newton_method(float(spot), float(K), 0, float(T), float(r), float(q), float(px), option_type)
                input("(Press [Enter] to continue...) ")

            if user_input == '3':
                print("(Enter spot price, volatility, risk-free rate, time to maturity (years), strike in decimals)")
                user_input  = input("(Separated by a space) ")
                spot, iv, r, T, K = user_input.split()
                user_input2 = input("(Enter option type [1. call / 2. put]) ")
                option_type = 'call' if user_input2 == '1' else 'put'

                steps = int(input("(Enter steps) "))
                bt = BinomialPricer(float(spot), float(K), float(T), float(iv), float(r), steps, option_type, 'american')
                bt.price(True if steps <= 10 else False)
                input("(Press [Enter] to continue...) ")

            if user_input == '4':
                print("(Enter spot price, volatility, risk-free rate, time to maturity (years), strike in decmials, n obs in integer)")
                user_input  = input("(Separated by a space) ")
                spot, iv, r, T, K, n = user_input.split()
                user_input2 = input("(Enter option type [1. call / 2. put]) ")
                option_type = 'call' if user_input2 == '1' else 'put'

                ga = AsianOption(float(spot), float(K), float(T), float(iv), float(r), int(n), option_type, option_class = 'geometric')
                user_input3 = input("(Enter pricing method [1. closed-form / 2. monte carlo]) ")

                if user_input3 == '1':
                    print("Option Price: {0:6f}".format(ga.price()))
                if user_input3 == '2':
                    steps = int(input("(Enter steps) "))
                    ga.price_monte_carlo(steps)

                input("(Press [Enter] to continue...) ")

            if user_input == '5':
                print("(Enter spot price, volatility, risk-free rate, time to maturity (years), strike in decmials, n obs in integer)")
                user_input  = input("(Separated by a space) ")
                spot, iv, r, T, K, n = user_input.split()
                user_input2 = input("(Enter option type [1. call / 2. put]) ")
                option_type = 'call' if user_input2 == '1' else 'put'

                ga = AsianOption(float(spot), float(K), float(T), float(iv), float(r), int(n), option_type, option_class = 'arithmetic')
                control = input("(Enter pricing method [1. control variate / 2. no control]) ")
                steps = int(input("(Enter steps) "))

                if control == '1':
                    ga.price_monte_carlo(steps, control = True)
                if control == '2':
                    ga.price_monte_carlo(steps, control = False)

                input("(Press [Enter] to continue...) ")

            if user_input == '6':
                print("(Enter asset 1 spot, asset 2 spot, asset 1 vol, asset 2 vol, risk-free rate, time to maturity (years), strike, asset correlation in decmials in integer)")
                user_input  = input("(Separated by a space) ")
                S1, S2, v1, v2, r, T, K, corr = user_input.split()
                user_input2 = input("(Enter option type [1. call / 2. put]) ")
                option_type = 'call' if user_input2 == '1' else 'put'

                ba = BasketOption(float(S1), float(S2), float(v1), float(v2), float(r), float(T), float(K), float(corr), option_type, option_class = 'geometric')
                user_input3 = input("(Enter pricing method [1. closed-form / 2. monte carlo]) ")

                if user_input3 == '1':
                    print("Option Price: {0:6f}".format(ba.price()))
                if user_input3 == '2':
                    steps = int(input("(Enter steps) "))
                    ba.price_monte_carlo(steps)

                input("(Press [Enter] to continue...) ")

            if user_input == '7':
                print("(Enter asset 1 spot, asset 2 spot, asset 1 vol, asset 2 vol, risk-free rate, time to maturity (years), strike, asset correlation in decmials in integer)")
                user_input  = input("(Separated by a space) ")
                S1, S2, v1, v2, r, T, K, corr = user_input.split()
                user_input2 = input("(Enter option type [1. call / 2. put]) ")
                option_type = 'call' if user_input2 == '1' else 'put'

                ba = BasketOption(float(S1), float(S2), float(v1), float(v2), float(r), float(T), float(K), float(corr), option_type, option_class = 'arithmetic')
                control = input("(Enter pricing method [1. control variate / 2. no control]) ")
                steps = int(input("(Enter steps) "))

                if control == '1':
                    ba.price_monte_carlo(steps, control = True)
                if control == '2':
                    ba.price_monte_carlo(steps, control = False)

                input("(Press [Enter] to continue...) ")

            if user_input == '9':
                running = False       
        except ValueError:
            print("(Incorrect amount or format of parameters entered. Please Try Again.) ")
    return

main()