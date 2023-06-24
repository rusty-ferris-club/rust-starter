use eyre::Result;

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn run() -> Result<bool> {
    println!("running!");
    Ok(true)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_run() {
        run().expect("foobar");
        assert_eq!("1", "1");
    }
}
