use std::net::{TcpListener, TcpStream};

mod http;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let mut count = 0;
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_stream(stream);
        count += 1;
        if count > 0 {
            break;
        }
    }
}

fn handle_stream(stream: TcpStream) {
    let req = http::request::Fields::new(stream);
}