// https://leetcode.com/submissions/detail/579071284/
impl Solution {
    pub fn find_buildings(heights: Vec<i32>) -> Vec<i32> {
        let mut max = 0;
        let mut result: Vec<i32> = heights.into_iter().enumerate().rev().map(|(index, height)| {
            // println!("{}, {}", index, height);
            if height > max {
                max = height;
                Some(index as i32)
            } else {
                None
            }
        }).filter_map(|x| x).collect();
        result.reverse();
        result
    }
}