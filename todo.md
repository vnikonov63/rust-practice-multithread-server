## Things to complete in this project
* Learn how to limit the value of the size argument with clap
* Add proper error handling with [`anyhow`](https://docs.rs/anyhow/latest/anyhow/). Also note, that I must not have `.unwrap()` or any other code that would make the code `panic!()` unless it is unrecoverable. But I cannot think of the situation when I would have an unrecoverable error in this code. 
* Do not use `thread::spawn` as it may `panic!()`, instead use `thread::Builder`
* Create a separate option to deal with server requests using 
  * [`Fork-join model`](https://en.wikipedia.org/wiki/Fork%E2%80%93join_model)
  * Async I/O model with [`tokio`](https://crates.io/crates/tokio)
    * Multi-threaded.
    * Single-threaded. This could be done using `#[tokio::main(flavor = "current_thread")]`
  * Low-level event-driven I/O and [`mio`](https://github.com/tokio-rs/mio)
* Brighten up the server, having more different routes.
## Completed tasks & a brief comment
