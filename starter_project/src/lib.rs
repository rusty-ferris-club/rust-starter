#![allow(clippy::missing_const_for_fn)]

mod data;
mod runner;
pub use self::runner::run;
pub use data::{CmdExit, CMD};
