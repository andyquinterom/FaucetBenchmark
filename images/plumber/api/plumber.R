# plumber.R

heavy_operation <- function(millis) {
  Sys.sleep(millis / 1000)
}

#* @param weight
#* @get /stress1
function(weight) {
  heavy_operation(5 * as.numeric(weight))
}

#* @param weight
#* @get /stress2
function(weight) {
  heavy_operation(10 * as.numeric(weight))
}

#* @param weight
#* @get /stress3
function(weight) {
  heavy_operation(15 * as.numeric(weight))
}
