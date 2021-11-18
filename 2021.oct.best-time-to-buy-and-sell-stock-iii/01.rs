// https://leetcode.com/submissions/detail/571880232/
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut cost1 = i32::MAX;
        let mut cost2 = i32::MAX;
        let mut profit1 = 0;
        let mut profit2 = 0;
        
        prices.into_iter().for_each(|price| {
            cost1 = i32::min(cost1, price);
            profit1 = i32::max(profit1, price - cost1);
            
            cost2 = i32::min(cost2, price - profit1);
            profit2 = i32::max(profit2, price - cost2);
        });
        
        profit2
    }
}