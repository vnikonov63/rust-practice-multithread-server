#!/bin/zsh
for MODE in spawn-infinite-threads single-thread-tokio multi-thread-tokio
do
  echo "RUNNING $MODE"
  cargo run --release -- $MODE &
  PID=$!

  # Make sure the server actually loads
  sleep 2

  wrk -t4 -c100 -d30s -s scripts/root-60-sleep-20-404-20.lua http://127.0.0.1:7878 > benchmarks/$MODE.txt

  kill $PID
done