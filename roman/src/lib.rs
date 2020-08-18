use std::collections::HashMap;

pub fn to_int(roman: String) -> String {
    let table : HashMap<char, i32>= [
            ('M', 1000),
            ('D', 500),
            ('C', 100),
            ('L', 50),
            ('X', 10),
            ('V', 5),
            ('I', 1)
    ].iter().cloned().collect();

    let mut sum = 0;

    let mut roman_chars_iter = roman.chars().peekable();
    while let Some(letter) = roman_chars_iter.next() {
        let translate = table.get(&letter).unwrap();
        let mut to_add: i32 = *translate;
        if letter == 'I' || letter == 'X' || letter == 'C' {
            if let Some(next_letter) = roman_chars_iter.peek() {
                let translate_next_letter = table.get(&next_letter).unwrap();
                println!("{} {} {} {}", translate_next_letter, translate, translate_next_letter > translate, translate_next_letter / translate);
                if translate_next_letter > translate && translate_next_letter / translate  <= 10 {
                    to_add = translate_next_letter - translate;
                    roman_chars_iter.next();
                }
            }
        }
        sum += to_add;
    }
    sum.to_string()
}
