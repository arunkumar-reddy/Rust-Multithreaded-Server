pub mod request;
pub mod response;
mod worker_pool;

use std::net::TcpListener;
use request::Request;
use response::*;
use worker_pool::WorkerPool;

pub fn start_server(host: String, port: String, handle_connection: fn(Request, Response))  {
    let address = format!("{}:{}", host, port);
    let listener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(error) => panic!("Could not start server on host: {} and port: {}: {:?}", host, port, error),
    };
    let mut worker_pool = WorkerPool::new(4);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                worker_pool.execute(stream, handle_connection);
            },
            Err(error) => panic!("Could not get client: {:?}", error)
        }
    }
}