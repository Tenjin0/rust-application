pub fn is_perfect_square(num: i32) -> bool {
    return is_perfect_square_it(num);
}

pub fn is_perfect_square_rec(num: i32, start: i32, end: i32) -> bool {
    if num == 0 || num == 1 {
        return true;
    }
    if start > end {
        return false;
    }
    let mid: i32 = (end - start) / 2 + start;
    println!("{} {} {}", mid, start, end);

    let (possible_square, overflowed) = (mid as i64).overflowing_mul(mid as i64);

    if overflowed {
        return is_perfect_square_rec(num, start, mid - 1);
    } else if possible_square == num as i64 {
        return true;
    } else if possible_square > num as i64 {
        return is_perfect_square_rec(num, start, mid - 1);
    } else {
        return is_perfect_square_rec(num, mid + 1, end);
    }
}

pub fn is_perfect_square_it(num: i32) -> bool {
    let mut start  = 1;
    let mut end = num;

    while start <= end {
        let mid: i32 = (end - start) / 2 + start;
        println!("{} {} {}", mid, start, end);
        let (possible_square, overflowed) = (mid as i64).overflowing_mul(mid as i64);

        if overflowed {
            end = mid - 1;
        } else if possible_square == num as i64 {
            return true;
        } else if possible_square > num as i64 {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }
    return false;
}
