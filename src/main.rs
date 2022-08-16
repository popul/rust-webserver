use std::{
    net::{TcpListener, TcpStream },
    io::{prelude::*, BufReader},
    fs
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let first_line = http_request.first().unwrap();

    let (status_line, file) = if first_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    }
    else {
        ("HTTP/1.1 404 OK", "404.html")
    };

    let contents = fs::read_to_string(file).unwrap();
    let length = contents.len();

    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, contents);

    stream.write_all(response.as_bytes()).unwrap();
}
