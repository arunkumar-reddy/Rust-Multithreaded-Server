use std::net::TcpStream;
use crate::server_core::request::Request;
use crate::server_core::response::Response;

pub struct Job {
    pub callback: fn(Request, Response),
    pub stream: TcpStream
}

impl Job {
    pub fn new(callback: fn(Request, Response), stream: TcpStream) -> Job {
        Job { callback , stream }
    }
}