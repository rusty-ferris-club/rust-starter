mod argh_ext;
use std::process::exit;

use argh::FromArgs;

/// example: foobar
#[derive(Debug, FromArgs)]
#[allow(clippy::struct_excessive_bools)]
struct AppArgs {
    /// task to run
    #[argh(positional)]
    task: Option<String>,

    /// list tasks
    #[argh(switch, short = 'l')]
    list: bool,

    /// root path (default ".")
    #[argh(option, short = 'p')]
    path: Option<String>,

    /// init local config
    #[argh(switch, short = 'i')]
    init: bool,
}

fn main() -> eyre::Result<()> {
    let args: AppArgs = argh_ext::from_env();

    let task = args.task;
    println!("{task:?}");

    let _path_s = args.path.unwrap_or_else(|| ".".to_string());

    if args.init {
        println!("wrote file.");
        exit(0);
    }

    let res = if args.list {
        println!("list!");
        Ok(true)
    } else {
        starter_project_simpler::runner::run()
    };

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
