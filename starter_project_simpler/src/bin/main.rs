#![allow(clippy::must_use_candidate)]

mod cmd;
use cmd::Opts;
use starter_project_simpler::data::CMD;
use starter_project_simpler::runner;

use anyhow::Result as AnyResult;
use clap::Parser;

/// Run
///
/// # Errors
///
/// This function will return an error
#[allow(clippy::unnecessary_wraps)]
fn run(opts: &Opts) -> AnyResult<bool> {
    log::info!("strip: {:?}", opts.strip);
    println!("going to run {CMD}");
    runner::run();
    Ok(true)
}

fn main() {
    // opts are good in main.rs, but if you ever go and make a lib, 
    // copy Opts to internal Opts data structure to avoid referencing clap
    let opts = Opts::parse();
    cmd::tracing(&opts);

    let res = run(&opts);

    cmd::exit_result(&res);
}
