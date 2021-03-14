impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut result = 0;
        let mut coins = coins;
        coins.sort();
        let mut amount = amount;
        coins.iter().rev().for_each(|coin| {
            let d = amount / *coin;
            let m = amount % *coin;
            
            println!("coin: {}, div: {}, mod: {}", coin, d, m);
            result += d;
            amount = m;
        });

        if amount != 0 {
            -1
        } else {
            result
        }
    }
}
