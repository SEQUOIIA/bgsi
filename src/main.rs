mod config;
mod model;
mod server;
mod api_controller;
mod handler;

#[macro_use]
extern crate actix_web;

fn main() {
    setup();
    println!("Hello, world!");
}


fn setup() {
    std::env::set_var("RUST_LOG", "trace,mio=info,hyper=info,actix_server=debug,actix_http=debug,want=info,reqwest=info");
    pretty_env_logger::init_timed();
}