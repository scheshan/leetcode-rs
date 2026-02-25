struct Solution {}

impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut cnt = Vec::with_capacity(arr.len());
        let mut ind = Vec::with_capacity(arr.len());
        for i in 0..arr.len() {
            ind.push(i);
            cnt.push(Self::count_one(arr[i]));
        }

        ind.sort_by(|l, r| {
            let lc = cnt[*l];
            let ln = arr[*l];
            let rc = cnt[*r];
            let rn = arr[*r];
            lc.cmp(&rc).then(ln.cmp(&rn))
        });

        let mut res = Vec::with_capacity(arr.len());
        for i in 0..arr.len() {
            res.push(arr[ind[i]]);
        }

        res
    }

    fn count_one(mut num: i32) -> i32 {
        let mut res = 0;

        while num != 0 {
            num = num & (num - 1);
            res += 1;
        }

        res
    }
}
