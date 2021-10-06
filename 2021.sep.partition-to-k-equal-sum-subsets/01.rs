// https://leetcode.com/submissions/detail/563500332/
fn rec(nums: &mut Vec<i32>, visited: &mut Vec<bool>, current: i32, target: i32, offset: usize) -> bool {
    // println!("current: {}, offset: {}", current, offset);
    if current == target {
        // println!("true");
        true
    } else {
        let mut p = offset;
        while p < nums.len() {
            // println!("p {}", p);
            if !visited[p] && current + nums[p] <= target {
                // println!("{}", nums[p]);
                visited[p] = true;
                if rec(nums, visited, current + nums[p], target, p + 1) {
                    return true
                } else {
                    visited[p] = false;
                }
            }
            p += 1;
        }
        false
    }
}

impl Solution {
    pub fn can_partition_k_subsets(mut nums: Vec<i32>, k: i32) -> bool {
        let total: i32 = nums.iter().sum();
        if total % k != 0 {
            false
        } else {
            nums.sort();
            nums.reverse();
            let sum = total / k;
            let mut visited: Vec<bool> = vec![false; nums.len()];
            // println!("{:?}", nums);
            for _ in 0..k {
                if !rec(&mut nums, &mut visited, 0, sum, 0) {
                    return false
                }
            };
            
            true
        }
    }
}