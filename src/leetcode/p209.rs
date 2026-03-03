struct Solution {}

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut res = i32::MAX;
        let mut s = 0;
        let mut l = 0;
        let mut r = 0;

        while r < nums.len() {
            s += nums[r];
            r += 1;

            while l < r && s >= target {
                res = res.min((r - l) as i32);
                s -= nums[l];
                l += 1;
            }
        }

        match res {
            i32::MAX => 0,
            _ => res
        }
    }
}