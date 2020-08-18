// https://leetcode.com/problems/majority-element/submissions/

use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut count_elements : HashMap<i32, usize> = HashMap::new();
    let mut max = nums[0];

    let threshold = nums.len() / 2;
    for num in nums {
        let opt_count = count_elements.get(&num);
        match opt_count {
            None => {
                count_elements.insert(num, 1);
            },
            Some(&count) => {
                count_elements.insert(num, count + 1);
            }
        };
        if count_elements.get(&num).unwrap() > count_elements.get(&max).unwrap() {
            max = num
        }
       
    }

    if count_elements.get(&max).unwrap() >= &threshold {
        max
    } else {
        0
    }
}
