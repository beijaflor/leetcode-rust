// https://leetcode.com/submissions/detail/548093907/
impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut visited: Vec<bool> = vec![false; nums.len()];
        let mut result = 0;
        (0..nums.len()).for_each(|start| {
            if visited[start] {
                return
            }
            let mut p = start as usize;
            p = nums[p] as usize;
            let mut current = 1;
            while start != p {
                p = nums[p] as usize;
                current += 1;
                visited[p] = true;
            }
            result = i32::max(result, current)
        });
        
        result
    }
}