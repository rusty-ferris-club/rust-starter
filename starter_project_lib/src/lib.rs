#![allow(clippy::must_use_candidate)]
mod data;
use data::Foobar;

pub fn some_logic() -> String {
    let f = Foobar { hey: true };
    if f.hey {
        "hey".to_string()
    } else {
        "ho".to_string()
    }
}
