struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();
        let mut m = 0;

        while l < r {
            m = l + ((r - l) >> 1);

            if nums[m] < target {
                l = m + 1;
            } else if nums[m] > target {
                r = m;
            } else {
                break;
            }
        }

        if nums[m] >= target {
            return m as i32;
        } else {
            return (m + 1) as i32;
        }
    }
}
