#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work_with_two_sum() {
        let nums = vec![-1,-2,-3,-4,-5];
        assert_eq!(two_sum(&nums, -8), [2, 4]);
    }

    #[test]
    fn it_work_with_two_sum_hash() {
        let nums = vec![-1,-2,-3,-4,-5];
        assert_eq!(two_sum_hash(&nums, -8), [2, 4]);
    }

}