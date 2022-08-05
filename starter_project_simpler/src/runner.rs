pub fn run() {
    println!("running!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_run() {
        run();
        assert_eq!("1", "1");
    }
}
