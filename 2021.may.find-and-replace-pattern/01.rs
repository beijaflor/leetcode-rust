// https://leetcode.com/submissions/detail/496270593/
fn convert(s: String) -> String {
    let mut bytes: Vec<u8> = s.as_bytes().to_vec();
    // println!("{:?}", bytes);
    let mut r: u8 = 'A' as u8;
    loop {
        let u = match bytes.iter().find(|u| *u > &('Z' as u8)) { None => break, Some(u) => *u };
        // println!("u8 {:?}, char {:?}", u, u as char);
        for index in 0..bytes.len() {
            if bytes[index] == u {
                bytes[index] = r
            }
        }
        r += 1;
        // println!("bytes: {:?}", bytes);
    }
    String::from_utf8(bytes.to_vec()).unwrap()
}

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let pattern = convert(pattern);
        let mut result: Vec<String> = Vec::new();
        
        words.into_iter().for_each(|word| {
            if pattern == convert(word.clone()) {
                result.push(word);
            }
            // println!("word: {:?}, convert: {:?}", word.clone(), convert(word));
        });
        
        result
    }
}