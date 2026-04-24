use std::time::Duration;
use tokio::{
    fs::read_to_string,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    time::sleep,
};

pub async fn handle_tokio_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let mut lines = buf_reader.lines();
    let mut http_request = Vec::new();

    while let Some(line) = lines.next_line().await.unwrap() {
        if line.is_empty() {
            break;
        }

        http_request.push(line);
    }

    // println!("{:#?}", http_request);

    let request_line = &http_request[0];

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "static-html/hello.html"),
        "GET /sleep HTTP/1.1" => {
            sleep(Duration::from_secs(5)).await;
            ("HTTP/1.1 200 OK", "static-html/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "static-html/404.html"),
    };

    let contents = read_to_string(filename).await.unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).await.unwrap();
}
