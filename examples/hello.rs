extern crate http;
extern crate iron;

use std::io::net::ip::Ipv4Addr;

use iron::{Iron, Chain, Alloy, Request, Response, Server, Status, Continue, FromFn};
use iron::mixin::Serve;

fn hello_world(_req: &mut Request, res: &mut Response, _alloy: &mut Alloy) -> Status {
    let _ = res.serve(::http::status::Ok, "Hello, world!");
    Continue
}

fn main() {
    let mut server: Server = Iron::new();
    server.chain.link(FromFn::new(hello_world));
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}

