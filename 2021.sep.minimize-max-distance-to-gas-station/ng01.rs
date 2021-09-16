/*
[10,19,25,27,56,63,70,87,96,97]
3
[1,2,3,4,5,6,7,8,9,10]
9
[23,24,36,39,46,56,57,65,84,98]
1
[23,24,36,39,46,56,57,65,98,99]
1
[1,2,3,4,5,6,7,8,9,10]
1
[1,2,3,4,5,6,7,8,9,99]
1
*/
use std::collections::BinaryHeap;

impl Solution {
    pub fn minmax_gas_dist(stations: Vec<i32>, k: i32) -> f64 {
        let mut gaps: BinaryHeap<i32> = BinaryHeap::new();
        (1..stations.len()).for_each(|index| {
            gaps.push(stations[index] - stations[index - 1])
        });
        
        println!("{:?}", gaps);
        
        let gap1 = gaps.pop().unwrap() as f64 / 2f64;
        
        println!("{}", gap1);

        (1..k).for_each(|_| {
            gaps.pop();
        });

        println!("{:?}", gaps);
        
        let gap2 = gaps.pop().unwrap_or(0) as f64;

        f64::max(gap1, gap2)
    }
}