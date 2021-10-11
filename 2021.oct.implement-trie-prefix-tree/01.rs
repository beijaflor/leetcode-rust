// https://leetcode.com/submissions/detail/567845397/
use std::collections::HashMap;

#[derive(Debug, Default)]
struct TrieNode {
    is_end: bool,
    children: HashMap<char, Box<TrieNode>>,
}

#[derive(Debug)]
struct Trie {
    root: Box<TrieNode>,
}

impl TrieNode {
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Trie {
            root: Box::new(TrieNode {
                is_end: false,
                children: HashMap::new(),
            })
        }
    }
    
    fn insert(&mut self, word: String) {
        let mut node: &mut TrieNode = &mut self.root;
        for char in word.chars() {
            node = node.children.entry(char).or_insert(Default::default());
        }
        node.is_end = true;
        // println!("{:?}", self);
    }
    
    fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for char in word.chars() {
            if let Some(next) = node.children.get(&char) {
                node = next;
            } else {
                return false
            };
        }
        node.is_end
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for char in prefix.chars() {
            if let Some(next) = node.children.get(&char) {
                node = next;
            } else {
                return false
            };
        }
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */