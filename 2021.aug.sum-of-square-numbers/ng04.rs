//
fn b_search(s: i32, e: i32, n: i32) -> bool {
    if s > e { return false }
    let mid = s + (e - s) / 2;
    if mid * mid == n {
        return true
    }
    if mid * mid > n {
        return b_search(s, mid - 1, n);
    }
    return b_search(mid + 1, e, n);
}

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut a = 0;
        while a * a <= c {
            let b = c - a * a;
            if b_search(0, b, b) {
                return true
            }
            a += 1;
        }
        false
    }
}