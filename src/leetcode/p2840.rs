struct Solution;

impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        let (m1, m2) = Self::generate_map(&s1);
        let (m3, m4) = Self::generate_map(&s2);

        for i in 0..26 {
            if m1[i] != m3[i] {
                return false;
            }
            if m2[i] != m4[i] {
                return false;
            }
        }

        true
    }

    fn generate_map(str: &String) -> ([usize; 26], [usize; 26]) {
        let mut m1 = [0; 26];
        let mut m2 = [0; 26];

        let mut ind = 0;
        for ch in str.chars() {
            if ind & 1 == 0 {
                m1[ch as usize - ('a' as usize)] += 1;
            } else {
                m2[ch as usize - ('a' as usize)] += 1;
            }

            ind += 1;
        }

        (m1, m2)
    }
}
