struct Solution {}

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut one = 0;
        let mut zero = 0;
        for student in students {
            match student {
                1 => one += 1,
                _ => zero += 1,
            }
        }

        for sandwich in sandwiches {
            if sandwich == 0 {
                if zero == 0 {
                    return one;
                } else {
                    zero -= 1;
                }
            } else {
                if one == 0 {
                    return zero;
                } else {
                    one -= 1;
                }
            }
        }

        0
    }
}
