// https://leetcode.com/submissions/detail/464418351/
impl Solution {
    pub fn minimum_length_encoding(mut words: Vec<String>) -> i32 {
        let mut encoded: String = String::from("");

        words.sort_by(|a, b| b.len().cmp(&a.len()));

        for s in words {
            // println!("{}", s);
            let w: String = s.to_string() + &String::from("#");
            let m = encoded.find(&w);
            // println!("{:?}", m);
            if m == None {
                encoded += &w;
            }
        }
        
        // println!("{}", encoded);
        encoded.len() as i32
    }
}