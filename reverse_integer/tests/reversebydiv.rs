#[cfg(test)]
mod tests {

    use reverse_integer::reverse_int_by_div2;

    #[test]
    fn it_works_with_positive_int() {
        assert_eq!(reverse_int_by_div2(123), 321);
    }

    #[test]
    fn it_works_with_negative_int() {
        assert_eq!(reverse_int_by_div2(-123), -321);
    }

    #[test]
    fn it_works_with_0_int() {
        assert_eq!(reverse_int_by_div2(120), 21);
    }

    #[test]
    fn it_works_with_long_int() {
        assert_eq!(reverse_int_by_div2(1534236469), 0);
    }
}
