// https://leetcode.com/submissions/detail/503930771/
use std::collections::BTreeSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 { return 0 }
        let mut heap: BTreeSet<i32> = BTreeSet::new();
        nums.iter().for_each(|num| {
            heap.insert(*num);
        });
        // println!("{:?}", heap);

        let mut longest = 0;
        nums.into_iter().for_each(|num| {
            if !heap.contains(&(num - 1)) {
                let mut current_num = num;
                let mut current_streak = 1;
                
                while heap.contains(&(current_num + 1)) {
                    current_num += 1;
                    current_streak += 1;
                }
                
                longest = i32::max(longest, current_streak);
            }
        });
        
        longest
    }
}

/*
class Solution {
    public int longestConsecutive(int[] nums) {
        Set<Integer> num_set = new HashSet<Integer>();
        for (int num : nums) {
            num_set.add(num);
        }

        int longestStreak = 0;

        for (int num : num_set) {
            if (!num_set.contains(num-1)) {
                int currentNum = num;
                int currentStreak = 1;

                while (num_set.contains(currentNum+1)) {
                    currentNum += 1;
                    currentStreak += 1;
                }

                longestStreak = Math.max(longestStreak, currentStreak);
            }
        }

        return longestStreak;
    }
}
*/