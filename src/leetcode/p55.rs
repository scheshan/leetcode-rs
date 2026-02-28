struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max = nums[0];

        for i in 0..nums.len() {
            if max < i as i32 {
                return false;
            }
            if max >= (nums.len() - 1) as i32 {
                return true;
            }

            max = max.max(nums[i] + i as i32);
        }

        false
    }
}
