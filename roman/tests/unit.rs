#[cfg(test)]
mod tests {
    use roman::to_int;
    #[test]
    fn it_works_1() {
        let roman_to_test = String::from("III");
        assert_eq!(to_int(roman_to_test), "3");
    }
    #[test]
    fn it_works_2() {
        let roman_to_test = String::from("IV");
        assert_eq!(to_int(roman_to_test), "4");
    }

    #[test]
    fn it_works_3() {
        let roman_to_test = String::from("IX");
        assert_eq!(to_int(roman_to_test), "9");
    }
}
