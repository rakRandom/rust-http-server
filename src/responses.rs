use crate::macros::*;
use std::{
    fs,
    io::prelude::*,
    net::TcpStream,
};

// ==================== Structs ====================

struct StatusCode;
impl StatusCode {
    fn ok() -> &'static str {
        "HTTP/1.1 200 Ok"
    }

    fn not_found() -> &'static str {
        "HTTP/1.1 404 Not Found"
    }

    fn bad_request() -> &'static str {
        "HTTP/1.1 400 Bad Request"
    }
}

// ==================== Methods ====================

pub fn send_file(mut stream: TcpStream, filename: &str) {
    match fs::read(path_to!(filename)) {
        Ok(buf_content) => {
            let length: usize = buf_content.len();

            let response: String = format!("{}\r\n\
                Content-Disposition: attachment; filename=\"{filename}\"\r\n\
                Content-Type: text/plain\r\n\
                Content-Length: {length}\r\n\r\n", 
                StatusCode::ok());
            
            // Write
            match stream.write_all(response.as_bytes()) {
                Ok(_) => println!("Response: {response}\r\n\r\n"),
                Err(_) => { 
                    println!("Failed response\r\n\r\n");
                    return;
                },
            };
            let _ = stream.write_all(&buf_content);
            let _ = stream.flush();
        }, 
        Err(_) => {
            render_404(stream);
        }
    };
}

pub fn send_bad_request(mut stream: TcpStream) {
    let response: String = format!("{}\r\n", StatusCode::bad_request()); 
    
    match stream.write_all(response.as_bytes()) {
        Ok(_) => println!("Response: {response}\r\n\r\n"), 
        Err(_) => { 
            println!("Failed response\r\n\r\n"); 
            return; 
        },
    };
}

pub fn render_template(mut stream: TcpStream, filename: &str) {
    // Parsing response
    match fs::read_to_string(path_to!(filename)) {
        Ok(content) => {
            let length: usize = content.len();
            let response: String = format!(
                "{}\r\nContent-Length: {length}\r\n\r\n{content}",
                StatusCode::ok()
            );
            // Sending response
            match stream.write_all(response.as_bytes()) {
                Ok(_) => println!("Response: {response}\r\n\r\n"),
                Err(_) => println!("Failed response\r\n\r\n"),
            };
        },
        Err(_) => {
            render_404(stream);
        }
    };
}

pub fn render_404(mut stream: TcpStream) {
    let length: usize;
    let mut response: String = format!("{}\r\n", StatusCode::not_found());

    match fs::read_to_string(path_to!("404.html")) {
        Ok(content) => {
            length = content.len();
            response = format!("{response}Content-Length: {length}\r\n\r\n{content}");
        },
        Err(_) => {
            response = format!("{response}\r\n404");
        }
    };

    match stream.write_all(response.as_bytes()) {
        Ok(_) => println!("Response: {response}\r\n\r\n"),
        Err(_) => println!("Failed response\r\n\r\n"),
    };
}
