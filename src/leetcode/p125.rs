struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chs: Vec<char> = s.to_lowercase().chars().collect();

        let mut l = 0;
        let mut r = s.len() - 1;

        while l < r {
            if !chs[l].is_alphanumeric() {
                l += 1;
                continue;
            }
            if !chs[r].is_alphanumeric() {
                r -= 1;
                continue;
            }

            if chs[l] != chs[r] {
                return false;
            }

            l += 1;
            r -= 1;
        }

        true
    }
}