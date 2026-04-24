counter = 0

paths = {
  "/", "/", "/", "/", "/", "/",
  "/sleep", "/sleep",
  "/404", "/404"
}

request = function()
  counter = counter + 1
  local i = ((counter - 1) % #paths) + 1
  return wrk.format("GET", paths[i])
end