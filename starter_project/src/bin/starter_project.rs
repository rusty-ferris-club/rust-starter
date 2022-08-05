#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::missing_const_for_fn)]

mod cmd;
use console::{style, Style};
use std::process::exit;

const DEFAULT_ERR_EXIT_CODE: i32 = 1;
pub const BANNER: &str = r#"
    B A N N E R
"#;

fn main() {
    let app = cmd::default::command().subcommand(cmd::validate::command());

    let v = app.render_version();
    let matches = app.clone().get_matches();

    let env = env_logger::Env::default().filter_or(
        "LOG",
        matches.value_of("log").unwrap_or(log::Level::Info.as_str()),
    );
    env_logger::init_from_env(env);

    if !matches.is_present("no_banner") {
        println!(
            "{}\n                    {}",
            style(BANNER).magenta(),
            style(v).dim()
        );
    }
    let res = match matches.subcommand() {
        None => cmd::default::run(&matches),
        Some(tup) => match tup {
            ("validate", subcommand_matches) => cmd::validate::run(&matches, subcommand_matches),
            _ => unreachable!(),
        },
    };

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
            log::debug!("{:?}", e);
            DEFAULT_ERR_EXIT_CODE
        }
    };
    exit(exit_with)
}
