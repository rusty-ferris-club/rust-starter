// use anyhow::anyhow;
// use anyhow::Result as AnyResult;
use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Definitions {
    pub providers: HashMap<String, String>,
}

pub const CMD: &str = r"hello";

#[cfg(test)]
mod tests {
    use insta::assert_yaml_snapshot;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(CMD.len(), 5);
        assert_yaml_snapshot!(("one", "two"));
    }
}
