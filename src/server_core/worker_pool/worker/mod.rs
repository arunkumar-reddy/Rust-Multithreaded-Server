pub mod job;

use std::thread;
use std::sync::mpsc::Receiver;
use crate::server_core::request::Request;
use crate::server_core::response::*;
use job::Job;

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Receiver<Job>) -> Worker {
        let thread = thread::spawn(move || loop {
            for mut job in &receiver {
                println!("Connection Established. Worker {} got a job; executing.", id);
                let parsed_request = Request::parse_request(&mut job.stream);
                match parsed_request {
                    Some(request) => {
                        let headers = ResponseHeaders::new(ContentType::TEXT);
                        let response = Response::new(String::from("200"), headers, None, job.stream);
                        let callback = job.callback;
                        callback(request, response);
                    },
                    None => panic!("Failed to parse stream to request in thread: {}", id)
                }
            }
        });
        Worker { id, thread }
    }
} 