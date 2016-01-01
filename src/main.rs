#[macro_use] extern crate nickel;
extern crate rusp;

use nickel::{Nickel, StaticFilesHandler};
use rusp::RuspApp;

mod config;
mod controllers;

fn main() {

    RuspApp::set_host("127.0.0.1:3001");

    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("assets/"));
    server.utilize(config::routes::build());

    server.listen(RuspApp::get_host());
}
 
