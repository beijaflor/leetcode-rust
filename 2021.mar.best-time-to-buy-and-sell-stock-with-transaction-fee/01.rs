impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut cash = 0;
        let mut hold = -prices[0];
        (0..prices.len()).for_each(|index| {
            // println!("{}", hold);
            cash = i32::max(cash, hold + prices[index] - fee);
            hold = i32::max(hold, cash - prices[index]);
        });
        cash
    }
}