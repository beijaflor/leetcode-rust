// https://leetcode.com/submissions/detail/440915121/
use std::collections::VecDeque;

fn find_posibility(word: &String, word_list: &Vec<String>) -> Vec<String> {
    let mut result = vec!();
    let chars = word.chars().collect::<Vec<char>>();
    for w in word_list {
        let count = w.chars().collect::<Vec<char>>().iter().enumerate().fold(0, |mut acc, (index, x)| {
            if *x != chars[index] {
                acc += 1;
            };
            acc
        });
        if count == 1 {
            result.push(w.to_string());
        }
    }
    result
}

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut word_list = word_list.into_iter().filter(|w| w != &begin_word).collect::<Vec<String>>();
        let mut q = VecDeque::new();
        let mut count = 1;

        if !word_list.contains(&end_word) {
            return 0
        };

        q.push_back(begin_word);
        while !q.is_empty() {
            count += 1;
            for _ in 0..q.len() {
                let word = q.pop_front().unwrap();
                let posibility = find_posibility(&word, &word_list);
                if posibility.len() == 0 {
                    continue;
                }
                if posibility.contains(&end_word) {
                    return count 
                }
                posibility.into_iter().for_each(|word| {
                    word_list.remove(word_list.iter().position(|x| x == &word).unwrap());
                    q.push_back(word);
                })
            }
        }
        0
    }
}
