// https://leetcode.com/submissions/detail/564192943/
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut result = 0;
        let mut p1 = 0;
        let mut p2 = 0;
        let (text1, text2) = if text1.len() < text2.len() { (text1, text2) } else { (text2, text1) };
        let mut chars1 = text1.chars().collect::<Vec<char>>();
        let mut chars2 = text2.chars().collect::<Vec<char>>();

        let mut previous: Vec<i32> = vec![0; chars1.len() + 1];

        (0..chars2.len()).rev().for_each(|col| {
            let mut current: Vec<i32> = vec![0; chars1.len() + 1];
            (0..chars1.len()).rev().for_each(|row| {
                if chars1[row] == chars2[col] {
                    current[row] = 1 + previous[row + 1];
                } else {
                    current[row] = i32::max(previous[row], current[row + 1]);
                }
            });
            previous = current;
        });
        
        previous[0]
    }
}