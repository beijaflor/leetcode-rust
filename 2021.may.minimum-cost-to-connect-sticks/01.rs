// https://leetcode.com/submissions/detail/500101834/
use std::collections::BinaryHeap;

impl Solution {
    pub fn connect_sticks(mut sticks: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        sticks.into_iter().for_each(|s| heap.push(-s));
        // println!("heap {:?}", heap);

        let mut result = 0;

        while heap.len() > 1 {
            let a = heap.pop().unwrap();
            let b = heap.pop().unwrap();
            let stick = (-a + -b);
            result += stick;
            heap.push(-stick);
        }
        
        result
    }
}