// https://leetcode.com/submissions/detail/563197320/
impl Solution {
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        let mut sub_total = 0;
        let mut prefix_sum: Vec<i32> = nums.into_iter().map(|num| {
            sub_total += num;
            sub_total
        }).collect();
        
        // println!("{:?}", prefix_sum);
        
        for len in (0..prefix_sum.len()).rev() {
            for slide in (0..prefix_sum.len() - len) {
                // println!("{} {}", len, slide);
                if slide == 0 {
                    if prefix_sum[len] == k {
                        return (len + 1) as i32
                    }
                } else {
                    if prefix_sum[len + slide] - prefix_sum[slide - 1] == k {
                        return (len + 1) as i32
                    }
                }
            }
        }
        
        0
    }
}