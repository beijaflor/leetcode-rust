// https://leetcode.com/submissions/detail/461701171/
use std::collections::{ HashMap };

struct FreqStack {
    stack: Vec<i32>,
    frequency: HashMap<i32, i32>,

}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {

    fn new() -> Self {
        return FreqStack {
            stack: Vec::new(),
            frequency: HashMap::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        self.frequency.entry(x).and_modify(|v| *v += 1).or_insert(1);
        self.stack.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        // println!("frequency: {:?}", self.frequency);
        let max = self.frequency.iter().fold(0, |acc, (_, value)| i32::max(acc, *value));
        let candidate: Vec<i32> = self.frequency.iter().filter(|(_, &value)| value == max).map(|(key, _)| *key).collect();
        let position: usize = self.stack.iter().rev().position(|v| candidate.iter().find(|vv| *v == **vv).is_some()).unwrap();
        let item = *self.stack.get(self.stack.len() - position - 1).unwrap();
        // println!("max: {}, candidate: {:?}", max, candidate);
        // println!("position: {}, stack: {:?}", position, self.stack);
        // println!("item: {}", item);
        self.stack.remove(self.stack.len() - position - 1);
        self.frequency.entry(item).and_modify(|c| *c -= 1);
        item
    }
}

/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 */