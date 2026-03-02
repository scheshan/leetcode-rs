struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        for i in 0..nums.len() - 1 {
            if i == 0 {
                res[i] = nums[i];
            } else {
                res[i] = nums[i] * res[i - 1];
            }
        }

        let mut suffix = 1;
        res[nums.len() - 1] = suffix;
        for i in (0..=nums.len() - 1).rev() {
            if i > 0 {
                res[i] = res[i - 1] * suffix;
                suffix *= nums[i];
            } else {
                res[i] = suffix;
            }
        }

        res
    }
}
