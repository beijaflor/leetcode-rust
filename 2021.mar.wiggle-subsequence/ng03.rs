use std::cmp::Ordering::{ Less, Greater, Equal};

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut up = 1;
        let mut down = 1;
        let mut iterator = nums.iter();
        let mut last = iterator.next().unwrap();
        iterator.for_each(|v| {
            match last.cmp(v) {
                Equal => (),
                Greater => down = up + 1,
                Less => up = down + 1,
            }
        });
        i32::max(up, down)
    }
}