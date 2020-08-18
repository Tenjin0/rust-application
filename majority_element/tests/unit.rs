#[cfg(test)]
mod tests {

    use majority_element::majority_element;
    #[test]

    fn it_works_1() {
        let a = vec![3,2,3];
        assert_eq!(majority_element(a), 3);
    }

    #[test]
    fn it_works_2() {
        let a = vec![2,2,1,1,1,2,2];
        assert_eq!(majority_element(a), 2);
    }

    // #[test]
    // fn it_works_3() {
    //     assert_eq!(reverse_int_by_str(120), 21);
    // }

    // #[test]
    // fn it_works_4() {
    //     assert_eq!(reverse_int_by_str(1534236469), 0);
    // }
}
