use std::thread;
use router::*;
use http::{Request, Response};
use std::net:: TcpStream;
use std::io::Write;
use std::io::prelude::*;
use std::sync::{Arc, Mutex, mpsc};
use middleware::*;

#[allow(dead_code)]
pub fn handle_stream(mut stream: TcpStream) {//, router: &Arc<RouteHandler>, middleware: &Arc<MiddlewareStore>


    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let req = Request::new(&mut buffer);
    let res;
    if let Some(val) = req {
        res = Response::new();
//            Manager::middleware_route_call(val, router, middleware);
    }else {
        let mut tmp = Response::new();
        tmp.set_status(404);
        res = tmp;
    }

    stream.write(&res.to_bytes()[..]).unwrap();
    stream.flush().unwrap();

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
