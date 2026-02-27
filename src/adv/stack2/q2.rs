struct Solution {}

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut deque: Vec<usize> = Vec::new();
        let mut res = vec![0; temperatures.len()];

        for i in (0..=temperatures.len() - 1).rev() {
            while !deque.is_empty() && temperatures[*deque.last().unwrap()] <= temperatures[i] {
                deque.pop();
            }

            if !deque.is_empty() {
                res[i] = (*deque.last().unwrap() - i) as i32;
            } else {
                res[i] = 0;
            }
            deque.push(i);
        }

        res
    }
}
