// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail!
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a hint.




fn five() -> u8 {
    5
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn you_can_assert_eq() {
        let five = five();
        assert_eq!(5, five);
    }
}
