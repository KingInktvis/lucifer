use std::thread;
use std::sync::mpsc;
use router::*;
use http::{Request, Response};
use std::net:: TcpStream;
use std::io::Write;
use std::io::prelude::*;
use std::sync::Arc;
use middleware::*;

pub struct Conductor {
}

#[allow(dead_code)]
impl Conductor {
    pub fn new() -> Conductor {
        Conductor {
            
        }
    }

    fn middleware_route_call(req: Request, router: &Arc<RouteHandler>, middleware: &Arc<MiddlewareStore>) -> Response {
        let (handle, args) = router.get_route(req.get_method(), req.get_route());
        let func = match handle {
            Some(f) => f,
            None => Manager::route404
        };
        let mut mw = middleware.get_handle(&func);
        mw.next(req, args)
    }

    fn route404(_req: Request, _args: Args) -> Response {
        let mut res = Response::new();
        res.set_status(404);
        res
    }
}