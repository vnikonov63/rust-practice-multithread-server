
## Usage:
* `_21_multi_thread_server [COMMAND]` or `cargo run [COMMAND]`

### Commands:
* `spawn-infinite-threads`  Run spawning unbounded threads (default behavior)
* `thread-pool`             Run using a fixed-size thread pool
* `single-thread-tokio`     Run using single threaded tokio runtime
* `help`                    Print this message or the help of the given subcommand(s)

### Options:
* `-h`, `--help`     Print help
* `-V`, `--version`  Print version

## Things to complete in this project
- [ ] Learn how to limit the value of the size argument with clap
- [ ] Add proper error handling with [`anyhow`](https://docs.rs/anyhow/latest/anyhow/). Also note, that I must not have `.unwrap()` or any other code that would make the code `panic!()` unless it is unrecoverable. But I cannot think of the situation when I would have an unrecoverable error in this code.
- [ ] Do not use `thread::spawn` as it may `panic!()`, instead use `thread::Builder`.
* Create a separate option to deal with server requests using:
  - [ ] [`Fork-join model`](https://en.wikipedia.org/wiki/Fork%E2%80%93join_model).
    * Async I/O model with [`tokio`](https://crates.io/crates/tokio).
      - [ ] Multi-threaded.
      - [x] Single-threaded. This could be done using `#[tokio::main(flavor = "current_thread")]`.
    - [ ] Low-level event-driven I/O and [`mio`](https://github.com/tokio-rs/mio).
- [ ] Brighten up the server, having more different routes.
- [ ] Add proper server logs, that persist for the user.
- [ ] Understand how can I write tests for this application
- [ ] Understand how can I measure the performance of each of the options to run the server with. And create a graph to compare them. For this I need to create requests from somewhere outside the browser. So the behavior can be recreated.

## Completed tasks & a brief comment

### Terminology
- **Thread Pool** - a group of spawned threads that are ready and waiting to handle a task. When a program has a task it needs to complete it asignes an available thread to it. 
- **Worker** - a structure that picks up the code and runs it in its thread.