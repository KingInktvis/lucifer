use http::Request;
use http::Response;

mod paths;
mod route_handler;

pub struct Paths {
    name: String,
    function: Option<fn (Request) -> Response>,
    sub: Vec<Paths>,
    wildcard: Vec<Paths>
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