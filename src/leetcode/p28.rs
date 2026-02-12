struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() > haystack.len() {
            return -1;
        }

        let mut slow = 0;
        while slow <= haystack.len() - needle.len() {
            let mut find = true;
            for i in 0..needle.len() {
                if haystack.chars().nth(slow + i) != needle.chars().nth(i) {
                    find = false;
                    break;
                }
            }

            if find {
                return slow as i32;
            }

            slow += 1;
        }

        -1
    }
}
