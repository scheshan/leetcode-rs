struct Solution {}

impl Solution {
    pub fn one_edit_away(first: String, second: String) -> bool {
        if (first.len() as i32 - (second.len() as i32)).abs() >= 2 {
            return false;
        }

        let arr1: Vec<char> = first.chars().collect();
        let arr2: Vec<char> = second.chars().collect();

        let mut i1 = 0;
        let mut i2 = 0;

        while i1 < arr1.len() && i2 < arr2.len() && arr1[i1] == arr2[i2] {
            i1 += 1;
            i2 += 1;
        }

        Self::array_equal(&arr1, i1 + 1, &arr2, i2)
            || Self::array_equal(&arr1, i1, &arr2, i2 + 1)
            || Self::array_equal(&arr1, i1 + 1, &arr2, i2 + 1)
    }

    fn array_equal(a1: &Vec<char>, mut i1: usize, a2: &Vec<char>, mut i2: usize) -> bool {
        if a1.len() - i1 != a2.len() - i2 {
            return false;
        }

        while i1 < a1.len() {
            if a1[i1] != a2[i2] {
                return false;
            }

            i1 += 1;
            i2 += 1;
        }

        true
    }
}
