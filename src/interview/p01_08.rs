struct Solution {}

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut rows = vec![false; matrix.len()];
        let mut cols = vec![false; matrix[0].len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 0 {
                    rows[i] = true;
                    cols[j] = true;
                }
            }
        }

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if rows[i] || cols[j] {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}
