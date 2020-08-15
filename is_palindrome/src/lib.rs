pub fn is_palindrome(x: i32) -> bool {

    let converted_int = x.to_string();
    let my_vec: Vec<char> = converted_int.chars().collect();
    let half = my_vec.len() / 2;


    for (pos, a_char) in my_vec.iter().enumerate() {
        let index_to_test = my_vec.len() - 1 - pos ;

        if a_char != &my_vec[index_to_test] {
            return false;
        }
    }
    true
}

pub fn is_palindrome2(x: i32) -> bool {
    let num_string = x.to_string();
    let half = num_string.len() / 2;

    num_string.bytes().take(half).eq(num_string.bytes().rev().take(half))

}