//! # Lucifer
//!
//! Lucifer is a library for creating web servers.
//! It contains its own http implementation routing and middleware.
//!
pub mod http;
pub mod router;
pub mod middleware;
mod handler;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{mpsc, Arc};
//use router::*;
//use middleware::*;

struct Worker {
    thread: thread::JoinHandle<()>,
    sender: mpsc::Sender<Orders>,
    receiver: mpsc::Receiver<Status>,
    available: bool
}

#[allow(dead_code)]
pub struct Server {
    thread_min: u32,
    thread_max: u32,
    threads: Vec<Worker>
}

enum Orders {
    Request(TcpStream),
    Quit
}

enum Status {
    Ready,
    Busy
}

#[allow(dead_code)]
impl Server {
    pub fn new() -> Server {
        Server {
            thread_min: 1,
            thread_max: 8,
            threads: Vec::new()
        }
    }

    fn boot_threads(&mut self) {
        for _ in 0..self.thread_min {
            let worker = Server::create_worker();
            self.threads.push(worker);
        }
    }

    pub fn listen(&mut self, address: &str) {
        self.boot_threads();
        let listener = TcpListener::bind(address).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.add_stream(stream),
                Err(_) => print!("Error while unwrapping stream")
            }
        }
    }

    fn add_stream(&mut self, stream: TcpStream) {
        for worker in self.threads.iter_mut() {
            let resp = worker.receiver.try_recv();
            match resp {
                Ok(resp) => {
                    use Status::*;
                    match resp {
                        Ready => worker.available = true,
                        Busy => worker.available = false
                    }
                },
                Err(mpsc::TryRecvError::Empty) => {},
                Err(mpsc::TryRecvError::Disconnected) => {
                    *worker = Server::create_worker();
                }
            }
            if worker.available {
                worker.sender.send(Orders::Request(stream));
                worker.available = false;
                return;
            }
        }
        let mut worker = Server::create_worker();
        worker.sender.send(Orders::Request(stream));
        self.threads.push(worker);
    }

    fn create_worker() -> Worker {
        let (tx_commands, rx_commands) = mpsc::channel();
        let (tx_status, rx_status) = mpsc::channel();
        let handle = thread::spawn(move || {
            for mes in rx_commands {
                match mes {
                    Orders::Request(stream) => {
                        handler::handle_stream(stream);
                    },
                    Orders::Quit => {
                        break;
                    }
                }
            }
        });
        Worker {
            available: true,
            thread: handle,
            sender: tx_commands,
            receiver: rx_status
        }
    }
}