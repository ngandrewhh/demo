# Max Sharpe Portfolio Construction
require(data.table)
require(quadprog)
require(Matrix)
require(zoo)
require(ggplot2)
setwd("C:\\Users\\user\\Downloads\\FINA4803_FYP\\Scenario Data")
options(stringAsFactors = FALSE)

# free parameters
prec <- 3               # number of decimals for stock weight
n_top <- 14             # maximum number of baskets allowed
subsample_range <- NULL # if subsample is to be used, changes @date variable
using <- 1              # 1 = stockwise, 2 = basket

# global parameters
scenario <- 7 # our chosen macro state
map <- function(w, slogi){
  r = data.frame(matrix(nrow = 1, ncol = 0))
  map_pos = 1
  
  for (i in 1:length(slogi)){
    if (slogi[i]) {
      r[,ncol(r) + 1] = w[map_pos]
      map_pos = map_pos+1
    } else {
      r[,ncol(r) + 1] = 0
    }
  }
  
  return(r)
}

# ***************************************************
#* TODO: change definition of vnorm to regularize    *
# **************************************************
compute_return <- function(optimized_weight, ref_return, macro){
  ref_return.r = ref_return[nrow(ref_return):1]
  optimized_return = data.frame(matrix(nrow = 0, ncol = 1))
  
  for (i in 1:nrow(ref_return.r)){
    tmp = data.frame(as.matrix(ref_return.r[i,]) %*% t(as.matrix(optimized_weight[macro[i],])))
    if (scenario == 5 || scenario == 7) { 
      if (i == 1 || i == 2) {
        optimized_return = rbind(optimized_return, 0)
        colnames(optimized_return) = "Portfolio"
      } 
      else {
        vnorm = 1 - sum(optimized_weight[macro[i], ][ref_return.r[i, ] == 0 & optimized_weight[macro[i], ] != 0])
        if (!vnorm) { vnorm = 1 }
        optimized_return = rbind(optimized_return/vnorm, setNames(tmp,"Portfolio"))
      }
    } 
    else { 
      if (i == 1) {
        optimized_return = rbind(optimized_return, 0)
        colnames(optimized_return) = "Portfolio"
      }
      else {
        vnorm = 1 - sum(optimized_weight[macro[i], ][ref_return.r[i, ] == 0 & optimized_weight[macro[i], ] != 0])
        if (!vnorm) { vnorm = 1 }
        optimized_return = rbind(optimized_return/vnorm, setNames(tmp,"Portfolio"))
      }
    }
  }
  return(optimized_return)
}

# load data
MacroCycle <- fread(paste0("FINA4803_S",scenario,"MacroCycle.csv"))
macro <- MacroCycle$state
PRMaster <- fread("..\\Factor Data\\FINA4803_PRMaster.csv")
date <- PRMaster[[1]]
PRMaster <- PRMaster[,-1]

