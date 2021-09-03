// https://leetcode.com/submissions/detail/548843761/
use std::collections::{VecDeque, HashSet};

fn orientation(p: &Vec<i32>, q: &Vec<i32>, r: &Vec<i32>) -> i32 {
    (q[1] - p[1]) * (r[0] - q[0]) - (q[0] - p[0]) * (r[1] - q[1])
}

impl Solution {
    pub fn outer_trees(mut trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        trees.sort_by(|lhv, rhv| {
            if lhv[0] == rhv[0] {
                lhv[1].cmp(&rhv[1])
            } else {
                lhv[0].cmp(&rhv[0])
            }
        });
        
        let mut hull: VecDeque<Vec<i32>> = VecDeque::new();
        
        (0..trees.len()).for_each(|index| {
            while hull.len() >= 2 && orientation(hull.get(hull.len() - 2).unwrap(), hull.get(hull.len() - 1).unwrap(), &trees[index]) > 0 {
                hull.pop_back();
            }
            hull.push_back(trees[index].clone());
        });
        
        hull.pop_back();
        println!("{:?}", hull);
        
        (0..trees.len()).rev().for_each(|index| {
            while hull.len() >= 2 && orientation(hull.get(hull.len() - 2).unwrap(), hull.get(hull.len() - 1).unwrap(), &trees[index]) > 0 {
                hull.pop_back();
            }
            hull.push_back(trees[index].clone());
        });
        
        let mut set: HashSet<Vec<i32>> = HashSet::new();
        hull.into_iter().for_each(|v| {
            set.insert(v);
        });
        
        set.into_iter().collect()
    }
}