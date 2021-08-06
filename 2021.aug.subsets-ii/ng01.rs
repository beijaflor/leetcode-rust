use std::collections::HashSet;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut set: HashSet<Vec<i32>> = HashSet::new();
        set.insert(vec![]);
        (0..nums.len()).for_each(|index| {
            let mut subset: Vec<i32> = vec![];
            (index..nums.len()).for_each(|index2| {
                subset.push(nums[index2]);
                set.insert(subset.clone());
            });
        });

        set.into_iter().collect()
    }
}