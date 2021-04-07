// https://leetcode.com/submissions/detail/477732044/
fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
        _ => false,
    }
}

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let first_half = s[0..s.len()/2].chars().fold(0, |acc, current| {
            if is_vowel(current) {
                acc + 1
            } else {
                acc
            }
        });
        let second_half = s[(s.len()/2)..s.len()].chars().fold(0, |acc, current| {
            if is_vowel(current) {
                acc + 1
            } else {
                acc
            }
        });
        // println!("{:?}, {:?}", first_half, second_half);
        first_half == second_half
    }
}