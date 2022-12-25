#![allow(clippy::must_use_candidate)]

use regex::Regex;
use starter_project_simpler::data::CMD;
use starter_project_simpler::runner;

use anyhow::Result as AnyResult;
use clap::{ArgAction, Parser};
use std::process::exit;
use tracing::metadata::LevelFilter;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

/// Zip eXtract: Unpack any archive
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    /// any kind of archive
    #[arg(value_name = "ARCHIVE")]
    archive: String,

    /// where to output extracted content
    #[arg(value_name = "OUT_DIR")]
    out_dir: String,

    /// strip N number of components from the path of extracted files
    #[arg(short, long, value_name = "NUM", default_value = "0")]
    strip: Option<usize>,

    /// only matching files will be extracted
    #[arg(short, long, value_name = "REGEX")]
    filter: Option<Regex>,

    /// list files in archive (don't extract)
    #[arg(short, long, action=ArgAction::SetTrue)]
    list: bool,

    /// list with metadata (don't extract)
    #[arg(long, action=ArgAction::SetTrue)]
    ls: bool,

    /// list with metadata (don't extract)
    #[arg(long, action=ArgAction::SetTrue)]
    verbose: bool,
}

/// Run
///
/// # Errors
///
/// This function will return an error
#[allow(clippy::unnecessary_wraps)]
fn run(opts: &Opts) -> AnyResult<bool> {
    log::info!("strip: {:?}", opts.strip);
    println!("going to run {CMD}");
    runner::run();
    Ok(true)
}

fn main() {
    let opts = Opts::parse();
    let level = if opts.verbose {
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

    match run(&opts) {
        Ok(ok) => {
            exit(i32::from(!ok));
        }
        Err(err) => {
            eprintln!("error: {err}");
            exit(1)
        }
    }
}
