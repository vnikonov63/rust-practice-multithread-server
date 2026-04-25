mod handlers;

use _21_multi_thread_server::thread_pool::ThreadPool;
use handlers::{
    std_connections::handle_std_connection, tokio_connections::handle_tokio_connection,
};
use std::{net::TcpListener, thread};

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
    /// Run using multithreaded tokio runtime, using each CPU core available on the system.
    MultiThreadTokio
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Command::SpawnInfiniteThreads) | None => {
            let listener = TcpListener::bind("127.0.0.1:7878")?;
            infinite_thread_generation(listener)?;
        }
        Some(Command::ThreadPool { size }) => {
            let listener = TcpListener::bind("127.0.0.1:7878")?;
            thread_pool(listener, size)?;
        }
        Some(Command::SingleThreadTokio) => {
            let rt = runtime::Builder::new_current_thread().enable_all().build()?;
            rt.block_on(tokio())?;
        }
        Some(Command::MultiThreadTokio) => {
            let threaded_rt = runtime::Runtime::new()?;
            threaded_rt.block_on(tokio())?;
        }
    }

    Ok(())
}

fn infinite_thread_generation(listener: TcpListener) -> std::io::Result<()> {
    for stream in listener.incoming() {
        let stream = stream?;

        let _ = thread::spawn(move || {
            if let Err(err) = handle_std_connection(stream) {
                eprintln!("Failed to handle std connection: {err}");
            }
        });
    }

    Ok(())
}

async fn tokio() -> std::io::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878")
        .await?;
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(handle_tokio_connection(stream));
    }
}

fn thread_pool(listener: TcpListener, pool_size: usize) -> std::io::Result<()> {
    let pool = ThreadPool::new(pool_size)?;

    for stream in listener.incoming() {
        let stream = stream?;
        pool.execute(|| {
            if let Err(err) = handle_std_connection(stream) {
                eprintln!("Failed to handle std connection: {err}");
            }
        })?;
    }
    Ok(())
}
