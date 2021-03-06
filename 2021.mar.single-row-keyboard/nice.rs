use std::collections::HashMap;

impl Solution {
    pub fn calculate_time(keyboard: String, word: String) -> i32 {
        // only English lowercase = 26
        let map = keyboard.chars().zip((0..26)).map(|(x,y)| (x,y as i32)).collect::<HashMap<char, i32>>();
        let mut diff: i32 = 0; 
        let mut prev: i32 = 0;
        for w in word.chars() {
            if let Some(&v) = map.get(&w) {
                diff += i32::abs(v - prev);
                prev = v;
            }
        }
        diff
    }
}