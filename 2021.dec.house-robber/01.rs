// https://leetcode.com/submissions/detail/595308975/
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 { return 0 }
        let mut iterator = nums.into_iter();
        let mut next = 0;
        let mut curr = iterator.next_back().unwrap();
        while let Some(num) = iterator.next_back() {
            let current = i32::max(curr, next + num);
            next = curr;
            curr = current;
        }
        curr
    }
}