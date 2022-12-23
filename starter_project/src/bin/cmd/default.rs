use anyhow::Result;
use clap::{crate_version, ArgAction};
use clap::{Arg, ArgMatches, Command};
use starter_project;
use tracing::info;

pub fn command() -> Command {
    Command::new("starter_project")
        .version(crate_version!())
        .about("A starter project for Rust")
        .arg(
            Arg::new("dry_run")
                .short('d')
                .long("dry-run")
                .value_name("EXAMPLE_KEY")
                .help("Dry run with examples given in EXAMPLE_KEY"),
        )
        .arg(
            Arg::new("reporter")
                .short('r')
                .long("reporter")
                .value_name("REPORTER")
                .value_parser(["console"])
                .help("Reporter to use (default: 'console')"),
        )
        .arg(
            Arg::new("no_banner")
                .long("no-banner")
                .help("Don't show the banner")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .help("Show details about interactions")
                .action(ArgAction::SetTrue),
        )
}

pub fn run(matches: &ArgMatches) -> Result<starter_project::CmdExit> {
    info!("default cmd {:?}", matches.get_one::<String>("reporter"));
    println!("going to run {}", starter_project::CMD);
    starter_project::run();
    Ok(starter_project::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}
