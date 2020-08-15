pub fn two_sum(nums: &Vec<i32>, target: i32) -> [i32; 2] {

    for i in 0..(nums.len() - 1) {
      
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                return [i as i32, j as i32] 
            }
        }
    }

    return [0, 0];
}

pub fn two_sum_hash(nums: &Vec<i32>, target: i32) -> Vec<i32> {

    use std::collections::HashMap;

    let mut map: HashMap<i32, usize> = HashMap::new();

    for i in 0..(nums.len() - 1) {
      
        let complement = map.get(&(target - nums[i]));
        if complement != Option::None {
           return  vec![i as i32, *complement.unwrap() as i32];
        } else {
            for j in (i + 1)..nums.len() {
                if map.get(&nums[j]) != Option::None {
                    map.insert(nums[j], i);
                }
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
    }

    panic!("No elements found in {:?} to sum the target: {}", nums, target)
}

