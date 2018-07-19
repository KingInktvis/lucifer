use std::net::TcpListener;
use http::*;

mod http;
mod router;
mod handler;
mod middleware;

fn main() {
    let mut server = Server::new();
    let mut routes = RouteHandler::new();
    let mut middleware = middleware::MiddlewareStore::new();
    middleware.add(Box::new(MW{}));
    routes.add_route(Method::GET, "/", root);
    server.listen("127.0.0.1:8000", routes, middleware);
}

fn root(_req: Request, _args: Args) -> Response {
    let mut res = Response::new();
    res.send_message(" <!DOCTYPE html>
<html>
<head>
<title>Page Title</title>
</head>
<body>

<h1>This is a Heading</h1>
<p>This is a paragraph.</p>

</body>
</html> ");
    res
}

struct MW {}
impl middleware::Middleware for MW {
    fn call(&self, req: Request, args: Args, handle: &mut middleware::MiddlewareHandle) -> Response {
        handle.next(req, args)
    }
}


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

