#[cfg(test)]
mod tests {

    use reverse_integer::reverse_int_by_str;
    #[test]

fn it_works_with_positive_int() {
        assert_eq!(reverse_int_by_str(123), 321);
    }

    #[test]
    fn it_works_with_negative_int() {
        assert_eq!(reverse_int_by_str(-123), -321);
    }

    #[test]
    fn it_works_with_0_int() {
        assert_eq!(reverse_int_by_str(120), 21);
    }

    #[test]
    fn it_works_with_long_int() {
        assert_eq!(reverse_int_by_str(1534236469), 0);
    }

    
}