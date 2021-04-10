// https://leetcode.com/submissions/detail/478710334/
use std::collections::HashMap;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut dict: HashMap<char, usize> = HashMap::new();
        order.chars().enumerate().for_each(|(index, c)| {
            dict.insert(c, index + 1);
        });
        // println!("{:?}", dict);
        
        let mut words_in_chars: Vec<Vec<char>> = words.into_iter().map(|str| {
            str.chars().collect::<Vec<char>>()
        }).collect();
        
        for index in 0..20 {
            let mut is_sorted = true;
            let mut duplicated: Vec<char> = vec![];
            let mut line: Vec<Option<&char>> = words_in_chars.iter().map(|word_in_chars| word_in_chars.get(index)).collect();
            // println!("line: {:?}", line);
            line.sort_by(|rhv, lhv| {
                // println!("lhv: {:?}, rhv: {:?}", lhv, rhv);
                let left = if let Some(pos) = lhv  { *dict.get(pos).unwrap() } else { 0 };
                let right = if let Some(pos) = rhv  { *dict.get(pos).unwrap() } else { 0 };
                // println!("left: {:?}, right: {:?}", left, right);
                if left == right && *lhv != None {
                    duplicated.push(*lhv.unwrap());
                }
                if left > right {
                    is_sorted = false;
                }
                left.cmp(&right)
            });
            if !is_sorted {
                return false
            }
            if duplicated.len() == 0 {
                return true
            }
            // println!("words_in_chars: {:?}", words_in_chars);
            // println!("duplicated: {:?}", duplicated);
            words_in_chars = words_in_chars.into_iter().filter(|word_in_chars| {
                if let Some(c) = word_in_chars.get(index) {
                    duplicated.contains(c)
                } else {
                    false
                }
            }).collect();
            // println!("words_in_chars: {:?}", words_in_chars);
        }
        true
    }
}