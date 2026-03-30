struct Solution {}

impl Solution {
    pub fn is_unique(astr: String) -> bool {
        let mut m = vec![false; 26];

        for ch in astr.chars() {
            let ind = ch as usize - ('a' as usize);
            if m[ind] {
                return false;
            }
            m[ind] = true;
        }

        true
    }
}
