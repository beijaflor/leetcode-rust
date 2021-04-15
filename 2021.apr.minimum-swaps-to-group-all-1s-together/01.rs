// https://leetcode.com/submissions/detail/480962761/
impl Solution {
    pub fn min_swaps(data: Vec<i32>) -> i32 {
        let count = data.iter().fold(0, |acc, cur| acc + cur );
        // println!("count: {}", count);

        let mut zeros = 0;
        for offset in 0..(count as usize) {
            if data[offset] == 0 {
                zeros += 1;
            }
        }
        // println!("count: {}, zeros: {}", count, zeros);
        let mut min = zeros;

        for index in 0..(data.len() - count as usize) {
            zeros -= if data[index] == 0 { 1 } else { 0 };
            zeros += if data[index + count as usize] == 0 { 1 } else { 0 };
            min = i32::min(min, zeros);
        }
        min
    }
}