use add_binary;

fn main() {

    let a = "110";
    let b = "11";
    println!("{:?}", add_binary::add_radix(a.to_string(), b.to_string()));
}
/*
11
 110 
 011
1001
*/
