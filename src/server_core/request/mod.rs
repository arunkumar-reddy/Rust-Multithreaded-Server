use std::io::prelude::*;
use std::net::TcpStream;
use bufstream::BufStream;

pub struct Request {
    pub http_version: String,
    pub method: String,
    pub path: String,
}

impl Request {
    pub fn new(http_version: String, method: String, path: String) -> Request {
        Request { http_version, method: method, path }
    }

    pub fn parse_request(stream: &mut TcpStream) -> Option<Request> {
        let mut buf_stream = BufStream::new(stream);
        let mut request = String::new();
        match buf_stream.read_line(&mut request) {
            Ok(_) => {
                let mut parts = request.split(" ");
                let method = match parts.next() {
                    Some(method) => method.trim().to_string(),
                    None => return None,
                };
                let path = match parts.next() {
                    Some(path) => path.trim().to_string(),
                    None => return None,
                };
                let http_version = match parts.next() {
                    Some(version) => version.trim().to_string(),
                    None => return None,
                };
                loop {
                    match parts.next() {
                        Some(part) => println!("Printing part: {}", part),
                        None => break
                    }
                }
                println!("Parsed request method: {} path: {} http_version: {}", method, path, http_version);
                Some(Request::new(http_version, method, path))
            },
            Err(error) => panic!("Could not read from stream: {:?}", error)
        }
    }
}