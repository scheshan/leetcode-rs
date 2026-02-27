struct Solution {}

use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::with_capacity(stones.len());
        for stone in stones {
            heap.push(stone);
        }

        while heap.len() > 1 {
            let x = heap.pop().unwrap();
            let y = heap.pop().unwrap();
            heap.push(x - y);
        }

        heap.pop().unwrap()
    }
}