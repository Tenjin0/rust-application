// https://leetcode.com/problems/running-sum-of-1d-array/

pub fn sum(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let mut sum: i32 = 0;
    for num in nums {
        sum += num;
        res.push(sum);
    }
    res
}
