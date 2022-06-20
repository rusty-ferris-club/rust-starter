use anyhow::Result as AnyResult;
use bumblefoot_lib;
use clap::crate_version;
use clap::{App, Arg, ArgMatches};
pub fn command() -> App<'static> {
    App::new("bumblefoot")
        .version(env!("VERGEN_GIT_SEMVER"))
        .version(crate_version!())
        .about("A starter project for Rust")
        .arg(
            Arg::new("dry_run")
                .short('d')
                .long("dry-run")
                .value_name("EXAMPLE_KEY")
                .about("Dry run with examples given in EXAMPLE_KEY")
                .takes_value(true),
        )
        .arg(
            Arg::new("reporter")
                .short('r')
                .long("reporter")
                .value_name("REPORTER")
                .takes_value(true)
                .possible_values(&["console"])
                .about("Reporter to use (default: 'console')"),
        )
        .arg(
            Arg::new("no_banner")
                .long("no-banner")
                .about("Don't show the banner")
                .takes_value(false),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .about("Show details about interactions")
                .takes_value(false),
        )
}

pub fn run(matches: &ArgMatches) -> AnyResult<bool> {
    log::info!("default cmd {:?}", matches.value_of("reporter"));
    println!("going to run {}", bumblefoot_lib::CMD);
    bumblefoot_lib::run();
    Ok(true)
}
