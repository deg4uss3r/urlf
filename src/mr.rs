use crate::config::Config;

use std::fmt;

use gitlab::{
    api::{projects::merge_requests::MergeRequestChanges, Query},
    Gitlab,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MergeRequestTester {
    author: Author,
    title: String,
    web_url: String,
    labels: Vec<String>,
    state: String,
    changes: Vec<Changes>,
}

#[derive(Debug, Deserialize)]
enum MrState {
    Open,
    Closed,
    Locked,
    Merged,
}

impl From<String> for MrState {
    fn from(input: String) -> Self {
        match input.to_ascii_lowercase().as_str() {
            "open" => Self::Open,
            "closed" => Self::Closed,
            "locked" => Self::Locked,
            "merged" => Self::Merged,
            _ => Self::Open, //TODO: return an error
        }
    }
}

#[derive(Debug, Deserialize)]
struct Changes {
    diff: String,
}

#[derive(Debug, Deserialize)]
struct Author {
    name: String,
    username: String,
}

#[derive(Debug, Deserialize)]
struct DiffCount {
    added: u64,
    removed: u64,
}

impl DiffCount {
    fn new() -> Self {
        DiffCount {
            added: 0,
            removed: 0,
        }
    }
}

impl fmt::Display for DiffCount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+{}/-{}", self.added, self.removed)
    }
}

fn diff_counter(diffs: Vec<Changes>) -> DiffCount {
    let mut diff_count = DiffCount::new();

    for d in diffs {
        let a = d.diff.split("\n").collect::<Vec<&str>>();
        let mut p = 0;
        let mut m = 0;
        for i in a {
            if i.starts_with('+') && !i.starts_with("+++") {
                p += 1;
            } else if i.starts_with('-') && !i.starts_with("---") {
                m += 1;
            } else {
                continue;
            }
        }

        diff_count.added += p;
        diff_count.removed += m;
    }

    diff_count
}

pub(super) fn merge_request(config: &Config, number: u64, client: Option<&Gitlab>) -> String {
    // TODO lookup via config
    // TODO safe parsing token
    // TODO Error handling
    if let Some(client) = client {
        let mr_endpoint = MergeRequestChanges::builder()
            .project(config.repository.as_ref())
            .merge_request(number)
            .build()
            .unwrap();
        // Call the endpoint. The return type decides how to represent the value.
        let mr: MergeRequestTester = mr_endpoint.query(client).unwrap();
        format!(
            "[{}]({}) `{}`",
            mr.title,
            mr.web_url,
            diff_counter(mr.changes)
        )
    } else {
        format!(
            "[!{}]({})",
            number,
            format!(
                "{}{}/merge_requests/{}",
                config.base_url, config.repository, number
            )
        )
    }
}
