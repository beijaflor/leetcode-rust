// https://leetcode.com/submissions/detail/536550151/
impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let zeros = s.chars().filter(|&c| c == '0').count() as i32;
        let ones = s.chars().filter(|&c| c == '1').count() as i32;

        let mut chars: Vec<char> = s.chars().collect();
        let mut count = 0;
        let mut dp: Vec<i32> = (0..chars.len()).rev().map(|index| {
            if chars[index] == '0' {
                count += 1;
            }
            count
        }).collect();
        
        dp.reverse();
        println!("{:?}", dp);

        let mut count = 0;
        let mut min = i32::MAX;
        (0..chars.len()).for_each(|index| {
            min = i32::min(min, dp[index] + count);
            if chars[index] == '1' {
                count += 1;
            }
        });
        
        i32::min(i32::min(zeros, ones), min)
    }
}