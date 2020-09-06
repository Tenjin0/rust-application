pub fn remove(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut previous: i32 = nums[nums.len() - 1];
    let mut count = 1;
    for i in (0..(nums.len() - 1)).rev() {
        if previous == nums[i] {
            nums.remove(i);
        } else {
            previous = nums[i];
            count = count + 1;
        }
      
    }
    count
}
