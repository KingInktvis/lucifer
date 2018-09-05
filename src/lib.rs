//! # Lucifer
//!
//! Lucifer is a library for creating web servers.
//! It contains its own http implementation routing and middleware.
//!
pub mod http;
pub mod router;
pub mod middleware;

use std::net::{TcpListener, TcpStream};
use std::thread;
//use router::*;
//use middleware::*;
//use std::io;

#[allow(dead_code)]
pub struct Server {
    thread_count: u32,
    connections: Vec<Option<TcpStream>>,
    threads: Vec<thread::JoinHandle<()>>
}

#[allow(dead_code)]
impl Server {
    pub fn new() -> Server {
        Server {
            thread_count: 2,
            connections: Vec::new(),
            threads: Vec::new()
        }
    }

    fn boot_threads(&mut self) {
        for i in 0..self.thread_count {

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
            self.check_connections();
        }
    }

    fn add_stream(&mut self, stream: TcpStream) {
        for conn in self.connections.iter_mut() {
            match *conn {
                Some(_) => {},
                None => {
                    *conn = Some(stream);
                    return
                }
            }
        }
        self.connections.push(Some(stream));
    }

    fn check_connections(&mut self) {
        for option in self.connections.iter_mut() {
            match option {
                Some(conn) =>{
                    let mut buf = [0; 8];
                    let len = conn.peek(&mut buf);
                    match len {
                        Ok(l) => {
//                            print!("{}\n", l);
//                            print!("{}", String::from_utf8_lossy(&buf[0..l]));
                        },
                        Err(_) => {}
                    }
                },
                None => {}
            }
        }
    }
}