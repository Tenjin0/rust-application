pub fn is_valid(string: &String) -> bool {

    let mut opened : Vec<u8> = Vec::new();

    for &char in string.as_bytes() {
        let &last = opened.last().unwrap_or(&b' ');
        match char {

            b'(' | b'[' | b'{' =>  {
                opened.push(char)
            },
            b')' => {
                if last != b'('  {
                    return false
                }
                opened.pop();
            },
            b']' => {
                if last != b'[' {
                    return false
                }
                opened.pop();
            },
            b'}' => {
                if last != b'{' {
                    return false
                }
                opened.pop();
            },
            _ => {

            }
        };
    }
    return opened.len() == 0;
}
