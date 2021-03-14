// https://leetcode.com/submissions/detail/467406885/
use std::collections::HashMap;

impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        let M: i64 = 1_000_000_007;
        let len = arr.len();
        arr.sort();
        
        let mut dp: Vec<i64> = Vec::with_capacity(len);
        (0..len).for_each(|_| dp.push(1));

        let mut index: HashMap<i32, i64> = HashMap::new();
        (0..len).for_each(|i| {
            index.insert(arr[i], i as i64);
        });

        for i in 0..len {
            for j in 0..i {
                if arr[i] % arr[j] == 0 {
                    let right = arr[i] / arr[j];
                    if index.contains_key(&right) {
                        dp[i] = (dp[i] + dp[j] * dp[*index.get(&right).unwrap() as usize]) % M;
                    }
                }
            }
        }
        
        let mut result = 0;
        (dp.into_iter().fold(0i64, |acc, v| acc + v as i64) % M) as i32
    }
}