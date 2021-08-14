// https://leetcode.com/submissions/detail/537309252/
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut counter: HashMap<String, Vec<String>> = HashMap::new();
        strs.into_iter().for_each(|str| {
            let mut key_vec: Vec<char> = str.chars().collect();
            key_vec.sort();
            counter.entry(key_vec.into_iter().collect::<String>()).and_modify(|c| c.push(str.clone())).or_insert(vec![str.clone()]);
        });
        
        counter.into_iter().map(|(key, vec)| vec).collect()
    }
}