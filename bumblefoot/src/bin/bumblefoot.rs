mod cmd;
use console::style;
use std::process::exit;

pub const BANNER: &str = r#"
    B A N N E R
"#;

fn main() {
    let app = cmd::default::command().subcommand(cmd::validate::command());

    let v = app.render_version();
    let matches = app.to_owned().get_matches();

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
