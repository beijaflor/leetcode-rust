// https://leetcode.com/submissions/detail/483892640/
use std::collections::HashMap;

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let width: i32 = wall[0].iter().sum();
        let height = wall.len() as i32;
        let mut gaps: HashMap<i32, i32> = HashMap::new();
        wall.into_iter().for_each(|row| {
            let mut index = 0;
            row.into_iter().for_each(|brick| {
                index += brick;
                if index != width {
                    gaps.entry(index).and_modify(|c| *c += 1).or_insert(1);
                }
            });
        });
        // println!("{:?}", gaps);
        height - gaps.into_iter().fold(0, |max, (_, current)| i32::max(max, current))
    }
}