use std::collections::HashMap;
use super::*;

   
#[allow(dead_code)]
pub struct Values {
    method: Method,
    target: String,
    options: HashMap<String, String>,
    body: Vec<u8>
}

impl Values {

    pub fn new(buffer: &[u8]) -> Option<Values> {
        let mut method = Method::GET;
        let mut target = String::new();
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
                            let tmp = Values::first_line(line);
                            if let Some(x) = tmp.0 {
                                method = x;
                            } else {
                                return None;
                            }
                            if let Some(x) = tmp.1 {
                                target = x;
                            } else {
                                return None;
                            }
                            first_line = false;
                        } else {
                            Values::add_line(line, &mut map);
                        }
                        start = j + 1;
                    }
                }
            }else {
                last_end = false;
            }
        }
        let body = Values::extract_body(&buffer[last_index+1..]);

        Some(Values {
            method: method,
            target: target,
            options: map,
            body: to_vec(body)
        })
    }

    fn extract_body(buffer: &[u8]) -> &[u8] {
        for (i, byte) in buffer.iter().enumerate().rev() {
            if *byte != 0x0 {
                return &buffer[..i+1];;
            }
        }
        &buffer[..0]
    }

    fn first_line(line: String) -> (Option<Method>, Option<String>){
        let space = line.find(' ');
        let mut method: Option<Method> = None;
        let mut target: Option<String> = None;
        if let Some(mut start) = space {
            method = match_method(&line[..start]);
            start = start + 1;
            let sub = &line[start..];
            
            let space2 = sub.find(' ');

            if let Some(end) = space2 {
                target = Some(String::from(&sub[..end])); 
            }
        }
        (method, target)
    }

    fn add_line(line: String, map: &mut HashMap<String, String>)  {
        let colon = line.find(':');
        if let Some(loc) = colon {
            let key = String::from(&line[..loc]);
            let value = String::from(&line[loc + 1..]);
            map.insert(key, value);
        }
    }

    pub fn body(&self) -> &Vec<u8> {
        &self.body
    }
}

fn to_vec(arr: &[u8]) -> Vec<u8> {
    let mut v = Vec::new();
    for i in arr.iter() {
        v.push(*i);
    }
    v
}

pub fn match_method(verb: &str) -> Option<Method> {
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