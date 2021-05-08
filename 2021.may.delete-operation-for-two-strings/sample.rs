use std::mem;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (bytes1, bytes2) = (word1.as_bytes(), word2.as_bytes());
        let (m, n) = (bytes1.len(), bytes2.len());
        let mut prev = vec![0; n + 1];
        let mut curr = prev.clone();
        for i in 0..m {
            for j in 0..n {
                curr[j + 1] = if bytes1[i] == bytes2[j] {
                    1 + prev[j]
                } else {
                    prev[j + 1].max(curr[j])
                }
            }
            mem::swap(&mut prev, &mut curr);
        }
        (m + n - prev[n] * 2) as i32
    }
}