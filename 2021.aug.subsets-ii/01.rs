// https://leetcode.com/submissions/detail/532673465/
use std::collections::HashSet;

fn helper(subsets: &mut HashSet<Vec<i32>>, current_subset: &mut Vec<i32>, nums: &Vec<i32>, index: i32) {
    subsets.insert(current_subset.clone());
    
    ((index as usize)..nums.len()).for_each(|index2| {
        if index2 != index as usize && index > 0 && nums[index2] == nums[index as usize - 1] {
            return
        } else {
            current_subset.push(nums[index2]);
            helper(subsets, current_subset, nums, index2 as i32 + 1);
            current_subset.remove(current_subset.len() - 1);
        }
    });
}

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut subsets: HashSet<Vec<i32>> = HashSet::new();
        let mut current_subset: Vec<i32> = vec![];
        
        helper(&mut subsets, &mut current_subset, &nums, 0);
        
        subsets.into_iter().collect()
    }
}