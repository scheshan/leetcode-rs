struct Solution {}

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len() << 1];

        for i in 0..nums.len() {
            res[i] = nums[i];
            res[i + nums.len()] = nums[i];
        }

        res
    }
}
