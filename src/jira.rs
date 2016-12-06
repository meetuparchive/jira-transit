extern crate goji;
extern crate hyper;

use super::{Config, Directive};

pub trait Jira: Sync + Send {
    fn transition(&self, directives: Vec<Directive>);
}

pub struct DefaultJira {
    client: hyper::Client,
    config: Config,
}

impl DefaultJira {
    pub fn new(client: hyper::Client, config: Config) -> DefaultJira {
        DefaultJira {
            client: client,
            config: config,
        }
    }
}

impl Jira for DefaultJira {
    fn transition(&self, directives: Vec<Directive>) {
        let jira = goji::Jira::new(self.config.jira_host.clone(),
                                   goji::Credentials::Basic(self.config.jira_username.clone(),
                                                            self.config.jira_password.clone()),
                                   &self.client);
        for d in directives {
            for option in jira.transitions(d.key).list() {
                println!("can transition via {:#?}", option);
            }
        }
    }
}
