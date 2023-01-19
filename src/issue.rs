use crate::config::Config;

use gitlab::{
    api::{projects::issues::Issue, Query},
    Gitlab,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct IssueTester {
    title: String,
    web_url: String,
}

pub(super) fn issue(config: &Config, number: u64, client: Option<&Gitlab>) -> String {
    if let Some(client) = client {
        let issue_endpoint = Issue::builder()
            .project(config.repository.as_ref())
            .issue(number)
            .build()
            .unwrap();
        let issue: IssueTester = issue_endpoint.query(client).unwrap();
        format!("[{}]({})", issue.title, issue.web_url)
    } else {
        //TODO take in config
        format!(
            "[!{}]({})",
            number,
            format!("{}{}/issues/{}", config.base_url, config.repository, number)
        )
    }
}
