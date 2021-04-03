/*
["0","0","1","1"]
2
2
["10","0001","111001","1","0"]
5
3
*/
use std::collections::BTreeSet;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut set: BTreeSet<(i32, i32, i32)> = BTreeSet::new();
        strs.into_iter().for_each(|str| {
            let mut zero = 0;
            let mut one = 0;
            str.chars().for_each(|char| {
                match char {
                    '0' => zero += 1,
                    '1' => one += 1,
                    _ => ()// NOP
                }
            });
            set.insert((zero + one, zero, one));
        });
        println!("{:?}", set);
        
        let mut result = 0;
        let mut at_most_zero = m;
        let mut at_most_one = n;

        for (_, zero, one) in set.into_iter() {
            println!("zero: {}, one: {}", zero, one);
            at_most_zero -= zero;
            at_most_one -= one;
            if at_most_zero < 0 || at_most_one < 0 {
                break
            }
            result += 1;
        }
        
        result
    }
}