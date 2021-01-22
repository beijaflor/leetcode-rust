// https://leetcode.com/submissions/detail/445301771/
#[derive(Debug)]
struct Palin {
    start_at: usize,
    length: usize,
    is_definite: bool,
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut pointer = 0;
        let mut candidates: Vec<Palin> = vec![];

        let chars = s.chars().collect::<Vec<char>>();
        let len = chars.len();

        while pointer < chars.len() {
            let char = chars[pointer];
            let tmp_pointer = pointer;
            let mut length = 1;
            pointer += 1;
            while tmp_pointer + length < len && chars[tmp_pointer + length] == char {
                length += 1;
                pointer += 1;
                // println!("pointer: {:?}, length: {:?}, char: {:?}, next_char: {:?}", pointer, length, char.to_string(), chars[pointer + length].to_string());
            }
            
            let mut candidate = { Palin {
                start_at: tmp_pointer,
                length: length,
                is_definite: true,
            }};
            
            // println!("candidata: {:?}", candidate);
            // println!("candidate.start_at > 0: {:?}", candidate.start_at > 0);
            // println!("candidate.start_at + candidate.length < len: {:?}", candidate.start_at + candidate.length < len);
            // println!("chars[candidate.start_at - 1]: {:?}", if candidate.start_at > 0 { chars[candidate.start_at - 1].to_string() } else { "None".to_string() });
            // println!("chars[candidate.start_at + candidate.length]: {:?}", if candidate.start_at + candidate.length < len { chars[candidate.start_at + candidate.length].to_string() } else { "None".to_string() });

            if
                candidate.start_at > 0
                && candidate.start_at + candidate.length < len
                && chars[candidate.start_at - 1] == chars[candidate.start_at + candidate.length]
            {
                candidate.start_at -= 1;
                candidate.length += 2;
                candidate.is_definite = false;
                length += 2;
            }
            
            if length > 1 {
                candidates.push(candidate);
            }

            // println!("candidates: {:?}", candidates);
        }

        let mut flag = true;
        while flag {
            flag = false;
            for index in 0..candidates.len() {
                let candidate = &mut candidates[index];
                // println!("candidate: {:?}", *candidate);
                if !candidate.is_definite {
                    flag = true;
                    if
                        candidate.start_at > 0
                        && candidate.start_at + candidate.length < len
                        && chars[candidate.start_at - 1] == chars[candidate.start_at + candidate.length]
                    {
                        candidate.start_at -= 1;
                        candidate.length += 2;
                    } else {
                        candidate.is_definite = true;
                    }
                }
            }
            // println!("candidates: {:?}", candidates);
        }

        match candidates.iter().max_by(|lhv, rhv| lhv.length.cmp(&rhv.length)) {
            None => chars[0].to_string(),
            Some(result) => {
                // println!("result: {:?} = {}", result,  chars[result.start_at..result.start_at + result.length].iter().collect::<String>());
                chars[result.start_at..result.start_at + result.length].iter().collect()
            },
        }
    }
}