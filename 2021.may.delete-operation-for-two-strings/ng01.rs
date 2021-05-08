// 文字順が重要だった
/*
"sea"
"eat"
"leetcode"
"etco"
"sea"
"ate"
*/
use std::collections::HashMap;

// for offset in 0..26 {
//     println!("{}", (('a' as u8) + offset) as char);
// }
        
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut counter: HashMap<char, i32> = HashMap::new();
        
        word1.chars().for_each(|char| {
            counter.entry(char).and_modify(|c| *c += 1).or_insert(1);
        });

        word2.chars().for_each(|char| {
            counter.entry(char).and_modify(|c| *c -= 1).or_insert(-1);
        });

        println!("{:?}", counter);
        
        counter.into_iter().fold(0, |acc, (_, cur)| {
            acc + cur.abs()
        })
    }
}