// https://leetcode.com/submissions/detail/599664468/
const M: i64 = 1_000_000_007;

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        if n <= 2 { return n }
        
        let mut curr = 5i64;
        let mut prev = 2i64;
        let mut before_prev = 1i64;
        
        (4..=n).for_each(|_| {
            let temp = prev;
            prev = curr;
            curr = (2 * curr + before_prev) % M;
            before_prev = temp;
        });
        
        (curr % M) as i32
    }
}