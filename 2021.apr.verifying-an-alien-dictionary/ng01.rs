/*
["word","world","row"]
"worldabcefghijkmnpqstuvxyz"
["a","b"]
"abcdefghijklmnopqrstuvwxyz"
["hello","leetcode"]
"hlabcdefgijkmnopqrstuvwxyz"
["apple","app"]
"abcdefghijklmnopqrstuvwxyz"
*/
use std::collections::HashMap;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut dict: HashMap<char, usize> = HashMap::new();
        order.chars().enumerate().for_each(|(index, c)| {
            dict.insert(c, index + 1);
        });
        println!("{:?}", dict);
        
        let mut words_in_chars: Vec<Vec<char>> = words.into_iter().map(|str| {
            str.chars().collect::<Vec<char>>()
        }).collect();
        
        for index in 0..20 {
            let mut is_sorted = true;
            words_in_chars = words_in_chars.clone().into_iter().enumerate().filter(|(word_index, word_in_chars)| {
                if let Some(next) = words_in_chars.get(word_index + 1) {
                    // println!("{:?}", next);
                    let pos = if let Some(pos) = word_in_chars.get(index) { *dict.get(pos).unwrap() } else { 0 };
                    let next_pos = if let Some(pos) = word_in_chars.get(index + 1)  { *dict.get(pos).unwrap() } else { 0 };
                    println!("pos: {:?}, next_pos: {:?}", pos, next_pos);
                    if pos > next_pos {
                        is_sorted = false;
                        false
                    } else if pos == next_pos {
                        true
                    } else {
                        false
                    }
                } else {
                    true
                }
            }).map(|(_,v)| v).collect();
            println!("words_in_chars: {:?}", words_in_chars);
            if words_in_chars.len() < 2 {
                return true
            }
            if !is_sorted {
                return false
            }
            // let mut iterator = words_in_chars.iter().enumerate();
            // if let Some((_, first)) = iterator.next() {
            //     while let Some((word_index, next)) = iterator.next() {
            //         let last_pos = if let Some(pos) = first.get(index) { *dict.get(pos).unwrap() } else { 0 };
            //         let next_pos = if let Some(pos) = next.get(index) { *dict.get(pos).unwrap() } else { 0 };
            //         println!("{:?}", last_pos);
            //         println!("{:?}", next_pos);
            //         if last_pos > next_pos {
            //             return false
            //         } else {
            //             words_in_chars.remove(word_index);
            //         }
            //     }
            // } else {
            //     panic!("will never come to this line")
            // }
        }
        true
    }
}