// https://leetcode.com/submissions/detail/462778725/
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();

        let mut last = std::i32::MIN;
        let wrong_pos = nums.iter().position(|v| {
            // println!("{}, {}", last, *v);
            if *v == last {
                true
            } else {
                last = *v;
                false
            }
        }).unwrap();
        // println!("wrong_pos {}", wrong_pos);
        let wrong = *nums.get(wrong_pos).unwrap();
        nums.remove(wrong_pos);
        
        let missing = match nums.iter().enumerate().position(|(index, v)| *v != (index + 1) as i32) {
            None => nums.len(),
            Some(v) => v
        } as i32 + 1;
        // println!("missing {}", missing);
        vec![wrong, missing]
    }
}