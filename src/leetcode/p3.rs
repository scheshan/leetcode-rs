struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut visit: HashSet<char> = HashSet::new();
        let arr: Vec<char> = s.chars().collect();
        let mut l = 0;
        let mut r = 0;
        let mut res = 0;

        while r < arr.len() {
            match visit.contains(&arr[r]) {
                true => {
                    visit.remove(&arr[l]);
                    l += 1;
                }
                false => {
                    visit.insert(arr[r]);
                    r += 1;
                    res = res.max(r - l);
                }
            }
        }

        res as i32
    }
}
