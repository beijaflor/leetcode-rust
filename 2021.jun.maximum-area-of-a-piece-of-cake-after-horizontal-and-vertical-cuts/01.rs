// https://leetcode.com/submissions/detail/502319626/
impl Solution {
    pub fn max_area(h: i32, w: i32, mut horizontal_cuts: Vec<i32>, mut vertical_cuts: Vec<i32>) -> i32 {
        horizontal_cuts.sort();
        horizontal_cuts.push(h);
        let max_h = horizontal_cuts.into_iter().fold((0, 0), |(last, max), current| {
            (current, i32::max(max, current - last))
        });
        vertical_cuts.sort();
        vertical_cuts.push(w);
        let max_v = vertical_cuts.into_iter().fold((0, 0), |(last, max), current| {
            (current, i32::max(max, current - last))
        });
        println!("{:?}, {:?}", max_h, max_v);
        ((max_h.1 as i64 * max_v.1 as i64) % 1_000_000_007) as i32
    }
}