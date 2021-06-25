// https://leetcode.com/submissions/detail/512814289/
use std::collections::{VecDeque, HashMap};

fn vec2hash(edges: Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    edges.into_iter().for_each(|edge| {
        map.entry(edge[0]).and_modify(|c| c.push(edge[1])).or_insert(vec![edge[1]]);
    });
    map
}

fn dig(map: &HashMap<i32, Vec<i32>>, mut visited: &mut Vec<bool>, mut stack: &mut Vec<i32>, node: i32, destination: i32) -> bool {
    if let Some(next_nodes) = map.get(&node) {
        if stack.contains(&node) { return false }
        stack.push(node);
        for node in next_nodes.into_iter() {
            let result = dig(&map, &mut visited, &mut stack, *node, destination);
            stack.pop();
            if !result {
                return false
            }
        }
        visited[node as usize] = true;
        true
    } else if node == destination {
        visited[node as usize] = true;
        true
    } else {
        false
    }
}

impl Solution {
    pub fn leads_to_destination(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut visited: Vec<bool> = vec![false; n as usize];
        let mut map = vec2hash(edges);
        let mut stack: Vec<i32> = Vec::new();

        // println!("{:?}", map);
        let result = dig(&map, &mut visited, &mut stack, source, destination);
        // println!("{}, {:?}", result, visited);
        dig(&map, &mut visited, &mut stack, source, destination) && visited.into_iter().find(|f| *f).is_some()
    }
}