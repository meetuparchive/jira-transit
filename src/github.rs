extern crate hyper;

use hubcaps::{self, Credentials};
use super::{Config, Pull};

/// Content associated with a pull request
#[derive(Debug, Default)]
pub struct Content {
    pub commits: Vec<String>,
    pub comments: Vec<String>,
}

/// interface for fetching pull request information
pub trait Github: Sync + Send {
    /// get a collection of content associated with a given pull
    fn content(&self, pull: Pull) -> Content;
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
    fn content(&self, pull: Pull) -> Content {
        let gh = hubcaps::Github::new(format!("{}/{}",
                                              env!("CARGO_PKG_NAME"),
                                              env!("CARGO_PKG_VERSION")),
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
        // fetch all commits
        let commits = match gh.repo(repo_uri[0], repo_uri[1])
            .pulls()
            .get(pull.number)
            .commits()
            .iter() {
            Ok(iter) => iter.collect::<Vec<_>>(),
            _ => vec![],
        };
        Content {
            commits: commits.iter()
                .map(|commit| commit.commit.message.clone())
                .collect::<Vec<_>>(),
            comments: comments.iter()
                .map(|comment| comment.body.clone())
                .collect::<Vec<_>>(),
        }
    }
}
