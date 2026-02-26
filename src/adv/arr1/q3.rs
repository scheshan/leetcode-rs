struct Solution {}

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut cur = 0;

        for num in nums {
            if num == 1 {
                cur += 1;
                res = res.max(cur);
            } else {
                cur = 0;
            }
        }

        res
    }
}
