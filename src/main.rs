use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use clap::{Parser, Subcommand};
use _21_multi_thread_server::thread_pool::ThreadPool;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Spawn unbounded threads (default behavior)
    SpawnInfiniteThreads,
    /// Run using a fixed-size thread pool
    ThreadPool {
        // TODO: learn how to limit the size argument here, it should be larger than 50
        /// Number of threads in the pool
        size: usize,
    },
}

fn main() {
    let args = Args::parse();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    match args.command {
        Some(Command::SpawnInfiniteThreads) | None => {
            infinite_thread_generation(listener);
        }
        Some(Command::ThreadPool { size }) => {
            thread_pool(listener, size);
        }
    }
}

fn infinite_thread_generation(listener: TcpListener) {
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let _ = thread::spawn(|| {
            handle_connection(stream);
        });
    }
}


fn thread_pool(listener: TcpListener, pool_size: usize) {
        let pool = ThreadPool::new(pool_size);

        for stream in listener.incoming().take(2) {
            let stream = stream.unwrap();
            pool.execute(|| {
                handle_connection(stream);
            })
        }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:#?}", http_request);

    let request_line = &http_request[0];

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}

// TODO: add proper error handling with anyhow
