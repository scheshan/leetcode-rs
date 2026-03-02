struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut res = 0;
        let mut chars = s.chars().rev();
        let mut state = 0;
        loop {
            if let Some(ch) = chars.next() {
                match ch {
                    'a'..='z' | 'A'..='Z' => {
                        if state == 0 {
                            state = 1;
                        }
                        if state == 1 {
                            res += 1;
                        }
                    }
                    _ => {
                        if state == 1 {
                            break;
                        }
                    }
                }
            } else {
                break;
            }
        }
        res
    }
}
