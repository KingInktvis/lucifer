//! # Lucifer
//!
//! Lucifer is a library for creating web servers.
//! It contains its own http implementation routing and middleware.
//!
pub mod http;
pub mod router;
pub mod middleware;
mod handler;

use std::net::TcpListener;
use router::*;
use handler::Manager;
use middleware::*;

#[allow(dead_code)]
pub struct Server {
    manager: Manager
}

#[allow(dead_code)]
impl Server {
    pub fn new() -> Server {
        Server {
            manager: Manager::new()
        }
    }

    pub fn set_thread_count(&mut self, count: u32) {
        self.manager.set_thread_count(count);
    }

    pub fn listen(&mut self, address: &str, routes: RouteHandler, middleware: MiddlewareStore) {
        let listener = TcpListener::bind(address).unwrap();
        self.manager.boot(routes, middleware);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.manager.pass_stream(stream),
                Err(_) => print!("Error while unwrapping stream")
            }
        }
    }
}

