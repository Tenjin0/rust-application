pub struct Common<'a> {
    pub strs: &'a Vec<String>,
}

impl<'a> Common<'a> {
    pub fn find(&'a self) -> String {
        if self.strs.len() == 0  {
            return String::from("");
        } else if self.strs.len() == 1 {
            return self.strs[0].clone();
        }
        let first_string = &self.strs[0];
        for j in (0..first_string.len()).rev() {
            let search_sub = &first_string[0..=j];
            let mut found = false;
            'inner: for k in 1..self.strs.len() {
                let concerned_string: &str = self.strs[k].as_str();

                found = concerned_string.starts_with(search_sub);

                if !found {
                    break 'inner;
                } 
            }
            if found {
                return String::from(search_sub);
            }
        }
        return String::from("");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
