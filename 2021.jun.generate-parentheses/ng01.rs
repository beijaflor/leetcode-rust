use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n > 1 {
            let mut ret = Solution::generate_parenthesis(n - 1);
            let mut hash: HashSet<String> = HashSet::new();
            ret.into_iter().flat_map(|str| {
                let mut vec = Vec::with_capacity(3);
                vec.push(format!("({})", str));
                vec.push(format!("(){}", str));
                vec.push(format!("{}()", str));
                vec
            }).for_each(|str| {
                hash.insert(str);
            });
            Vec::from_iter(hash)
        } else {
            vec!["()".to_string()]
        }
    }
}