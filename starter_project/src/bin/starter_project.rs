#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::missing_const_for_fn)]

mod cmd;
use console::{style, Style};
use std::process::exit;
use tracing::debug;
use tracing::metadata::LevelFilter;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

const DEFAULT_ERR_EXIT_CODE: i32 = 1;
pub const BANNER: &str = r#"
    B A N N E R
"#;

fn main() {
    let app = cmd::default::command().subcommand(cmd::validate::command());

    let v = app.render_version();
    let matches = app.get_matches();

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

    if !matches.get_flag("no_banner") {
        println!(
            "{}\n                    {}",
            style(BANNER).magenta(),
            style(v).dim()
        );
    }
    let res = matches.subcommand().map_or_else(
        || cmd::default::run(&matches),
        |tup| match tup {
            ("validate", subcommand_matches) => cmd::validate::run(&matches, subcommand_matches),
            _ => unreachable!(),
        },
    );

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
