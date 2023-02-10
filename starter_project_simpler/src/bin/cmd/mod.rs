use anyhow::Result;
use clap::{ArgAction, Parser};
use regex::Regex;
use std::process::exit;
use tracing::metadata::LevelFilter;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

/// Zip eXtract: Unpack any archive
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    /// any kind of archive
    #[arg(value_name = "ARCHIVE")]
    pub archive: String,

    /// where to output extracted content
    #[arg(value_name = "OUT_DIR")]
    pub out_dir: String,

    /// strip N number of components from the path of extracted files
    #[arg(short, long, value_name = "NUM", default_value = "0")]
    pub strip: Option<usize>,

    /// only matching files will be extracted
    #[arg(short, long, value_name = "REGEX")]
    pub filter: Option<Regex>,

    /// list files in archive (don't extract)
    #[arg(short, long, action=ArgAction::SetTrue)]
    pub list: bool,

    /// list with metadata (don't extract)
    #[arg(long, action=ArgAction::SetTrue)]
    pub ls: bool,

    /// list with metadata (don't extract)
    #[arg(long, action=ArgAction::SetTrue)]
    pub verbose: bool,
}

pub fn tracing(opts: &Opts) {
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
}

pub fn exit_result(res: &Result<bool>) {
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
