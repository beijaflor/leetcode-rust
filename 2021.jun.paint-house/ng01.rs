/**
[[3,5,3],[6,17,6],[7,13,18],[9,10,18]]
*/
impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        costs.into_iter().fold(0, |acc, paint| {
            acc + paint.iter().min().unwrap()
        })
    }
}