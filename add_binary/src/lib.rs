use std::cmp;

pub fn add(a: String, b: String) -> String {

    let max = cmp::max(a.len(), b.len());
    let rev_a: Vec<char> = format!("{:0>width$}", a, width=max).chars().rev().collect();
    let rev_b: Vec<char> = format!("{:0>width$}", b, width=max).chars().rev().collect();
    let mut res = String::from("");

    let mut carry = false;
    for i in 0..max {
        let add_a = rev_a[i];
        let add_b = rev_b[i];
        let to_add;
        if add_a == '0' && add_b == '0' {
            to_add = if carry {"1"} else { "0" };
            carry = false;
        } else if (add_a == '1' && add_b == '0') || (add_a == '0' && add_b == '1') {
            to_add = if carry {"0"} else { "1" };
            carry = to_add == "0";
        } else {
            to_add = if carry {"1"} else { "0" };
            carry = true;
        }
        res = String::from(to_add) + &res;
    }

    if carry {
        res = String::from("1") + &res;
    }
    res
}

pub fn add_radix(a: String, b: String) -> String {
    let aa = i128::from_str_radix(&a, 2).unwrap_or(0);
    let bb = i128::from_str_radix(&b, 2).unwrap_or(0);
    println!("{:b}", aa);
    format!("{:b}",  aa + bb).to_string()
}
