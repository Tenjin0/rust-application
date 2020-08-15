// https://leetcode.com/problems/power-of-two

fn is_power_of_two(n: i32) -> bool {
    let mut rem = n;
    let mut rest = 0;

    while (rem > 1 || rem < -1) && rest == 0 {
        rest = rem % 2;
        rem /= 2;
        println!("{}, {}", rem, rest)
    }
    rest == 0 && n != 0
}

fn main() {
    println!("{}", is_power_of_two(-16));
}
