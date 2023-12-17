pub mod default;
pub mod validate;
use std::process::exit;

use anyhow::Result;
use clap::ArgMatches;
use console::{style, Style};
use starter_project::CmdExit;
use tracing::debug;
use tracing::metadata::LevelFilter;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

pub const BANNER: &str = r"
    B A N N E R
";
pub fn banner(v: &str, matches: &ArgMatches) {
    if !matches.get_flag("no_banner") {
        println!(
            "{}\n                    {}",
            style(BANNER).magenta(),
            style(v).dim()
        );
    }
}

pub fn tracing(matches: &ArgMatches) {
    let level = if matches.get_flag("verbose") {
        LevelFilter::INFO
    } else {
        LevelFilter::OFF
    };
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

const DEFAULT_ERR_EXIT_CODE: i32 = 1;
pub fn result_exit(res: Result<CmdExit>) {
    let exit_with = match res {
        Ok(cmd) => {
            if let Some(message) = cmd.message {
                let style = if exitcode::is_success(cmd.code) {
                    Style::new().green()
                } else {
                    Style::new().red()
                };
                eprintln!("{}", style.apply_to(message));
            }
            cmd.code
        }
        Err(e) => {
            debug!("{:?}", e);
            DEFAULT_ERR_EXIT_CODE
        }
    };
    exit(exit_with)
}
