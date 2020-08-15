// https://leetcode.com/problems/reverse-integer/
use std::panic;

pub fn reverse_int_by_str(x: i32) -> i32 {
    let converted_int : std::string::String = format!("{}", x);
    let mut reversed_int =  String::from("");
    for elem in converted_int.chars().rev() {
        if elem == '-' {
            reversed_int = elem.to_string() + &reversed_int;
        } else {
            reversed_int = reversed_int + &elem.to_string();

        }
    }
    reversed_int.parse::<i32>().unwrap_or(0)
}

pub fn reverse_int_by_div(x: i32) -> i32 {

    let mut rem = x;

    let mut reversed_int: i32 = 0;

    while rem != 0 {
        reversed_int = match reversed_int.checked_mul(10) {
            Some(mul) => {
                match mul.checked_add(rem % 10) {
                    Some(add) => {
                        rem /= 10;
                        add
                    },
                    None => return 0
                }
            },
            None => return 0,
        };
    }
    reversed_int
}


pub fn reverse_int_by_div2(x: i32) -> i32 {

    let mut rem = x;

    let mut reversed_int: i32 = 0;

    while rem != 0 {
        let result = panic::catch_unwind(|| {
            reversed_int * 10 + rem % 10
        });
        if let Err(_) = result {
            return 0;
        } else {
            reversed_int = result.unwrap();
            rem /= 10;
        }
       
    }
    reversed_int
}
