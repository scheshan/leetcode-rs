struct Solution {}

use std::collections::{HashSet};

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut visit = HashSet::new();

        let mut l = 0;
        let mut r = 0;

        while r < nums.len() {
            if r - l > k as usize {
                visit.remove(&nums[l]);
                l += 1;
            }

            if !visit.insert(nums[r]) {
                return true;
            }
            r += 1;
        }
        false
    }
}
