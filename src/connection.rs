use chrono;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::TcpStream,
};


// ==================== Structs ====================

struct StatusCode;

impl StatusCode {
    pub fn ok() -> &'static str {
        "HTTP/1.1 200 OK"
    }

    pub fn not_found() -> &'static str {
        "HTTP/1.1 404 NOT FOUND"
    }
}


// ==================== Macros ====================

#[macro_export]
macro_rules! path_to {
    ($x:expr) => { format!("static/{}", $x) };
}


// ==================== Handler ====================

pub fn handle_connection(stream: TcpStream) {
    // ========== Getting http request body ==========

    let (http_request, stream) = 
    match get_request_body(stream) {
        None => return,
        Some(value) => {value}
    };
    let request_line: String = http_request[0].clone();


    // ========== Parsing request ==========

    // Getting path
    let temp: Vec<&str> = request_line.split(' ').collect();
    if temp.len() < 3 { return; }  // No request line error

    let path: &str = temp[1];


    // ===== Routing the request =====

    // If the path is the root, send the index.html
    if path == "/" {
        render_template(stream, "index.html");
    }

    // If the path is to the endpoint "file" and the password is correct, send the requested file
    else if path.starts_with("/file") {
        //
        let subpath: Vec<&str> = (&path[6..path.len()]).split('?').collect();

        if subpath.len() < 2 {
            render_404(stream);
            return;
        }
        let filename: &str = subpath[0];
        let args: Vec<(&str, &str)> = match parse_args(subpath[1]) {
            Some(value) => value,
            None => {
                render_404(stream);
                return;
            }
        };

        // Verifying if the password was passed as a parameter
        let password = match get_arg(args, "p") {
            Some(value) => value,
            None => {
                render_404(stream);
                return;
            }
        };
        if password != "1234" {  // IMPORTANT: Change the password and don't include it in any commit 
            render_404(stream);
            return;
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


// ==================== Methods ====================

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
    match fs::read(path_to!(filename)) {
        Ok(buf_content) => {
            let length: usize = buf_content.len();

            let response: String = format!("{}\r\n\
                Content-Disposition: attachment; filename=\"{filename}\"\r\n\
                Content-Type: text/plain\r\n\
                Content-Length: {length}\r\n\r\n", 
                StatusCode::ok());
            
            stream.write_all(response.as_bytes()).unwrap();
            stream.write_all(&buf_content).unwrap();
            stream.flush().unwrap();
        }, 
        Err(_) => {
            render_404(stream);
        }
    };
}

fn render_template(mut stream: TcpStream, filename: &str) {
    // Parsing response
    let contents: String = fs::read_to_string(path_to!(filename)).unwrap();
    let length: usize = contents.len();

    let response: String = format!(
        "{}\r\nContent-Length: {length}\r\n\r\n{contents}",
        StatusCode::ok()
    );

    // Sending response
    stream.write_all(response.as_bytes()).unwrap();
    println!("Response: {response}\r\n\r\n");
}

fn render_404(mut stream: TcpStream) {
    let contents: String = fs::read_to_string(path_to!("404.html")).unwrap();
    let length: usize = contents.len();
    let response: String = format!("{}\r\nContent-Length: {length}\r\n\r\n{contents}", StatusCode::not_found());
    stream.write_all(response.as_bytes()).unwrap();
}
