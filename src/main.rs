use std::net::TcpListener;
use http::*;

mod http;
mod router;
mod handler;

fn main() {
    let mut server = Server::new();
    let mut routes = RouteHandler::new();
    routes.add_route(Method::GET, "/", root);
    server.listen("127.0.0.1:8000", routes);
}

fn root(_req: Request) -> Response {
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


use router::*;
use handler::Manager;
use std::sync::Arc;

#[allow(dead_code)]
pub struct Server {
    routes: RouteHandler,
    manager: Manager
}

#[allow(dead_code)]
impl Server {
    fn new() -> Server {
        Server {
            routes: RouteHandler::new(),
            manager: Manager::new()
        }
    }

    pub fn add_route(&mut self, method: Method, route: &str, function: fn (Request) -> Response) {
        self.routes.add_route(method, route, function);
    }

    fn get_route(&self, method: Method, route: &str) -> Option<fn (Request) -> Response> {
        self.routes.get_route(method, route)
    }

    fn listen(&mut self, address: &str, routes: RouteHandler) {
        let listener = TcpListener::bind(address).unwrap();
        self.manager.boot(routes);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.manager.pass_stream(stream),
                Err(_) => print!("Error while unwrapping stream")
            }
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_routes() {
        let mut server = Server::new();
        server.add_route(Method::GET, "/some", test);
        match server.get_route(Method::GET, "/some") {
            Some(_) => {},
            None => panic!("Server routing error")
        }
    }

    fn test (_req: Request) -> Response {
        Response::new()
    }
}