struct Solution {}

use std::collections::HashSet;

impl Solution {

    pub fn can_permute_palindrome(s: String) -> bool {
        let mut words = 0;
        let mut map: HashSet<char> = HashSet::new();

        for ch in s.chars() {
            match map.contains(&ch) {
                true => {
                    map.remove(&ch);
                    words -= 1;
                }
                false => {
                    map.insert(ch);
                    words += 1;
                }
            }
        }

        words < 2
    }
}
