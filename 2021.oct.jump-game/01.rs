// https://leetcode.com/submissions/detail/564831182/
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() < 2 { return true }
        let goal = nums.len() - 1;
        let mut p = 0;
        while p < goal {
            // println!("{}", p);
            let steps = nums[p] as usize;
            if steps == 0 { return false }
            let mut next = p;
            let mut max = 0;
            // println!("{}", steps);
            for index in 1..=steps {
                // println!("{}", index);
                if p + index >= goal { return true }
                if max < index + (nums[p + index] as usize) {
                    next = p + index;
                    max = index + (nums[p + index] as usize);
                }
            };
            if p == next { return false }
            p = next
        }
        true
    }
}