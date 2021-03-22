// https://leetcode.com/submissions/detail/470856160/
use std::collections::{ HashMap, BTreeSet };

fn count(n: i32) -> Vec<i32> {
    let mut n = n;
    let mut result = Vec::with_capacity(10);
    (0..10).for_each(|_| result.push(0));
    while n > 0 {
        result[(n % 10) as usize] += 1;
        n /= 10;
    }
    result
}

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let answers = count(n);
        for index in 0..32_i32 {
            if answers == count(1 << index) {
                return true
            }
        }
        false
    }
}
/*
class Solution {
    public boolean reorderedPowerOf2(int N) {
        int[] A = count(N);
        for (int i = 0; i < 31; ++i)
            if (Arrays.equals(A, count(1 << i)))
                return true;
        return false;
    }

    // Returns the count of digits of N
    // Eg. N = 112223334, returns [0,2,3,3,1,0,0,0,0,0]
    public int[] count(int N) {
        int[] ans = new int[10];
        while (N > 0) {
            ans[N % 10]++;
            N /= 10;
        }
        return ans;
    }
}
*/