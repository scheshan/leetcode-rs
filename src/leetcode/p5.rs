struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut res = "";

        for i in 0..s.len() {
            let str = Self::longest_palindrome_0(&s, i, i);
            if str.len() > res.len() {
                res = str;
            }

            let str = Self::longest_palindrome_0(&s, i, i + 1);
            if str.len() > res.len() {
                res = str;
            }
        }

        res.to_string()
    }

    fn longest_palindrome_0(s: &String, mut l: usize, mut r: usize) -> &str {
        let mut res = "";
        while l >= 0 && r < s.len() && s.chars().nth(l) == s.chars().nth(r) {
            res = &s[l..=r];
            l -= 1;
            r += 1;
        }
        res
    }
}
