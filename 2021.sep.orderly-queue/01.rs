// https://leetcode.com/submissions/detail/550148736/
impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        if k == 1 {
            let mut result: String = chars.iter().collect();
            (0..chars.len()).for_each(|index| {
                let mut tmp = chars[index..chars.len()].to_vec();
                tmp.append(&mut chars[0..index].to_vec());
                let tmp: String = tmp.into_iter().collect();
                // println!("{:?}", tmp);
                if tmp.cmp(&result) == std::cmp::Ordering::Less {
                    result = tmp;
                }
            });
            result
        } else {
            chars.sort();
            chars.into_iter().collect()
        }
    }
}