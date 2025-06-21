pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    // Function annotated with "#[test]" are automagically
    // collected and executed via "cargo test"
    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4, "two plus two should give four");
    }

    #[test]
    fn another() {
        panic!("This fails");
    }

    #[test]
    fn test_with_result() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 does not equal 4"))
        }
    }
}
