mod cmd;
use console::{style, Style};
use std::process::exit;

pub const BANNER: &str = r#"
    B A N N E R
"#;

fn main() {
    // just use $ LOG=1 mybin
    let env = env_logger::Env::default().filter_or("LOG", "info");
    env_logger::init_from_env(env);

    let app = cmd::default::command().subcommand(cmd::validate::command());

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
        None => cmd::default::run(&matches),
        Some(tup) => match tup {
            ("validate", subcommand_matches) => cmd::validate::run(&matches, subcommand_matches),
            _ => unreachable!(),
        },
    };

    if let Some(message) = res.message {
        let style = if exitcode::is_success(res.code) {
            Style::new().green()
        } else {
            Style::new().red()
        };
        eprintln!("{}", style.apply_to(message));
    }
    exit(res.code)
}
