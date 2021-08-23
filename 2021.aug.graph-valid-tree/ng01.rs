/*
7
[[0,1],[0,2],[0,3],[1,4],[5,0],[6,0]]
5
[[0, 1], [1, 2], [3, 4]]
5
[[0,1],[1,2],[2,3],[1,3],[1,4]]
*/

use std::collections::{VecDeque, HashMap};

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        // let mut p = edges[0][0];
        let mut visited: Vec<bool> = vec![false; n as usize];

        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        edges.into_iter().for_each(|edge| {
            map.entry(edge[0]).and_modify(|c| c.push(edge[1])).or_insert(vec![edge[1]]);
            map.entry(edge[1]).and_modify(|c| c.push(edge[0])).or_insert(vec![edge[0]]);
        });
        
        let mut tip: Vec<i32> = (0..n).filter(|index| {
            if let Some(dir) = map.get(index) {
                dir.len() == 1
            } else {
                false
            }            
        }).collect();
        
        println!("tip: {:?}", tip);
        println!("map: {:?}", map);

        if let Some(mut p) = tip.pop() {
            visited[p as usize] = true;
            let mut q: VecDeque<i32> = VecDeque::new();
            map.get(&p).unwrap().iter().for_each(|e| q.push_back(*e));
            while let Some(node) = q.pop_front() {
                if visited[node as usize] {
                    return false
                }
                visited[node as usize] = true;
                map.get(&node).unwrap().iter().for_each(|e| q.push_back(*e));
            }
        }
        
        // let mut queue: VecDeque<Vec<i32>> = VecDeque::new();
        // edges.into_iter().for_each(|e| queue.push_back(e));

        println!("tip: {:?}", tip);
        println!("map: {:?}", map);

        !visited.into_iter().find(|x| !x).is_some()
    }
}