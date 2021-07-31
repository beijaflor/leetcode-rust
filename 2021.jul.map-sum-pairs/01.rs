// https://leetcode.com/submissions/detail/530897772
/*
["MapSum", "insert", "sum", "insert", "sum"]
[[], ["apple",3], ["ap"], ["app",2], ["ap"]]
["MapSum", "insert", "sum", "insert", "sum"]
[[], ["apple",3], ["ap"], ["app",2], ["apb"]]
["MapSum", "insert", "sum", "insert", "sum"]
[[], ["apple",3], ["apple"], ["app",2], ["ap"]]
["MapSum", "insert", "sum", "insert", "insert", "sum"]
[[], ["apple",3], ["ap"], ["app",2], ["apple", 2], ["ap"]]
*/
use std::collections::HashMap;

struct MapSum {
    prefix_map: HashMap<String, i32>,
    value_map: HashMap<String, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MapSum {
            prefix_map: HashMap::new(),
            value_map: HashMap::new(),
        }
    }
    
    fn insert(&mut self, key: String, val: i32) {
        let diff = val - *self.value_map.get(&key).unwrap_or(&0);
        self.value_map.insert(key.clone(), val);
        // println!("{}", diff);
        
        (0..key.len() + 1).for_each(|index| {
            let prefix_key = key[0..index].to_string();
            // println!("prefix_key {}", key[0..index].to_string());
            self.prefix_map.entry(prefix_key).and_modify(|c| *c += diff).or_insert(diff);
        });
    }
    
    fn sum(&self, prefix: String) -> i32 {
        // println!("{:?}", self.prefix_map);
        *self.prefix_map.get(&prefix).unwrap_or(&0)
    }
}

/**
 * Your MapSum object will be instantiated and called as such:
 * let obj = MapSum::new();
 * obj.insert(key, val);
 * let ret_2: i32 = obj.sum(prefix);
 */