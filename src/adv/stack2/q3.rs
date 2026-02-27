struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut left = vec![0; heights.len()];
        let mut right = vec![0; heights.len()];

        let mut deque = Vec::with_capacity(heights.len());
        for i in 0..heights.len() {
            while !deque.is_empty() && heights[*deque.last().unwrap()] >= heights[i] {
                deque.pop();
            }
            if !deque.is_empty() {
                left[i] = *deque.last().unwrap() as i32;
            } else {
                left[i] = -1;
            }
            deque.push(i);
        }

        deque.clear();
        for i in (0..=heights.len() - 1).rev() {
            while !deque.is_empty() && heights[*deque.last().unwrap()] >= heights[i] {
                deque.pop();
            }
            if !deque.is_empty() {
                right[i] = *deque.last().unwrap() as i32;
            } else {
                right[i] = heights.len() as i32;
            }
            deque.push(i);
        }

        let mut res = 0;
        for i in 0..heights.len() {
            res = res.max(heights[i] * (right[i] - left[i] - 1));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        Solution::largest_rectangle_area(vec![2, 0, 2]);
    }
}
