struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut visit = HashSet::new();
        visit.insert(0);

        let mut n = n;
        while n != 0 {
            let m = Self::calculate(n);
            if m == 1 {
                return true;
            }
            if visit.contains(&m) {
                return false;
            }
            visit.insert(m);
            n = m;
        }

        false
    }

    fn calculate(n: i32) -> i32 {
        let mut res = 0;
        if n == 0 {
            return res;
        }

        let str = n.to_string();
        for ch in str.chars() {
            let m = (ch as i32 - '0' as i32).pow(2);
            res += m;
        }

        res
    }
}
