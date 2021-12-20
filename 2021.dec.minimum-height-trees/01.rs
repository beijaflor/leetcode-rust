// https://leetcode.com/submissions/detail/602520748/
use std::collections::HashSet;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0]
        }
        
        let mut neighbors = vec![HashSet::<i32>::new(); n as usize];
        edges.into_iter().for_each(|edge| {
            let start = edge[0];
            let end = edge[1];
            neighbors[start as usize].insert(end);
            neighbors[end as usize].insert(start);
        });
        
        // println!("neighbors: {:?}", neighbors);
        
        let mut leaves = (0..n).filter(|index| {
            neighbors[*index as usize].len() == 1
        }).collect::<Vec<i32>>();
        
        // println!("leaves: {:?}", leaves);
        
        let mut remaining = n;
        while remaining > 2 {
            remaining -= leaves.len() as i32;
            let mut new_leaves = vec![];
            
            leaves.iter().for_each(|leaf| {
                let neighbor = if let Some(neighbor) = neighbors[*leaf as usize].iter().next() { *neighbor } else { return };
                neighbors[neighbor as usize].remove(leaf);
                if neighbors[neighbor as usize].len() == 1 {
                    new_leaves.push(neighbor);
                }
            });
            leaves = new_leaves;
        }
        
        leaves
    }
}