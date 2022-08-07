#![allow(clippy::missing_const_for_fn)]
// #![warn(missing_docs)] // uncomment for docs

mod data;
mod runner;
pub use self::runner::run;
pub use data::{CmdExit, CMD};
