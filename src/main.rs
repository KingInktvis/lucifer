use std::net::{TcpListener, TcpStream};
use std::io::Write;

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

fn handle_stream(mut stream: TcpStream) {
    let req = http::request::Fields::new(&mut stream);
    let mut res = http::response::Header::new();
    let ren = res.render();
    stream.write(&ren.as_bytes()).unwrap();
    stream.flush().unwrap();
}