use longuest_common_prefix::Common;

fn main() {

    let vec_str: Vec<String> = vec![String::from("flower"),String::from("flow"),String::from("flight")];
    let common = Common {
        strs: &vec_str
    };


    let result = common.find();
    println!("{:?} common: {}", vec_str, result)
}
