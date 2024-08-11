mod thread_pool;
mod connection;
use crate::{ 
    thread_pool::ThreadPool, 
    connection::handle_connection
};

use std::net::{
    TcpListener, TcpStream
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
