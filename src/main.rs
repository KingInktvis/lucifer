use std::net::{TcpListener, TcpStream};
use std::io::Write;

mod http;
mod router;
fn main() {
    let mut server = Server::new();
    server.listen("127.0.0.1:8000");
}


use router::Paths;
struct Server {
    routes: Paths
}

impl Server {
    fn new() -> Server {
        Server {
            routes: Paths::new_root()
        }
    }

    fn add_route(&mut self, route: &str, function: String) {
        self.routes.new_route(route, function);
    }

    fn listen(&self, address: &str) {
        let listener = TcpListener::bind(address).unwrap();
        let mut count = 0;
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_stream(stream);
            count += 1;
            if count > 0 {
                break;
            }
        }
    }

    fn handle_stream(&self, mut stream: TcpStream) {
        let req = http::request::Values::new(&mut stream);

        let mut res = http::response::Values::new();
        if let Some(val) = req {
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
            res.add_header(String::from("Content-Type: text/html"));
        }else {
            print!("FUCK\n");
        }

        // let ren = res.render();
        stream.write(&res.to_bytes()[..]).unwrap();
        stream.flush().unwrap();
    }
}