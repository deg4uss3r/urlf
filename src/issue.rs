use crate::{config::Config, error::Error};

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

pub(super) fn issue(
    config: &Config,
    number: u64,
    client: Option<&Gitlab>,
) -> Result<String, Error> {
    if let Some(client) = client {
        let issue_endpoint = Issue::builder()
            .project(config.repository.as_ref())
            .issue(number)
            .build()?;
        let issue: IssueTester = issue_endpoint
            .query(client)
            .map_err(|_| Error::EndPointError)?;

        Ok(format!("[{}]({})", issue.title, issue.web_url))
    } else {
        Ok(format!(
            "[!{}]({})",
            number,
            format!("{}{}/issues/{}", config.base_url, config.repository, number)
        ))
    }
}
