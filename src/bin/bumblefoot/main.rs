mod cmd;
use cmd::validate;
use console::style;
use std::process::exit;

pub const BANNER: &str = r#"
    B A N N E R
"#;

fn main() {
    // just use $ LOG=1 mybin
    let env = env_logger::Env::default().filter_or("LOG", "info");
    env_logger::init_from_env(env);

    let app = cmd::default::command().subcommand(validate::command());

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
