use std::net::{TcpListener, TcpStream};
use std::io::Write;

mod http;
mod router;
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