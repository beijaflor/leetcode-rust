// https://leetcode.com/submissions/detail/507324809/
use std::collections::HashMap;

fn is_palin(word: &String, mut start: usize, mut end: usize) -> bool {
    let chars = word.chars().collect::<Vec<char>>();
    while start < end {
        if chars[start] != chars[end] { return false }
        start += 1;
        end -= 1;
    }
    true
}

fn all_valid_prefixes(word: &String) -> Vec<String> {
    let mut list: Vec<String> = Vec::new();
    for index in 0..word.len() {
        if is_palin(word, index, word.len() - 1) {
            list.push(word[0..index].to_string());
        }
    }
    list
}

fn all_valid_suffixes(word: &String) -> Vec<String> {
    let mut list: Vec<String> = Vec::new();
    for index in 0..word.len() {
        if is_palin(word, 0, index) {
            list.push(word[index + 1..word.len()].to_string());
        }
    }
    list
}

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut map: HashMap<String, usize> = HashMap::new();
        words.into_iter().enumerate().for_each(|(i, w)| {
            map.insert(w, i);
        });
        
        let mut result: Vec<Vec<i32>> = Vec::new();
        for (word, current_index) in map.iter() {
            let reverse = word.chars().rev().collect::<String>();
            
            if let Some(index) = map.get(&reverse) {
                if index != current_index {
                    result.push(vec![*current_index as i32, *index as i32]);
                }
            }
            
            for suffix in all_valid_suffixes(&word).into_iter() {
                // println!("{}", suffix);
                let rev = suffix.chars().rev().collect::<String>();
                if let Some(index) = map.get(&rev) {
                    if index != current_index {
                        result.push(vec![*index as i32, *current_index as i32]);
                    }
                }
            }

            for prefix in all_valid_prefixes(&word).into_iter() {
                // println!("{}", prefix);
                let rev = prefix.chars().rev().collect::<String>();
                if let Some(index) = map.get(&rev) {
                    if index != current_index {
                        result.push(vec![*current_index as i32, *index as i32]);
                    }
                }
            }
        }

        result
    }
}