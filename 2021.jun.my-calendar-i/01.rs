// https://leetcode.com/submissions/detail/505840035/
use std::collections::BinaryHeap;

struct MyCalendar {
    books: BinaryHeap<(i32, i32)>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {

    fn new() -> Self {
        MyCalendar { books: BinaryHeap::new() }
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        if self.is_intersect(start, end) {
            false
        } else {            
            self.books.push((start, end));
            true
        }
    }
    
    fn is_intersect(&mut self, start: i32, end: i32) -> bool {
        let sorted = self.books.clone().into_sorted_vec();
        for index in 0..sorted.len() {
            if start >= sorted[index].0 && start < sorted[index].1 {
                return true
            }
            if end > sorted[index].0 && end <= sorted[index].1 {
                return true
            }
            if start < sorted[index].0 && sorted[index].1 < end {
                return true
            }
        }
        false
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */