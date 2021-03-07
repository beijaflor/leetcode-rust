// https://leetcode.com/submissions/detail/464703319/
struct MyHashMap {
    map: Vec<Option<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    fn fill(&mut self, key: i32) {
        for index in self.map.len()..(key + 1) as usize {
            self.map.push(None);
        }
    }
    
    /** Initialize your data structure here. */
    fn new() -> Self {
        const map: Vec<Option<i32>> = Vec::new();
        MyHashMap {
            map,
        }
    }
    
    /** value will always be non-negative. */
    fn put(&mut self, key: i32, value: i32) {
        self.fill(key);
        self.map[key as usize] = Some(value);
    }
    
    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    fn get(&mut self, key: i32) -> i32 {
        self.fill(key);
        match self.map.get(key as usize).unwrap() {
            None => -1,
            Some(v) => *v,
        }
    }
    
    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    fn remove(&mut self, key: i32) {
        self.fill(key);
        self.map[key as usize] = None;
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */