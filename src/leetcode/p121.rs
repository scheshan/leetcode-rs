struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut lowest = prices[0];
        let mut res = 0;

        for i in 1..prices.len() {
            res = res.max(prices[i] - lowest);
            lowest = lowest.min(prices[i]);
        }

        res
    }
}