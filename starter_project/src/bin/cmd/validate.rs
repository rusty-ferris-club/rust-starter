use anyhow::Result;
use clap::{Arg, ArgMatches, Command};

pub fn command() -> Command<'static> {
    Command::new("validate")
        .about("Validate keys")
        .arg(
            Arg::new("list")
                .long("list")
                .help("Show provider list")
                .takes_value(false),
        )
        .arg(
            Arg::new("csv_in")
                .long("--csv-in")
                .value_name("FILE")
                .help("Read providers and params via CSV")
                .takes_value(true),
        )
}

pub fn run(
    _matches: &ArgMatches,
    _subcommand_matches: &ArgMatches,
) -> Result<starter_project::CmdExit> {
    Ok(starter_project::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}
