use std::collections::HashMap;

pub mod request;
pub mod response;
pub mod content_type;

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
    pub method: Method,
    pub target: String,
    pub options: HashMap<String, String>,
    pub body: Vec<u8>
}

pub struct Response {
    pub status: u32,
    pub fields: Vec<String>,
    pub content_type: ContentType,
    pub body: Vec<u8>
}

pub enum ContentType {
    JSON,
    JS,
    CSS,
    HTML,
    ICO,
    PLAIN
}

