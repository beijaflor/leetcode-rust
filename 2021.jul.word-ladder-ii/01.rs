// https://leetcode.com/submissions/detail/527783961/
use std::collections::{HashMap, VecDeque};

fn is_match(str1: &str, str2: &str) -> bool {
    let chars1: Vec<char> = str1.chars().collect();
    let chars2: Vec<char> = str2.chars().collect();
    for index in 0..chars1.len() {
        let mut A = chars1.clone();
        A[index] = '*';
        let mut B = chars2.clone();
        B[index] = '*';
        if A == B {
            return true
        }
    }
    
    false
}

impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, mut word_list: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
        // if !word_list.contains(&begin_word) {
            word_list.push(begin_word);
        // }
        if let Some(goal_index) = word_list.iter().position(|word| *word == end_word) {
            // word_list.push(end_word);
            (0..word_list.len()).for_each(|index| {
                (0..word_list.len()).for_each(|target_index| {
                    // println!("{}, {}", index, target_index);
                    if index != target_index {                    
                        if is_match(&word_list[index], &word_list[target_index]) {
                            map.entry(index).and_modify(|c| c.push(target_index)).or_insert(vec![target_index]);
                        }
                    }
                });
            });

            // println!("{:?}", map);

            let mut queue: VecDeque<Vec<usize>> = VecDeque::new();
            queue.push_back(vec![goal_index]);

            while !queue.is_empty() {
                let mut result: Vec<Vec<usize>> = Vec::new();
                let mut next_queue: VecDeque<Vec<usize>> = VecDeque::new();
                queue.into_iter().for_each(|mut q| {
                    let index = q.last().unwrap();
                    let cand = match map.get(index) { None => vec![], Some(cand) => cand.to_vec() };
                    // println!("cand: {:?}", cand);
                    cand.iter().for_each(|next| {
                        // println!("q: {:?}, next: {}, contains: {}", q, next, q.contains(&*next));
                        if !q.contains(&*next) {
                            let mut next_q = q.clone();
                            next_q.push(*next);
                            if *next == word_list.len() - 1 {
                                result.push(next_q.clone());
                            }
                            next_queue.push_back(next_q);
                        }
                    });
                    // println!("{:?}", next_queue);
                });
                queue = next_queue;
                if !result.is_empty() {
                    // println!("result {:?}", result);
                    return result.into_iter().map(|mut r| {
                        r.reverse();
                        r.into_iter().map(|index| word_list[index].clone()).collect::<Vec<String>>()
                    }).collect()
                }
            }

            // println!("{:?}", word_list);
            // println!("{:?}", map);
        }

        vec![]        
    }
}