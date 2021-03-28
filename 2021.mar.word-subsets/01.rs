// https://leetcode.com/submissions/detail/473301224/
use std::collections::BTreeMap;

fn string_to_set(word: &str) -> BTreeMap<char, usize> {
    let mut set: BTreeMap<char, usize> = BTreeMap::new();
    word.chars().for_each(|c| {
        set.entry(c).and_modify(|c| *c += 1).or_insert(1);
    });
    set
}

fn is_subset(word: &str, dict_set: &BTreeMap<char, usize>) -> bool {
    let word_set = string_to_set(word);
    // println!("word_set {:?}, dict_set {:?}", word_set, dict_set);
    !dict_set.iter().find(|(char, dict_count)| {
        // println!("char: {} @ {:?}", char, word_set.get(*char));
        match word_set.get(*char) {
            None => true,
            Some(word_count) => word_count < *dict_count
        }
    }).is_some()
}

impl Solution {
    pub fn word_subsets(a: Vec<String>, b: Vec<String>) -> Vec<String> {
        let mut set: BTreeMap<char, usize> = BTreeMap::new();
        b.into_iter().for_each(|word| {
            string_to_set(&word).into_iter().for_each(|(c, count)| {
                set.entry(c).and_modify(|c| *c = usize::max(*c, count)).or_insert(count);
            });
        });
        // println!("{:?}", set);
        a.into_iter().filter(|word| {
            is_subset(word, &set)
        }).collect::<Vec<String>>()
    }
}