struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();

        while l < r {
            let mid = l + ((r - l) >> 1);
            let mid_num = nums[mid];

            if mid_num < target {
                l = mid + 1;
            } else if mid_num > target {
                r = mid;
            } else {
                return mid as i32;
            }
        }

        return -1;
    }
}
