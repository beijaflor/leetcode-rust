/**
[1,2,3]
[1,10,2,9]
[1,0,0,8,6]
*/
impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let total: i32 = nums.iter().sum();
        let average = total / nums.len() as i32;
        nums.iter().fold(0, |acc, num| {
            acc + (average - num).abs()
        })
    }
}