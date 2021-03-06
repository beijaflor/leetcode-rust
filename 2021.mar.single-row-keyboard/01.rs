// https://leetcode.com/submissions/detail/464046786/
use std::collections::HashMap;

fn make_keyboard_map(keyboard: String) -> HashMap<char, i32> {
    let mut map = HashMap::with_capacity(26);
    keyboard.chars().enumerate().for_each(|(pos, key)| { map.insert(key, pos as i32); });
    map    
}

impl Solution {
    pub fn calculate_time(keyboard: String, word: String) -> i32 {
        let first_latter = keyboard.chars().next().unwrap();
        let mut times = 0;
        let keys = make_keyboard_map(keyboard);
        // println!("{:?}", keys);

        let mut chars = word.chars();
        let mut pos = keys.get(&first_latter).unwrap();
        while let Some(char) = chars.next() {
            // println!("{}", char);
            let next_pos = keys.get(&char).unwrap();
            let diff = pos - next_pos;
            if diff < 0 {
                times += -diff;
            } else {
                times += diff;
            }
            pos = next_pos;
        }

        times
    }
}