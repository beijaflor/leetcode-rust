impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let ill = String::from("");
        let mut word_chars: Vec<Vec<char>> = words.into_iter().map(|word| word.chars().collect::<Vec<char>>()).collect();
        let mut dict: Vec<char> = Vec::new();
        let mut index = 0;
        
        loop {
            word_chars = word_chars.into_iter().filter(|chars| chars.len() < index).collect();
            if word_chars.len() < 2 { break }
            let mut last: char = word_chars[0][index];
            for row in 1..word_chars.len() {
                let char = word_chars[row][index];
                if last != char {
                    let last_position = dict.iter().position(|d| *d == last).unwrap();
                    match dict.iter().position(|d| *d == char) {
                        None => {
                            dict.insert(last_position + 1, char);
                        },
                        Some(position) => {
                            if position < last_position {
                                return ill
                            }
                        },
                    }
                }
                last = char;
                index += 1;
            }
        }
        dict.into_iter().collect::<String>()
    }
}