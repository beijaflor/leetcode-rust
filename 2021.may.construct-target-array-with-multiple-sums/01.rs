// https://leetcode.com/submissions/detail/491121123/
use std::collections::BinaryHeap;

impl Solution {
    pub fn is_possible(mut target: Vec<i32>) -> bool {
        // println!("\n\n");

        let len = target.len() as i32;
        
        if len == 1 { return target[0] == 1 }

        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut total = 0;
        target.into_iter().for_each(|v| {
            total += v;
            heap.push(v)
        });

        // println!("heap: {:?}", heap);
        // println!("total: {:?}", total);
        
        while let Some(current) = heap.pop() {
            if current == 1 { break }

            let rest = total - current;
            // println!("rest: {}", rest);

            if rest == 1 { return true }
            let next = current % rest;
            
            // println!("next: {}", next);

            if next == 0 || next == current {
                return false
            }
            total = total - current + next;
            
            // println!("total: {}", total);
            heap.push(next);
        }
        
        // println!("heap: {:?}\n\n", heap);
        
        true
    }
}