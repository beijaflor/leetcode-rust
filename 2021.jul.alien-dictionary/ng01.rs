/*
["wrt","wrf","er","ett","rftt"]
["z","x"]
["z","x","z"]
*/
impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let ill = String::from("");
        let mut dict: Vec<char> = Vec::new();
        let mut last: char = ' ';
        for w in words.iter() {
            let c = w.chars().next().unwrap();
            if last != c {
                match dict.iter().find(|d| *d == &c) {
                    None => dict.push(c),
                    Some(d) => return ill,
                }
            }
            last = c;
        };

        for w in words.iter() {
            let mut chars = w.chars();
            let mut last: char = chars.next().unwrap();
            for char in chars {
                println!("char: {}", char);
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
                last = char
            }
        }

        dict.into_iter().collect::<String>()
    }
}