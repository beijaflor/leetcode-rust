/**
[[1,5,3],[2,9,4]]
[[1],[2],[3]]
[[10,15,12,14,18,5],[5,12,18,13,15,8],[4,7,4,2,10,18],[20,9,9,19,20,5],[10,15,10,15,16,20],[9,6,11,10,12,11],[7,10,6,12,20,8],[3,4,4,18,10,2]]
 */
impl Solution {
    pub fn min_cost_ii(costs: Vec<Vec<i32>>) -> i32 {
        let N = costs.len();
        if N == 1 { return *costs[0].iter().min().unwrap() }

        let N2 = costs[0].len();
        if N2 == 1 { return i32::MAX }
        
        let mut iter = costs.into_iter();
        let mut starting = iter.next().unwrap();
        iter.fold(starting, |acc, mut current| {
            let mut sorted = current.clone();
            sorted.sort();
            let first_min = sorted[0];
            let next_min = sorted[1];
            (0..current.len()).map(|index| {
                let min = if current[index] == first_min {
                    next_min
                } else {
                    first_min
                };
                acc[index] + min
            }).collect::<Vec<i32>>()
        }).into_iter().min().unwrap()
    }
}