use std::collections::HashMap;

struct MapSum {
    map: HashMap<String, i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MapSum {
            map: HashMap::new(),
        }
    }
    
    fn insert(&mut self, key: String, val: i32) {
        self.map.insert(key, val);
    }
    
    fn sum(&self, prefix: String) -> i32 {
        println!("{:?}", self.map);
        (0..prefix.len() + 1).fold(0, |acc, index| {
            let prefix_key = prefix[0..index].to_string();
            println!("{:?}", prefix_key);
            acc + *self.map.get(&prefix_key).unwrap_or(&0)
        })
    }
}

/**
 * Your MapSum object will be instantiated and called as such:
 * let obj = MapSum::new();
 * obj.insert(key, val);
 * let ret_2: i32 = obj.sum(prefix);
 */