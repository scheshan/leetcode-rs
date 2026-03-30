struct Solution {}

impl Solution {
    pub fn compress_string(s: String) -> String {
        let mut res = String::with_capacity(s.len());

        let mut pre = ' ';
        let mut cnt = 0;

        for ch in s.chars() {
            if ch == pre {
                cnt += 1;
            } else {
                if pre != ' ' {
                    res.push(pre);
                    res.push_str(format!("{}", cnt).as_str());
                }
                pre = ch;
                cnt = 1;
            }
        }

        res.push(pre);
        res.push_str(format!("{}", cnt).as_str());

        if res.len() >= s.len() { s } else { res }
    }
}
