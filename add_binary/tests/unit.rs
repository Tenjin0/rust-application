mod tests {

    use add_binary::add;

    #[test]
    fn it_works_1() {
        let a = "11";
        let b = "1";
        assert_eq!(add(a.to_string(), b.to_string()), "100");
    }

    #[test]
    fn it_works_2() {
        let a = "1010";
        let b = "1011";
        assert_eq!(add(a.to_string(), b.to_string()), "10101");
    }

    #[test]
    fn it_works_3() {
        let a = "1111";
        let b = "1111";
        assert_eq!(add(a.to_string(), b.to_string()), "11110");
    }

//     #[test]
//     fn it_works_4() {
//         let a = "110";
//         let b = "11";
//         assert_eq!(add(a.to_string(), b.to_string()), "1001");
//     }
}
