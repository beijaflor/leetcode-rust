// https://leetcode.com/submissions/detail/511579295/
impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut result = 0;
        let target = s.chars().collect::<Vec<char>>();
        words.into_iter().for_each(|word| {
            // println!("\nword: {:?}", word);
            let mut pointer = 0;
            if !word.chars().find(|char| {
                while pointer < target.len() {
                    // println!("char: {}, target: {}", char, target[pointer]);
                    if &target[pointer] == char {
                        pointer += 1;
                        return false
                    }
                    pointer += 1;
                }
                return true
            }).is_some() {
                // println!("added");
                result += 1;
            }
        });
        result
    }
}