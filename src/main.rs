extern crate afterparty;
extern crate env_logger;
extern crate envy;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate jira_transit;
#[macro_use]
extern crate chan;
extern crate chan_signal;

use chan_signal::Signal;
use std::process::exit;
use afterparty::Hub;
use hyper::Server;
use jira_transit::{Config, DefaultGithub, DefaultJira, Transit};

fn run(_sdone: chan::Sender<()>, config: Config) {
    let github_secret = config.github_secret.clone();
    let github = DefaultGithub::new(hyper::Client::new(), config.clone());
    let jira = DefaultJira::new(hyper::Client::new(), config.clone());
    let transit = Transit::new(Box::new(github), Box::new(jira));

    // wire up wehbook registrations
    let mut hub = Hub::new();
    // register interest in _all_ github events
    hub.handle_authenticated("pull_request", github_secret, transit);
    let svc = Server::http("0.0.0.0:4567")
        .unwrap()
        .handle(hub);
    info!("ready to go");
    svc.unwrap();
}

fn main() {
    env_logger::init().unwrap();
    match envy::from_env::<Config>() {
        Ok(config) => {
            let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);
            let (sdone, rdone) = chan::sync(0);
            // start the application in a thread
            ::std::thread::spawn(move || run(sdone, config));
            // wait for process signal or application to stop
            chan_select! {
                signal.recv() -> signal => {
                    println!("received signal: {:?}", signal)
                },
                rdone.recv() => {
                    println!("Program completed normally.")
                }
            }
        },
        Err(envy::Error::MissingValue(field)) => {
            panic!("missing required env var {}",
                   field.to_owned().to_uppercase())
        }
        Err(envy::Error::Custom(msg)) => panic!(msg),
    }
}
