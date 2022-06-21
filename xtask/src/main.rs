use anyhow::anyhow;
use anyhow::Result as AnyResult;
use clap::{AppSettings, Command};
use dialoguer::{theme::ColorfulTheme, Confirm};
use fs_extra as fsx;
use fsx::dir::CopyOptions;
use std::path::{Path, PathBuf};

const TEMPLATE_PROJECT_NAME: &str = "bumblefoot";
fn main() -> Result<(), anyhow::Error> {
    let cli = Command::new("xtask")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(Command::new("simple"))
        .subcommand(Command::new("dual"));
    let matches = cli.get_matches();

    let root = root_dir();
    let res = match matches.subcommand() {
        // simple/, dual/ overlays
        Some(("simple", _)) => {
            // remove bin
            fsx::dir::remove(root.join(TEMPLATE_PROJECT_NAME).join("src/bin"))?;
            if exists(root.join(TEMPLATE_PROJECT_NAME).join("src/main.rs"))
                && !confirm("this will overwrite existing contents. continue?")
            {
                return Err(anyhow!("user aborted"));
            }
            copy_contents(
                root.join("xtask/src/simple"),
                root.join(TEMPLATE_PROJECT_NAME),
                true,
            )?;

            println!("scaffolded as a dual crate.");

            Ok(())
        }
        Some(("dual", _)) => {
            // copy & create bin contents
            // copy dual cargo
            // copy dual main.rs
            // run build
            if exists(root.join(TEMPLATE_PROJECT_NAME).join("src/bin"))
                && !confirm("this will overwrite existing contents. continue?")
            {
                return Err(anyhow!("user aborted"));
            }

            fsx::file::remove(root.join(TEMPLATE_PROJECT_NAME).join("src/main.rs"))?;
            copy_contents(
                root.join("xtask/src/dual"),
                root.join(TEMPLATE_PROJECT_NAME),
                true,
            )?;

            println!("scaffolded as a simple CLI only crate.");

            Ok(())
        }
        _ => unreachable!("unreachable branch"),
    };
    res
}

fn exists<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    std::path::Path::exists(path.as_ref())
}

fn copy_contents<P, Q>(from: P, to: Q, overwrite: bool) -> AnyResult<u64>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let mut opts = CopyOptions::new();
    opts.content_only = true;
    opts.overwrite = overwrite;
    fsx::dir::copy(from, to, &opts).map_err(anyhow::Error::msg)
}

fn confirm(question: &str) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(question)
        .interact()
        .unwrap()
}
fn root_dir() -> PathBuf {
    let mut xtask_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    xtask_dir.pop();
    xtask_dir
}
