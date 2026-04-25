use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::{fs, thread};

pub fn handle_std_connection(mut stream: TcpStream) -> std::io::Result<()>{
    let buf_reader = BufReader::new(&stream);
    let mut http_request = Vec::new();
    for line in buf_reader.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        http_request.push(line);
    }

    let request_line = &http_request[0];

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "static-html/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "static-html/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "static-html/404.html"),
    };

    let contents = fs::read_to_string(filename)?;
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes())?;

    Ok(())
}
