use clap::{crate_version, Arg, Command};

pub(super) fn build() -> Command<'static> {
    Command::new("urlf")
    .version(crate_version!())
    .subcommand_required(true)
    .arg(
        Arg::new("repo")
            .short('r')
            .long("repo")
            .help("repository if not default")
            .takes_value(true)
            .required(false),
    )
    .arg(
        Arg::new("api")
            .long("online")
            .help("Reach out to the GitLab API to resolve the title and fetch more details (requires a read-only API token)")
            .takes_value(true)
            .required(false),
    )
    .arg(
        Arg::new("cfg")
            .long("cfg")
            .short('c')
            .help("name of the config file")
            .default_value("default")
            .takes_value(true)
            .required(false),
    )
    .subcommand(Command::new("mr")
                .about("Create a formatted URL for an MR")
                .arg(
                    Arg::new("number")
                        .short('n')
                        .long("number")
                        .help("MR number")
                        .validator(|s| s.parse::<u64>())
                        .required(true)
                        .takes_value(true)
                )
            )
        .subcommand(
            Command::new("issue")
                .about("Create a formatted URL for an issue")
                .arg(
                    Arg::new("number")
                        .short('n')
                        .long("number")
                        .validator(|s| s.parse::<u64>())
                        .help("Issue number")
                        .required(true)
                        .takes_value(true),
                )
            )
        .subcommand(
            Command::new("config")
                .about("updates or creates a new config file")
                .arg(
                    Arg::new("location")
                        .long("location")
                        .help("prints the location of the default config file")
                        .required(false)
                        .takes_value(false)
                )
                .arg(
                    Arg::new("url")
                        .long("url")
                        .help("base url (e.g. gitlab.com)")
                        .required_unless_present("location")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("repo")
                        .long("repo")
                        .help("repository to query (e.g. gitlab-org/gitlab)")
                        .required_unless_present("location")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("api")
                        .long("api")
                        .help("API key (read-only)")
                        .required(false)
                        .takes_value(true),
                )
            )
}
