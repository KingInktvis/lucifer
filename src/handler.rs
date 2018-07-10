use std::thread;
use std::sync::mpsc;
use router::*;
use http::{Request, Response};
use std::net:: TcpStream;
use std::io::Write;
use std::io::prelude::*;
use std::sync::Arc;

pub struct Manager {
    amount: u32,
    workers: Vec<thread::JoinHandle<()>>,
    tx: Vec<mpsc::Sender<TcpStream>>,
    share: Arc<RouteHandler>
}

#[allow(dead_code)]
impl Manager {
    pub fn new() -> Manager {
        Manager {
            amount: 1,
            workers: Vec::new(),
            tx: Vec::new(),
            share: Arc::new(RouteHandler::new())
        }
    }

    pub fn set_thread_count(&mut self, amount: u32) {
        self.amount = amount;
    }

    pub fn boot(&mut self, router: RouteHandler) {
        self.share = Arc::new(router);
        for _i in  1..self.amount {
            let (tx, rx) = mpsc::channel();
            let route_access = self.share.clone();
            let handle = thread::spawn(move || {
                for mess in rx {
                    Manager::handle_stream(mess, &route_access);
                }
            });
            self.workers.push(handle);
            self.tx.push(tx);
        }
    }

    pub fn pass_stream(&mut self, mut stream: TcpStream) {

    }

    fn handle_stream(mut stream: TcpStream, router: &Arc<RouteHandler>) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        let req = Request::new(&mut buffer);
        let res;
        if let Some(val) = req {
            let handle = router.get_route(val.get_method(),
                                        val.get_route());

            res = match handle {
                Some(func) => func(val),
                None => {
                    let mut tmp = Response::new();
                    tmp.set_status(404);
                    tmp
                }
            }
        }else {
            let mut tmp = Response::new();
            tmp.set_status(404);
            res = tmp;
        }

        stream.write(&res.to_bytes()[..]).unwrap();
        stream.flush().unwrap();
    }

    fn shutdown(&mut self) {
        self.tx.clear();
        for handle in self.workers.pop() {
            handle.join();
        }
    }
}
