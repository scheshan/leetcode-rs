struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut n = 0;
        let mut vote = 0;

        for num in nums {
            if vote == 0 {
                n = num;
                vote += 1;
            } else if n == num {
                vote += 1;
            } else {
                vote -= 1;
            }
        }

        n
    }
}