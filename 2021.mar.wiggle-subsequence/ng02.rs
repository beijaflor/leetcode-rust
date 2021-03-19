use std::cmp::Ordering::{ Less, Greater, Equal};

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut last = nums[0];
        let mut last_cmp = Equal;
        let filtered: Vec<i32> = nums.into_iter().filter(|v| {
            if last_cmp == Equal {
                last_cmp = last.cmp(v);
                last = *v;
                last_cmp != Equal
            } else {                
                let cmp = last.cmp(v);
                match cmp {
                    Equal => {
                        last_cmp = last.cmp(v);
                        false
                    },
                    Greater => if last_cmp == Less {
                        last = *v;
                        last_cmp = Greater;
                        true
                    } else {
                        last = i32::min(*v, last);
                        false
                    },
                    Less => if last_cmp == Greater {
                        last = *v;
                        last_cmp = Less;
                        true
                    } else {
                        last = i32::max(*v, last);
                        false
                    }
                }
            }
        }).collect();
        println!("{:?}", filtered);
        (filtered.len() + 1) as i32
    }
}