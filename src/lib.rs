extern crate afterparty;
extern crate goji;
#[macro_use]
extern crate log;
extern crate hubcaps;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use afterparty::{Delivery, Event, Hook};

mod config;
mod github;
mod jira;
mod parse;

// re-exports
pub use github::{DefaultGithub, Github};
pub use jira::{DefaultJira, Jira};
pub use config::Config;
pub use parse::Directive;

pub struct Pull {
    pub number: u64,
    pub repo_slug: String,
}

pub struct Transit {
    github: Box<Github>,
    jira: Box<Jira>,
}

impl Transit {
    pub fn new<'a>(github: Box<Github>, jira: Box<Jira>) -> Transit {
        Transit {
            github: github,
            jira: jira,
        }
    }

    pub fn process(&self, pull: Pull) {
        let directives = self.github.pull_directives(pull);
        self.jira.transition(directives)
    }
}

impl Hook for Transit {
    fn handle(&self, delivery: &Delivery) {
        info!("recv {} delivery {}", delivery.event, delivery.id);
        match delivery.payload {
            Event::PullRequest { ref action, ref pull_request, ref repository, .. }
                if action == "merged" && pull_request.merged => {
                self.process(Pull {
                    number: pull_request.number,
                    repo_slug: repository.full_name.clone(),
                })
            }
            _ => (),
        }
    }
}
