// https://leetcode.com/submissions/detail/450138331/
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut position: usize = std::usize::MAX;
        for index in (0..(nums.len() - 1)).rev() {
            if nums[index] < nums[index + 1] {
                position = index;
                break
            }
        }

        if position == std::usize::MAX {
            nums.sort_by(|a, b| a.cmp(b));
            return
        }
        println!("position: {:?}", position);

        if let Some(rep) = nums.iter().rev().position(|v| *v > nums[position]) {
            let rep_pos = nums.len() - rep - 1;
            nums.swap(rep_pos, position);

            let mut pointer = nums.len() - 1;
            position += 1;
            while position < pointer {
                nums.swap(position, pointer);
                position += 1;
                pointer -= 1;
            }
        };
    }
}