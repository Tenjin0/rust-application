fn longuest_sub<'a>(a: &'a str, b: &'a str) -> Option<&'a str> {
    Option::None
}

fn main() {
    let s = String::from("Hi Patrice nice to meet you");
    let s_split =  s.split(" ");
    let v_split: Vec<&str> = s_split.collect();

    println!("{:?}", v_split);

}
