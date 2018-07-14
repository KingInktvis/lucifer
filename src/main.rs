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

fn root(_req: Request, args: Args) -> Response {
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

