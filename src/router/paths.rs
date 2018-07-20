use http::Request;
use http::Response;
use super::Paths;
use std::collections::HashMap;
use router::Args;

#[allow(dead_code)]
impl Paths {
    pub fn new_root() -> Paths {
        Paths {
            name: String::from(""),
            function: None,
            sub: Vec::new(),
            variables: Vec::new(),
            wildcard: false
        }
    }

    pub fn new_route(&mut self, route: &str, func: fn (Request, Args) -> Response) {
        let split = Paths::route_vec(route);
        self.add_route(&split[1..], func);
    }

    fn add_route(&mut self, route: &[&str], func: fn (Request, Args) -> Response) {
        if route.len() > 0 {
            if let Some(c) = route[0].chars().next() {
                if c == '*' {
                    self.function = Some(func);
                    self.wildcard = true;
                } else if c == ':' {
                    Paths::add_route_to_vec(route, &mut self.variables, func);
                }else {
                    Paths::add_route_to_vec(route, &mut self.sub, func);
                }
            }
        }else{
            self.function = Some(func);
        }
    }

    fn add_route_to_vec(route: &[&str], store: &mut Vec<Paths>, func: fn (Request, Args) -> Response) {
        //Search for existing route.
        for i in store.iter_mut() {
            if i.name == route[0] {
                i.add_route(&route[1..], func);
                return;
            }
        }
        //Create new route if there is not one already.
        store.push(Paths {
            name: String::from(route[0]),
            function: None,
            sub: Vec::new(),
            variables: Vec::new(),
            wildcard: false
        });
        let i = store.len() - 1;
        let item = &mut store[i];
        item.add_route(&route[1..], func);
    }

    /// Split a given route str at the '/' into a vector of the different parts
    fn route_vec(route: &str) -> Vec<&str> {
        let mut list = Vec::new();
        let mut start = 0;
        for (i, c) in route.as_bytes().iter().enumerate() {
            if *c == b'/' {
                let this = &route[start..i];
                start = i + 1;
                list.push(this);
            }
        }
        if route.len() > start {
            let rest = &route[start..];
            list.push(rest);
        }
        list
    }

    fn find_sub(&self, name: &str) -> Option<&Paths> {
        for item in self.sub.iter() {
            if *item.name == *name {
                return Some(item);
            }
        }
        None
    }

    pub fn router(&self, path: &str) -> Option<(fn (Request, Args) -> Response, Args)> {
        let args = HashMap::new();
        self.router_with_args(path, args)
    }

    pub fn router_with_args(&self, path: &str, mut args: Args) -> Option<(fn (Request, Args) -> Response, Args)> {
        let v = Paths::route_vec(path);
        match self.vec_router(&v[1..], &mut args){
            Some(f) => Some((f, args)),
            None => None
        }
    }

    fn vec_router(&self, route: &[&str], args: &mut Args) -> Option<fn (Request, Args) -> Response> {
        if route.len() == 0 {
            if let Some(f) = &self.function {
                return Some(*f);
            }else{
                return None;
            }
        }
        if self.wildcard {
            let mut tmp = String::new();
            for i in route.iter() {
                tmp.push('/');
                tmp.push_str(i);
            }
            args.insert(String::from("*"), tmp);
            return match self.function {
                Some(f) => Some(f),
                None => None
            }
        }
        match self.find_sub(route[0]) {
            Some(p) => return p.vec_router(&route[1..], args),
            None => return self.route_variable(route, args)
        }
    }

    fn route_variable(&self, path: &[&str], args: &mut Args) -> Option<fn (Request, Args) -> Response> {
        for i in self.variables.iter() {
            let res = i.vec_router(&path[1..], args);
            match res {
                Some(func) => {
                    args.insert(i.name.clone(), String::from(path[0]));
                    return Some(func)
                },
                None => {}
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use router::Args;

    #[test]
    fn route_vec() {
        let route = "/some/thing";
        let v = Paths::route_vec(route);
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], "");
        assert_eq!(v[1], "some");
        assert_eq!(v[2], "thing");


        let route = "/";
        let v = Paths::route_vec(route);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0], "");
    }

    #[test]
    fn routes() {
        let mut router = Paths::new_root();
        router.new_route("/other/object", empty);
        router.new_route("/some/thing", test1);
        router.new_route("/", empty);
        let test = router.find_sub("some");
        match test {
            Some(_route) => {},
            None => panic!("Route not found"),
        }
        let test = router.router("/some/thing");
        match test {
            Some((func, _args)) => {if func != test1 {panic!("wrong return value")}},
            None => panic!("Router fn does not return Some.")
        }
        let test = router.router("/some/thing");
        match test {
            Some((func, _args)) => {if func != test1 {panic!("wrong return value 2nd visit")}},
            None => panic!("Router fn does not return Some at 2nd visit.")
        }
        let test = router.router("/other/object");
        match test {
            Some((func, _args)) => {if func != empty {panic!("wrong return value")}},
            None => panic!("Router fn does not return Some.")
        }

        let test = router.router("/");
        match test {
            Some((func, args)) => {if func != empty {panic!("wrong return value")}},
            None => panic!("Router fn does not return Some at /.")
        }
    }

    #[test]
    fn variable_routes() {
        let mut router = Paths::new_root();
        router.new_route("/:variables/test", empty);
        router.new_route("/test/:variables/test2", empty);
        let test = router.router("/test/random/test2");
        match test {
            Some((_func, args)) => {
                match args.get(":variables") {
                    Some(s) => assert_eq!(s, "random"),
                    None => panic!("Variable not found")
                }
            },
            None => panic!("Router fn does not return Some with variables.")
        }
    }

    #[test]
    fn wildcard_routes() {
        let mut router = Paths::new_root();
        router.new_route("/test/*", empty);

        let test = router.router("/test/random/test2");
        match test {
            Some((_func, args)) => {
                match args.get("*") {
                    Some(s) => assert_eq!(s, "/random/test2"),
                    None => panic!("Wildcard not found")
                }
            },
            None => panic!("Router fn does not return Some with wildcard route.")
        }
    }

    fn empty(_req: Request, _args: Args) -> Response {
        let mut res = Response::new();
        res.send_message("empty");
        res
    }

    fn test1(_req: Request, _args: Args) -> Response {
        let mut res = Response::new();
        res.send_message("test1");
        res
    }
}