// https://leetcode.com/submissions/detail/522374014/
use std::collections::HashMap;
use std::cmp::Ordering;

impl Solution {
    pub fn custom_sort_string(order: String, str: String) -> String {
        let mut comparator: HashMap<char, usize> = HashMap::new();
        order.chars().enumerate().for_each(|(index, char)| {
            comparator.insert(char, index);
        });
        
        let mut undef_order = Vec::<char>::new();
        let mut order = str.chars().filter(|char| {
            match comparator.get(&char) {
                None => {
                    undef_order.push(*char);
                    false
                },
                Some(_) => true,
            }
        }).collect::<Vec<char>>();

        order.sort_by(|lhv, rhv| comparator.get(&lhv).unwrap().cmp(&comparator.get(&rhv).unwrap()) );
        
        order.into_iter().collect::<String>() + &undef_order.into_iter().collect::<String>()
    }
}