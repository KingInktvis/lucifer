use std::net::TcpStream;
use std::collections::HashMap;
use std::io::prelude::*;
use super::*;

   
#[allow(dead_code)]
pub struct Fields {
    method: Method,
    target: String,
    options: HashMap<String, String>
}

impl Fields {

    pub fn new(mut stream: TcpStream) -> Fields {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        
        let mut start = 0;
        let mut last_end = false;
        let mut iter = buffer.iter().enumerate();
        let mut map = HashMap::new();
        let mut last_index = 0;
        let mut first_line = true;
        while let Some((i, item)) = iter.next() {
            //Check for CRLF
            if *item == 0xD {
                if let Some((j, item2)) = iter.next() {
                    if *item2 == 0xA {
                        if last_end {
                            last_index = j;
                            break;
                        }
                        last_end = true;

                        let s = &buffer[start..i];
                        let line = String::from(String::from_utf8_lossy(s));
                        if first_line {
                            Fields::first_line(line);
                            first_line = false;
                        } else {
                            Fields::add_line(line, &mut map);
                        }
                        start = j + 1;
                    }
                }
            }else {
                last_end = false;
            }
        }
        Fields {
            method: Method::GET,
            target: String::from("/"),
            options: map
        }
    }

    fn method(&self) -> &Method {
        &self.method
    }

    fn first_line(line: String) -> (Option<Method>, Option<String>){
        let space = line.find(' ');
        let mut method: Option<Method> = None;
        let mut target: Option<String> = None;
        if let Some(mut start) = space {
            method = Fields::match_method(&line[..start]);
            start = start + 1;
            let sub = &line[start..];
            
            let space2 = sub.find(' ');

            if let Some(end) = space2 {
                target = Some(String::from(&sub[..end])); 
            }
        }
        (method, target)
    }

    fn match_method(verb: &str) -> Option<Method> {
        match verb {
            "GET" => Some(Method::GET),
            "POST" => Some(Method::POST),
            "PUT" => Some(Method::PUT),
            "PATCH" => Some(Method::PATCH),
            "DELETE" => Some(Method::DELETE),
            "HEAD" => Some(Method::HEAD),
            "TRACE" => Some(Method::TRACE),
            "OPTIONS" => Some(Method::OPTIONS),
            "CONNECT" => Some(Method::CONNECT),
            _ => None
        }
    }

    fn add_line(line: String, map: &mut HashMap<String, String>)  {
        let colon = line.find(':');
        if let Some(loc) = colon {
            let key = String::from(&line[..loc]);
            let value = String::from(&line[loc + 1..]);
            map.insert(key, value);
        }
    }
}
