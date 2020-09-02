use check_parentheses;

fn main() {
    let s = String::from("()");
    println!("{}: {}", s,  check_parentheses::is_valid(&s));

    let s = String::from("()[]{}");
    println!("{}: {}", s,  check_parentheses::is_valid(&s));

    let s = String::from("(]");
    println!("{}: {}", s,  check_parentheses::is_valid(&s));

    let s = String::from("([)]");
    println!("{}: {}", s,  check_parentheses::is_valid(&s));

    let s = String::from("{[]}");
    println!("{}: {}", s,  check_parentheses::is_valid(&s));
}
