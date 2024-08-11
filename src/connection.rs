use chrono;
use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};
use crate::responses::*;


// ==================== Handlers ====================

pub fn handle_connection(stream: TcpStream) {
    // ========== Getting http request body ==========

    let (request_body, stream) = 
    match get_request(stream) {
        None => return,
        Some(value) => value
    };
    let request_line: String = request_body[0].clone();

    // ========== Parsing request ==========

    // Getting path and Routing the request
    match request_line.split(' ').collect::<Vec<&str>>().as_slice() {
        [method, path, "HTTP/1.1"] => {
            match *method {
                "GET" => handle_get(stream, path),
                "POST" => handle_post(stream, request_body),
                // Bad request
                &_ => send_bad_request(stream)
            }
        }, 
        // Bad request
        &[] | &[_] | &[_, _] | &[&_, _, _] | &[_, _, _, _, ..] => send_bad_request(stream)
    };
}

fn handle_get(stream: TcpStream, path: &str) {
    // If the path is the root, send the index.html
    if path == "/" {
        render_template(stream, "index.html");
    }

    // If the path is to the endpoint "file" and the password is correct, send the requested file
    else if path.starts_with("/file/") {
        // Getting the subpath
        let subpath: Vec<&str> = (&path[6..path.len()]).split('?').collect();
        if subpath.len() < 2 { render_404(stream); return; }

        let filename: &str = subpath[0];
        let args: Vec<(&str, &str)> = match parse_args(subpath[1]) {
            Some(value) => value,
            None => { render_404(stream); return; }
        };

        // Verifying if the password was passed as a parameter
        let password = match get_arg(args, "p") {
            Some(value) => value,
            None => { render_404(stream); return; }
        };
        if password != "1234" {  // IMPORTANT: Change the password and don't include it in any commit 
            render_404(stream); return;
        }

        send_file(stream, filename);
    }
    // If is in the root but it is a file, render it
    else if path.contains('.') {
        let filename: &str = &path[1..path.len()];
        render_template(stream, filename);
    }
    // Else, render 404 page
    else {
        render_404(stream);
    }
}

fn handle_post(mut stream: TcpStream, request_body: Vec<String>) {
    println!("{:#?}", request_body);
    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
}


// ==================== Methods ====================

fn get_request(mut stream: TcpStream) -> Option<(Vec<String>, TcpStream)> {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);

    let request_body: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap_or("".to_string()))
        .take_while(|line| !line.is_empty())
        .collect();
    
    if request_body.len() == 0 { return None; }  // No body request error

    println!("Request {:?}: {request_body:#?}\r\n", chrono::offset::Local::now());
    
    Some((request_body, stream))
}

fn parse_args(args: &str) -> Option<Vec<(&str, &str)>> {
    let list: Vec<&str> = args.split('&').collect();
    let length: usize = list.len();

    if length == 0 { return None; }

    let mut keys: Vec<&str> = Vec::new();
    let mut new: Vec<(&str, &str)> = Vec::new();

    for item in list {
        let item: Vec<&str> = item.split('=').collect();
        if item.len() != 2 {
            continue;
        }
        let key: &str = item[0];
        let value: &str = item[1];

        if !keys.contains(&key) && !key.is_empty() && !value.is_empty() {
            keys.push(key);
            new.push((key, value));
        }
    }
    Some(new)
}

fn get_arg(list: Vec<(&str, &str)>, name: &str) -> Option<String> {
    for (key, value) in list {
        if key == name {
            return Some(value.to_string());
        }
    }
    None
}
