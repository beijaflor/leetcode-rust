// https://leetcode.com/submissions/detail/471193315/
use std::collections::{ BTreeSet, BTreeMap };
use std::iter::FromIterator;

fn process_word(word: &str) -> String {
    let mut word = word;
    word.chars().map(|mut c| {
        c.make_ascii_uppercase();
        match c {
            'A' | 'E' | 'I' | 'O' | 'U' => '*',
            c => c,
        }
    }).collect::<String>()
}

fn make_upper_map(wordlist: &Vec<String>) -> BTreeMap<String, String> {
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    wordlist.iter().for_each(|word| {
        let upper = word.to_ascii_uppercase();
        map.entry(upper).or_insert(word.to_string());
    });
    map
}

fn make_ortho_map(wordlist: &Vec<String>) -> BTreeMap<String, String> {
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    wordlist.iter().for_each(|word| {
        let ortho = process_word(word);
        map.entry(ortho).or_insert(word.to_string());
    });
    map
}

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let exact_set: BTreeSet<String> = BTreeSet::from_iter(wordlist.clone().into_iter());
        let upper_map = make_upper_map(&wordlist);
        let ortho_map = make_ortho_map(&wordlist);
        // println!("{:?}", upper_map);
        queries.into_iter().map(|q| {
            if let Some(find_exact) = exact_set.get(&q) {
                return q
            };
            if let Some(find_upper) = upper_map.get(&q.to_ascii_uppercase()) {
                return find_upper.to_string()
            };
            if let Some(find_ortho) = ortho_map.get(&process_word(&q)) {
                return find_ortho.to_string()
            };
            String::from("")
        }).collect::<Vec<String>>()
    }
}