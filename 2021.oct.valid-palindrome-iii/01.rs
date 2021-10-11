// https://leetcode.com/submissions/detail/567819925/
fn dp(memo: &mut Vec<Vec<i32>>, s: &Vec<char>, i: usize, j: usize) -> i32 {
    if i == j {
        return 0;
    }
    
    if i == j - 1 {
        return if s[i] != s[j] { 1 } else { 0 }
    }
    
    if memo[i][j] == -1 {
        if s[i] == s[j] {
            memo[i][j] = dp(memo, s, i + 1, j - 1);
        } else {
            memo[i][j] = 1 + i32::min(
                dp(memo, s, i + 1, j),
                dp(memo, s, i, j - 1)
            )
        }
    }
    
    memo[i][j]
}

impl Solution {
    pub fn is_valid_palindrome(s: String, k: i32) -> bool {
        let mut memo: Vec<Vec<i32>> = vec![vec![-1; s.len()]; s.len()];
        let chars = s.chars().collect::<Vec<char>>();
        dp(&mut memo, &chars, 0, chars.len() - 1) <= k
    }
}