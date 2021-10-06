use std::collections::HashMap;

fn rec(n:i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if let Some(result) = memo.get(&n) {
        *result
    } else {
        if n < 3 { return n }
        let result = Solution::climb_stairs(n - 1) + Solution::climb_stairs(n - 2);
        memo.insert(n, result);
        result
    }
}


impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo: HashMap<i32, i32> = HashMap::new();
        rec(n, &mut memo)
    }
}