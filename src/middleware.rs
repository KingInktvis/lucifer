use router::*;
use http::*;

pub trait Middleware {
    fn call(&self, req: Request, args: Args, handle: &mut MiddlewareHandle) -> Response;
}

pub struct MiddlewareStore {
    middleware: Vec<Box<Middleware+Sync+Send>>
}

#[allow(dead_code)]
impl<'a, 'b> MiddlewareStore {
    pub fn new() -> MiddlewareStore {
        MiddlewareStore {
            middleware: Vec::new()
        }
    }

    pub fn add(&mut self, mw: Box<Middleware+Sync+Send>) {
        self.middleware.push(mw);
    }

    pub fn get_handle(&'a self, router: &'b Fn(Request, Args) -> Response)-> MiddlewareHandle<'a, 'b> {
        MiddlewareHandle::new(self, router)
    }
}

pub struct MiddlewareHandle<'a, 'b> {
    store: &'a MiddlewareStore,
    pointer: usize,
    router: &'b Fn(Request, Args) -> Response
}

impl<'a, 'b> MiddlewareHandle<'a, 'b> {
    pub fn new(store: &'a MiddlewareStore, router: &'b Fn(Request, Args) -> Response) -> MiddlewareHandle<'a, 'b> {
        MiddlewareHandle {
            store,
            pointer: 0,
            router
        }
    }

    pub fn next(&mut self, req: Request, args: Args) -> Response {
        if self.pointer < self.store.middleware.len() {
            let next = &self.store.middleware[self.pointer];
            self.pointer += 1;
            next.call(req, args, self)
        } else {
            let tmp = (self.router)(req,args);
            tmp
        }
    }
}