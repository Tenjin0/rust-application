use duplicate;

fn main() {

    let mut nums: Vec<i32> = vec![0,0,1,1,1,2,2,3,3,4];
    println!("{} {:?}", nums.len(), nums);
    let count = duplicate::remove(&mut nums);

    println!("{} {:?}", count, nums);
}
