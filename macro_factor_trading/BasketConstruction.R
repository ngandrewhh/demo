library(data.table)
library(zoo)
library(ggplot2)
setwd("C:\\Users\\user\\Downloads\\FINA4803_FYP\\Factor Data")
options(stringAsFactors = FALSE)

# Free Parameters
cutoff = 0.1 # top/bot X% of stocks included in the baskets
  
# Global Variables
factors = c("1MPriceReversal","2YBetaCIP","2YBetaHSCI","6MSharpeRatio",
                   "11MPriceMomentum","12MBuybackYield","12MDivGrowth",
                   "12MEPSGrowth","12MFEPS","12MFSalesGrowth","12MPB",
                   "AltmanZScore","NDE","ShortInterestRatio")
# not_in_used = c("6MSharpeRatioD")
returns = c("1MReturn","1DReturn")

bmk_returns = c("1MReturnHSCI","1DReturnHSCI")

init_variables = c(factors, returns, bmk_returns)

initalize_model <- function() {
  for (i in 1:length(init_variables)) {
    for (j in 1:length(init_variables[i])) {
      assign(init_variables[i][j], 
             fread(paste0("FINA4803_",init_variables[i][j],".csv")),
             envir = .GlobalEnv)
    }  
  }
}

# Initialization
initalize_model()
cat("Model Initialization Completed.\n")

# Extract Date and Securities
date = `1MReturn`$Date
rdate = date[length(date):1]
ddate = `1DReturn`$Date
secs = colnames(`1MReturn`)[-1]

for (i in 1:length(init_variables)) {
  for (j in 1:length(init_variables[i])) {
    assign(init_variables[i][j],get(init_variables[i][j])[,1:=NULL])
  }  
}

# NA Cleaning in Returns
for (i in 1:length(returns)) {
  df = get(returns[i])
  df[is.na(df)] = 0
  assign(returns[i],df)
}

# Applying transformations
# Note that this part is now unused.

# Ranking of Factors
prank <- function(x) {return(rank(x, na.last = "keep", ties.method = "average"))}# , cols = NULL, na.last = NA))}

for (i in 1:length(factors)){
  cat("Applying to",factors[i],"...\n")
  dt = get(factors[i])
  dt = data.table(t(apply(dt, 1, prank)))
  assign(factors[i], setnames(dt, secs))
}

cat("Ranking of Factors Completed.\n")

# Construction of H/L Baskets
basket_forming <- function(factor_name) {
  df = get(factor_name)

  for (i in 1:length(date)){
    x = df[i]
    rcount = (length(secs) - sum(is.na(x)))
    rparti = round(rcount * cutoff)
    lcutoff = rcount - rparti
    hcutoff = rparti
    lrow = x[,x > lcutoff & !is.na(x)]
    hrow = x[,x < hcutoff & !is.na(x)]
    assign(paste0("H",factor_name), rbind(get(paste0("H",factor_name)), hrow), envir = .GlobalEnv)
    assign(paste0("L",factor_name), rbind(get(paste0("L",factor_name)), lrow), envir = .GlobalEnv)
  }
}

for (i in 1:length(factors)){
  assign(paste0("H",factors[i]),data.frame(matrix(nrow = 0, ncol = length(secs))))
  assign(paste0("L",factors[i]),data.frame(matrix(nrow = 0, ncol = length(secs))))
  cat("Constructing H/L Baskets for",factors[i],"...\n")
  basket_forming(factors[i])
}
cat("Construction of Factor Baskets Completed.\n")

# Applying Weights
cat("Application of Weights Completed.\n")

# Returns Calculation
options_HL = c("H","L")

cat("Calculating Returns of Factor Baskets.\n")
for (i in 1:length(factors)){
  for (j in 1:2){
    x = data.table(get(paste0(options_HL[j],factors[i])))
    vec_norm = rowSums(x == TRUE)
    basket_return_m = as.matrix(`1MReturn`) %*% t(as.matrix(x))
    basket_return = diag(basket_return_m)/vec_norm
    assign(paste0("R",options_HL[j],factors[i]),basket_return)
  }
}

cat("Calculation of Returns Completed.\n")

# Returns Aggregation
RMaster = cbind(RH11MPriceMomentum,RH12MBuybackYield,RH12MDivGrowth,
            RH12MEPSGrowth,RH12MFEPS,RH12MFSalesGrowth,RH12MPB,
            RH1MPriceReversal,RH2YBetaCIP,RH2YBetaHSCI,RH6MSharpeRatio,
            RHAltmanZScore,RHNDE,RHShortInterestRatio,RL11MPriceMomentum,
            RL12MBuybackYield,RL12MDivGrowth,RL12MEPSGrowth,RL12MFEPS,
            RL12MFSalesGrowth,RL12MPB,RL1MPriceReversal,RL2YBetaCIP,
            RL2YBetaHSCI,RL6MSharpeRatio,RLAltmanZScore,RLNDE,
            RLShortInterestRatio)

# Creating Periodic Return Master
PRMaster = RMaster[nrow(RMaster):1, ]
rownames(PRMaster) = date[length(date):1]
PRMaster = data.frame(PRMaster)

# Creating Cumulative Return Master
CRMaster = RMaster[nrow(RMaster):1, ]
CRMaster[is.nan(CRMaster)] = 0
CRMaster = CRMaster + 1
CRMaster = apply(CRMaster, 2, cumprod)
CRMaster = CRMaster - 1
rownames(CRMaster) = date[length(date):1]

# Zooing
ZRMaster = as.zoo(CRMaster)
time(ZRMaster) = as.Date(date[length(date):1], format = "%d/%m/%Y")

# Exporting Graphs
# for (i in 1:ncol(ZRMaster)){ 
#   g = autoplot.zoo(ZRMaster[,i])
#   g = g + xlab("Year") + ylab(colnames(ZRMaster)[i]) + geom_line(size = 1)
#   ggsave(paste0(colnames(ZRMaster)[i],".png"), plot = g, device = png(),
#          height = 8, width = 10, units = "cm")
# }

# Exporting PRMaster
# fwrite(PRMaster,"FINA4803_PRMaster.csv", row.names = TRUE)

# Exporting HSCI Return
R1MReturnHSCI = `1MReturnHSCI`[nrow(`1MReturnHSCI`):1]
R1MReturnHSCI = R1MReturnHSCI + 1
R1MReturnHSCI = cumprod(R1MReturnHSCI)
R1MReturnHSCI = R1MReturnHSCI - 1

z.R1MReturnHSCI = as.zoo(R1MReturnHSCI)
time(z.R1MReturnHSCI) = as.Date(rdate, format = "%d/%m/%Y")
# g = autoplot.zoo(z.R1MReturnHSCI)
# g = g + xlab("Year") + ylab("HSCI Index") + geom_line(size = 1)
# ggsave("R1MHSCIReturn.png", plot = g, device = png(),height = 8, width = 10, units = "cm")
