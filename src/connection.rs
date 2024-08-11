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
        let args: Vec<&str> = subpath[1].split('&').collect();

        // Verifying if the password was passed as a parameter
        // Todo: Improve later, maybe turn it into a generic function to parse args
        // Note: This isn't safe, the url can be /file/a.txt?p=0&p=1&p=2&p=... and break the system
        let mut exit: bool = true;
        for arg in args {
            if arg == "p=1234" {  // Change the password
                exit = false; 
                break;
            }
        }
        if exit {
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
