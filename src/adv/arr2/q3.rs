struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut visit = HashSet::new();

        for num in &nums {
            visit.insert(*num);
        }

        let mut res = Vec::with_capacity(nums.len() - visit.len());
        for i in 1..=nums.len() as i32 {
            if !visit.contains(&i) {
                res.push(i);
            } else {
                visit.insert(i);
            }
        }

        res
    }
}
