fn sum_digits(num: i32) -> i32 {
    let mut sum = 0;
    let mut copy = num;
    while copy != 0 {
        let digit = copy % 10;
        copy = copy / 10;
        sum += digit;
    }
    sum
}

pub fn add_digits(num: i32) -> i32 {
    let mut copy = num;
    if copy < 10 {
        return copy
    }
    while copy >= 10 {
        copy = sum_digits(copy);
    }
    copy
    
}

pub fn add_digits_o1(num: i32) -> i32 {
    if num == 0 { 
        return 0; 
    }
    return (num - 1) % 9 + 1
    
}
