impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        for i in 0..words.len() {
            'outer: for j in 0..words.len() {
                if i == j { continue }
                
                let mut chars = words[i].chars().collect::<Vec<char>>();
                let mut c2 = words[j].chars().collect::<Vec<char>>();
                chars.append(&mut c2);
                let len = chars.len();
                for index in 0..(len / 2) {
                    if chars[index] != chars[len - index - 1] {
                        continue 'outer
                    }
                }
                result.push(vec![i as i32, j as i32]);
            }
        }

        result
    }
}