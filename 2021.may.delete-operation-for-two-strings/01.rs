// https://leetcode.com/submissions/detail/489979795/
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let chars1 = word1.chars().collect::<Vec<char>>();
        let chars2 = word2.chars().collect::<Vec<char>>();
      
        let mut dp: Vec<Vec<i32>> = vec![vec![0; word1.len() + 1]; word2.len() + 1];
        for x in 0..word1.len() + 1 {
            for y in 0..word2.len() + 1 {
                if x == 0 || y == 0 {
                    dp[y][x] = (x + y) as i32;
                } else if chars1[x - 1] == chars2[y - 1] {
                    dp[y][x] = dp[y - 1][x - 1];
                } else {
                    dp[y][x] = 1 + i32::min(dp[y - 1][x], dp[y][x - 1]);
                }
            }
        }
        
        // println!("{:?}", dp);
        
        dp[word2.len()][word1.len()]
    }
}