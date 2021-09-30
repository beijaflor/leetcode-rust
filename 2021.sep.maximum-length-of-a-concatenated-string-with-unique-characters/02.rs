// https://leetcode.com/submissions/detail/559199744/
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

fn backtracking(arr: &mut Vec<i32>, pos: usize, res: i32, len: usize) -> i32 {
    let mut best = len as i32;
    let mut res = res;
    let mut len = len;
    
    for index in (pos..arr.len()) {
        let char = arr[index] & ((1 << 26) - 1);
        let new_len = arr[index] >> 26;
        
        if (res & char) != 0 {
            continue
        }
        
        res += char;
        len += new_len as usize;
        best = i32::max(best, backtracking(arr, index + 1, res, len));
        
        res -= char;
        len -= new_len as usize;
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
        backtracking(&mut arr, 0, 0, 0)
    }
}