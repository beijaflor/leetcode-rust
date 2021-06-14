// https://leetcode.com/submissions/detail/507055597/
impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let N = stations.len();
        let mut dp: Vec<i64> = vec![0; N + 1];
        dp[0] = start_fuel as i64;
        (0..N).for_each(|index| {
            (0..index + 1).rev().for_each(|t| {
                // println!("{}", t);
                if dp[t] >= stations[index][0] as i64 {
                    dp[t + 1] = i64::max(dp[t + 1], dp[t] + stations[index][1] as i64);
                }
            });
        });
        
        // println!("{:?}", dp);
        
        for (i, t) in dp.into_iter().enumerate() {
            if t as i32 >= target {
                return i as i32
            }
        }
        -1
    }
}