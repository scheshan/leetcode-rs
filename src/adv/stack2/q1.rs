struct Solution {}

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut deque = Vec::new();
        let mut res = vec![0; prices.len()];

        for i in (0..=prices.len() - 1).rev() {
            while !deque.is_empty() && prices[*deque.last().unwrap()] > prices[i] {
                deque.pop();
            }

            if !deque.is_empty() {
                res[i] = prices[i] - prices[*deque.last().unwrap()];
            } else {
                res[i] = prices[i];
            }
            deque.push(i);
        }

        res
    }
}
