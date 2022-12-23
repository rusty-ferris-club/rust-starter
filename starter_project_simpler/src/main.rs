#![allow(clippy::must_use_candidate)]

use starter_project_simpler::data::CMD;
use starter_project_simpler::runner;

use anyhow::Result as AnyResult;
use clap::{crate_version, ArgAction};
use clap::{Arg, ArgMatches, Command};
use console::style;
use std::process::exit;
use tracing::metadata::LevelFilter;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

pub const BANNER: &str = r#"
    B A N N E R
"#;

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

/// Run
///
/// # Errors
///
/// This function will return an error
#[allow(clippy::unnecessary_wraps)]
fn run(matches: &ArgMatches) -> AnyResult<bool> {
    log::info!("default cmd {:?}", matches.get_one::<String>("reporter"));
    println!("going to run {CMD}");
    runner::run();
    Ok(true)
}

fn main() {
    // just use $ LOG=1 mybin

    let app = command();

    let v = app.render_version();
    let matches = app.get_matches();

    if !matches.get_flag("no_banner") {
        println!(
            "{}\n                    {}",
            style(BANNER).magenta(),
            style(v).dim()
        );
    }

    let level = if matches.get_flag("verbose") {
        LevelFilter::INFO
    } else {
        LevelFilter::OFF
    };

    // set up tracing.
    // use info! or trace! etc. to log
    // to instrument use `#[tracing::instrument(level = "trace", skip(session), err)]`
    Registry::default()
        .with(tracing_tree::HierarchicalLayer::new(2))
        .with(
            EnvFilter::builder()
                .with_default_directive(level.into())
                .with_env_var("LOG")
                .from_env_lossy(),
        )
        .init();

    // actual logic is in 'run'.
    // subcommand is an error, but you can swap it later if you bring in subcommands
    let res = match matches.subcommand() {
        None => run(&matches),
        _ => Ok(false),
    };

    match res {
        Ok(ok) => {
            exit(i32::from(!ok));
        }
        Err(err) => {
            eprintln!("error: {err}");
            exit(1)
        }
    }
}
