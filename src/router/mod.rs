use http::Request;
use http::Response;
use std::collections::HashMap;

mod paths;
mod route_handler;

///An easy alias for the argument list to make function definitions easier to write.
pub type Args = HashMap<String, String>;

pub struct Paths {
    name: String,
    function: Option<fn (Request, Args) -> Response>,
    sub: Vec<Paths>,
    variables: Vec<Paths>,
    wildcard: bool
}

pub struct RouteHandler {
    get: Paths,
    head: Paths,
    post: Paths,
    put: Paths,
    delete: Paths,
    trace: Paths,
    options: Paths,
    connect: Paths,
    patch: Paths
}