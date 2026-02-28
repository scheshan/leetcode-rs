struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut state = (i32::MIN, 0);
        let mut res = 0;

        for price in prices {
            state = (state.0.max(state.1 - price), state.1.max(state.0 + price));
            res = res.max(state.1);
        }

        res
    }
}