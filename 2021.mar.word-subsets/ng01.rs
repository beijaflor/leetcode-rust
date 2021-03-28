/*
["amazon","apple","facebook","google","leetcode"]
["e","o"]
["amazon","apple","facebook","google","leetcode"]
["l","e"]
["amazon","apple","facebook","google","leetcode"]
["e","oo"]
["amazon","apple","facebook","google","leetcode"]
["lo","eo"]
["amazon","apple","facebook","google","leetcode"]
["ec","oc","ceo"]
*/
impl Solution {
    pub fn word_subsets(a: Vec<String>, b: Vec<String>) -> Vec<String> {
        a.into_iter().filter(|word| {
            let mut word = word.to_string();
            if b.iter().find(|dict| {
                let pos = word.find(*dict);
                match pos {
                    None => true,
                    Some(pos) => {
                        let len = dict.len();
                        let replace = (0..len).map(|_| "*").collect::<Vec<&str>>().join("");
                        word.replace_range(pos..(pos + len - 1), &replace);
                        false
                    }
                }
            }).is_some() {
                false
            } else {
                true
            }
        }).collect::<Vec<String>>()
    }
}