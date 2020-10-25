mod worker;

use std::net::TcpStream;
use std::sync::mpsc;
use worker::Worker;
use worker::job::Job;
use crate::server_core::request::Request;
use crate::server_core::response::Response;

pub struct WorkerPool {
    workers: Vec<Worker>,
    senders: Vec<mpsc::Sender<Job> >,
    active_worker: usize
}

impl WorkerPool {
    pub fn new(size: usize) -> WorkerPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let mut senders = Vec::with_capacity(size);
        for id in 0..size {
            let (sender, receiver) = mpsc::channel();
            workers.push(Worker::new(id, receiver));
            senders.push(sender);
        }
        WorkerPool { workers, senders, active_worker: 0 }
    }

    pub fn execute(&mut self, stream: TcpStream, callback: fn(Request, Response)) {
        let job = Job::new(callback, stream);
        match self.senders[self.active_worker].send(job) {
            Ok(_) => self.active_worker = ( self.active_worker + 1 ) % self.workers.len(),
            Err(error) => panic!("Failed to send the job to worker {} with error: {:?}", self.active_worker, error)
        }
    }
}
