// https://leetcode.com/submissions/detail/576754621/

use std::collections::BTreeSet;

struct MinStack {
    min: i32,
    stack: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        MinStack {
            min: i32::MAX,
            stack: Vec::new(),
        }
    }
    
    fn push(&mut self, val: i32) {
        self.stack.push(val);
        self.min = self.min.min(val);
    }
    
    fn pop(&mut self) {
        if let Some(popped) = self.stack.pop() {
            if popped == self.min {
                if self.stack.is_empty() {
                    self.min = i32::MAX;
                } else {
                    let mut clone = self.stack.clone();
                    clone.sort();
                    self.min = clone[0];
                }
            }
        }
    }
    
    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        self.min
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */