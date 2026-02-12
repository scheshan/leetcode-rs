struct Solution {}

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = vec![0usize; 1001];
        for num in nums1 {
            map[num as usize] += 1;
        }

        let mut res = Vec::new();
        for num in nums2 {
            if map[num as usize] > 0 {
                res.push(num);
                map[num as usize] -= 1;
            }
        }

        res
    }
}
