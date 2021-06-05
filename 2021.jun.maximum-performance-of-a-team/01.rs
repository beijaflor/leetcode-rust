// https://leetcode.com/submissions/detail/503442810/
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut engineers = speed.into_iter().zip(efficiency.into_iter()).map(|(speed, efficiency)| (efficiency, speed)).collect::<Vec<(i32, i32)>>();
        let mut output = 0;
        engineers.sort_by(|lhv, rhv| rhv.0.cmp(&lhv.0));
        // engineers.into_iter().for_each(|(speed, efficiency)| {
        //     println!("speed: {}, efficiency: {}", speed, efficiency);
        // });
        
        let mut speedHeap: BinaryHeap<i32> = BinaryHeap::new();
        
        let mut sum: i128 = 0;
        let mut perf: i128 = 0;
        
        for (efficiency, speed) in engineers.into_iter() {
            if speedHeap.len() > k - 1 {
                sum += speedHeap.pop().unwrap() as i128;
            }
            speedHeap.push(-speed);
            
            sum += speed as i128;
            perf = i128::max(perf, sum * efficiency as i128);
        }
        (perf % 1_000_000_007) as i32
    }
}