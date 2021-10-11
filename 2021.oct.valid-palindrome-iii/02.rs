// https://leetcode.com/submissions/detail/567822635/
impl Solution {
    pub fn is_valid_palindrome(s: String, k: i32) -> bool {
        let mut memo: Vec<i32> = vec![0; s.len()];
        let chars = s.chars().collect::<Vec<char>>();
        (0..=(chars.len() - 2)).rev().for_each(|index1| {
            let mut prev = 0;
            ((index1 + 1)..chars.len()).for_each(|index2| {
                let mut temp = memo[index2];
                
                if chars[index1] == chars[index2] {
                    memo[index2] = prev;
                } else {
                    memo[index2] = 1 + i32::min(memo[index2], memo[index2 - 1]);
                }
                prev = temp
            });
        });

        memo[chars.len() - 1] <= k
    }
}