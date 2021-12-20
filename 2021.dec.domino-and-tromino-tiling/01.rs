// https://leetcode.com/submissions/detail/599655900/
const M: i64 = 1_000_000_007;

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        if n <= 2 { return n }
        
        let mut fPrev = 1i64;
        let mut fCurr = 2i64;
        let mut pCurr = 1i64;
        
        (3..=n).for_each(|_| {
            let temp = fCurr;
            fCurr = (fCurr + fPrev + 2 * pCurr) % M;
            pCurr = (pCurr + fPrev) % M;
            fPrev = temp;
        });
        
        fCurr as i32
    }
}