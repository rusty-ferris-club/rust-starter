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
        .subcommand(Command::new("dual"))
        .subcommand(Command::new("vars"));
    let matches = cli.get_matches();

    let root = root_dir();
    let project = root.join(TEMPLATE_PROJECT_NAME);
    let res = match matches.subcommand() {
        Some(("simple", _)) => {
            remove_dir(project.join("src/bin"))?;
            if exists(project.join("src/main.rs"))
                && !confirm("this will overwrite existing contents. continue?")
            {
                return Err(anyhow!("user aborted"));
            }
            copy_contents(root.join("xtask/src/simple"), project, true)?;

            println!("scaffolded as a simple CLI only crate.");

            Ok(())
        }
        Some(("dual", _)) => {
            if exists(project.join("src/bin"))
                && !confirm("this will overwrite existing contents. continue?")
            {
                return Err(anyhow!("user aborted"));
            }

            remove_file(project.join("src/main.rs"))?;
            copy_contents(root.join("xtask/src/dual"), project, true)?;

            println!("scaffolded as a dual crate.");

            Ok(())
        }
        Some(("eject", _)) => {
            // cargo builds and runs before eject!
            // $ cargo xtask eject my-project
            // or (pick what looks better and less surprising):
            // $ cargo xtask eject --as my-project
            // cargo builds and runs after eject!

            // take param for a name
            // name must be legal because it has to be a: file name, module name, crate name
            // check the name in crates (if taken), ask if to proceed if it is.
            // prompt - no going back, are you sure?
            // rename folders
            // rename files
            // replace in files

            // notes:
            // omnipotent - works whether simple or dual mode, with no specific details in this task, unless we have to
            // create variations of a name?
            Ok(())
        }
        Some(("vars", _)) => {
            println!("project root: {:?}", project);
            println!("root: {:?}", root);
            Ok(())
        }
        _ => unreachable!("unreachable branch"),
    };
    res
}

fn remove_file<P>(path: P) -> AnyResult<()>
where
    P: AsRef<Path>,
{
    fsx::file::remove(path).map_err(anyhow::Error::msg)
}

fn remove_dir<P>(path: P) -> AnyResult<()>
where
    P: AsRef<Path>,
{
    fsx::dir::remove(path).map_err(anyhow::Error::msg)
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
