// use anyhow::anyhow;
// use anyhow::Result as AnyResult;
use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Definitions {
    pub providers: HashMap<String, String>,
}

pub struct CmdExit {
    pub code: exitcode::ExitCode,
    pub message: Option<String>,
}

pub const CMD: &str = r"hello";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(CMD.len(), 5);
    }
}
