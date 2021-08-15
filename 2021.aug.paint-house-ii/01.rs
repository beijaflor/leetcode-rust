// https://leetcode.com/submissions/detail/538908002/
impl Solution {
    pub fn min_cost_ii(costs: Vec<Vec<i32>>) -> i32 {
        let N = costs.len();
        
        if N == 1 { return *costs[0].iter().min().unwrap() }
        
        costs.into_iter().fold(vec![0; N], |acc, current| {
            (0..current.len()).map(|index| {
                current[index] + acc.iter().enumerate().fold(i32::MAX, |acc, (j, target)| {
                    if index == j {
                        acc
                    } else {
                        i32::min(acc, *target)
                    }
                })
            }).collect::<Vec<i32>>()
        }).into_iter().min().unwrap()
    }
}