# Lucifer
Lucifer is a web server library.
Lucifer comes with its own http implementation. 
Furthermore it comes with routing and the possibility to add middleware.

## Getting started ##
First you have to add lucifer as an dependency to the project you want to use it in.
Now add lucifer to your file.`extern crate lucifer;`

Functions you want to add to routes do need to have the signature
```rust
fn [function_name](req: Request, args: Args)
```
When starting the server it requires a RouteHandler and a MiddleWareStore.
Lets first forget about the MiddlewareStore. The RouteHandler is a object to which you add the routes you want the server to listen for.
A root route can be added and listened to by the the server like this.
```rust
extern crate lucifer;

use lucifer::http::*;
use lucifer::router::*;
use lucifer::middleware::*;

fn main() {
    let mut server = lucifer::Server::new();
    let mut router = RouteHanlder::new();
    router.add(Method::GET, "/", root_function);
    server.listen("127.0.0.1:8000", router, MiddlewareStore::new())
}
```

## Routes ##
Routes are handled by the RouteHandler. Routes can be added via the `add_route` method and the belonging function can be retrieved via the `get_route` method.

For adding and retrieving routes all HTTP verbs can be used which are defined in the enum `http::Method`.
These verbs are:
- get
- head
- post
- put
- delete
- trace
- options
- connect
- patch

The routes themselves can be static routes but can also contain variables which will be accessible when the function is called.
To add a variable part in the route start it the a ':' continued by the variable name. Do not use the same name multiple times in a url or there will be a name conflict and one of the two will not be accessible in the function.
This value can in the function be retrieved from the args. In the args the variable name still constrains the ':' to make the origin of the variables clear.

Besides single variables the remainder of a route can also be made variable. This can be done by using an '*' for the route, for example `/static/*`. This wil call the same function for every routes requested that starts with `/static/` and the remainder of the uri can be retrieved from the args with `args.get("*")`.
### Example of the static route ###
In this example the `/static/` route will serve files requested or return a 404 error.
```rust
extern crate lucifer;

use lucifer::http::*;
use lucifer::router::*;
use lucifer::middleware::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut server = lucifer::Server::new();
    let mut routes = RouteHandler::new();
    routes.add_route(Method::GET, "/static/*", static_content);
    server.listen("127.0.0.1:8000", routes, MiddlewareStore::new());
}

fn static_content(_req: Request, args: Args) -> Response {
    let mut res = Response::new();
    match args.get("*") {
        Some(resource) => {
            let mut location = String::from("./static/");
            location.push_str(resource);
            match File::open(location) {
                Ok(mut file) => {
                    let mut contents = String::new();
                    match file.read_to_string(&mut contents) {
                        Ok(_) => {res.send_message(&contents);},
                        Err(_) => {res.set_status(500);}
                    }
                },
                Err(_) => {
                    res.set_status(404);
                }
            }
        },
        None => {
            res.set_status(404);
        }
    }
    res
}
```
Warning
that the above code is not safe and without input sensitization it can be abused to read unintended files on the system.