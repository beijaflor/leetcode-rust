// https://leetcode.com/submissions/detail/457410393/
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut lp = 0;
        let mut rp = height.len() - 1;
        while lp < rp {
            result = i32::max(result, i32::min(height[lp], height[rp]) * (rp - lp) as i32);
            if height[lp] < height[rp] {
                lp += 1;
            } else {
                rp -= 1;
            }
        }
        result
    }
}