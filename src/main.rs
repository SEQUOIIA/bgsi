mod config;
mod model;
mod server;
mod api_controller;
mod handler;

#[cfg(test)]
mod test;

#[macro_use]
extern crate actix_web;

use crate::config::Config;
use crate::server::serve;

fn main() {
    setup();
    let conf = Config::load();
    println!("{:?}", conf.data);

    serve();
}


fn setup() {
    std::env::set_var("RUST_LOG", "trace,mio=info,hyper=info,actix_server=debug,actix_http=debug,want=info,reqwest=info");
    pretty_env_logger::init_timed();
}