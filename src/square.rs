pub fn square(num: i64) -> i64 {
    i64::pow(num, 2)
}

#[cfg(test)]
mod square_tests {
    use super::*;

    #[test]
    fn square_test() {
        assert_eq!(square(5), 25);
        assert_eq!(square(10), 100);
        assert_eq!(square(23), 529);
    }
}
