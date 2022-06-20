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

    #[test]
    fn test_foo() {
        assert_eq!(CMD.len(), 5);
    }
}
