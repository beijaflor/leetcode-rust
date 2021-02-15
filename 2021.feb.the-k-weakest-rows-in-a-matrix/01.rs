// https://leetcode.com/submissions/detail/456362726/
use std::collections::BTreeSet;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut set: BTreeSet<(i32, usize)> = BTreeSet::new();
        mat.iter().enumerate().for_each(|(index, vec)| {
            let count = vec.iter().fold(0, |acc, v| acc + v);
            set.insert((count, index));
        });
        // println!("{:?}", set);
        let mut iterator = set.into_iter();
        let mut result = Vec::with_capacity(k as usize);
        for _ in 0..k {
            result.push(iterator.next().unwrap().1 as i32);
        }
        // println!("{:?}", result);
        result
    }
}