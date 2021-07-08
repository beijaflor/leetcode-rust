// https://leetcode.com/submissions/detail/519332901/
fn find_next_space(s: &mut Vec<char>, p: usize) -> usize {
    for index in (p + 1)..s.len() {
        if s[index] == ' ' {
            return index
        }
    }
    s.len()
}

impl Solution {
    pub fn reverse_words(s: &mut Vec<char>) {
        s.reverse();
        let mut p1 = 0;
        let mut p2 = find_next_space(s, 0);
        while p1 < s.len() {
            (0..((p2 - p1) / 2)).for_each(|index| {
                // println!("p1 {}, p2 {}, index {}", p1, p2, index);
                s.swap(p1 + index, p2 - index - 1);
            });
            
            p1 = p2 + 1;
            p2 = find_next_space(s, p2);
        }
    }
}