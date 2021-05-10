/*
[1,1000000000]
[9,3,5]
[1,1,1,2]
[8,5]
*/
use std::collections::BinaryHeap;

impl Solution {
    pub fn is_possible(mut target: Vec<i32>) -> bool {
        let len = target.len() as i32;
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
            total -= current;
            let next = current - total;
            if next < 1 { return false }
            total += next;
            println!("next: {}", next);
            heap.push(next);
        }
        
        // println!("heap: {:?}\n\n", heap);
        
        true
    }
}