// https://leetcode.com/submissions/detail/514172335/
use std::collections::{BTreeMap, HashMap};

impl Solution {
    pub fn candy(mut ratings: Vec<i32>) -> i32 {
        let len = ratings.len();
        if len == 1 { return 1 }
        let mut candies: Vec<i32> = vec![0; ratings.len()];
        let mut total = 0;
        let mut children: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
        ratings.into_iter().enumerate().for_each(|(index, rate)| {
            children.entry(rate).and_modify(|c| c.push(index)).or_insert(vec![index]);
        });
        
        children.into_iter().for_each(|(rate, index_list)| {
            // println!("rate: {}, index: {:?}", rate, index_list);
            let mut store: HashMap<usize, i32> = HashMap::new();
            index_list.into_iter().for_each(|index| {
                let candy =
                    if index == 0 {
                        candies[index + 1]
                    } else if index == len - 1 {
                        candies[index - 1]
                    } else {
                        i32::max(candies[index - 1], candies[index + 1])
                };
                total += candy + 1;
                store.insert(index, candy + 1);
            });
            // println!("{:?}", store);
            store.into_iter().for_each(|(index, candy)| candies[index] = candy);
        });

        total
    }
}