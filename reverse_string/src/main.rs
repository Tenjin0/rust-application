use reverse_string::reverse_string;

fn main() {

    let mut chars: Vec<char> = String::from("hello").chars().collect();
    reverse_string(&mut chars);

    println!("{:?}", chars);
}
