use anyhow::Result as AnyResult;
use bumblefoot;
use clap::crate_version;
use clap::{Arg, ArgMatches, Command};

pub fn command() -> Command<'static> {
    Command::new("bumblefoot")
        .version(env!("VERGEN_GIT_SEMVER"))
        .version(crate_version!())
        .about("A starter project for Rust")
        .arg(
            Arg::new("dry_run")
                .short('d')
                .long("dry-run")
                .value_name("EXAMPLE_KEY")
                .help("Dry run with examples given in EXAMPLE_KEY")
                .takes_value(true),
        )
        .arg(
            Arg::new("reporter")
                .short('r')
                .long("reporter")
                .value_name("REPORTER")
                .takes_value(true)
                .possible_values(&["console"])
                .help("Reporter to use (default: 'console')"),
        )
        .arg(
            Arg::new("no_banner")
                .long("no-banner")
                .help("Don't show the banner")
                .takes_value(false),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .help("Show details about interactions")
                .takes_value(false),
        )
}

pub fn run(matches: &ArgMatches) -> AnyResult<bool> {
    log::info!("default cmd {:?}", matches.value_of("reporter"));
    println!("going to run {}", bumblefoot::CMD);
    bumblefoot::run();
    Ok(true)
}
