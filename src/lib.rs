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
mod directive;

// re-exports
pub use github::{DefaultGithub, Github};
pub use jira::{DefaultJira, Jira};
pub use config::Config;
pub use directive::Directive;

/// a pull is a reference to a github pull requset for a given repo
pub struct Pull {
    /// number of pull request
    pub number: u64,
    /// / repo slug in owner/repo format
    pub repo_slug: String,
}

/// the primary orchestator for handling github webhooks
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

    /// process a pull request
    pub fn process(&self, pull: Pull) {
        let github::Content { commits, comments } = self.github.content(pull);
        let commit_directives = commits.iter().fold(vec![], |mut result, commit| {
            for d in directive::parse(commit.as_ref()) {
                result.push(d)
            }
            result
        });
        let combined_directives = comments.iter().fold(commit_directives, |mut result, comment| {
            for d in directive::parse(comment.as_ref()) {
                result.push(d)
            }
            result
        });
        self.jira.transition(combined_directives)
    }
}

impl Hook for Transit {
    fn handle(&self, delivery: &Delivery) {
        info!("recv {} delivery {}", delivery.event, delivery.id);
        match delivery.payload {
            /// handle all merged pull request events
            Event::PullRequest { ref action, ref pull_request, ref repository, .. }
                if action == "closed" && pull_request.merged => {
                self.process(Pull {
                    number: pull_request.number,
                    repo_slug: repository.full_name.clone(),
                })
            }
            _ => (),
        }
    }
}
