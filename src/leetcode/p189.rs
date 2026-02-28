struct Solution {}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len() as i32;
        let k = k % n;
        Self::reverse(nums, 0, n - 1);
        Self::reverse(nums, 0, k - 1);
        Self::reverse(nums, k, n - 1);
    }

    fn reverse(nums: &mut Vec<i32>, mut l: i32, mut r: i32) {
        while l < r {
            let tmp = nums[l as usize];
            nums[l as usize] = nums[r as usize];
            nums[r as usize] = tmp;

            l += 1;
            r -= 1;
        }
    }
}
