use std::thread;
use std::sync::mpsc;
use router::*;
use http::{Request, Response};
use std::net:: TcpStream;
use std::io::Write;
use std::io::prelude::*;
use std::sync::Arc;
use middleware::*;
use std::collections::HashMap;

pub struct Manager {
    amount: usize,
    workers: Vec<thread::JoinHandle<()>>,
    tx: Vec<mpsc::Sender<TcpStream>>,
    share: Arc<RouteHandler>,
    middleware: Arc<MiddlewareStore>,
    next_worker: usize
}

#[allow(dead_code)]
impl Manager {
    pub fn new() -> Manager {
        Manager {
            amount: 1,
            workers: Vec::new(),
            tx: Vec::new(),
            share: Arc::new(RouteHandler::new()),
            middleware: Arc::new(MiddlewareStore::new()),
            next_worker: 0
        }
    }

    pub fn set_thread_count(&mut self, amount: u32) {
        if amount > 0 {
            self.amount = amount as usize;
        }
    }

    pub fn boot(&mut self, router: RouteHandler, middleware: MiddlewareStore) {
        self.share = Arc::new(router);
        self.middleware = Arc::new(middleware);

        for _ in  0..self.amount {
            let (tx, rx) = mpsc::channel();
            let route_access = self.share.clone();
            let middleware_store = self.middleware.clone();

            let handle = thread::spawn(move || {
                for mess in rx {
                    Manager::handle_stream(mess, &route_access, &middleware_store);
                }
            });
            self.workers.push(handle);
            self.tx.push(tx);
        }
    }

    pub fn pass_stream(&mut self, stream: TcpStream) {
        if self.next_worker >= self.amount {
            self.next_worker = 0;
        }
        match self.tx.get(self.next_worker) {
            Some(t) => t.send(stream).unwrap(),
            None => {}
        }
        self.next_worker += 1;
    }

    fn handle_stream(mut stream: TcpStream, router: &Arc<RouteHandler>, middleware: &Arc<MiddlewareStore>) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        let req = Request::new(&mut buffer);
        let res;
        if let Some(val) = req {
            res = Manager::middleware_route_call(val, router, middleware);
        }else {
            let mut tmp = Response::new();
            tmp.set_status(404);
            res = tmp;
        }

        stream.write(&res.to_bytes()[..]).unwrap();
        stream.flush().unwrap();
    }

    fn middleware_route_call(req: Request, router: &Arc<RouteHandler>, middleware: &Arc<MiddlewareStore>) -> Response {
        let func = |req: Request, args: Args| {
            let handle = router.get_route(req.get_method(),
                                          req.get_route());
            match handle {
                Some((func, args)) => func(req, args),
                None => {
                    let mut tmp = Response::new();
                    tmp.set_status(404);
                    tmp
                }
            }
        };
        let mut mw = middleware.get_handle(&func);
        mw.next(req, HashMap::new())
    }
}