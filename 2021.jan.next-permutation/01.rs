// https://leetcode.com/submissions/detail/450135334/
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
        // println!("position: {:?}", position);

        if let Some(rep) = nums.iter().rev().position(|v| *v > nums[position]) {
            let rep_pos = nums.len() - rep - 1;
            let tmp = nums[rep_pos];
            nums[rep_pos] = nums[position];
            nums[position] = tmp;
            
            let mut clone: Vec<i32> = Vec::with_capacity(nums.len() - position);
            for pos in (position + 1)..nums.len() {
                clone.push(nums[pos]);
            }
            clone.sort();
            // println!("{:?}", clone);
            nums.splice(position + 1..nums.len(), clone).collect::<Vec<i32>>();
        };
    }

}