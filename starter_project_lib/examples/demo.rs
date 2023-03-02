#![allow(clippy::cognitive_complexity)]
use clap::{arg, command};
use starter_project_lib::some_logic;

fn main() {
    let matches = command!()
        .arg(arg!(<archive> "Archive to extract"))
        .arg(arg!(<out> "Output folder"))
        .arg(arg!(
            -s --strip "Strip the first component of the archive"
        ))
        .get_matches();

    let _archive = matches.get_one::<String>("archive").expect("required");
    let _to = matches.get_one::<String>("out").expect("required");
    let _strip = usize::from(matches.get_flag("strip"));
    let res = some_logic();
    println!("{res:?}");
}
