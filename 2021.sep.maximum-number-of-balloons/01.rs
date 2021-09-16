// https://leetcode.com/submissions/detail/554251330/
use std::collections::HashMap;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut balloon: HashMap<char, i32> = HashMap::new();
        "balon".chars().for_each(|char| { balloon.insert(char, 0); });
        
        text.chars().for_each(|char| {
            match char {
                'b' | 'a' | 'l' | 'o' | 'n' => {
                    balloon.entry(char).and_modify(|c| *c += 1);
                },
                _ => {},
            }
        });
        
        balloon.entry('l').and_modify(|c| *c /= 2);        
        balloon.entry('o').and_modify(|c| *c /= 2);

        // println!("{:?}", balloon);
    
        balloon.into_iter().map(|(_, v)| v).min().unwrap()
    }
}