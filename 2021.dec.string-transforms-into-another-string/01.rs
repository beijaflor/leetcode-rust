// https://leetcode.com/submissions/detail/605837556/
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn can_convert(str1: String, str2: String) -> bool {
        if str1 == str2 {
            return true
        }

        let mut conversion_mappings = HashMap::<char, char>::new();
        let mut unique_characters_in_str2 = HashSet::<char>::new();

        for (char1, char2) in str1.chars().zip(str2.chars()) {
            // println!("{} {}", char1, char2);
            
            if !conversion_mappings.contains_key(&char1) {
                conversion_mappings.insert(char1, char2);
                unique_characters_in_str2.insert(char2);
            } else if *conversion_mappings.get(&char1).unwrap() != char2 {
                return false
            }
        }
        
        unique_characters_in_str2.len() < 26
    }
}