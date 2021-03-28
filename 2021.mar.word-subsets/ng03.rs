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
        let mut sets: Vec<BTreeMap<char, usize>> = b.into_iter().map(|word| {
            let set = string_to_set(&word);
            set
        }).collect();
        a.into_iter().filter(|word| !sets.iter().find(|set| {
            !is_subset(word, set)
        }).is_some()).collect::<Vec<String>>()
    }
}