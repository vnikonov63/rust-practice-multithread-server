use _21_multi_thread_server::thread_pool::ThreadPool;
use clap::{Parser, Subcommand};
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
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
        let (mut stream, _) = listener.accept().await.unwrap();
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

fn handle_std_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:#?}", http_request);

    let request_line = &http_request[0];

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "static-html/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "static-html/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "static-html/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}

async fn handle_tokio_connection(mut stream: tokio::net::TcpStream) {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
    let buf_reader = tokio::io::BufReader::new(&mut stream);
    let mut lines = buf_reader.lines();
    let mut http_request = Vec::new();

    while let Some(line) = lines.next_line().await.unwrap() {
        if line.is_empty() {
            break;
        }

        http_request.push(line);
    }

    println!("{:#?}", http_request);

    let request_line = &http_request[0];

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "static-html/hello.html"),
        "GET /sleep HTTP/1.1" => {
            tokio::time::sleep(Duration::from_secs(5)).await;
            ("HTTP/1.1 200 OK", "static-html/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "static-html/404.html"),
    };

    let contents = tokio::fs::read_to_string(filename).await.unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).await.unwrap();
}