if (using == 1) {
  # select top factor return baskets
  n_top_baskets = data.frame(matrix(nrow = 0, ncol = n_top), stringsAsFactors = FALSE)
  
  for (i in 1:4){
    n_top_baskets.s = apply(PRMaster[macro == i,], 2, mean, na.rm = TRUE)
    n_top_baskets.top = n_top_baskets.s[rank(-n_top_baskets.s) <= n_top]
    n_top_baskets[nrow(n_top_baskets)+1,] =
      t(data.frame(names(n_top_baskets.top), stringsAsFactors = FALSE))
  }
  
  row.names(n_top_baskets) = c(1,2,3,4)
  cat("Retrieved top", n_top,"return baskets...\n")
  n_top_baskets
  
  # construct portfolio from relevant stocks
  optimized_weight = data.frame(matrix(nrow = 0, ncol = length(secs)))
  optimized_pa = data.frame(matrix(nrow = 0, ncol = 1))
  
  for (i in 1:4){
    stocks_picked = data.frame(matrix(nrow = length(date), ncol = length(secs)))
    stocks_picked[is.na(stocks_picked)] = 0
  
    for (j in 1:n_top){
      stocks_picked = stocks_picked + get(substring(n_top_baskets[i,j],2))
    }
  
    # Stocking picking based on whether a stock appear in the most recent
    # month's factor basket !! Will have forelooking bias
    slogi <- c(stocks_picked[1, ] > 0) # stock pick logical vector
    mData <- `1MReturn`[, slogi, with = FALSE]
    mData <- as.matrix(mData)
    nA    <- sum(slogi)
    cat("Total stocks relevant:", nA,"for state", i, "...\n")
    rf    <- 0     # riskfree rate (2.5% pa)
    mu    <- apply(mData, 2, mean, na.rm = TRUE)    # means
    mu2   <- mu - rf                  # excess means

    # ***************************************************
    #* TODO: change definition of Dmat  to regularize    *
    # **************************************************
    
    # qp
    Dmat <- cov(mData, use = "complete.obs")
    # diagonal = diag(Dmat)
    # Dmat <- matrix(0, nrow = nA, ncol = nA)
    # diag(Dmat) = diagonal
    dvec <- array(0, dim = c(nA,1))

    # No Short Selling
    Amat <- matrix (1, nrow=nA)
    Amat <- cbind(1, diag(nA))
    bvec <- 1
    bvec <- c(bvec, rep(0, nA))

    meq  <- 1
    solQP <- solve.QP(nearPD(Dmat)$mat, dvec, Amat, bvec, meq = 1)

    # rescale variables to obtain weights
    w <- as.matrix(solQP$solution/sum(solQP$solution))
    optimized_weight <- rbind(optimized_weight, map(w, slogi))
    # colnames(optimized_weight) <- secs
    # View(t(optimized_weight))
    
    SR <- t(w) %*% mu2 / sqrt(t(w) %*% Dmat %*% w)
    optimized_pa <- rbind(optimized_pa, SR)
    
    # qp (original)
    # Dmat <- cov(mData, use = "complete.obs")
    # dvec <- array(0, dim = c(nA,1))
    # 
    # # No Short Selling
    # Amat <- matrix (1, nrow=nA)
    # Amat <- cbind(1, diag(nA))
    # bvec <- 1
    # bvec <- c(bvec, rep(0, nA))
    # 
    # meq  <- 1
    # solQP <- solve.QP(nearPD(Dmat)$mat, dvec, Amat, bvec, meq = 1)

    # rescale variables to obtain weights
    # w <- as.matrix(solQP$solution/sum(solQP$solution))
    # optimized_weight <- rbind(optimized_weight, map(w, slogi))
    
    # compute sharpe ratio
    # SR <- t(w) %*% mu2 / sqrt(t(w) %/*% Dmat %*% w)
    # optimized_pa <- rbind(optimized_pa, SR)
    # cat("Portfolio Weight Optimization Completed.\n")
  }
  
  # Overfitting treatment if applicable
  optimized_weight = round(optimized_weight, prec)
  for (i in 1:4){
    optimized_weight[i, ] = optimized_weight[i, ]/sum(optimized_weight[i, ])
  }
  
  # Save weight
  colnames(optimized_weight) <- secs
  fwrite(optimized_weight, paste0("FINA4803_S",scenario,"Weight.csv"), row.names =TRUE)
  
  # Compute Benched Return
  cat("Computing Benched Return...\n")
  optimized_returns = data.frame(matrix(nrow = length(date), ncol = 0))
  optimized_return = compute_return(optimized_weight, `1MReturn`, macro)
  optimized_returns = cbind(optimized_returns, optimized_return)
  
  # What-If Scenarios of Nonsensical Investment
  for (j in 1:4){
    optimized_return = compute_return(optimized_weight, `1MReturn`, c(rep(j, length(date)))) 
    optimized_returns = cbind(optimized_returns, optimized_return)
  }
  
} 
if (using == 2) {
  
  # optimization by basket
  optimized_weight = data.frame(matrix(nrow = 0, ncol = 28))
  optimized_pa = data.frame(matrix(nrow = 0, ncol = 1))
  
  for (i in 1:4) {
    nA    <- 28
    mData <- PRMaster[macro == i,]
    mData <- as.matrix(mData)
    rf     <- 0     # riskfree rate (2.5% pa)
    mu     <- apply(mData, 2, mean, na.rm = TRUE)    # means
    mu2    <- mu - rf                  # excess means
    
    # qp
    Dmat <- cov(mData, use = "complete.obs")
    dvec <- array(0, dim = c(nA,1))
    
    # Yay Short Selling
    # Amat <- as.matrix(mu2)
    # bvec <- 1 # set expectation of portfolio excess return to 1
    
    # No Short Selling
    Amat <- matrix (1, nrow=nA)
    Amat <- cbind(1, diag(nA))
    bvec <- 1
    bvec <- c(bvec, rep(0, nA))
    
    meq  <- 1
    solQP <- solve.QP(nearPD(Dmat)$mat, dvec, Amat, bvec, meq = 1)
    # solQP <- solve.QP(cov(mData, use = "p"), zeros, mData, bVec, meq = 1)
    
    # rescale variables to obtain weights
    w <- as.matrix(solQP$solution/sum(solQP$solution))
    optimized_weight <- rbind(optimized_weight, t(w))
    
    # compute sharpe ratio
    SR <- t(w) %*% mu2 / sqrt(t(w) %*% cov(mData, use = "complete.obs") %*% w)
    optimized_pa <- rbind(optimized_pa, SR)
  }
  
  colnames(optimized_weight) <- colnames(PRMaster)
  fwrite(optimized_weight, paste0("FINA4803_S",scenario,"Weight.csv"), row.names =TRUE)

  # Compute Benched Return
  optimized_returns = data.frame(matrix(nrow = length(date), ncol = 0))
  PRMaster[is.na(PRMaster)] = 0
  
  optimized_return = data.frame(matrix(nrow = 0, ncol = 1))
  for (i in 1:nrow(PRMaster)){
    tmp = data.frame(as.matrix(PRMaster[i,]) %*% t(as.matrix(optimized_weight[macro[i],])))
    if (scenario == 5 || scenario == 7) { 
      if (i == 1 || i == 2) {
        optimized_return = rbind(optimized_return, 0)
        colnames(optimized_return) = "Portfolio"
      } 
      else {
        optimized_return = rbind(optimized_return, setNames(tmp,"Portfolio"))
      }
    } 
    else { 
      if (i == 1) {
        optimized_return = rbind(optimized_return, 0)
        colnames(optimized_return) = "Portfolio"
      }
      else {
        optimized_return = rbind(optimized_return, setNames(tmp,"Portfolio"))
      }
    }
  }
  
  optimized_returns = cbind(optimized_returns, optimized_return)
  
  # What-If Scenarios of Nonsensical Investment
  for (j in 1:4){
    optimized_return = data.frame(matrix(nrow = 0, ncol = 1))
  
    for (i in 1:nrow(PRMaster)){
      tmp = data.frame(as.matrix(PRMaster[i,]) %*% t(as.matrix(optimized_weight[j,])))
      if (scenario == 5 || scenario == 7) { 
        if (i == 1 || i == 2) {
          optimized_return = rbind(optimized_return, 0)
          colnames(optimized_return) = "Portfolio"
        } 
        else {
          optimized_return = rbind(optimized_return, setNames(tmp,"Portfolio"))
        }
      } 
      else { 
        if (i == 1) {
          optimized_return = rbind(optimized_return, 0)
          colnames(optimized_return) = "Portfolio"
        }
        else {
          optimized_return = rbind(optimized_return, setNames(tmp,"Portfolio"))
        }
      }
    }
    optimized_returns = cbind(optimized_returns, optimized_return)
  }
}

