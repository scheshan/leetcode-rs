use std::collections::{HashSet};

struct Solution {}

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut s1 = HashSet::new();
        for num in nums1 {
            s1.insert(num);
        }

        let mut s2 = HashSet::new();
        for num in nums2 {
            if s1.contains(&num) {
                s2.insert(num);
            }
        }

        let mut res = Vec::with_capacity(s2.len());
        for num in s2 {
            res.push(num);
        }

        res
    }
}
