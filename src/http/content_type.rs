use super::ContentType;

impl ContentType {
    pub fn to_str(&self) -> &str {
        use self::ContentType::*;
        let string = match self {
            HTML => "text/html",
            JS => "text/javascript",
            JSON => "application/json",
            CSS => "text/css",
            ICO => "image/vnd.microsoft.icon",
            PLAIN => "text/plain"
        };
        string
    }

    pub fn to_header(&self) -> String {
        let mut header = String::from("Content-Type: ");
        header.push_str(self.to_str());
        header
    }
}