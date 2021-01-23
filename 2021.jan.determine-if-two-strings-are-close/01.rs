// https://leetcode.com/submissions/detail/446541489/
use std::collections::HashMap;

fn make_hash(word: String) -> HashMap<char, i32> {
    let mut result_map: HashMap<char, i32> = HashMap::new();
    word.chars().for_each(|c| {
        result_map.entry(c).and_modify(|v| *v += 1).or_insert(0);
    });
    result_map
}

fn hash_to_counts(hash: HashMap<char, i32>) -> (Vec<char>, Vec<i32>) {
    let mut result_vec_key = Vec::with_capacity(hash.len());
    let mut result_vec_value = Vec::with_capacity(hash.len());
    hash.iter().for_each(|(key, value)| {
        result_vec_key.push(*key);
        result_vec_value.push(*value);
    });
    result_vec_key.sort();
    result_vec_value.sort();
    (result_vec_key, result_vec_value)
}

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        // let mut sorted1 = word1.chars().collect::<Vec<char>>();
        // sorted1.sort();
        // let mut sorted2 = word2.chars().collect::<Vec<char>>();
        // sorted2.sort();
        
        // if sorted1 != sorted2 { return false }
        
        let hash1 = make_hash(word1);
        let hash2 = make_hash(word2);
        // println!("{:?}, {:?}", hash1, hash2);

        let vec1 = hash_to_counts(hash1);
        let vec2 = hash_to_counts(hash2);
        // println!("{:?}, {:?}", vec1, vec2);
        vec1 == vec2
    }
}