use anyhow::Result as AnyResult;
use bumblefoot_lib;
use clap::{App, Arg, ArgMatches};

pub fn command() -> App<'static> {
    App::new("validate")
        .about("Validate keys")
        .arg(
            Arg::new("list")
                .long("list")
                .about("Show provider list")
                .takes_value(false),
        )
        .arg(
            Arg::new("csv_in")
                .long("--csv-in")
                .value_name("FILE")
                .about("Read providers and params via CSV")
                .takes_value(true),
        )
}

pub fn run(_matches: &ArgMatches, _subcommand_matches: &ArgMatches) -> AnyResult<bool> {
    println!("going to run {}", bumblefoot_lib::CMD);
    bumblefoot_lib::run();
    Ok(true)
}
