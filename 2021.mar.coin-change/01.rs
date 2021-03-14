// https://leetcode.com/submissions/detail/466721985/
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let max = amount + 1;
        let mut dp: Vec<i32> = Vec::with_capacity(amount as usize + 1);
        (0..amount + 1).for_each(|_| dp.push(max));
        dp[0] = 0;
        for i in 1..(amount + 1) as usize {
            for j in 0..coins.len() {
                if coins[j] <= i as i32 {
                    dp[i] = i32::min(dp[i], dp[i - coins[j] as usize] + 1);
                }
            }
        }

        println!("{:?}", dp);
        
        if dp[amount as usize] > amount {
            -1
        } else {
            dp[amount as usize]
        }
    }
}