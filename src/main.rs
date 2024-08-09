use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
    //
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    //
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "static/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "static/404.html")
    };

    //
    let contents: String = fs::read_to_string(filename).unwrap();
    let length: usize = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    //
    stream.write_all(response.as_bytes()).unwrap();

    println!("Response: {response}");
}

fn main() {
    let listener: TcpListener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        handle_connection(stream);
    }
}
