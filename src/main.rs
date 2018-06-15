use std::net::{TcpListener, TcpStream};

mod request;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8002").unwrap();
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
    // let mut buffer = [0; 512];

    // stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    // let req = request::Request::new();
    let req = request::Fields::new(stream);
}