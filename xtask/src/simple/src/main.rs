use anyhow::Result as AnyResult;
use clap::crate_version;
use clap::{App, Arg, ArgMatches};
use console::style;
use std::process::exit;
pub const BANNER: &str = r#"
    B A N N E R
"#;

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
            Arg::new("verbose")
                .long("verbose")
                .help("Show details about interactions")
                .takes_value(false),
        )
}

pub fn run(matches: &ArgMatches) -> AnyResult<bool> {
    log::info!("default cmd {:?}", matches.value_of("reporter"));
    println!("going to run {}", bumblefoot::CMD);
    bumblefoot::run();
    Ok(true)
}

fn main() {
    // just use $ LOG=1 mybin
    let env = env_logger::Env::default().filter_or("LOG", "info");
    env_logger::init_from_env(env);

    let app = command();

    let v = app.render_version();
    let matches = app.to_owned().get_matches();

    if !matches.is_present("no_banner") {
        println!(
            "{}\n                    {}",
            style(BANNER).magenta(),
            style(v).dim()
        );
    }
    let res = match matches.subcommand() {
        None => run(&matches),
        _ => Ok(false),
    };

    match res {
        Ok(ok) => {
            exit(if ok { 0 } else { 1 });
        }
        Err(err) => {
            eprintln!("error: {}", err);
            exit(1)
        }
    }
}
