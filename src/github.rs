extern crate hyper;

use hubcaps::{self, Credentials};
use super::{Config, Directive, Pull};
use super::parse;

/// interface for fetching pull request information
pub trait Github: Sync + Send {
    fn pull_directives(&self, pull: Pull) -> Vec<Directive>;
}

pub struct DefaultGithub {
    client: hyper::Client,
    config: Config,
}

impl DefaultGithub {
    pub fn new(client: hyper::Client, config: Config) -> DefaultGithub {
        DefaultGithub {
            client: client,
            config: config,
        }
    }
}

impl Github for DefaultGithub {
    fn pull_directives(&self, pull: Pull) -> Vec<Directive> {
        let gh = hubcaps::Github::new("jira-transit/0.1",
                                      &self.client,
                                      Credentials::Token(self.config.github_token.clone()));
        let repo_uri = pull.repo_slug.split("/").collect::<Vec<_>>();
        // fetch all comments
        let comments = gh.repo(repo_uri[0], repo_uri[1])
            .pulls()
            .get(pull.number)
            .comments()
            .list(&Default::default())
            .unwrap_or(vec![]);
        let commits = match gh.repo(repo_uri[0], repo_uri[1])
            .pulls()
            .get(pull.number)
            .commits()
            .iter() {
                Ok(iter) => iter.collect::<Vec<_>>(),
                _ => vec!()
            };
        let commit_directives = commits.iter().fold(vec![], |mut result, commit| {
            for d in parse::directives(commit.commit.message.clone()) {
                result.push(d)
            }
            result
        });
        comments.iter().fold(commit_directives, |mut result, comment| {
            for d in parse::directives(comment.body.clone()) {
                result.push(d)
            }
            result
        })
    }
}
