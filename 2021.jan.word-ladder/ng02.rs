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

struct LadderQueue {
    word: String,
    count: i32,
    word_list: Vec<String>,
}

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let word_list = word_list.into_iter().filter(|w| w != &begin_word).collect::<Vec<String>>();
        let mut q: VecDeque<LadderQueue> = VecDeque::new();
        q.push_back({ LadderQueue {
            word: begin_word,
            count: 1,
            word_list: word_list,
        } });
        while let Some(queue) = q.pop_front() {
            let posibility = find_posibility(&queue.word, &queue.word_list);
            let count = queue.count + 1;
            if posibility.len() == 0 {
                return 0
            }
            if posibility.contains(&end_word) {
                return count 
            }
            // println!("{} in {} -> {:?}", queue.word, queue.count, posibility);
            posibility.into_iter().for_each(|word| {
                let new_word_list = queue.word_list.clone().into_iter().filter(|w| w != &word).collect::<Vec<String>>();
                let queue = { LadderQueue {
                    word: word,
                    count: count,
                    word_list: new_word_list,
                } };
                q.push_back(queue);
            })
        }
        1
    }
}
