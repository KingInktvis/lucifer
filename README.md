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