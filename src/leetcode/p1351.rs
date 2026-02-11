struct Solution {}

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut prev = grid[0].len();

        for i in 0..grid.len() {
            let mut l = 0;
            let mut r = prev;

            while l < r {
                let mid = l + ((r - l) >> 1);
                if grid[i][mid] >= 0 {
                    l = mid + 1;
                } else {
                    r = mid;
                    prev = r;
                }
            }

            res += (grid[0].len() - prev) as i32;
        }

        res
    }
}
