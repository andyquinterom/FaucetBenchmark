# plumber.R

heavy_operation <- function(iterations) {
  value = 0;
  for (i in 1:(iterations * 1000)) {
    value = (sin(value) ^ 2) + (cos(value) ^ 2)
  }
  return(value)
}

#* Do heavy operation with 1000 iterations
#* @get /stress1
function() {
  heavy_operation(1e3)
}

#* Do heavy operation with 10_000 iterations
#* @get /stress2
function() {
  heavy_operation(1e4)
}

#* Do heavy operation with 100_000 iterations
#* @get /stress3
function() {
  heavy_operation(1e5)
}
