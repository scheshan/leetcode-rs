struct Solution {}

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut cnt_map = vec![0; 101];
        for num in &nums {
            cnt_map[*num as usize] += 1;
        }

        for i in 1..cnt_map.len() {
            cnt_map[i] += cnt_map[i - 1];
        }

        let mut res = Vec::with_capacity(nums.len());
        for num in nums {
            if num == 0 {
                res.push(0);
            } else {
                res.push(cnt_map[num as usize - 1]);
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
        Solution::smaller_numbers_than_current(vec![5, 0, 10, 0, 10, 6]);
    }
}
