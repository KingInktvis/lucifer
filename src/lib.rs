
mod http;
mod router;
mod handler;
mod middleware;

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
    fn new() -> Server {
        Server {
            manager: Manager::new()
        }
    }

    fn set_thread_count(&mut self, count: u32) {
        self.manager.set_thread_count(count);
    }

    fn listen(&mut self, address: &str, routes: RouteHandler, middleware: MiddlewareStore) {
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

