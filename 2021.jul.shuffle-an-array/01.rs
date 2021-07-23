// https://leetcode.com/submissions/detail/525526493/
use rand::seq::SliceRandom;

struct Solution {
    original: Vec<i32>,
    random: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        Solution {
            original: nums.clone(),
            random: nums,
        }
    }
    
    /** Resets the array to its original configuration and return it. */
    fn reset(&mut self) -> Vec<i32> {
        self.random = self.original.clone();
        self.original.clone()
    }
    
    /** Returns a random shuffling of the array. */
    fn shuffle(&mut self) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        self.random.shuffle(&mut rng);
        self.random.clone()
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: Vec<i32> = obj.reset();
 * let ret_2: Vec<i32> = obj.shuffle();
 */