// https://leetcode.com/submissions/detail/526854680/
use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let ill = String::from("");

        let mut adjacency_list: HashMap<char, Vec<char>> = HashMap::new();
        let mut counts: HashMap<char, i32> = HashMap::new();
        
        words.iter().for_each(|word| {
            word.chars().for_each(|char| {
                counts.insert(char, 0);
                adjacency_list.insert(char, vec![]);
            });
        });
        
        for index in 0..(words.len() - 1) {
            let word1 = &words[index];
            let word2 = &words[index + 1];
            
            if word1.len() > word2.len() && word1.starts_with(word2) {
                return ill
            }
            
            for (char1, char2) in word1.chars().zip(word2.chars()) {
                // println!("{} {}", char1, char2);
                if char1 != char2 {
                    adjacency_list.entry(char1).and_modify(|c| c.push(char2)); // .or_insert(vec![char2]);
                    counts.entry(char2).and_modify(|c| *c += 1); // .or_insert(1);
                    break
                }
            }
        }
        
        let mut result: Vec<char> = Vec::new();
        let mut queue: VecDeque<char> = VecDeque::new();
        
        for char in counts.keys() {
            if counts.get(char) == Some(&0) {
                queue.push_back(*char);
            }
        }
        
        while let Some(char) = queue.pop_front() {
            result.push(char);
            for next in adjacency_list.get(&char).unwrap() {
                counts.entry(*next).and_modify(|c| *c -= 1);
                if counts.get(&next) == Some(&0) {
                    queue.push_back(*next);
                }
            }
        }
        
        // println!("result {:?}\n, counts {:?}\n, adjList {:?}", result, counts, adjacency_list);
        
        if result.len() < counts.len() {
            return ill
        }
        
        result.into_iter().collect()
    }
}