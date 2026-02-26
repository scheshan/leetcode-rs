struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut visit = HashSet::with_capacity(nums.len());
        let mut sum = 0;
        let mut duplicate = 0;

        let total = ((1 + nums.len()) * nums.len()) >> 1;

        for num in nums {
            if visit.contains(&num) {
                duplicate = num;
            } else {
                visit.insert(num);
                sum += num;
            }
        }

        vec![duplicate, total as i32 - sum]
    }
}
