// https://leetcode.com/submissions/detail/523471034/
impl Solution {
    pub fn get_modified_array(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {
        let len = length as usize;
        let mut seq: Vec<i32> = vec![0; len + 1];
        
        updates.into_iter().for_each(|update| {
            seq[update[0] as usize] += update[2];
            seq[update[1] as usize + 1] += -(update[2]);
        });
        
        let mut result = Vec::with_capacity(len);
        (0..len).fold(0, |acc, index| {
            let current = acc + seq[index];
            result.push(current.clone());
            current
        });
        
        result
    }
}