// use anyhow::anyhow;
// use anyhow::Result as AnyResult;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Definitions {
    pub providers: HashMap<String, String>,
}

pub const CMD: &str = r#"hello"#;

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_foo() {
        assert_eq!(CMD.len(), 5);
        assert_yaml_snapshot!(("one", "two"));
    }
}
