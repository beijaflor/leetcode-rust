// https://leetcode.com/submissions/detail/468098526/
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

struct Codec {
    base_url: String,
	map: RefCell<HashMap<u64, String>>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {
            base_url: String::from("http://tinyurl.com/"),
            map: RefCell::new(HashMap::new()),
        }
    }
	
    // Encodes a URL to a shortened URL.
    fn encode(&self, longURL: String) -> String {
        let mut hasher = DefaultHasher::new();
        longURL.hash(&mut hasher);
        let hash = hasher.finish();
        let url = format!("{}{}", self.base_url, hash.to_string());
        self.map.borrow_mut().insert(hash, longURL);
        url
    }
	
    // Decodes a shortened URL to its original URL.
    fn decode(&self, shortURL: String) -> String {
        let hash = shortURL.replace(&self.base_url, "").parse::<u64>().unwrap();
        match self.map.borrow().get(&hash) {
            None => String::from(""),
            Some(url) => url.to_string(),
        }
    }
}
/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: VecVec<String> = obj.decode(s);
 */