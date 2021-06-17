use std::collections::HashSet;
use std::iter::FromIterator;

fn dig(n: i32) -> Vec<Vec<i32>> {
    if n > 1 {
        let mut ret = dig(n - 1);
        let mut hash: HashSet<Vec<i32>> = HashSet::new();
        ret.into_iter().for_each(|vec| {
            let mut v1 = vec.clone();
            v1.push(1);
            hash.insert(v1);
            let mut v2 = vec.clone();
            v2.insert(0, 1);
            hash.insert(v2);
            (0..vec.len()).for_each(|index| {
                let mut v3 = vec.clone();
                v3[index] += 1;
                hash.insert(v3);
            });
        });
        Vec::from_iter(hash)
    } else {
        vec![vec![1]]
    }
}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let result = dig(n);
        println!("{:?}", result);
        vec![]
    }
}