use http::Request;
use http::Response;
use super::Paths;
use std::collections::HashMap;
use router::Args;

#[allow(dead_code)]
impl Paths {
    /// Creates a root node.
    pub fn new_root() -> Paths {
        Paths {
            name: String::from(""),
            function: None,
            sub: Vec::new(),
            variables: Vec::new(),
            wildcard: false
        }
    }

    ///Add a function to the to the given path.
    pub fn new_route(&mut self, route: &str, func: fn (Request, Args) -> Response) {
        let split = Paths::route_vec(route);
        self.add_route(&split, func);
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

    ///Takes the URI and return a tuple (route Vec<&str>, query (name &str, value &str), fragment &str).
    fn split_uri(uri: &str) -> (Vec<&str>, Vec<(&str, &str)>, &str) {
        let (route, fragment) = Paths::route_fragment_str(uri);
        let (route, query) = Paths::route_query_str(route);
        let query = Paths::route_query_vec(query);
        let route = Paths::route_vec(route);
        (route, query, fragment)
    }

    /// Split a given route str at the '/' into a vector of the different parts
    fn route_vec(route: &str) -> Vec<&str> {
        let mut list = Vec::new();
        let mut start = 0;
        let mut first = true;
        for (i, c) in route.as_bytes().iter().enumerate() {
            if *c == b'/' {
                if first {
                    first = false;
                }else{
                    let this = &route[start..i];
                    list.push(this);
                }
                start = i + 1;
            }
        }
        if route.len() > start {
            let rest = &route[start..];
            list.push(rest);
        }
        list
    }

    fn route_query_str(route: &str) -> (&str, &str) {
        for (index, character) in route.as_bytes().iter().enumerate().rev() {
            if *character == b'?' {
                let query = &route[index+1..];
                let remaining_route = &route[..index];
                return (remaining_route, query);
            }
        }
        (route, "")
    }

    fn route_query_vec(query: &str) -> Vec<(&str, &str)> {
        let mut start = 0;
        let mut query_vec = Vec::new();
        for (index, character) in query.as_bytes().iter().enumerate() {
            if *character == b'&' {
                query_vec.push(&query[start..index]);
                start = index + 1;
            }
        }
        query_vec.push(&query[start..]);

        let mut tuple_vec = Vec::new();
        for arg in query_vec.iter() {
            for (index, character) in arg.as_bytes().iter().enumerate() {
                if *character == b'=' {
                    let name = &arg[..index];
                    let value = &arg[index+1..];
                    tuple_vec.push((name, value));
                }
            }
        }
        tuple_vec
    }

    fn route_fragment_str(route: &str) -> (&str, &str) {
        for (index, character) in route.as_bytes().iter().enumerate().rev() {
            if *character == b'#' {
                let fragment = &route[index+1..];
                let cut_route = &route[..index];
                return (cut_route, fragment);
            }
        }
        (route, "")
    }

    fn find_sub(&self, name: &str) -> Option<&Paths> {
        for item in self.sub.iter() {
            if *item.name == *name {
                return Some(item);
            }
        }
        None
    }

    ///Search and return a tuple with Some(function) if the route exists (else None) and returns the arguments present in the route.
    pub fn router(&self, path: &str) -> (Option<fn (Request, Args) -> Response>, Args) {
        let args = HashMap::new();
        self.router_with_args(path, args)
    }

    ///Search and return a tuple with Some(function) if the route exists (else None) and returns the arguments present in the route.
    /// Takes the route &str and preexisting argument list to use.
    pub fn router_with_args(&self, path: &str, mut args: Args) -> (Option<fn (Request, Args) -> Response>, Args) {
        let (route, query, fragment) = Paths::split_uri(path);
        Paths::add_query_to_args(query, &mut args);
        args.insert(String::from("#"), String::from(fragment));
        match self.vec_router(&route, &mut args){
            Some(f) => (Some(f), args),
            None => (None, args)
        }
    }

    fn add_query_to_args(query: Vec<(&str, &str)>, args: &mut Args) {
        for (name, value) in query.iter() {
            let mut new_name = String::from("?");
            new_name.push_str(name);
            let value = String::from(*value);
            args.insert(new_name, value);
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
        assert_eq!(v.len(), 2);
//        assert_eq!(v[0], "");
        assert_eq!(v[0], "some");
        assert_eq!(v[1], "thing");


        let route = "/";
        let v = Paths::route_vec(route);
        assert_eq!(v.len(), 0);
//        assert_eq!(v[0], "");
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

    #[test]
    fn fragment() {
        let route = "/some/thing#fragment";
        let (route, res) = Paths::route_fragment_str(route);
        assert_eq!(res, "fragment");
        assert_eq!(route, "/some/thing");
    }

    #[test]
    fn query() {
        let route = "/some?test=one";
        let (_, query) = Paths::route_query_str(route);
        assert_eq!(query, "test=one");
        let res = Paths::route_query_vec(query);
        assert_eq!(res, vec![("test", "one")]);

        let route = "/some?test=one&testing=two&three=something";
        let (_, query) = Paths::route_query_str(route);
        assert_eq!(query, "test=one&testing=two&three=something");
        let res = Paths::route_query_vec(query);
        assert_eq!(res, vec![("test", "one"), ("testing", "two"), ("three", "something")]);
    }

    #[test]
    fn add_query_to_args() {
        let query = vec![("test", "one"), ("testing", "two"), ("three", "something")];
        let mut args = HashMap::new();
        Paths::add_query_to_args(query, &mut args);
        match args.get("?testing") {
            Some(value) => assert_eq!(value, "two"),
            None => panic!("Query key not found.")
        }
    }

    #[test]
    fn split_uri() {
        let uri = "/some?name=test#frag";
        let (route, query, fragment) = Paths::split_uri(uri);
        assert_eq!(query, vec![("name", "test")]);
        assert_eq!(fragment, "frag");
        assert_eq!(route, vec!["some"]);
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