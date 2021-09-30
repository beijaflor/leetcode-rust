// https://leetcode.com/submissions/detail/559184279/
use std::collections::HashSet;

fn word_to_bit_set(set: &mut HashSet<i32>, word: String) {
    let mut bit_set: i32 = 0;
    let len = word.len();
    for char in word.chars() {
        let mask = 1 << (char as i32 - 'a' as i32);
        if (bit_set & mask) > 0 {
            return
        }
        bit_set += mask;
    }
    set.insert(bit_set + ((len as i32) << 26));
}

fn dfs(arr: &mut Vec<i32>, pos: usize, res: i32) -> i32 {
    let old_char = res & ((1 << 26) - 1);
    let old_len = res >> 26;
    let mut best = old_len;
    
    for index in (pos..arr.len()) {
        let char = arr[index] & ((1 << 26) - 1);
        let len = arr[index] >> 26;
        
        if (char & old_char) != 0 {
            continue
        }
        
        let res = char + old_char + (len + old_len << 26);
        best = i32::max(best, dfs(arr, index + 1, res));
    }
    best
}

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();
        arr.into_iter().for_each(|word| {
            word_to_bit_set(&mut set, word);
        });
        
        let mut arr: Vec<i32> = set.into_iter().collect();
        dfs(&mut arr, 0, 0)
    }
}