pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use search_insert::{search_quick};

    #[test]
    fn it_works_1() {
        let a = vec![1, 2, 3, 4, 5];
        let target = 1;
        assert_eq!(search_quick(a, target), 0)
    }

    #[test]
    fn it_works_2() {
        let a = vec![1, 2, 3, 4, 5];
        let target = 2;
        assert_eq!(search_quick(a, target), 1)
    }
    #[test]
    fn it_works_3() {
        let a = vec![1, 2, 3, 4, 5];
        let target = 3;
        assert_eq!(search_quick(a, target), 2)
    }
    #[test]
    fn it_works_4() {
        let a = vec![1, 2, 3, 4, 5];
        let target = 4;
        assert_eq!(search_quick(a, target), 3)
    }
    #[test]
    fn it_works_5() {
        let a = vec![1, 2, 3, 4, 5];
        let target = 5;
        assert_eq!(search_quick(a, target), 4)
    }

    #[test]
    fn it_does_not_work_1() {
        let a = vec![1, 2, 3, 5];
        let target = 4;
        assert_eq!(search_quick(a, target), -1)
    }

    #[test]
    fn it_does_not_work_2() {
        let a = vec![1, 2, 3, 5];
        let target = 0;
        assert_eq!(search_quick(a, target), -1)
    }
    #[test]
    fn it_does_not_work_3() {
        let a = vec![1, 2, 3, 5];
        let target = 6;
        assert_eq!(search_quick(a, target), -1)
    }

}
