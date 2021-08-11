// https://leetcode.com/submissions/detail/535015833/
fn find_minimum_cuts(start_index: usize, end_index: usize, dp: &mut Vec<i32>, s: &str) {
    let mut start = start_index;
    let mut end = end_index;
    while start >= 0 && end < s.len() && s.chars().nth(start) == s.chars().nth(end) {
        let new_cut = if start == 0 { 0 } else { dp[start - 1] + 1 };
        dp[end] = i32::min(dp[end], new_cut);
        start -= 1;
        end += 1;
    }
}


impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let mut dp: Vec<i32> = (0..s.len() as i32).collect();

        (0..s.len()).for_each(|mid| {
            find_minimum_cuts(mid, mid, &mut dp, &s);
            find_minimum_cuts(mid - 1, mid, &mut dp, &s);
        });

        dp[s.len() - 1]
    }
}