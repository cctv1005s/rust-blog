extern crate iron;

use iron::prelude::*;
use iron::status;

// 我希望的形式
// js 的形式写 rust ?
// class.get('/app', () => {})

pub fn main_controller(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
}