struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut l = m as usize;
        let mut r = n as usize;
        let mut ind = nums1.len();

        while ind > 0 {
            if l > 0 && r > 0 {
                if nums1[l - 1] < nums2[r - 1] {
                    nums1[ind - 1] = nums2[r - 1];
                    r -= 1;
                } else {
                    nums1[ind - 1] = nums1[l - 1];
                    l -= 1;
                }
            } else if l > 0 {
                nums1[ind - 1] = nums1[l - 1];
                l -= 1;
            } else {
                nums1[ind - 1] = nums2[r - 1];
                r -= 1;
            }
            ind -= 1;
        }
    }
}