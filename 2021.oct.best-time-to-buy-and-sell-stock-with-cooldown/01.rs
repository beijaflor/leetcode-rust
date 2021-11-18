// https://leetcode.com/submissions/detail/571654254/
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut mp: Vec<i32> = vec![0; prices.len() + 2];
        
        for index in (0..prices.len()).rev() {
            let mut c1 = 0;
            for sell in ((index + 1)..prices.len()) {
                let profit = prices[sell] - prices[index] + mp[sell + 2];
                c1 = i32::max(profit, c1);
            }
            
            mp[index] = i32::max(c1, mp[index + 1]);
        }
        mp[0]
    }
}