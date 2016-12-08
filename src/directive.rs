extern crate regex;

use regex::Regex;

/// keywords to look for in pull request content
const KEYWORDS: &'static [&'static str] = &["closes", "closed", "fix", "fixes", "fixed",
                                            "resolve", "resolves", "resolved"];

/// a keyword action associated with a jira key
#[derive(Debug, PartialEq, Clone)]
pub struct Directive {
    pub action: String,
    pub key: String,
}

impl Directive {
    pub fn new<A, K>(action: A, key: K) -> Directive
        where A: Into<String>,
              K: Into<String>
    {
        Directive {
            action: action.into(),
            key: key.into(),
        }
    }
}

/// parses a vector of directives from a corpus of text
/// https://help.github.com/articles/closing-issues-via-commit-messages/
/// https://confluence.atlassian.com/bitbucket/processing-jira-software-issues-with-smart-commit-messages-298979931.html#ProcessingJIRASoftwareissueswithSmartCommitmessages-Notes
pub fn parse<T>(txt: T) -> Vec<Directive>
    where T: Into<String>
{
    lazy_static! {
        static ref PATTERN: Regex = {
            let mut pat = String::from("(");
            pat.push_str(&KEYWORDS.join("|"));
            pat.push_str(") #([A-Z]{2,}-\\d+)");
            Regex::new(
                &pat
            ).unwrap()
        };
    }
    PATTERN.captures_iter(&txt.into())
        .fold(vec![], |mut result, capture| {
            match (capture.at(1), capture.at(2)) {
                (Some(action), Some(key)) => {
                    result.push(Directive::new(action, key));
                    result
                }
                _ => result,
            }
        })
}


#[cfg(test)]
mod tests {
    use super::{parse, KEYWORDS, Directive};

    #[test]
    fn it_parses_directive() {
        for kw in KEYWORDS {
            assert_eq!(directives(format!("{} #AB-123", *kw)),
                       vec![Directive::new(*kw, "AB-123")])
        }
    }

    #[test]
    fn it_parses_multiple_directives() {
        assert_eq!(directives("closes #AB-123 fixes #CDF-456"),
                   vec![Directive::new("closes", "AB-123"), Directive::new("fixes", "CDF-456")])
    }

    #[test]
    fn it_does_not_parse_directives_with_lower_case_keys() {
        for kw in KEYWORDS {
            assert_eq!(directives(format!("{} #an-123", *kw)), vec![])
        }
    }
}
