struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut slow = 0;
        let mut fast = 0;

        while fast < nums.len() {
            nums[slow] = nums[fast];
            fast += 1;
            if slow > 0 && nums[slow] == nums[slow - 1] {
                while fast < nums.len() && nums[fast] == nums[fast - 1] {
                    fast += 1;
                }
            }
            slow += 1;
        }
        slow as i32
    }
}
