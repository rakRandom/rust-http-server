mod thread_pool;
mod connection;
mod responses;
mod macros;
use crate::{ 
    thread_pool::ThreadPool, 
    connection::handle_connection
};

use std::net::{
    TcpListener, TcpStream
};

// ==================== Main ====================

fn main() {
    let listener: TcpListener = match TcpListener::bind("0.0.0.0:7878") {
        Ok(listener) => listener,
        Err(_) => {
            println!("Failed bind to 0.0.0.0:7878"); 
            return;
        }
    };
    let pool: ThreadPool = ThreadPool::new(4);

    println!("Starting the server");
    for stream in listener.incoming() {
        let stream: TcpStream = match stream {
            Ok(stream) => stream,
            Err(_) => {continue;}
        };

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    
    println!("Shutting down");
}
