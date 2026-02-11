struct Solution {}

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut l = 0;
        let mut r = letters.len();
        let mut res = 0;

        while l < r {
            let mid = l + ((r - l) >> 1);
            if letters[mid] <= target {
                l = mid + 1;
            } else {
                res = mid;
                r = mid;
            }
        }

        letters[res]
    }
}
