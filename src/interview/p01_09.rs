struct Solution {}

impl Solution {
    pub fn is_fliped_string(s1: String, s2: String) -> bool {
        match s2.len() {
            0 => s1.len() == 0,
            _ => {
                let s = format!("{}{}", &s1, &s1);
                s.contains(&s2)
            }
        }
    }
}
