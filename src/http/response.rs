use super::Response;

#[allow(dead_code)]
impl Response {
    pub fn new() -> Response {
        Response {
            status: 200,
            fields: Vec::new(),
            body: Vec::new()
        }
    }

    pub fn send_message(&mut self, message: &str) {
        let mut bytes = Vec::new();
        for b in message.as_bytes().iter() {
            bytes.push(*b);
        }
        self.body = bytes;
    }

    pub fn set_status(&mut self, new: u32) -> &mut Response {
        self.status = new;
        self
    }

    pub fn add_header(&mut self, header: String) {
        self.fields.push(header);
    }

    pub fn render(&self) -> String {
        let mut header = String::new();
        //status line
        header.push_str("HTTP/1.1 ");
        header.push_str(&self.status.to_string());
        header.push(' ');
        header.push_str(&map_status_message(self.status));
        header.push_str("\r\n");

        for line in self.fields.iter() {
            header.push_str(&line);
            header.push_str("\r\n");
        }
        header.push_str("\r\n");

        header
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        push_str(&mut res, "HTTP/1.1 ");
        push_str(&mut res, &self.status.to_string());
        push_str(&mut res, " ");
        push_str(&mut res, &map_status_message(self.status));
        push_str(&mut res, "\r\n");

        for line in self.fields.iter() {
            push_str(&mut res, &line);
            push_str(&mut res, "\r\n");
        }

        for i in self.body.iter() {
            res.push(*i);
        }

        res
    }

    
}

fn push_str(vec: &mut Vec<u8>, string: &str) {
    for i in string.as_bytes().iter() {
        vec.push(*i);
    }
}

pub fn map_status_message(code: u32) -> String {
    let mess = match code {
        100 => "Continue",
        101 => "Switching Protocols",
        102 => "Processing",
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        203 => "Non-authoritative Information",
        204 => "No Content",
        205 => "Reset Content",
        206 => "Partial Content",
        207 => "Multi-Status",
        208 => "Already Reported",
        226 => "IM Used",
        300 => "Multiple Choices",
        301 => "Moved Permanently",
        302 => "Found",
        303 => "See Other",
        304 => "Not Modified",
        305 => "Use Proxy",
        307 => "Temporary Redirect",
        308 => "Permanent Redirect",
        400 => "Bad Request",
        401 => "Unauthorized",
        402 => "Payment Required",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        406 => "Not Acceptable",
        407 => "Proxy Authentication Required",
        408 => "Request Timeout",
        409 => "Conflict",
        410 => "Gone",
        411 => "Length Required",
        412 => "Precondition Failed",
        413 => "Payload Too Large",
        414 => "Request-URI Too Long",
        415 => "Unsupported Media Type",
        416 => "Requested Range Not Satisfiable",
        417 => "Expectation Failed",
        418 => "I'm a teapot",
        421 => "Misdirected Request",
        422 => "Unprocessable Entity",
        423 => "Locked",
        424 => "Failed Dependency",
        426 => "Upgrade Required",
        428 => "Precondition Required",
        429 => "Too Many Requests",
        431 => "Request Header Fields Too Large",
        444 => "Connection Closed Without Response",
        451 => "Unavailable For Legal Reasons",
        499 => "Client Closed Request",
        500 => "Internal Server Error",
        501 => "Not Implemented",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        505 => "HTTP Version Not Supported",
        506 => "Variant Also Negotiates",
        507 => "Insufficient Storage",
        508 => "Loop Detected",
        510 => "Not Extended",
        511 => "Network Authentication Required",
        599 => "Network Connect Timeout Error",
        _ => ""
    };
    String::from(mess)
}