mod handlers;

use _21_multi_thread_server::thread_pool::ThreadPool;
use handlers::{
    std_connections::handle_std_connection, tokio_connections::handle_tokio_connection,
};
use std::{io::prelude::*, net::TcpListener, thread, time::Duration};

use clap::{Parser, Subcommand};
use tokio::runtime;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Run spawning unbounded threads (default behavior)
    SpawnInfiniteThreads,
    /// Run using a fixed-size thread pool
    ThreadPool {
        /// Number of threads in the pool
        size: usize,
    },
    /// Run using single threaded tokio runtime
    SingleThreadTokio,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Command::SpawnInfiniteThreads) | None => {
            let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
            infinite_thread_generation(listener);
        }
        Some(Command::ThreadPool { size }) => {
            let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
            thread_pool(listener, size);
        }
        Some(Command::SingleThreadTokio) => {
            let rt = runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(tokio_single_threaded())
        }
    }
}

fn infinite_thread_generation(listener: TcpListener) {
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let _ = thread::spawn(|| {
            handle_std_connection(stream);
        });
    }
}

async fn tokio_single_threaded() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878")
        .await
        .unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(handle_tokio_connection(stream));
    }
}

fn thread_pool(listener: TcpListener, pool_size: usize) {
    let pool = ThreadPool::new(pool_size);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_std_connection(stream);
        })
    }
}
