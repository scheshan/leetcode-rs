struct Solution {}

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut res = Vec::new();
        let mut ind = 0;
        for i in 1..=n {
            if i < target[ind] {
                res.push("Push".to_string());
                res.push("Pop".to_string());
            } else {
                res.push("Push".to_string());
                ind += 1;

                if ind >= target.len() {
                    break;
                }
            }
        }

        res
    }
}
