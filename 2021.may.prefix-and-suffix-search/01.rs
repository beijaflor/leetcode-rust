// https://leetcode.com/submissions/detail/487713035/
use std::collections::HashMap;

struct WordFilter {
    words: Vec<String>,
    cache: HashMap<String, i32>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let dict: Vec<String> = words.into_iter().map(|mut str| {
            let (prefix, suffix) = if str.len() > 9 {
                let prefix = &str[0..10];
                let suffix = &str[str.len() - 10..];
                (prefix.to_string(), suffix.to_string())
            } else {
                (str.clone(), str.clone())
            };
            // println!("prefix: {:?}", prefix);
            // println!("suffix: {:?}", suffix);
            suffix + "#" + &prefix
        }).collect::<Vec<String>>();
        // println!("{:?}", dict);
        WordFilter {
            words: dict,
            cache: HashMap::new(),
        }
    }
    
    fn f(&mut self, prefix: String, suffix: String) -> i32 {
        let mut max_size = 0;
        let mut position = -1;
        let query = suffix.clone() + "#" + &prefix.clone();
        let cache = self.cache.get(&query);
        match cache {
            None => {
                for index in 0..self.words.len() {
                    if self.words[index].len() >= max_size {
                        let start = self.words[index].len() / 2 - suffix.len();
                        let end = self.words[index].len() / 2 + prefix.len() + 1;
                        // println!("start: {}, end: {}", start, end);
                        let target = &self.words[index][start..end];
                        // println!("query: {:?}, target: {:?}", query, target);
                        if query == target {
                            max_size = self.words[index].len();
                            position = index as i32;
                        }
                    }
                }
                self.cache.insert(query, position);
                position
            },
            Some(cache) => {
                *cache
            }
        }
    }
}

/**
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(prefix, suffix);
 */