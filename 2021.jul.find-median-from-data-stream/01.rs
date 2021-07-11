// https://leetcode.com/submissions/detail/520999496/
use std::collections::BinaryHeap;

struct MedianFinder {
    cache: Vec<i32>,
    len: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    /** initialize your data structure here. */
    fn new() -> Self {
        MedianFinder { cache: Vec::new(), len: 0 }
    }
    
    fn add_num(&mut self, num: i32) {
        match self.cache.binary_search(&num) {
            Ok(index) | Err(index) => {
                self.cache.insert(index, num);
            }
        }
        self.len += 1;
    }
    
    fn find_median(&self) -> f64 {
        if self.len == 1 {
            *self.cache.get(self.len - 1).unwrap() as f64
        } else if self.len % 2 == 1 {
            // println!("odd");
            *self.cache.get(self.len / 2).unwrap() as f64
        } else {
            // println!("even");
            (*self.cache.get(self.len / 2 - 1).unwrap() as f64 + *self.cache.get(self.len / 2).unwrap() as f64) / 2.0f64
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */