struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut l = 0;
        let mut r = s.len() - 1;

        while l < r {
            let ch = s[l];
            s[l] = s[r];
            s[r] = ch;
            l += 1;
            r -= 1;
        }
    }
}