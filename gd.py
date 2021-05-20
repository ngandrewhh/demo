import copy
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
np.set_printoptions(precision = 6, suppress = True)

def stnorm(x):
    return (x - np.mean(x)) / np.std(x) if np.std(x) != 0 else x

def stnorm_zm(x):
    return (x) / np.std(x) if np.std(x) != 0 else x

def gradient_descent(m_input, m_transform, m_objective, const_inp = None, const_p = None, const_obj = None, \
                     cond_input = 25, cond_obj = 5, conv_input = 0.95, conv_obj = 0.1, step_size = 0.1, iter_min = 0.1, iter_num = 5000, \
                     gradient_precision = 2, norm_method = stnorm_zm, mem_all = False, verbose = False):
    ''' Returns score, input, score series of all attempts '''
    
    x = np.array(m_input, dtype = np.double)
    v = np.array(m_transform)
    o = np.array(m_objective)
    b = const_inp or 0
    c = np.array(const_p) if const_p else np.zeros_like(o)
    d = np.array(const_obj) if const_obj else np.zeros_like(o)
    z = []

    cond_input = abs(cond_input)
    cond_obj = abs(cond_obj)

    # Initialization
    max_score = -np.inf
    max_input = None

    # Initial Transform
    parts1 = np.matmul(x, v) + c
    parts2 = np.multiply(np.power(parts1 + d, np.abs(o)), np.sign(o))
    score  = np.sum(parts2)
    z.append(score)

    i = 0

    while i < iter_num:
        i += 1

        # Compute Forward Gradient
        ndim        = np.shape(x)[0]
        sign_st     = np.zeros(ndim, dtype = np.int)
        size_st     = np.zeros(ndim, dtype = np.double)
        t           = copy.deepcopy(x)

        for n in range(ndim):
            t[n]        = t[n] + 0.0001
            trial1      = np.matmul(t, v) + c
            trial2      = np.multiply(np.power(trial1 + d, np.abs(o)), np.sign(o))
            trial_s     = np.sum(trial2)

            obj_gain    = np.round(trial_s - score, gradient_precision)/0.0001
            size_st[n]  = obj_gain
            sign_st[n]  = np.sign(obj_gain)
            t[n]        = t[n] - 0.0001

        # Calc Step Adj Factor
        st_adj  = np.max([iter_min, 1 - i/iter_num]) ** 2

        # Calc Step
        dx      = step_size * st_adj * (norm_method(size_st) if norm_method else size_st)

        # Assert Input Convergence Margin
        x =  (cond_input * conv_input * (1 - st_adj)) * (x> cond_input) + x * np.logical_not(x> cond_input)
        x = -(cond_input * conv_input * (1 - st_adj)) * (x<-cond_input) + x * np.logical_not(x<-cond_input)

        # Assert Output Convergence Margin
        convm_obj = np.sum(x + dx) + b
        if convm_obj >  cond_obj:
            dx += np.sum( dx * (dx < 0) * conv_obj )
        if convm_obj < -cond_obj:
            dx += np.sum( dx * (dx > 0) * conv_obj )

        # Cache Last Input
        xm1 = x
        dxm1 = dx

        # Do Gradient Descent
        x       = np.round(x + dx, gradient_precision)
        desce1  = np.matmul(x, v) + c
        desce2  = np.multiply(np.power(desce1 + d, np.abs(o)), np.sign(o))
        score_c = np.sum(desce2)
        score_g = score_c - score
        score   = score_c
        z.append(score)

        if mem_all and score > max_score:
            max_score = score
            max_input = copy.deepcopy(x)

        if verbose and (i % 100) == 0:
            print("===== Iteration",i, "========================")
            print("Objective Score:", score)
            print("Score Gain:", score_g)
            print("Sum of Input:", np.sum(x))

    if mem_all:
        score = max_score
        x     = max_input

    print("===== Summary ========================")
    print("Objective Score:", score)
    print("Sum of Input:", np.sum(x))
    print("Input:", x)

    return score, x, z

if __name__ == '__main__':
    ''' Input Parameters '''
    _const_score = 0                # Const to the sum of input
    _const_arr = [1, 2, 3, 4, 5]    # Const to the transformed input 
    _score_arr = [-2, -2, 1, 1, -2] # Power of the objective elems: e.g. 1 means linear +ve score, -2 means quadratic -ve score
    _objcn_arr = None               # Constant of the objective elems: e.g. a score power of -2 and a score of -30 means to not deviate from (x - 30) ^ -2
    _input_arr = (np.random.rand(len(_score_arr)) - 0.5) * 50
    
    np.random.seed(0)
    _trans_arr = np.random.rand(5, 5) # Define transformation of the input. Only square matrix has defined behavior
    
    max_score = -np.inf
    max_input = None
    attempts = []

    ''' Other Parameters:
        I/O:
        cond_input, cond_obj governs the absolute max of the input and sum of the input respectively;
        conv_input, conv_obj governs how drastically should the input, step size in descent should be adjusted to fit the conditions;

        Descent Parameters:
        step_size is the step size of gradient descent (direction of step is governed in runtime)
        iter_min governs the minimum step taken - the step sizes are reducing over the iterations in quadratic fashion
        iter_num is the number of total iterations
        
        Other Parameters:
        gradient_precision is the precision of rounding - naively reduce overfitting
        norm_method provides another way to normalize the step size
        mem_all = True would output the best attempt instead of the latest attempt
        verbose = True would print details every 1
    '''

    for i in range( 1 ):
        argmax_score, argmax_input, attempt = gradient_descent(_input_arr, _trans_arr, _score_arr, \
                                                               const_inp = _const_score, const_p = _const_arr, const_obj = _objcn_arr, \
                                                               cond_input = 15, cond_obj = 10, conv_input = 0.95, conv_obj = 0.05, step_size = 0.005, iter_min = 0.1, iter_num = 10000, \
                                                               gradient_precision = 6, norm_method = stnorm_zm, mem_all = True, verbose = True)
        attempts.extend(attempt)
        if argmax_score > max_score:
            max_score = argmax_score
            max_input = argmax_input

    series_attempts = np.array(attempts)
    series_attempts = series_attempts * (series_attempts > 0 * 0)
    pd.Series(series_attempts).plot()
    plt.show()

