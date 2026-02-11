struct Solution {}

use std::collections::{HashMap};

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            let d = target - nums[i];
            match map.get(&d) {
                Some(j) => {
                    return vec![*j as i32, i as i32];
                }
                None => {
                    map.insert(nums[i], i);
                }
            }
        }

        return vec![-1, -1];
    }
}
