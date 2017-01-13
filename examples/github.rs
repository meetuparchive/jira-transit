extern crate jira_transit;
extern crate hyper;

use jira_transit::{Config, DefaultGithub, Github, Pull, Transit};
use std::env;

fn main() {
    match (env::var("GH_TOKEN"), env::var("GH_REPO"), env::var("GH_PULL")) {
        (Ok(token), Ok(repo), Ok(pr)) => {
            let gh = DefaultGithub::new(hyper::Client::new(), Config {
                github_token: token,
                ..Default::default()
            });
            let content = gh.content(Pull::new(pr.parse().unwrap(), repo));
            println!("content {:?}", content);
            let directives = Transit::parse_content(content);
            println!("directives {:?}", directives);
        },
        _ => ()
    }
}
