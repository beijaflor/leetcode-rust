// https://leetcode.com/submissions/detail/591564013/
use std::collections::HashMap;

struct FirstUnique {
    queue: Vec<i32>,
    appeirs: HashMap<i32, bool>,
    position: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FirstUnique {

    fn new(nums: Vec<i32>) -> Self {
        let mut fu = FirstUnique {
            queue: Vec::new(),
            appeirs: HashMap::new(),
            position: 0,
        };
        nums.into_iter().for_each(|num| { fu.add(num); });
        fu
    }
    
    fn show_first_unique(&self) -> i32 {
        if self.position == self.queue.len() {
            -1
        } else {
            self.queue[self.position]        
        }
    }
    
    fn add(&mut self, value: i32) {
        self.queue.push(value);
        if let Some(state) = self.appeirs.get(&value) {
            if !state {
                self.appeirs.insert(value, true);
            }
        } else {
            self.appeirs.insert(value, false);
        }

        while let Some(current) = self.queue.get(self.position) {
            if let Some(state) = self.appeirs.get(current) {
                if *state {
                    self.position += 1;
                    continue
                }
            }
            break
        }
        // println!("{:?} {:?} {}", self.queue, self.appeirs, self.position);
    }
}

/**
 * Your FirstUnique object will be instantiated and called as such:
 * let obj = FirstUnique::new(nums);
 * let ret_1: i32 = obj.show_first_unique();
 * obj.add(value);
 */