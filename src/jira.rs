extern crate goji;
extern crate hyper;

use super::{Config, Directive};
use goji::{TransitionOption, TransitionTriggerOptions};

/// interface for fetching transitioning jira issues
pub trait Jira: Sync + Send {
    /// transition a list of issues
    fn transition(&self, directives: Vec<Directive>);
    /// returns Some trigger options for a matching directive
    fn trigger(&self,
               directive: Directive,
               option: TransitionOption)
               -> Option<TransitionTriggerOptions> {
        // exercise for reader...
        // check option.name agains directive.action
        // use option.id to create TransitionTriggerOptions
        // Some(TransitionTriggerOptions::builder(option.id).resolution("Done").build())
        // for example
        None
    }
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
            match jira.transitions(d.key.clone()).list() {
                Ok(options) => {
                    for option in options {
                        debug!("{} can transition to {} ({})",
                               d.key,
                               option.name,
                               option.id);
                        if let Some(trigger) = self.trigger(d.clone(), option) {
                            match jira.transitions(d.key.clone()).trigger(trigger) {
                                Ok(_) => info!("transitioned issue {}", d.key),
                                Err(err) => {
                                    error!("error transitioning issue {}, {:#?}", d.key, err)
                                }
                            }
                            break;
                        }
                    }
                }
                Err(err) => error!("jira error {:#?}", err),
            }
        }
    }
}
