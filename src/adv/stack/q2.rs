struct Solution {}

use std::str::FromStr;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut deque = vec![0; tokens.len()];
        let mut ind = 0;

        for str in tokens {
            match str.as_str() {
                "+" => {
                    deque[ind - 2] = deque[ind - 2] + deque[ind - 1];
                    ind -= 1;
                }
                "-" => {
                    deque[ind - 2] = deque[ind - 2] - deque[ind - 1];
                    ind -= 1;
                }
                "*" => {
                    deque[ind - 2] = deque[ind - 2] * deque[ind - 1];
                    ind -= 1;
                }
                "/" => {
                    deque[ind - 2] = deque[ind - 2] / deque[ind - 1];
                    ind -= 1;
                }
                _ => {
                    deque[ind] = i32::from_str(str.as_str()).unwrap();
                    ind += 1;
                }
            }
        }

        deque[0]
    }
}
