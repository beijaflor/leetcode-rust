// https://leetcode.com/submissions/detail/498999182/
impl Solution {
    pub fn max_product(mut words: Vec<String>) -> i32 {
        words.sort_by(|l, r| r.len().cmp(&l.len()));
        let mut p = 0;
        let mut result: i32 = 0;
        while p < words.len() {
            if result > (words[p].len() * words[p].len()) as i32 {
                break
            }
            let chars = words[p].chars().collect::<Vec<char>>();
            let target = words[(p + 1)..words.len()].iter().find(|word| {
                !chars.iter().find(|c| word.find(**c).is_some()).is_some()
            });
            if let Some(target) = target {
                result = i32::max(result, (words[p].len() * target.len()) as i32);
            }
            p += 1;
        }
        result
    }
}