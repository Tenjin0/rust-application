use std::io::Error;
use std::io::ErrorKind;

fn inner_quicksort(nums: &[i32], target: i32, start: usize, end: usize) -> Result<i32, std::io::Error> {
    if start == end {
        return Err(Error::new(ErrorKind::InvalidData, "empty array input"));
    }
    let half = start + (end - start) / 2;
    // println!("nums: {:?} target: {} start: {} end: {} half: {}", nums, target, start, end, half);

    if nums[half] == target {
        // println!("found {} {}",half, nums[half], );
        return Ok((half) as i32);
    } else {
        if target < nums[half] {
            return inner_quicksort(nums , target, start, half);
        } else if target > nums[half] {
            return inner_quicksort(nums, target, half + 1, end);
        } else {
            return Err(Error::new(ErrorKind::NotFound, format!("target not found {}", target)));
        }
    }
}

pub fn quicksort(nums: &[i32], target: i32) -> Result<i32, std::io::Error> {

    if nums.len() == 0 {
        return Err(Error::new(ErrorKind::InvalidData, "empty array input"));
    }
    return inner_quicksort(nums , target, 0, nums.len());
}

pub fn search_quick(nums: Vec<i32>, target: i32) -> i32 {

    return quicksort(nums.as_slice(), target).unwrap_or(-1);
}


pub fn search_linear(nums: Vec<i32>, target: i32) -> i32 {

    for (pos, &num) in nums.iter().enumerate() {
        if num == target {
            return pos as i32;
        }
    }

    return 0;
}
