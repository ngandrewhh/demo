library(data.table)
setwd("C:\\Users\\user\\Downloads\\")
options(stringAsFactors = FALSE)

# test = fread("https://www.sfc.hk/web/EN/pdf/spr/2019/03/22/Short_Position_Reporting_Aggregated_Data_20190322.csv")
xr = seq(as.Date("20190322", "%Y%m%d"), as.Date("20120831", "%Y%m%d"), by = -7)

for (i in 1:length(xr)) {
  x = xr[i]
  name = paste0(format(x,"%Y%m%d"),".data")
  url = paste0("http://www.sfc.hk/web/EN/pdf/spr/",format(x,"%Y"),"/",format(x,"%m"),"/",format(x,"%d"),"/Short_Position_Reporting_Aggregated_Data_",format(x,"%Y%m%d"),".csv")
  assign(name, tryCatch(fread(url), error = function(e) NA))
}

StockList = fread("FINA4803_StockList.csv")
df = data.frame(matrix(nrow = 0, ncol = nrow(StockList)))
colnames(df) = as.character(StockList$Stock)
xr.name = paste0(format(xr,"%Y%m%d"),".data")

for (i in 1:length(xr)) {
  to_merge = get(xr.name[i])
  if (is.na(to_merge)) {rbind(df, NA)}
  else {
    df2 = data.frame(to_merge[[4]], row.names = as.character(to_merge[[2]]))
    df2 = t(df2)
    df2 = t(df2[1, match(colnames(df), colnames(df2))])
    row.names(df2) = to_merge$Date[1]
    colnames(df2) = colnames(df)
    df = rbind(df, df2)
  }
}

colnames(df) = paste0(colnames(df), " HK Equity")
write.csv(df, file = "FINA4803_ShortInterest.csv")

# for (i in 1:length(xr)) {
#   x = xr[i]
#   name = paste0(format(x,"%Y%m%d"),".data")
#   if (!is.na(get(name))) {
#     oname = paste0(format(x,"%Y%m%d"),".csv")    
#     fwrite(get(name), oname)
#   }
# }
