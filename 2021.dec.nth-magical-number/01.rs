// https://leetcode.com/submissions/detail/600132965/
const M: i64 = 1_000_000_007;

fn gcd(x: i32, y: i32) -> i32 {
    if x == 0 {
        y
    } else {
        gcd(y % x, x)
    }
}

impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        let l = (a / gcd(a, b) * b) as i64;
        let mut lo = 0i64;
        let mut hi = n as i64 * i32::min(a, b) as i64;
        while lo < hi {
            let mi = (lo + hi) / 2;
            if mi / a as i64 + mi / b as i64 - mi / l < n as i64 {
                lo = mi + 1;
            } else {
                hi = mi;
            }
        }
        
        (lo % M) as i32
    }
}