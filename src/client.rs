use clap::ArgMatches;
use gitlab::{Gitlab, GitlabError};

use super::config::Config;

pub(super) fn create_client(domain: &str, token: &str) -> Result<Gitlab, GitlabError> {
    Gitlab::new(domain, token)
}

pub(crate) fn try_create_client(matches: &ArgMatches, config: &Config) -> Option<Gitlab> {
    // if the optional resolve title is given create the client (resolve-title must be given the token)
    if matches.is_present("api") || config.api_key.is_some() {
        // always prefer the CLI API argument over the config
        create_client(
            &config.base_url,
            matches
                .value_of("api")
                .unwrap_or_else(|| &config.api_key.as_ref().unwrap()),
            // `unwrap_or_else` here so it's computed when needed
            // e.g. if there's an API key provided on the CLI we want this to pass
            // `unwrap_or` will cause this to fail because it will be computed at the same time
            //
            // The above `unwrap` is safe since we only compute it when the `api` is missing from the CLI
            // and check that the config has an API Key
        )
        .ok()
    } else {
        None
    }
}
