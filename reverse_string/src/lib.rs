pub fn reverse_string(s: &mut Vec<char>) {
    let half = s.len() / 2;
    for i in 0..half {
        let oppose_indice = s.len() - i - 1;
        let tmp = s[oppose_indice];
        s[oppose_indice] = s[i];
        s[i] = tmp;
    }
}
