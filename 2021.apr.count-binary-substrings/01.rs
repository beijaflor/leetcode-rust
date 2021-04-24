// https://leetcode.com/submissions/detail/484402367/
impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut counts: Vec<i32> = vec![];
        let mut chars = s.chars();
        let mut current = chars.next().unwrap();
        let mut count = 1;
        chars.for_each(|c| {
            if c == current {
                count += 1;
            } else {
                counts.push(count);
                current = c;
                count = 1;
            }
        });
        counts.push(count);

        // println!("counts: {:?}\n\n", counts);

        let mut result = 0;
        for index in 0..(counts.len() - 1) {
            result += i32::min(counts[index], counts[index + 1]);
        }
        
        result
    }
}