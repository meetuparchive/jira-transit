extern crate goji;
extern crate hyper;

use super::{Config, Directive};
use goji::{TransitionOption, TransitionTriggerOptions};

/// returns Some trigger options for a matching directive
fn trigger(config: Config, option: &TransitionOption) -> Option<TransitionTriggerOptions> {
    if config.transition == option.name {
        Some(TransitionTriggerOptions::new(option.id.clone()))
    } else {
        None
    }
}

/// interface for transitioning jira issues
pub trait Jira: Sync + Send {
    /// transition a list of issues
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
            info!("fetching transitions for jira issue {}", d.key);
            match jira.transitions(d.key.clone()).list() {
                Ok(options) => {
                    for option in options {
                        info!("{} can transition to {} ({})",
                              d.key,
                              option.name,
                              option.id);
                        if let Some(trigger) = trigger(self.config.clone(), &option) {
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

#[cfg(test)]
mod tests {
    use super::trigger;
    use super::super::Config;
    use goji::{TransitionTo, TransitionOption};

    #[test]
    fn it_matches_transition_from_config() {
        // Setup a config
        // Make some transition options
        // Match an appropriate trigger options
        let transition_name = "To QA";
        let config = Config { transition: transition_name.to_owned(), ..Default::default() };
        let transition_option = TransitionOption {
            id: "781".to_string(),
            name: "To QA".to_string(),
            to: TransitionTo {
                name: "test name".to_string(),
                id: "123".to_string(),
            },
        };

        assert_eq!(trigger(config, &transition_option)
                       .map(|trigger_options| trigger_options.transition.id),
                   Some(transition_option.id))
    }

    #[test]
    fn it_doesnt_match_transition_from_config() {
        let transition_name = "To Space";
        let config = Config { transition: transition_name.to_owned(), ..Default::default() };
        let transition_option = TransitionOption {
            id: "781".to_string(),
            name: "To QA".to_string(),
            to: TransitionTo {
                name: "test name".to_string(),
                id: "234".to_string(),
            },
        };

        assert_eq!(trigger(config, &transition_option)
                       .map(|trigger_options| trigger_options.transition.id),
                   None)
    }
}
