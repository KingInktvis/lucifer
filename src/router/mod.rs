#[allow(dead_code)]
pub struct Paths {
    name: String,
    function: Option<String>,
    sub: Vec<Paths>,
    wildcard: Vec<Paths>
}

#[allow(dead_code)]
impl Paths {
    pub fn new_root() -> Paths {
        Paths {
            name: String::from(""),
            function: None,
            sub: Vec::new(),
            wildcard: Vec::new()
        }
    }

    pub fn new_route(&mut self, route: &str, func: String) {
        let split = Paths::route_vec(route);
        self.add_route(&split[1..], func);
    }

    fn add_route(&mut self, route: &[&str], func: String) {
        if route.len() > 0 {
            if let Some(c) = route[0].chars().next() {
                if c == ':' {
                    Paths::add_route_to_vec(route, &mut self.wildcard, func);
                }else {
                    Paths::add_route_to_vec(route, &mut self.sub, func);
                }
            }
        }else{
            self.function = Some(func);
        }
    }

    fn add_route_to_vec(route: &[&str], store: &mut Vec<Paths>, func: String) {
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
            wildcard: Vec::new()
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
        if route.len() >= start {
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

    fn router(&self, path: &str) -> Option<&String> {
        let v = Paths::route_vec(path);
        self.vec_router(&v[1..])
    }

    fn vec_router(&self, route: &[&str]) -> Option<&String> {
        if route.len() == 0 {
            if let Some(f) = &self.function {
                return Some(&f);
            }else{
                return None;
            }
        }
        if let Some(p) = self.find_sub(route[0]) {
            return p.vec_router(&route[1..]);
        }else{
            return self.route_wildcard(route);
        }
    }

    fn route_wildcard(&self, path: &[&str]) -> Option<&String> {
        for i in self.wildcard.iter() {
            let res = i.vec_router(&path[1..]);
            match res {
                Some(func) => return Some(func),
                None => {}
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn route_vec() {
        let route = "/some/thing";
        let v = Paths::route_vec(route);
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], "");
        assert_eq!(v[1], "some");
        assert_eq!(v[2], "thing");
    }

    #[test]
    fn routes() {
        let mut router = Paths::new_root();
        router.new_route("/other/object", String::from("test2"));
        router.new_route("/some/thing", String::from("test"));
        let test = router.find_sub("some");
        match test {
            Some(route) => {},
            None => panic!("Route not found"),
        }
        let test = router.router("/some/thing");
        match test {
            Some(value) => {if value != "test" {panic!("wrong return value")}},
            None => panic!("Router fn does not return Some.")
        }
        let test = router.router("/other/object");
        match test {
            Some(value) => {if value != "test2" {panic!("wrong return value")}},
            None => panic!("Router fn does not return Some.")
        }
    }

    #[test]
    fn wildcard_routes() {
        let mut router = Paths::new_root();
        router.new_route("/:wildcard/test", String::from("wc"));
        router.new_route("/test/:wildcard/test2", String::from("wc2"));
        let test = router.router("/test/random/test2");
        match test {
            Some(value) => {},
            None => panic!("Router fn does not return Some with wildcard.")
        }
    }
}