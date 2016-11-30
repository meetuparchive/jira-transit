extern crate afterparty;
extern crate env_logger;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate jira_transit;

use jira_transit::Transit;
use afterparty::Hub;
use hyper::Server;

fn main() {
    env_logger::init().unwrap();

    let transit = Transit {};

    let mut hub = Hub::new();
     hub.handle("*", transit);
     let svc = Server::http("0.0.0.0:4567")
        .unwrap()
        .handle(hub);
     info!("ready to go");
     svc.unwrap();
}
