#[allow(dead_code)]
struct Paths {
    name: String,
    function: Option<String>,
    sub: Vec<Paths>,
    wildcard: Vec<Paths>
}

#[allow(dead_code)]
impl Paths {
    fn new_root() -> Paths {
        Paths {
            name: String::from(""),
            function: None,
            sub: Vec::new(),
            wildcard: Vec::new()
        }
    }

    fn new_route(&mut self, route: &str) {
        let split = Paths::route_vec(route);
        self.add_route(&split[1..]);
    }

    fn add_route(&mut self, route: &[&str]) {
        if route.len() > 0 {
            if let Some(c) = route[0].chars().next() {
                if c == ':' {
                    self.add_wildcard_route(route[0]);
                    let i = self.wildcard.len() - 1;
                    self.wildcard[i].add_route(&route[1..]);
                }else {
                    self.add_sub_route(route);
                }
            }
        }
    }

    fn add_wildcard_route(&mut self, name: &str) {
        self.wildcard.push(Paths {
            name: String::from(&name[1..]),
            function: None,
            sub: Vec::new(),
            wildcard: Vec::new()
        });
    }

    fn add_sub_route(&mut self, route: &[&str]) {
        let exists;
        {
            let current = self.find_sub(route[0]);
            exists = match current {
                Some(_item) => true,
                None => false,
            }
        }
        let mut item;
        if exists {
            for i in self.sub.iter_mut() {
                if i.name == route[0] {
                    item = i;
                    break;
                }
            }
        }else{
            self.sub.push(Paths {
                name: String::from(route[0]),
                function: None,
                sub: Vec::new(),
                wildcard: Vec::new()
            });
            let i = self.sub.len() - 1;
            item = &mut self.sub[i];
        }
        // item.add_route(&route[1..]);
    }

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
} 