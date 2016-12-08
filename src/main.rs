extern crate afterparty;
extern crate env_logger;
extern crate envy;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate jira_transit;

use afterparty::Hub;
use hyper::Server;
use jira_transit::{Config, DefaultGithub, DefaultJira, Transit};

fn run(config: Config) {
    let github_secret = config.github_secret.clone();
    let github = DefaultGithub::new(hyper::Client::new(), config.clone());
    let jira = DefaultJira::new(hyper::Client::new(), config.clone());
    let transit = Transit::new(Box::new(github), Box::new(jira));

    // wire up wehbook registrations
    let mut hub = Hub::new();
    // register interest in _all_ github events
    hub.handle_authenticated("*", github_secret, transit);
    let svc = Server::http("0.0.0.0:4567")
        .unwrap()
        .handle(hub);
    info!("ready to go");
    svc.unwrap();
}

fn main() {
    env_logger::init().unwrap();
    match envy::from_env::<Config>() {
        Ok(config) => run(config),
        Err(envy::Error::MissingValue(field)) => {
            panic!("missing required env var {}",
                   field.to_owned().to_uppercase())
        }
        Err(envy::Error::Custom(msg)) => panic!(msg),
    }
}
