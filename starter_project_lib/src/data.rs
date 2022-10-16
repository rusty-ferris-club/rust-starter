use serde::Serialize;

#[derive(Serialize)]
pub struct Foobar {
    pub hey: bool,
}

#[cfg(test)]
mod tests {
    use insta::assert_yaml_snapshot;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(5, 5);
        assert_yaml_snapshot!(Foobar { hey: true });
    }
}
