struct Solution {}

impl Solution {
    pub fn replace_spaces(s: String, length: i32) -> String {
        let mut res = String::with_capacity(s.len());
        let space = "%20";
        let mut chars = s.chars();
        for i in 0..length {
            let ch = chars.next().unwrap();
            match ch {
                ' ' => res.push_str(space),
                _ => res.push(ch),
            }
        }

        res
    }
}
