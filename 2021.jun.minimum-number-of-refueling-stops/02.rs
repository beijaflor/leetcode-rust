// https://leetcode.com/submissions/detail/507057832/
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut queue: BinaryHeap<i32> = BinaryHeap::new();
        let mut result = 0;
        let mut prev = 0;
        let mut tank = start_fuel;
        for station in stations.into_iter() {
            let location = station[0];
            let capacity = station[1];
            tank -= location - prev;
            while !queue.is_empty() && tank < 0 {
                tank += queue.pop().unwrap();
                result += 1;
            }
            
            if tank < 0 { return -1 }
            queue.push(capacity);
            prev = location;
        };
        
        {
            tank -= target - prev;
            while !queue.is_empty() && tank < 0 {
                tank += queue.pop().unwrap();
                result += 1;
            }
            
            if tank < 0 { return -1 }
        }
        
        result
    }
}