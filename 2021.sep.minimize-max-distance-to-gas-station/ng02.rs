use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Total<T>(pub T);

impl<T: PartialEq> Eq for Total<T> {}

impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Total<T>) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl Solution {
    pub fn minmax_gas_dist(stations: Vec<i32>, k: i32) -> f64 {
        let mut gaps: BinaryHeap<Total<f64>> = BinaryHeap::new();
        (1..stations.len()).for_each(|index| {
            gaps.push(Total((stations[index] - stations[index - 1]) as f64));
        });
        
        println!("{:?}", gaps);

        (0..k).for_each(|_| {
            let new_gap = (gaps.pop().unwrap()).0 / 2f64;
            gaps.push(Total(new_gap));
        });

        println!("{:?}", gaps);
        
        gaps.pop().unwrap().0
    }
}