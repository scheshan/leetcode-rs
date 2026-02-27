struct Solution {}

use std::str::FromStr;

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut res = vec![0; n as usize];
        let mut deque: Vec<(i32, i32)> = Vec::new();

        for log in logs {
            let arr: Vec<&str> = log.split(":").collect();
            let id = i32::from_str(arr[0]).unwrap();
            let op = arr[1];
            let time = i32::from_str(arr[2]).unwrap();
            match op {
                "start" => {
                    if !deque.is_empty() {
                        let pre = deque.last_mut().unwrap();
                        res[pre.0 as usize] += time - pre.1;
                        pre.1 = time;
                    }
                    deque.push((id, time));
                }
                _ => {
                    let log = deque.pop().unwrap();
                    res[id as usize] += time - log.1 + 1;
                    if !deque.is_empty() {
                        let pre = deque.last_mut().unwrap();
                        pre.1 = time + 1;
                    }
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        Solution::exclusive_time(
            2,
            vec![
                "0:start:0".to_string(),
                "1:start:2".to_string(),
                "1:end:5".to_string(),
                "0:end:6".to_string(),
            ],
        );
    }
}
