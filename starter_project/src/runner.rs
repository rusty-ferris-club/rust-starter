pub fn run() {
    println!("running!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run();
        assert_eq!(5, 5);
    }
}
