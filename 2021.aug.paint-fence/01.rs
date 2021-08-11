// https://leetcode.com/submissions/detail/535814347/
impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        if n == 1 { return k }
        
        let mut two = k;
        let mut one = k * k;
        (3..=n).for_each(|index| {
            let curr = (k -1) * (one + two);
            two = one;
            one = curr;
        });
        
        one
    }
}