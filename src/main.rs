
extern crate iron;
mod routes;

use iron::prelude::*;
use routes::main_controller;

fn main() {
    let _server = Iron::new(main_controller).http("localhost:3000").unwrap();
    println!("On 3000");
}
