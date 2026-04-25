
## Usage:
* `_21_multi_thread_server [COMMAND]` or `cargo run [COMMAND]`

### Commands:
* `spawn-infinite-threads`  Run spawning unbounded threads (default behavior)
* `thread-pool [size]`       Run using a fixed-size thread pool of `[size]` 
* `single-thread-tokio`     Run using single threaded tokio runtime
* `help`                    Print this message or the help of the given subcommand(s)

### Options:
* `-h`, `--help`     Print help
* `-V`, `--version`  Print version

### Benchmarking:
* Start up a desired server in release mode with `cargo run --release -- [COMMAND]` 
* And in a separate terminal window run 
```wrk -t4 -c100 -d30s -s scripts/root-60-sleep-20-404-20.lua http://127.0.0.1:7878```
* Above would simulate 100 users constantly sending requests for all 3 available routes (root, sleep, 404) in 3:1:1 proportion respectively for 30 seconds using 4 threads to generate load.
* To test all of the available options you can use `scripts/benchmark.sh` which would run all of the server implementation options and write benchmark results to files within `./benchmarks`.


## Things to complete in this project
* [ ] Learn how to limit the value of the size argument with clap
* [ ] Add proper error handling with [`anyhow`](https://docs.rs/anyhow/latest/anyhow/)
* [x] Remove `unwrap` from the project.
- [x] Do not use `thread::spawn` as it may `panic!()`, instead use `thread::Builder`.
* [ ] Create a separate option to deal with server requests using:
    * [ ] [`Fork-join model`](https://en.wikipedia.org/wiki/Fork%E2%80%93join_model).
    * [x] Async I/O model with [`tokio`](https://crates.io/crates/tokio).
        * [x] Multi-threaded.
        * [x] Single-threaded. This could be done using `#[tokio::main(flavor = "current_thread")]`.
    * [ ] Low-level event-driven I/O and [`mio`](https://github.com/tokio-rs/mio).
* [ ] Brighten up the server, having more different routes.
* [ ] Add proper server logs, that persist for the user.
* [ ] Understand how can I write tests for this application
* [ ] Understand how can I measure the performance of each of the options to run the server with.
  * [x] Use `wrk` to simulate `HTTP/1.1` requests. 
  * [x] Use `.lua` scripts to simulate calls to all 3 endpoints the server has in somewhat mixed fashion.
  * [x] Write a script to automate starting up the server, running the `wrk` load, shutting down the server for different server options and storing the result in `./benchmarks` 
  * [ ] Graph to visualize results. 

## Learning takeaways

### Terminology
- **Thread Pool** - a group of spawned threads that are ready and waiting to handle a task. When a program has a task it needs to complete it asignes an available thread to it. 
- **Worker** - a structure that picks up the code and runs it in its thread.