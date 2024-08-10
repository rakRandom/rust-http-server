use cli_file_transfer::ThreadPool;
use chrono;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener: TcpListener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool: ThreadPool = ThreadPool::new(4);
    
    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    //
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line: String = http_request[0].clone();

    println!("Request {:?}: {http_request:#?}\r\n", chrono::offset::Local::now());

    //
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "static/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "static/404.html")
    };

    //
    let contents: String = fs::read_to_string(filename).unwrap();
    let length: usize = contents.len();

    let response: String = 
    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    //
    stream.write_all(response.as_bytes()).unwrap();

    println!("Response: {response}\r\n\r\n");
}
