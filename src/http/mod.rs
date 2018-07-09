use std::collections::HashMap;

pub mod request;
pub mod response;

#[allow(dead_code)]
#[derive(Clone)]
pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    TRACE,
    OPTIONS,
    CONNECT,
    PATCH
}

#[allow(dead_code)]
pub struct Request {
    method: Method,
    target: String,
    options: HashMap<String, String>,
    body: Vec<u8>
}

pub struct Response {
    status: u32,
    fields: Vec<String>,
    body: Vec<u8>
}