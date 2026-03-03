struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let arr1: Vec<char> = s.chars().collect();
        let arr2: Vec<char> = t.chars().collect();
        let mut i = 0;
        let mut j = 0;

        while i < arr1.len() && j < arr2.len() {
            if arr1[i] == arr2[j] {
                i += 1;
                j += 1;
            } else {
                j += 1;
            }
        }

        i == arr1.len()
    }
}
