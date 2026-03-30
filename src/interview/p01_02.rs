struct Solution {}

impl Solution {
    pub fn check_permutation(s1: String, s2: String) -> bool {
        let m1 = Self::generate_map(&s1);
        let m2 = Self::generate_map(&s2);

        for i in 0..26 {
            if m1[i] != m2[i] {
                return false;
            }
        }
        true
    }

    fn generate_map(s: &String) -> [usize; 26] {
        let mut m = [0usize; 26];

        for ch in s.chars() {
            m[ch as usize - ('a' as usize)] += 1;
        }

        m
    }
}