# Compute Periodic Returns and Perf Ana
optimized_returns = cbind(optimized_returns, `1MReturnHSCI`[length(date):1])
colnames(optimized_returns) = c("Portfolio","W1","W2","W3","W4","HSCI")

optimized_pa = rbind(optimized_pa, mean(unlist(optimized_returns[,1]))/sd(unlist(optimized_returns[,1])))
row.names(optimized_pa) = c("1","2","3","4","P")
optimized_pa = cbind(c(sd(optimized_returns[macro == 1,1]),sd(optimized_returns[macro == 2,1]),sd(optimized_returns[macro == 3,1]),sd(optimized_returns[macro == 4,1]),sd(unlist(optimized_returns[,1]))),optimized_pa)
optimized_pa = cbind(c(mean(optimized_returns[macro == 1,1]),mean(optimized_returns[macro == 2,1]),mean(optimized_returns[macro == 3,1]),mean(optimized_returns[macro == 4,1]),mean(unlist(optimized_returns[,1]))),optimized_pa)
colnames(optimized_pa) = c("Mean", "Volatility", "SharpeRatio")
fwrite(optimized_pa, paste0("FINA4803_S",scenario,"PA.csv"), row.names = TRUE)

# Cumulative Return
optimized_returns = optimized_returns + 1
optimized_returns = apply(optimized_returns, 2, cumprod)
optimized_returns = optimized_returns - 1
z.optimized_returns = as.zoo(optimized_returns)
time(z.optimized_returns) = as.Date(date, format = "%d/%m/%Y")

g = autoplot.zoo(z.optimized_returns, facets = NULL)
g = g + xlab("Year") + ylab(paste0("Scenario ",scenario)) + geom_line(size = 1)
ggsave(paste0("FINA4803_S",scenario,"Return.png"), plot = g, # device = png(),
       height = 8, width = 13, units = "cm")
g

########################
# UNUSED CODE SECTIONS #
########################

# Chunk 5 years
# optimized_return5y = optimized_return[157:230,]
# optimized_return5y = optimized_return5y + 1
# optimized_return5y = cumprod(optimized_return5y)
# optimized_return5y = optimized_return5y - 1
# z.optimized_return5y = as.zoo(optimized_return5y)
# time(z.optimized_return5y) = as.Date(date[157:230], format = "%d/%m/%Y")
# 
# g = autoplot.zoo(z.optimized_return5y)
# g = g + xlab("Year") + ylab(paste0("Scenario ",scenario)) + geom_line(size = 1)
# ggsave(paste0("FINA4803_S",scenario,"Return5Y.png"), plot = g, device = png(),
#        height = 8, width = 10, units = "cm")

# Stock-wise Tangency Portfolio (isSingular, UNUSED)
# sigma = solve(cov(`1MReturn`, use = "complete.obs"))

# Compute Return
# optimized_return = data.frame(matrix(nrow = 0, ncol = 1))
# PRMaster[is.na(PRMaster)] = 0
# 
# for (i in 1:nrow(PRMaster)){
#   tmp = data.frame(as.matrix(PRMaster[i,]) %*% t(as.matrix(optimized_weight[macro[i],])))
#   if (i == 1) {
#     optimized_return = rbind(optimized_return, 0)
#     colnames(optimized_return) = "Portfolio"
#   } else {
#     optimized_return = rbind(optimized_return, setNames(tmp,"Portfolio"))
#   }
# }
