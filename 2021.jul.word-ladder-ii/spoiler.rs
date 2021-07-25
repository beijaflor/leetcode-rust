use std::ops::Deref;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
        if !word_list.contains(&end_word) {
            return vec![];
        };
        let mut word_warehouse = word_list.iter().map(String::deref).collect::<HashSet<_>>();
        let mut group_set_1 = HashSet::new();
        group_set_1.insert(begin_word.deref());
        let mut group_set_2 = HashSet::new();
        group_set_2.insert(end_word.deref());
        for word in group_set_1.iter() {
            word_warehouse.remove(word);
        };
        for word in group_set_2.iter() {
            word_warehouse.remove(word);
        };
        let mut word_link_map = HashMap::new();
        let mut found = false;
        let mut reversed = false;
        while !group_set_1.is_empty() && !group_set_2.is_empty() && !found {
            if group_set_1.len() > group_set_2.len() {
                std::mem::swap(&mut group_set_1, &mut group_set_2);
                reversed = !reversed;
            };
            let mut group_set_t: HashSet<&str> = HashSet::new();
            for &word1 in group_set_1.iter() {
                let mut curr = word1.to_owned();
                let bytes = unsafe { curr.as_bytes_mut() };
                for i in 0..bytes.len() {
                    let ch = bytes[i];
                    for ch in b'a'..=b'z' {
                        bytes[i] = ch;
                        let word_str = unsafe { std::str::from_utf8_unchecked(bytes) };
                        if let Some(word) = group_set_2.get(word_str).cloned() {
                            found = true;
                            let (from, to) = if reversed {
                                (word, word1)
                            } else {
                                (word1, word)
                            };
                            word_link_map.entry(from).or_insert(vec![]).push(to);
                        } else if let Some(word) = word_warehouse.get(word_str).cloned() {
                            if !found {
                                group_set_t.insert(word);
                                let (from, to) = if reversed {
                                    (word, word1)
                                } else {
                                    (word1, word)
                                };
                                word_link_map.entry(from).or_insert(vec![]).push(to);
                            };
                        };
                    };
                    bytes[i] = ch;
                };
            };
            for word in group_set_t.iter() {
                word_warehouse.remove(word);
            };
            std::mem::swap(&mut group_set_1, &mut group_set_t);
        };
        let mut stack = vec![];
        let mut results = vec![];
        recursive_result(&word_link_map, &begin_word, &end_word, &mut stack, &mut results);
        results
    }
}

fn recursive_result(
    word_link_map: &std::collections::HashMap<&str, Vec<&str>>,
    from: &str,
    end: &str,
    stack: &mut Vec<String>,
    results: &mut Vec<Vec<String>>,
) {
    stack.push(from.to_owned());
    if from == end {
        results.push(stack.iter().cloned().collect());
    } else if let Some(inner) = word_link_map.get(from) {
        for word in inner.iter() {
            recursive_result(word_link_map, word, end, stack, results);
        };
    };
    stack.pop();
}