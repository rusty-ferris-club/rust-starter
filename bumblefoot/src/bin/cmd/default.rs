use clap::crate_version;
use clap::{AppSettings, Arg, Command};

pub fn command() -> Command<'static> {
    Command::new("bumblefoot")
        .version(env!("VERGEN_GIT_SEMVER"))
        .version(crate_version!())
        .about("A starter project for Rust")
        .setting(AppSettings::ArgRequiredElseHelp)
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
            Arg::new("log")
                .long("log")
                .help("Set logging level")
                .value_name("LEVEL")
                .possible_values(vec![
                    log::LevelFilter::Off.as_str(),
                    log::LevelFilter::Trace.as_str(),
                    log::LevelFilter::Debug.as_str(),
                    log::LevelFilter::Info.as_str(),
                    log::LevelFilter::Warn.as_str(),
                    log::LevelFilter::Error.as_str(),
                ])
                .default_value(log::Level::Info.as_str())
                .ignore_case(true)
                .takes_value(true),
        )
}
