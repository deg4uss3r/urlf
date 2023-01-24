mod cli;
mod client;
mod config;
mod error;
mod issue;
mod mr;

use crate::client::try_create_client;
use crate::error::Error;

use confy::get_configuration_file_path;

//TODO:
// Errors
// Anything else for issues/MRs?
// How to build in recovery for said errors
// Test and make sure it works with other domains/repositories (via cmdline)

fn main() -> Result<(), Error> {
    let mut app = cli::build();
    let matches = app.clone().get_matches();

    // use `ArgMatches::occurrences_of == 0` to check if the user provided this at runtime
    // the default config name is provided by the `cli` module (it's `default`)
    let config_name = matches.value_of("cfg").ok_or(Error::InvalidArg)?;

    match matches.subcommand() {
        Some(("mr", args)) => {
            // load the config or create and store the `Default` implementation
            let config = config::load(config_name)?;
            let gl_client = try_create_client(&matches, &config);

            println!(
                "{}",
                mr::merge_request(
                    &config,
                    args.value_of("number").unwrap_or_default().parse()?,
                    gl_client.as_ref()
                )?
            );
        }
        Some(("issue", args)) => {
            // load the config or create and store the `Default` implementation
            let config = config::load(config_name)?;
            let gl_client = try_create_client(&matches, &config);

            println!(
                "{}",
                issue::issue(
                    &config,
                    args.value_of("number").ok_or(Error::InvalidArg)?.parse()?,
                    gl_client.as_ref()
                )?
            );
        }
        Some(("config", args)) => {
            // default value is defined in the cli module as "default" it will never error here
            let config_name = matches.value_of("cfg").ok_or(Error::InvalidArg)?;

            if args.is_present("location") {
                if std::fs::metadata(get_configuration_file_path(
                    env!("CARGO_CRATE_NAME"),
                    config_name,
                )?)
                .is_ok()
                {
                    println!(
                        "\nConfig file is located at: {}",
                        get_configuration_file_path(env!("CARGO_CRATE_NAME"), config_name)?
                            .display()
                    );
                } else {
                    eprintln!(
                        "\nError: config files does not exist at this location: {}",
                        get_configuration_file_path(env!("CARGO_CRATE_NAME"), config_name)?
                            .display()
                    )
                }

                return Ok(());
            }

            let base_url = args.value_of("url").ok_or(Error::InvalidArg)?;
            let repo = args.value_of("repo").ok_or(Error::InvalidArg)?;
            let api = args.value_of("api");

            // error reporting
            let _ = config::update_or_store(
                config_name,
                base_url.to_string(),
                repo.to_string(),
                api.map(str::to_string),
            );
        }
        _ => app.print_long_help()?,
    }

    Ok(())
}
