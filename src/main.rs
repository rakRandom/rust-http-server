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

    if http_request.len() == 0 {
        return;
    }

    let request_line: String = http_request[0].clone();

    println!("Request {:?}: {http_request:#?}\r\n", chrono::offset::Local::now());

    //
    let (status_line, filename) = 
    match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "static/index.html"),
        "GET /file HTTP/1.1" => ("HTTP/1.1 200 OK", "static/file.txt"),
        &_ => ("HTTP/1.1 404 NOT FOUND", "static/404.html"),
    };

    if filename == "static/file.txt" {
        send_file(stream, status_line, filename);
        return;
    }

    //
    let contents: String = fs::read_to_string(filename).unwrap();
    let length: usize = contents.len();

    let response: String = 
    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    //
    stream.write_all(response.as_bytes()).unwrap();

    println!("Response: {response}\r\n\r\n");
}

fn send_file(mut stream: TcpStream, status_line: &str, filename: &str) {
    let buf_content: Vec<u8> = fs::read(filename).unwrap();
    let length: usize = buf_content.len();
    let filename: &str = filename.split('/').collect::<Vec<_>>()[1];

    let response = format!("{status_line}\r\n\
    Content-Disposition: attachment; filename=\"{filename}\"\r\n\
    Content-Type: text/plain\r\n\
    Content-Length: {length}\r\n\r\n");

    stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(&buf_content).unwrap();
    stream.flush().unwrap();
}
