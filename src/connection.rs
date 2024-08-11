use chrono;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::TcpStream,
};


// ==================== Macros ====================

#[macro_export]
macro_rules! path_to {
    ($x:expr) => { format!("static/{}", $x) };
}


// ==================== Handler ====================

pub fn handle_connection(stream: TcpStream) {
    // Getting http request body
    let (http_request, stream) = 
    match get_request_body(stream) {
        None => return,
        Some(value) => {value}
    };
    let request_line: String = http_request[0].clone();

    // Parsing request
    let temp: Vec<&str> = request_line.split(' ').collect::<Vec<_>>();
    if temp.len() < 1 { return; }  // No request line error

    let path: &str = temp[1];

    if path.contains('.') {
        send_file(stream, &path[1..path.len()]);
    }
    else {
        render_template(
            stream, 
            match request_line.as_str() {
            "GET / HTTP/1.1" => "index.html",
            &_ => "404.html",
        });
    }
}


// ==================== Methods ====================

fn get_request_body(mut stream: TcpStream) -> Option<(Vec<String>, TcpStream)> {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);

    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if http_request.len() == 0 { return None; }  // No body request error

    println!("Request {:?}: {http_request:#?}\r\n", chrono::offset::Local::now());

    Some((http_request, stream))
}

fn send_file(mut stream: TcpStream, filename: &str) {
    let response: String;

    match fs::read(path_to!(filename)) {
        Ok(buf_content) => {
            let length: usize = buf_content.len();

            response = format!("HTTP/1.1 200 OK\r\n\
            Content-Disposition: attachment; filename=\"{filename}\"\r\n\
            Content-Type: text/plain\r\n\
            Content-Length: {length}\r\n\r\n");
            stream.write_all(response.as_bytes()).unwrap();
            stream.write_all(&buf_content).unwrap();
            stream.flush().unwrap();
        }, 
        Err(_) => {
            response = format!("HTTP/1.1 404 FILE NOT FOUND\r\n");
            stream.write_all(response.as_bytes()).unwrap();
        }
    };
}

fn render_template(mut stream: TcpStream, filename: &str) {
    // Parsing response
    let status_line = "HTTP/1.1 200 OK";
    let contents: String = fs::read_to_string(path_to!(filename)).unwrap();
    let length: usize = contents.len();

    let response: String = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // Sending response
    stream.write_all(response.as_bytes()).unwrap();
    println!("Response: {response}\r\n\r\n");
}
