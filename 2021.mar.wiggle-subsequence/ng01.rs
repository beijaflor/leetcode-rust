use std::cmp::Ordering::{ Less, Greater, Equal};

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut iterator = nums.iter();
        let first = iterator.next();
        match first {
            None => 0,
            Some(first) => {
                let mut max = 1;
                let initial = (1, first, Equal);
                let r = iterator.fold(initial, |(count, last, cmp), current| {
                    println!("count {}, last {}, current {}, cmp {:?}", count, last, current, cmp);
                    if cmp == Equal {
                        max = i32::max(max, count + 1);
                        (count + 1, current, current.cmp(&last))                    
                    } else if (current - last < 0 && cmp == Greater) || (current - last > 0 && cmp == Less) {
                        max = i32::max(max, count + 1);
                        (count + 1, current, current.cmp(&last))
                    } else {
                        max = i32::max(max, 2);
                        (2, current, current.cmp(&last))                    
                    }
                });
                println!("{:?}", r);
                max
            }
        }
    }
}