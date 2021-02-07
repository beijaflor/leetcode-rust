// https://leetcode.com/submissions/detail/453254565/
fn reverse(vec: &mut Vec<i32>) {
    let mut counter = 1;
    for index in (0..vec.len()).rev() {
        let val = vec[index];
        if val <= counter {
            break
        }
        vec[index] = counter;
        counter += 1;
    }
}

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut result = Vec::with_capacity(s.len());
        let mut counter = 10_000;
        for char in s.chars() {
            if char == c {
                reverse(&mut result);
                result.push(0);
                counter = 1;
            } else {
                result.push(counter);
                counter += 1;
            }
        }
        result
    }
}