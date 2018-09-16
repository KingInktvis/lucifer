use router::*;
use http::{Request, Response};
use std::net:: TcpStream;
use std::io::Write;
use std::io::prelude::*;
use std::sync::Arc;
use std::time::Duration;
use middleware::*;

#[allow(dead_code)]
pub fn handle_stream(mut stream: TcpStream, router: &Arc<RouteHandler>, middleware: &Arc<MiddlewareStore>) {
    stream.set_read_timeout(Some(Duration::new(1, 0)));
    loop {
        let mut buffer = read_buffer(&mut stream);
        match buffer {
            Some(mut buffer) => {
                let req = Request::new(&mut buffer);
                let res;
                if let Some(val) = req {
                    res = middleware_route_call(val, router, middleware);
                }else {
                    let mut tmp = Response::new();
                    tmp.set_status(404);
                    res = tmp;
                }
                stream.write(&res.to_bytes()[..]).unwrap();
                stream.flush().unwrap();
            },
            None => {
                break;
            }
        }
    }
}

fn middleware_route_call(req: Request, router: &Arc<RouteHandler>, middleware: &Arc<MiddlewareStore>) -> Response {
    let (handle, args) = router.get_route(req.get_method(), req.get_route());
    let func = match handle {
        Some(f) => f,
        None => route404
    };
    let mut mw = middleware.get_handle(&func);
    mw.next(req, args)
}

fn route404(_req: Request, _args: Args) -> Response {
    let mut res = Response::new();
    res.set_status(404);
    res
}

fn extend_vec(vector: &mut Vec<u8>) {
    for _ in 0..512 {
        vector.push(0);
    }
}

fn read_buffer(stream: &mut TcpStream) -> Option<Vec<u8>> {
    let size = 1024;
    let mut buffer = vec![0; size];
    for i in 0..32 {
        let start = i * size;
        let end = (i + 1) * size;
        let len_res = stream.read(&mut buffer[start..end]);
        match len_res {
            Ok(len) => {
                if len == size {
                    extend_vec(&mut buffer);
                }else {
                    if len == 0 && i == 0 {
                        return None;
                    }
                    break;
                }
            },
            Err(_) => {
                return None;
            }
        }
    }
    Some(buffer)
}