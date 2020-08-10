fn main() {
    let s1 = String::from("azertyuiopqsdfghjklm");
    println!("{:?}", s1.chars());

    // for (i, c) in s1.chars().enumerate() {
    //     println!("i: {}, char: {}", i, c)

    // }

    let mut i = 0;
    let mut j = 0;
    let max = s1.len();
    let chars = s1.chars();
    while i < max {
        j = max;
        while i < j {
            print!("{:?}", (&s1[i..j]));
            j -= 1;
        }
        println!();
        i += 1;
    }
}
