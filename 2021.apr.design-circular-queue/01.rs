// https://leetcode.com/submissions/detail/476425280/
use std::collections::VecDeque;

struct MyCircularQueue {
    queue: VecDeque<i32>,
    size: usize,
    usage: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {

    fn new(k: i32) -> Self {
        let size = k as usize;
        { MyCircularQueue {
            queue: VecDeque::with_capacity(size),
            size: size,
            usage: 0,
        }}
    }
    
    fn en_queue(&mut self, value: i32) -> bool {
        if self.size == self.usage { return false }
        self.queue.push_back(value);
        self.usage += 1;
        true
    }
    
    fn de_queue(&mut self) -> bool {
        if self.usage == 0 { return false }
        self.usage -= 1;
        self.queue.pop_front();
        true
    }
    
    fn front(&self) -> i32 {
        match self.queue.front() {
            None => -1,
            Some(val) => *val,
        }
    }
    
    fn rear(&self) -> i32 {
        match self.queue.back() {
            None => -1,
            Some(val) => *val,
        }
    }
    
    fn is_empty(&self) -> bool {
        self.usage == 0
    }
    
    fn is_full(&self) -> bool {
        self.size == self.usage
    }
}

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */