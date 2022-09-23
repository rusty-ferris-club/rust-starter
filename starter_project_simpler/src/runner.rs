pub fn run() {
    println!("running!");
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_run() {
        run();
        assert_eq!("1", "1");
    }
}
