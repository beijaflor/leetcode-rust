use std::collections::BTreeMap;

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

fn make_map(wordlist: Vec<String>) -> Vec<(String, String, String)> {
    let mut key_map: Vec<(String, String, String)> = Vec::new();
    wordlist.into_iter().for_each(|word| {
        let upper = word.to_ascii_uppercase();
        key_map.push((process_word(&word), upper, word));
    });
    key_map
}

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let mut key_map: Vec<(String, String, String)> = make_map(wordlist);
        queries.into_iter().map(|q| {
            if let Some(find_exact) = key_map.iter().find(|(_, _, word)| *word == q) {
                return find_exact.2.to_string()
            };
            if let Some(find_upper) = key_map.iter().find(|(_, upper, _)| *upper == q.to_ascii_uppercase()) {
                return find_upper.2.to_string()
            };
            if let Some(find_ortho) = key_map.iter().find(|(key, _, _)| *key == process_word(&q)) {
                return find_ortho.2.to_string()
            };
            String::from("")
        }).collect::<Vec<String>>()
    }
}