// https://leetcode.com/submissions/detail/537310715/
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut counter: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
        strs.into_iter().for_each(|str| {
            let mut key_vec: Vec<u8> = str.as_bytes().to_vec();
            key_vec.sort();
            counter.entry(key_vec).or_insert(Vec::new()).push(str);
        });
        
        counter.into_iter().map(|(key, vec)| vec).collect()
    }
}