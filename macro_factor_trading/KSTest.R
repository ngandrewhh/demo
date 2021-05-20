# Running KS Tests
library(data.table)
library(stats)
setwd("C:\\Users\\user\\Downloads\\")
options(stringAsFactors = FALSE)
# scenario = 1

# Global Variables
factors = c("1MPriceReversal","2YBetaCIP","2YBetaHSCI","6MSharpeRatio",
            "11MPriceMomentum","12MBuybackYield","12MDivGrowth",
            "12MEPSGrowth","12MFEPS","12MFSalesGrowth","12MPB",
            "AltmanZScore","NDE","ShortInterestRatio")
# not_in_used = c("6MSharpeRatioD")
returns = c("1MReturn","1DReturn")

bmk_returns = c("1MReturnHSCI","1DReturnHSCI")

init_variables = c(factors, returns, bmk_returns)

# Load Data
PRMaster = fread("FINA4803_PRMaster.csv")
MacroCycle = fread(paste0("FINA4803_S",scenario,"MacroCycle.csv"))

# Cbind Data
PRMaster = cbind(PRMaster, MacroCycle$state)
PRMaster[,1] = NULL
assign("KSTest",data.frame(matrix(nrow = 4, ncol = 28)))

options_KS = c(c(1,2),c(1,3),c(2,4),c(3,4))

# Running Pairwise KS Tests
for (i in 1:28){
  for (j in 1:4){
    KSTest[j,i] = ks.test(unlist(PRMaster[,..i][PRMaster[, V2 == options_KS[2*j-1]]]),
                          unlist(PRMaster[,..i][PRMaster[, V2 == options_KS[2*j]]]))$p.value    
  }
}

colnames(KSTest) <- colnames(PRMaster)[1:28]
fwrite(KSTest,paste0("FINA4803_S",scenario,"KSTest.csv"))
