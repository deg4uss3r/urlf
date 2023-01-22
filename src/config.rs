use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub base_url: String,
    pub repository: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
}

/// `Config` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            base_url: "gitlab.com/".into(),
            repository: "gitlab-org/gitlab".into(),
            api_key: None,
        }
    }
}

pub(crate) fn store(config: &Config, config_name: &str) -> Result<(), Error> {
    confy::store(env!("CARGO_CRATE_NAME"), config_name, config)?;
    Ok(())
}

pub(crate) fn load(config_name: &str) -> Result<Config, Error> {
    let config = confy::load(env!("CARGO_CRATE_NAME"), config_name);

    match config {
        Ok(c) => Ok(c),
        Err(_) => {
            eprintln!("Error: config not loaded using the default config");
            let c = Default::default();
            store(&c, config_name)?;
            Ok(c)
        }
    }
}

pub(crate) fn update_or_store(
    config_name: &str,
    base_url: String,
    repository: String,
    api_key: Option<String>,
) -> Result<(), Error> {
    let config = Config {
        base_url,
        repository,
        api_key,
    };

    store(&config, config_name)
}
