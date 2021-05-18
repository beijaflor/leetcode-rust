// https://leetcode.com/submissions/detail/494638442/
use std::collections::HashMap;

impl Solution {
    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        let mut map: HashMap<String, i32> = HashMap::new();
        words.sort_by(|lhv, rhv| lhv.len().cmp(&rhv.len()));
        let mut longest = 1;
        
        words.into_iter().for_each(|word| {
            let chars = word.chars().collect::<Vec<char>>();
            let candidates = (0..word.len()).map(|pos| {
                let mut clone = chars.clone();
                clone.remove(pos);
                clone.into_iter().collect::<String>()
            }).collect::<Vec<String>>();

            // println!("word: {:?}", word);

            let current_length = candidates.into_iter().fold(1, |max, word| {
                let previous_length = map.get(&word).unwrap_or(&0);
                i32::max(max, previous_length + 1)
            });
            map.insert(word, current_length);
            
            longest = i32::max(longest, current_length);
        });
        
        longest
    }
}